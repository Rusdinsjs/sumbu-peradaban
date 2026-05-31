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

    const currentName = decodeURIComponent($page.params.name).toLowerCase();
    
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
      name: decodeURIComponent($page.params.name) || 'Lokasi Sejarah',
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

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-5xl mx-auto">
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
    <!-- Header Card (CV Lokasi Style) -->
    <div class="glass p-8 rounded-3xl border border-gold-500/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-12 -left-12 w-48 h-48 bg-gold-500/10 rounded-full blur-3xl"></div>
      
      <div class="flex items-start sm:items-center gap-5 relative z-10">
        <div class="w-20 h-20 rounded-2xl bg-gold-500/10 border border-gold-500/25 flex items-center justify-center text-4xl shadow-[0_0_20px_rgba(212,168,83,0.15)] flex-shrink-0">
          📍
        </div>
        
        <div class="flex flex-col gap-1">
          <div class="flex flex-wrap items-center gap-2">
            <span class="px-2 py-0.5 bg-gold-500/15 text-gold-400 border border-gold-500/20 text-[9px] font-extrabold uppercase rounded tracking-wider">
              {loc.type}
            </span>
            <CurationBadge tier={loc.curationTier as any} size="sm" />
          </div>
          
          <h1 class="text-xl sm:text-2xl font-black text-text-primary leading-tight mt-1">
            {loc.name}
          </h1>
          
          <p class="text-xs text-text-secondary font-medium mt-0.5">
            Wilayah/Kawasan: <span class="text-gold-400 font-bold">{loc.ancientName}</span> — 
            Koordinat: <span class="text-emerald-400 font-mono font-bold">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</span>
          </p>
        </div>
      </div>

      <!-- Strategic Value Badge -->
      <div class="px-5 py-3 rounded-2xl bg-navy-950/60 border border-border/10 flex flex-row md:flex-col items-center gap-3 md:gap-1 text-center min-w-[130px] self-stretch md:self-auto relative z-10 justify-between sm:justify-center">
        <span class="text-2xl">🏆</span>
        <div class="flex flex-col md:items-center">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-wider">Signifikansi</span>
          <span class="text-xs font-black text-gold-400 font-mono">Poros Geopolitik</span>
        </div>
      </div>
    </div>

    <!-- Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Geography & History Timeline (Grid 1 & 2) -->
      <div class="lg:col-span-2 flex flex-col gap-6">
        
        <!-- Short Description / Geography Climate -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
            <span>🗺️</span> Karakteristik Geografis & Geopolitik Lokasi
          </h2>
          <p class="text-xs text-text-secondary leading-relaxed whitespace-pre-wrap font-normal">
            {loc.description}
          </p>
          <div class="p-4 rounded-2xl bg-navy-950/60 border border-border/10 mt-2">
            <span class="text-[10px] uppercase font-bold text-gold-400 block mb-1.5">Nilai Strategis Militer & Ekonomi:</span>
            <p class="text-xs text-text-muted leading-relaxed font-normal">{loc.strategicValue}</p>
          </div>
        </div>

        <!-- Demographics -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-3">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>👥</span> Demografi & Kependudukan Historis
          </h2>
          {#if loc.demographics}
            <p class="text-xs text-text-secondary leading-relaxed whitespace-pre-wrap font-normal">
              {loc.demographics}
            </p>
          {:else}
            <p class="text-xs text-text-muted italic leading-relaxed font-normal">
              Belum ada data demografi dan kependudukan historis yang terdaftar untuk lokasi ini.
            </p>
          {/if}
        </div>

        <!-- Socio-Cultural -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-3">
          <h2 class="text-xs font-bold text-purple-400 uppercase tracking-wider flex items-center gap-2">
            <span>🎭</span> Kehidupan Sosial & Kebudayaan
          </h2>
          {#if loc.socioCultural}
            <p class="text-xs text-text-secondary leading-relaxed whitespace-pre-wrap font-normal">
              {loc.socioCultural}
            </p>
          {:else}
            <p class="text-xs text-text-muted italic leading-relaxed font-normal">
              Belum ada data kehidupan sosial, tradisi, dan kebudayaan yang terdaftar untuk lokasi ini.
            </p>
          {/if}
        </div>

        <!-- Life Milestones Timeline -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-6">
          <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
            <span>📅</span> Peristiwa Bersejarah & Kronologi Tempat
          </h2>

          {#if loc.timeline && loc.timeline.length > 0}
            <div class="relative border-l border-border/10 pl-6 ml-3 space-y-6">
              {#each loc.timeline as ev}
                <div class="relative group">
                  <!-- Timeline node dot -->
                  <span class="absolute -left-[31px] top-1.5 w-3 h-3 bg-amber-500 rounded-full border-2 border-surface shadow-[0_0_6px_rgba(245,158,11,0.8)] group-hover:scale-110 transition-transform"></span>
                  
                  <div>
                    <span class="text-[9px] font-bold text-gold-400 bg-gold-500/10 px-2 py-0.5 rounded font-mono">
                      {ev.gregorianDate.year} M
                    </span>
                    <a href="/event/{ev.uuid}" class="block text-xs font-bold text-text-primary hover:text-gold-400 mt-2 transition-colors">
                      {ev.title}
                    </a>
                    {#if ev.description}
                      <p class="text-[11px] text-text-muted mt-1 leading-relaxed line-clamp-2">{ev.description}</p>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-xs text-text-muted italic">Belum ada peristiwa terdaftar di lokasi ini.</p>
          {/if}
        </div>
      </div>

      <!-- Right Column: Interactive Map & Related Local Events (Grid 3) -->
      <div class="flex flex-col gap-6">
        
        <!-- 📍 INTERACTIVE MINI MAP WIDGET -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <div class="flex justify-between items-center">
            <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
              <span>🌍</span> Geospasial Interaktif
            </h2>
            <button 
              onclick={() => isMapModalOpen = true}
              class="text-[10px] bg-gold-500/10 hover:bg-gold-500/20 border border-gold-500/30 text-gold-400 px-2.5 py-1 rounded-lg font-bold transition-all flex items-center gap-1 cursor-pointer"
            >
              <span>⛶</span> Perbesar
            </button>
          </div>
          
          <!-- Clickable map container -->
          <button 
            onclick={() => isMapModalOpen = true}
            class="w-full h-[220px] rounded-2xl overflow-hidden border border-border/15 relative z-0 shadow-inner group/map text-left block cursor-zoom-in"
          >
            <!-- Hover effect visual overlay -->
            <div class="absolute inset-0 bg-navy-950/0 group-hover/map:bg-navy-950/20 transition-colors z-10 flex items-center justify-center pointer-events-none">
              <span class="bg-navy-900/90 text-gold-400 border border-gold-500/30 text-[10px] font-bold px-3 py-1.5 rounded-lg opacity-0 group-hover/map:opacity-100 transition-opacity shadow-lg">
                🔍 Perbesar Peta
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
          
          <div class="flex justify-between items-center text-[10px] text-text-muted border-t border-border/10 pt-3">
            <span>Koordinat: <strong class="text-emerald-400 font-mono">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</strong></span>
            <a href="/map" class="text-gold-500 hover:text-gold-400 font-bold transition-colors">
              Peta Utama →
            </a>
          </div>
        </div>

        <!-- Visited / Related Actors -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
            <span>👥</span> Tokoh yang Pernah Berkunjung
          </h2>

          {#if loc.actors && loc.actors.length > 0}
            <div class="flex flex-col gap-2">
              {#each loc.actors as rel}
                <a href="/actor/{rel.actor.uuid}" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-gold-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-base">👤</span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="text-[11px] font-bold text-text-primary group-hover:text-gold-400 transition-colors truncate">
                        {rel.actor.name}
                      </span>
                      <span class="text-[9px] text-emerald-400 font-bold uppercase tracking-wider mt-0.5">
                        {rel.relationshipType}
                      </span>
                    </div>
                  </div>
                  <span class="text-[10px] text-gold-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada catatan tokoh terdaftar berkunjung.</p>
          {/if}
        </div>

        <!-- Historical Sources Checklist -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
            <span>📚</span> Kitab Rujukan Sejarah
          </h2>

          {#if loc.sources && loc.sources.length > 0}
            <div class="flex flex-col gap-3">
              {#each loc.sources as rel}
                <a href="/source/{rel.source.sourceId}" class="group block p-3.5 bg-navy-950/60 hover:bg-navy-900 border border-border/5 rounded-2xl transition-all">
                  <div class="flex justify-between items-start gap-2">
                    <span class="text-[11px] font-bold text-text-primary group-hover:text-gold-400 transition-colors line-clamp-1">{rel.source.title || 'Manuskrip Sejarah'}</span>
                    {#if rel.source.reliabilityScore !== null}
                      <span class="text-[9px] font-bold text-emerald-400 font-mono">{(rel.source.reliabilityScore * 100).toFixed(0)}%</span>
                    {/if}
                  </div>
                  <div class="flex flex-col gap-0.5 mt-1">
                    <span class="text-[9px] text-text-muted">{rel.source.author || 'Penyusun Anonim'}</span>
                    <span class="text-[9px] text-emerald-400 font-semibold uppercase tracking-wider mt-0.5">{rel.relationshipType}</span>
                  </div>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Belum ada rujukan manuskrip sejarah yang memuat koordinat ini.</p>
          {/if}
        </div>

        <!-- 📁 Media Links Gallery -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-gold-400 uppercase tracking-wider flex items-center gap-2">
            <span>🎥</span> Berkas Media & Scan Naskah
          </h2>

          {#if loc.mediaLinks && loc.mediaLinks.length > 0}
            <div class="flex flex-col gap-2">
              {#each loc.mediaLinks as media}
                <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-gold-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-lg">
                      {#if media.mediaType === 'image'}
                        🖼️
                      {:else if media.mediaType === 'audio'}
                        🎙️
                      {:else}
                        📄
                      {/if}
                    </span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="text-[11px] font-bold text-text-primary group-hover:text-gold-400 transition-colors truncate">
                        {media.title || 'Lihat Berkas'}
                      </span>
                      <span class="text-[9px] text-text-muted uppercase tracking-wider mt-0.5">
                        {media.mediaType}
                      </span>
                    </div>
                  </div>
                  <span class="text-[10px] text-gold-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada berkas media gambar/audio pendukung yang dilampirkan.</p>
          {/if}
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
    class="fixed inset-0 z-50 flex items-center justify-center bg-navy-950/85 backdrop-blur-md animate-fade-in p-4"
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
          class="w-7 h-7 rounded-lg bg-navy-900 border border-border/10 hover:border-gold-500/40 text-text-muted hover:text-gold-400 flex items-center justify-center font-bold transition-all text-xs cursor-pointer"
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
        <span>Koordinat Akurat: <span class="text-emerald-400 font-mono font-bold">{loc.lat.toFixed(6)}° N, {loc.lng.toFixed(6)}° E</span></span>
      </div>
    </div>
  </div>
{/if}
