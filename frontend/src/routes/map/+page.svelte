<script lang="ts">
  import MapView from '$lib/components/MapView.svelte';
  import { slide } from 'svelte/transition';
  let { data } = $props<{ data: any }>();

  // Derive location event counts from central API graph data
  const locationEventMap = $derived.by(() => {
    const map = new Map();
    const edges = data.fullGraphData?.edges || [];
    const nodes = data.fullGraphData?.nodes || [];
    
    edges.forEach((e: any) => {
      let locId = null;
      let eventId = null;

      const sourceNode = nodes.find((n: any) => n.data.id === e.data.source);
      const targetNode = nodes.find((n: any) => n.data.id === e.data.target);

      if (sourceNode?.data.type === 'location' && targetNode?.data.type === 'event') {
        locId = sourceNode.data.id;
        eventId = targetNode.data.id;
      } else if (targetNode?.data.type === 'location' && sourceNode?.data.type === 'event') {
        locId = targetNode.data.id;
        eventId = sourceNode.data.id;
      }

      if (locId && eventId) {
        const evNode = nodes.find((n: any) => n.data.id === eventId);
        if (evNode) {
          if (!map.has(locId)) map.set(locId, []);
          map.get(locId).push(evNode);
        }
      }
    });

    // Also map by location name since the tree matches by name
    const nameMap = new Map();
    map.forEach((events, locId) => {
      const locNode = nodes.find((n: any) => n.data.id === locId);
      if (locNode) nameMap.set(locNode.data.label, events);
    });
    
    return nameMap;
  });

  let mapRef: MapView;

  let formattedLocations = $derived(
    (data.locationsData || [])
      .filter((l: any) => l.lat && l.lng)
      .map((l: any) => ({
        name: l.name,
        lat: l.lat,
        lng: l.lng,
        type: l.curationTier?.toLowerCase() === 'canonical' ? 'Lokasi Utama' : 'Lokasi Sejarah',
      }))
  );

  const locationHierarchyTemplate = [
    {
      region: "🌍 Jazirah Arab (Arabia)",
      provinces: [
        {
          name: "📍 Wilayah Hijaz",
          locations: ["Makkah", "Madinah", "Yatrib", "Tha'if", "Jeddah", "Badr", "Uhud"]
        },
        {
          name: "📍 Wilayah Najd & Utara",
          locations: ["Yamamah", "Khaibar", "Fadak", "Tabuk", "Madyan"]
        },
        {
          name: "📍 Yaman & Selatan Arabia",
          locations: ["Sana'a", "Najran", "Aden", "Hadramaut"]
        }
      ]
    },
    {
      region: "🌍 Syam & Anatolia (Levant & Asia Minor)",
      provinces: [
        {
          name: "📍 Jund Filastin (Palestina)",
          locations: ["Baitul Maqdis", "Yerusalem", "Aelia", "Gaza", "Mu'tah", "Hebron", "Jericho"]
        },
        {
          name: "📍 Jund Dimashq (Suriah & Lebanon)",
          locations: ["Damaskus", "Yarmuk", "Homs", "Aleppo", "Antiokhia"]
        },
        {
          name: "📍 Anatolia (Kekaisaran Romawi Timur)",
          locations: ["Konstantinopel", "Nicaea", "Edessa", "Manzikert"]
        }
      ]
    },
    {
      region: "🌍 Mesopotamia & Persia Raya",
      provinces: [
        {
          name: "📍 As-Sawad (Irak Kuno)",
          locations: ["Kufah", "Basrah", "Qadisiyyah", "Karbala", "Baghdad", "Samarra"]
        },
        {
          name: "📍 Farsi & Pegunungan Zagros",
          locations: ["Al-Mada'in", "Ctesiphon", "Nihawand", "Isfahan", "Shiraz"]
        },
        {
          name: "📍 Khurasan Raya",
          locations: ["Nishapur", "Merv", "Balkh", "Herat"]
        }
      ]
    },
    {
      region: "🌍 Afrika Utara & Andalusia",
      provinces: [
        {
          name: "📍 Mesir & Lembah Nil",
          locations: ["Fustat", "Alexandria", "Kairo", "Aswan"]
        },
        {
          name: "📍 Al-Maghrib (Afrika Utara Barat)",
          locations: ["Kairouan", "Fes", "Marrakesh", "Carthage"]
        },
        {
          name: "📍 Andalusia (Semenanjung Iberia)",
          locations: ["Cordoba", "Granada", "Toledo", "Seville"]
        }
      ]
    },
    {
      region: "🌍 Asia Tengah & Jalur Sutra",
      provinces: [
        {
          name: "📍 Transoxiana (Ma Wara' an-Nahr)",
          locations: ["Bukhara", "Samarkand", "Tashkent", "Khwarazm"]
        }
      ]
    },
    {
      region: "🌍 Asia Selatan & Timur",
      provinces: [
        {
          name: "📍 Anak Benua India",
          locations: ["Sind", "Delhi", "Lahore", "Agra"]
        },
        {
          name: "📍 Tiongkok Kuno",
          locations: ["Chang'an", "Guangzhou", "Xinjiang"]
        }
      ]
    },
    {
      region: "🌍 Asia Tenggara & Nusantara",
      provinces: [
        {
          name: "📍 Jalur Maritim Nusantara",
          locations: ["Samudera Pasai", "Malaka", "Demak", "Banten", "Ternate", "Tidore", "Gowa"]
        }
      ]
    }
  ];

  function buildTree(locs: any[]) {
    let tree = locationHierarchyTemplate.map(reg => ({
      region: reg.region,
      provinces: reg.provinces.map(prov => ({
        name: prov.name,
        matchers: prov.locations,
        items: [] as any[]
      }))
    }));

    let unclassified = {
      region: "🌍 Wilayah Geopolitik Lainnya",
      provinces: [
        { name: "📍 Titik Tidak Terklasifikasi", matchers: [], items: [] as any[] }
      ]
    };

    locs.forEach(loc => {
      let matched = false;
      for (const reg of tree) {
        for (const prov of reg.provinces) {
          if (prov.matchers.some(m => loc.name.toLowerCase().includes(m.toLowerCase()))) {
            prov.items.push(loc);
            matched = true;
            break;
          }
        }
        if (matched) break;
      }
      if (!matched) {
        unclassified.provinces[0].items.push(loc);
      }
    });

    const finalTree = tree
      .map(reg => ({
        ...reg,
        provinces: reg.provinces.filter(p => p.items.length > 0)
      }))
      .filter(reg => reg.provinces.length > 0);

    if (unclassified.provinces[0].items.length > 0) {
      finalTree.push(unclassified);
    }

    return finalTree;
  }

  let treeData = $derived(buildTree(formattedLocations));

  let expandedRegions = $state<Record<string, boolean>>({});
  let initialized = false;

  $effect(() => {
    if (treeData.length > 0 && !initialized) {
      expandedRegions[treeData[0].region] = true;
      initialized = true;
    }
  });

  function toggleRegion(region: string) {
    expandedRegions[region] = !expandedRegions[region];
  }

  function handleLocClick(loc: { name: string; lat: number; lng: number; type: string }) {
    mapRef?.flyTo(loc.lng, loc.lat);
  }
