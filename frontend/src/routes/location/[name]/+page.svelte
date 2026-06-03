<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';
  import MapView from '$lib/components/MapView.svelte';
  import { slide } from 'svelte/transition';

  import { page } from '$app/stores';

  let { data } = $props<{ data: { location: any } }>();
  let dbLoc = $derived(data.location);

  // Fallback dictionary for canonical sites if dbLoc isn't in graph yet
  const locationRegistryFallback: Record<string, {
    name: string;
    ancientName: string;
    type: string;
    curationTier: 'canonical' | 'reviewed' | 'verified' | 'draft';
    lat: number;
    lng: number;
    strategicValue: string;
    description: string;
  }> = {
    'makkah': {
      name: 'Makkah Al-Mukarramah',
      ancientName: 'Bakkah / Macoraba',
      type: 'Suaka Haram & Hub Perdagangan Kosmopolitan',
      curationTier: 'canonical',
      lat: 21.4225,
      lng: 39.8262,
      strategicValue: 'Jantung perlintasan dagang Hijaz selatan, pusat ziarah religius lintas klan semenanjung Arab sejak zaman Nabi Ibrahim AS.',
      description: 'Kota kelahiran Nabi Muhammad ﷺ, kiblat ibadah umat Islam sedunia, dan episentrum awal dimulainya risalah ketauhidan.'
    },
    'madinah': {
      name: 'Madinah Al-Munawwarah',
      ancientName: 'Yatsrib',
      type: 'Pusat Sanitasi & Titik Oasis Hijau',
      curationTier: 'canonical',
      lat: 24.4672,
      lng: 39.6112,
      strategicValue: 'Titik perlintasan rute karavan dagang Hijaz utara dan pusat agrikultur subur berpohon kurma.',
      description: 'Kota suci yang menjadi tempat hijrah Nabi Muhammad ﷺ dan pusat berdirinya institusi politik daulah peradaban Islam pertama.'
    },
    'baitul maqdis': {
      name: 'Baitul Maqdis (Al-Quds / Yerusalem)',
      ancientName: 'Jebus / Aelia Capitolina',
      type: 'Tanah Suci & Titik Konvergensi Para Nabi',
      curationTier: 'canonical',
      lat: 31.7780,
      lng: 35.2354,
      strategicValue: 'Pusat keagamaan abadi tiga agama samawi, titik temu perlintasan benua Asia dan Afrika, serta benteng pertahanan spiritual peradaban.',
      description: 'Kota suci yang menjadi tempat suaka para nabi Bani Israel, kiblat pertama umat Islam, serta lokasi peristiwa agung Isra\' Mi\'raj Rasulullah ﷺ.'
    }
  };

  // Resolve location properties with fallback
  const loc = $derived.by(() => {
    if (dbLoc) {
      return {
        uuid: dbLoc.uuid,
        name: dbLoc.name,
        ancientName: dbLoc.region || 'Tidak Tercatat',
        type: dbLoc.locationType || (dbLoc.isTranscendental ? 'Situs Transendental / Gaib' : 'Titik Wilayah Sejarah'),
        curationTier: dbLoc.curationTier?.toLowerCase() || 'draft',
        lat: dbLoc.lat ?? 21.4225,
        lng: dbLoc.lng ?? 39.8262,
        strategicValue: dbLoc.historicalRole || 'Wilayah pertahanan taktis atau perlintasan sejarah penting.',
        description: dbLoc.geographyClimate || 'Tempat bersejarah yang terintegrasi dengan poros penyebaran ajaran dan perluasan sumbu peradaban dunia Islam klasik.',
        demographics: dbLoc.demographics || '',
        socioCultural: dbLoc.socioCultural || '',
        mediaLinks: dbLoc.mediaLinks || [],
        timeline: dbLoc.timeline || [],
        relatedLocations: dbLoc.relatedLocations || [],
        actors: dbLoc.actors || [],
        sources: dbLoc.sources || []
      };
    }

    const currentName = decodeURIComponent($page.params.name || '').toLowerCase();
    
    // Find matching fallback by substring
    let fallback = null;
    for (const [key, value] of Object.entries(locationRegistryFallback)) {
      if (currentName.includes(key)) {
        fallback = value;
        break;
      }
    }

    if (fallback) {
      return {
        ...fallback,
        demographics: '',
        socioCultural: '',
        mediaLinks: [],
        timeline: [],
        relatedLocations: [],
        actors: [],
        sources: []
      };
    }

    // Try finding in fallback registry
    return {
      uuid: '',
      name: decodeURIComponent($page.params.name || '') || 'Lokasi Sejarah',
      ancientName: 'Tidak Tercatat',
      type: 'Titik Wilayah Pengaruh',
      curationTier: 'draft' as const,
      lat: 21.4225,
      lng: 39.8262,
      strategicValue: 'Wilayah pertahanan taktis atau perlintasan sejarah penting.',
      description: 'Tempat bersejarah yang terintegrasi dengan poros penyebaran ajaran dan perluasan sumbu peradaban dunia Islam klasik.',
      demographics: '',
      socioCultural: '',
      mediaLinks: [],
      timeline: [],
      relatedLocations: [],
      actors: [],
      sources: []
    };
  });

  let isMapModalOpen = $state(false);
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back link -->
  <div class="flex justify-between items-center">
    <a href="/map" class="text-xs text-gold-400 hover:text-gold-300 transition-colors font-bold flex items-center gap-1.5 self-start">
      ← Kembali ke Peta Sejarah
    </a>
  </div>

  {#if !dbLoc}
    <div class="glass p-12 rounded-3xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-4xl">📍</span>
      <h2 class="text-lg font-bold text-text-primary">Data Lokasi Tidak Ditemukan</h2>
      <p class="text-xs text-text-muted">Entitas lokasi sejarah tidak terdaftar atau koordinat belum ditetapkan.</p>
      <a href="/map" class="mt-2 px-5 py-2.5 bg-gold-500/10 hover:bg-gold-500/20 text-gold-400 text-xs font-bold rounded-xl border border-gold-500/20 transition-all">
        Buka Peta Sejarah
      </a>
    </div>
  {:else}
    <!-- Document-Style Container -->
    <div class="glass p-8 lg:p-12 rounded-3xl border border-gold-500/10 flex flex-col gap-10 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-24 -left-24 w-96 h-96 bg-gold-500/10 rounded-full blur-3xl pointer-events-none"></div>
      
      <!-- Header Section -->
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-8 relative z-10 border-b border-border/5 pb-8">
        <div class="flex items-start sm:items-center gap-6">
          {#if loc.mediaLinks && loc.mediaLinks.some((m: any) => m.mediaType === 'image')}
            <div class="w-24 h-24 sm:w-32 sm:h-32 rounded-3xl bg-gold-500/10 border border-gold-500/25 overflow-hidden shadow-[0_0_30px_rgba(212,168,83,0.15)] flex-shrink-0 relative">
              <img src={loc.mediaLinks.find((m: any) => m.mediaType === 'image').url} alt={loc.name} class="w-full h-full object-cover" />
            </div>
          {:else}
            <div class="w-24 h-24 sm:w-32 sm:h-32 rounded-3xl bg-gold-500/10 border border-gold-500/25 flex items-center justify-center text-5xl sm:text-6xl shadow-[0_0_30px_rgba(212,168,83,0.15)] flex-shrink-0">
              📍
            </div>
          {/if}
          
          <div class="flex flex-col gap-1.5">
            <div class="flex flex-wrap items-center gap-2">
              <span class="px-2.5 py-0.5 bg-gold-500/15 text-gold-400 border border-gold-500/20 text-[10px] font-extrabold uppercase rounded tracking-widest">
                {loc.type}
              </span>
              <CurationBadge tier={loc.curationTier as any} size="sm" />
            </div>
            
            <h1 class="text-2xl sm:text-4xl font-black text-text-primary leading-tight mt-1 tracking-tight">
              {loc.name}
            </h1>
            
            <p class="text-xs sm:text-sm text-text-secondary font-medium mt-1">
              Wilayah/Kawasan: <span class="text-gold-400 font-bold">{loc.ancientName}</span> — 
              Koordinat: <span class="text-verdigris-400 font-mono font-bold">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</span>
            </p>
          </div>
        </div>

        <!-- Strategic Value Minimalist Info -->
        <div class="flex flex-col items-start md:items-end gap-1">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-widest">Signifikansi</span>
          <span class="text-lg sm:text-2xl font-black text-gold-400 font-mono flex items-center gap-2">
            <span>🏆</span> Poros Geopolitik
          </span>
        </div>
      </div>

      <!-- Seamless Content Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 relative z-10">
        
        <!-- Main Column: Geography, Demographics, Timeline -->
        <div class="lg:col-span-2 flex flex-col gap-12">
          
          <!-- Karakteristik Geografis -->
          <div class="flex flex-col gap-4 relative">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">🗺️</span> Karakteristik Geografis & Geopolitik
            </h2>
            <p class="text-[13px] text-text-secondary leading-loose whitespace-pre-wrap font-normal pl-4 border-l-2 border-gold-500/20">
              {loc.description}
            </p>
            <div class="p-4 rounded-xl bg-iron-950/40 border-l border-border/10 mt-2">
              <span class="text-[10px] uppercase font-bold text-gold-400 block mb-1.5 tracking-widest">Nilai Strategis Militer & Ekonomi:</span>
              <p class="text-[12px] text-text-muted leading-relaxed font-normal">{loc.strategicValue}</p>
            </div>
          </div>

          <!-- Demographics -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-verdigris-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">👥</span> Demografi & Kependudukan
            </h2>
            {#if loc.demographics}
              <p class="text-[13px] text-text-secondary leading-loose whitespace-pre-wrap font-normal">
                {loc.demographics}
              </p>
            {:else}
              <p class="text-[12px] text-text-muted italic leading-relaxed font-normal">
                Belum ada data demografi.
              </p>
            {/if}
          </div>

          <!-- Socio-Cultural -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-purple-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">🎭</span> Kehidupan Sosial & Kebudayaan
            </h2>
            {#if loc.socioCultural}
              <p class="text-[13px] text-text-secondary leading-loose whitespace-pre-wrap font-normal">
                {loc.socioCultural}
              </p>
            {:else}
              <p class="text-[12px] text-text-muted italic leading-relaxed font-normal">
                Belum ada data kehidupan sosial.
              </p>
            {/if}
          </div>

          <!-- Life Milestones Timeline -->
          <div class="flex flex-col gap-6 mt-4">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">📅</span> Peristiwa Bersejarah & Kronologi
            </h2>

            {#if loc.timeline && loc.timeline.length > 0}
              <div class="relative border-l border-border/10 pl-6 ml-3 space-y-8 mt-2">
                {#each loc.timeline as ev}
                  <div class="relative group">
                    <span class="absolute -left-[31px] top-1 w-2.5 h-2.5 bg-amber-500 rounded-full border-2 border-iron-950 shadow-[0_0_8px_rgba(245,158,11,0.6)] group-hover:scale-125 transition-transform"></span>
                    
                    <div class="flex flex-col">
                      <span class="text-[10px] font-bold text-gold-400 font-mono tracking-widest">
                        {ev.gregorianDate.year} M
                      </span>
                      <a href="/event/{ev.uuid}" class="block text-sm font-bold text-text-primary hover:text-gold-400 mt-1 transition-colors">
                        {ev.title}
                      </a>
                      {#if ev.description}
                        <p class="text-[12px] text-text-muted mt-2 leading-relaxed line-clamp-2">{ev.description}</p>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-[12px] text-text-muted italic">Belum ada peristiwa terdaftar di lokasi ini.</p>
            {/if}
          </div>
        </div>

        <!-- Right Column: Interactive Map & Related Local Events -->
        <div class="flex flex-col gap-10 border-t lg:border-t-0 lg:border-l border-border/5 pt-8 lg:pt-0 lg:pl-10">
          
          <!-- 📍 INTERACTIVE MINI MAP WIDGET -->
          <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center">
              <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
                <span>🌍</span> Geospasial
              </h2>
              <button 
                onclick={() => isMapModalOpen = true}
                class="text-[10px] hover:bg-gold-500/10 text-gold-400 px-2 py-1 rounded transition-all flex items-center gap-1 cursor-pointer tracking-widest font-bold"
              >
                <span>⛶</span> PERBESAR
              </button>
            </div>
            
            <!-- Clickable map container -->
            <button 
              onclick={() => isMapModalOpen = true}
              class="w-full h-[220px] rounded-xl overflow-hidden border border-border/10 relative z-0 shadow-inner group/map text-left block cursor-zoom-in"
            >
              <div class="absolute inset-0 bg-iron-950/0 group-hover/map:bg-iron-950/20 transition-colors z-10 flex items-center justify-center pointer-events-none">
                <span class="bg-iron-900/90 text-gold-400 border border-gold-500/30 text-[10px] font-bold px-3 py-1.5 rounded-lg opacity-0 group-hover/map:opacity-100 transition-opacity shadow-lg tracking-widest uppercase">
                  Perbesar Peta
                </span>
              </div>

              <MapView 
                locations={[{
                  uuid: loc.uuid,
                  name: loc.name,
                  lat: loc.lat,
                  lng: loc.lng,
                  type: loc.type,
                  description: loc.description
                }]}
                center={[loc.lng, loc.lat]}
                zoom={7}
              />
            </button>
            
            <div class="flex justify-between items-center text-[10px] text-text-muted">
              <span>Koordinat: <strong class="text-verdigris-400 font-mono">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</strong></span>
              <a href="/map" class="text-gold-500 hover:text-gold-400 font-bold transition-colors">
                Peta Utama →
              </a>
            </div>
          </div>

          <!-- Visited / Related Actors -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span>👥</span> Tokoh Terkait
            </h2>

            {#if loc.actors && loc.actors.length > 0}
              <div class="flex flex-col gap-3">
                {#each loc.actors as rel}
                  <a href="/actor/{rel.actor.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors">👤</div>
                    <div class="flex flex-col">
                      <span class="text-[12px] font-bold text-text-primary group-hover:text-gold-400 transition-colors">{rel.actor.name}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada tokoh terdaftar berkunjung.</p>
            {/if}
          </div>

          <!-- Historical Sources Checklist -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span>📚</span> Rujukan Sejarah
            </h2>

            {#if loc.sources && loc.sources.length > 0}
              <div class="flex flex-col gap-3">
                {#each loc.sources as rel}
                  <a href="/source/{rel.source.sourceId}" class="group block p-3 bg-iron-950/40 hover:bg-iron-900 border border-border/5 rounded-xl transition-all">
                    <div class="flex justify-between items-start gap-2">
                      <span class="text-[12px] font-bold text-text-primary group-hover:text-gold-400 transition-colors line-clamp-1">{rel.source.title || 'Manuskrip Sejarah'}</span>
                      {#if rel.source.reliabilityScore !== null}
                        <span class="text-[10px] font-bold text-verdigris-400 font-mono">{(rel.source.reliabilityScore * 100).toFixed(0)}%</span>
                      {/if}
                    </div>
                    <div class="flex flex-col gap-0.5 mt-1">
                      <span class="text-[10px] text-text-muted">{rel.source.author || 'Penyusun Anonim'}</span>
                      <span class="text-[9px] text-verdigris-400 font-semibold uppercase tracking-widest mt-0.5">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Belum ada rujukan sejarah yang memuat koordinat ini.</p>
            {/if}
          </div>

          <!-- 📁 Media Links Gallery -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span>🎥</span> Berkas & Scan
            </h2>

            {#if loc.mediaLinks && loc.mediaLinks.length > 0}
              <div class="flex flex-col gap-3">
                {#each loc.mediaLinks as media}
                  <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors">
                      {media.mediaType === 'image' ? '🖼️' : media.mediaType === 'audio' ? '🎙️' : '📄'}
                    </div>
                    <div class="flex flex-col">
                      <span class="text-[12px] font-bold text-text-primary group-hover:text-gold-400 transition-colors truncate max-w-[150px]">{media.title || 'Lihat Berkas'}</span>
                      <span class="text-[9px] text-text-muted uppercase tracking-widest">{media.mediaType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada berkas media dilampirkan.</p>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- 🌐 LARGE GEOSPATIAL MAP MODAL (80% viewport size) -->
{#if isMapModalOpen && dbLoc}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-iron-950/85 backdrop-blur-md animate-fade-in p-4"
    onclick={() => isMapModalOpen = false}
  >
    <div 
      class="glass w-[90vw] h-[85vh] max-w-6xl rounded-2xl border border-gold-500/25 overflow-hidden flex flex-col p-4 shadow-[0_0_50px_rgba(212,168,83,0.15)]"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Modal Header -->
      <div class="flex justify-between items-center mb-3 pb-2 border-b border-border/10">
        <div>
          <h3 class="text-sm font-extrabold text-gold-400 flex items-center gap-1.5">
            <span>🗺️</span> Peta Detail: {loc.name}
          </h3>
          <p class="text-[9px] text-text-muted mt-0.5">Navigasi geospasial klasik interaktif — Geser dan gulir untuk menjelajahi area.</p>
        </div>
        <button 
          class="w-7 h-7 rounded-lg bg-iron-900 border border-border/10 hover:border-gold-500/40 text-text-muted hover:text-gold-400 flex items-center justify-center font-bold transition-all text-xs cursor-pointer"
          onclick={() => isMapModalOpen = false}
        >
          ✕
        </button>
      </div>
      
      <!-- Larger Map Container -->
      <div class="flex-1 rounded-xl overflow-hidden border border-border/15 relative z-0">
        <MapView 
          locations={[{
            name: loc.name,
            lat: loc.lat,
            lng: loc.lng,
            type: loc.type,
            description: loc.description
          }]}
          center={[loc.lng, loc.lat]}
          zoom={9}
        />
      </div>
      
      <!-- Modal Footer Info -->
      <div class="flex justify-between items-center mt-3 text-[9px] text-text-muted">
        <span>Klasifikasi: <span class="text-amber-400 font-bold">{loc.type}</span></span>
        <span>Koordinat Akurat: <span class="text-verdigris-400 font-mono font-bold">{loc.lat.toFixed(6)}° N, {loc.lng.toFixed(6)}° E</span></span>
      </div>
    </div>
  </div>
{/if}