</script>

<div class="w-full h-[calc(100vh-80px)] flex gap-6 animate-fade-in">
  <!-- Left Panel: Historical Locations list -->
  <div class="w-80 flex flex-col gap-5 flex-shrink-0 h-full">
    <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col gap-4">
      <h2 class="text-base font-extrabold text-gold-400">Peta Geografis Peradaban</h2>
      <p class="text-xs text-text-secondary leading-relaxed">
        Visualisasi 3 Dimensi geospasial lokasi-lokasi penting sejarah Islam dan keterhubungannya dengan konseptual transendental.
      </p>
    </div>

    <!-- Location Tree -->
    <div class="glass p-5 rounded-2xl border border-border/10 flex-1 overflow-y-auto flex flex-col gap-3">
      <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider mb-2">Hierarki Geopolitik Klasik</h3>
      
      <div class="flex flex-col gap-5">
        {#each treeData as region}
          <div class="flex flex-col gap-2">
            <button 
              class="w-full flex items-center justify-between border-b border-border/10 pb-1.5 cursor-pointer hover:border-gold-500/50 transition-colors group"
              onclick={() => toggleRegion(region.region)}
            >
              <h4 class="text-xs font-extrabold text-gold-400 group-hover:text-gold-300 transition-colors">{region.region}</h4>
              <span class="text-[10px] text-text-muted transition-transform duration-200" style={expandedRegions[region.region] ? 'transform: rotate(180deg);' : ''}>▼</span>
            </button>
            
            {#if expandedRegions[region.region]}
              <div class="pl-2 flex flex-col gap-3 mt-1" transition:slide={{ duration: 250 }}>
                {#each region.provinces as prov}
                  <div class="flex flex-col gap-1.5">
                    <span class="text-[10px] font-bold text-emerald-400 uppercase tracking-wider">{prov.name}</span>
                    
                    <div class="pl-4 flex flex-col gap-1 border-l border-border/10 ml-1.5">
                      {#each prov.items as loc}
                        {@const locEvents = locationEventMap.get(loc.name)}
                        <button
                          onclick={() => handleLocClick(loc)}
                          class="px-3 py-2 rounded-lg bg-navy-950/40 hover:bg-navy-900/80 border border-transparent hover:border-gold-500/20 transition-all cursor-pointer text-left w-full group flex flex-col"
                        >
                          <span class="text-[11px] font-medium text-text-primary group-hover:text-gold-400 transition-colors flex items-center gap-2">
                            <span class="w-1 h-1 bg-gold-500 rounded-full shadow-[0_0_4px_rgba(212,168,83,0.8)]"></span>
                            {loc.name}
                            {#if locEvents && locEvents.length > 0}
                              <span class="text-[8px] font-bold text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-1.5 py-0 rounded ml-auto">{locEvents.length}</span>
                            {/if}
                          </span>
                          {#if loc.type}
                            <span class="text-[9px] text-text-muted mt-0.5 ml-3">{loc.type}</span>
                          {/if}
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {:else}
          <div class="p-3 rounded-xl bg-navy-950/60 border border-border/10 text-center">
            <p class="text-xs text-text-muted">Gagal memuat struktur wilayah geopolitik.</p>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Map container -->
  <div class="flex-1 h-full glass rounded-2xl border border-border/10 overflow-hidden p-4">
    <MapView bind:this={mapRef} locations={formattedLocations.length > 0 ? formattedLocations : undefined} />
  </div>
</div>
