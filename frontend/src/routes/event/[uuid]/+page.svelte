<script lang="ts">
  import { page } from '$app/state';
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: any }>();

  const uuid = $derived(page.params.uuid);
  const dbEvent = $derived(data.event || {
    title: 'Peristiwa Tidak Ditemukan',
    description: 'Data peristiwa tidak tersedia di database.',
    curationTier: 'draft',
    islamicDate: { year: '?' },
    gregorianDate: { year: '?' },
    precision: 'Unknown'
  });

  const event = $derived.by(() => {
    const nodes = data.fullGraphData?.nodes || [];
    const edges = data.fullGraphData?.edges || [];
    
    let actors: any[] = [];
    let locations: any[] = [];
    let sources: any[] = [];
    
    // Find all edges connected to this event UUID
    edges.forEach((e: any) => {
      let relatedNodeId = null;
      let relationship = e.data.relationship || 'Terkait';

      if (e.data.target === uuid) {
        relatedNodeId = e.data.source;
      } else if (e.data.source === uuid) {
        relatedNodeId = e.data.target;
      }

      if (relatedNodeId) {
        const relatedNode = nodes.find((n: any) => n.data.id === relatedNodeId);
        if (relatedNode) {
          if (relatedNode.data.type === 'actor') {
            actors.push({ name: relatedNode.data.label, role: relationship });
          } else if (relatedNode.data.type === 'location') {
            // Note: locations in graph typically don't have lat/lng directly in the basic node data, we'll fake it for display or remove it
            locations.push({ name: relatedNode.data.label, type: relatedNode.data.tier || 'Titik Kritis', lat: 21.0, lng: 39.0 });
          } else if (relatedNode.data.type === 'source') {
            sources.push({ 
              id: relatedNode.data.id, 
              domain: relatedNode.data.tier || 'Verified', 
              text: relatedNode.data.label, 
              score: 0.95 
            });
          }
        }
      }
    });

    return {
      title: dbEvent.title,
      hijriYear: `${dbEvent.islamicDate?.year || '?'} H`,
      gregorianYear: `${dbEvent.gregorianDate?.year || '?'} M`,
      description: dbEvent.description,
      tier: dbEvent.curationTier,
      precision: dbEvent.precision || 'Exact',
      globalHook: {
        is_connected_to_global: true,
        global_pivot_category: 'Pivot Dinamika Sosial & Politik'
      },
      actors,
      locations,
      sources
    };
  });
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back Link -->
  <a href="/timeline" class="text-xs text-gold-500 hover:text-gold-400 transition-colors font-bold flex items-center gap-1">
    ← Kembali ke Timeline
  </a>

  <!-- Header Card -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
    <div>
      <div class="flex items-center gap-3">
        <span class="text-[10px] uppercase font-bold tracking-wider text-gold-500">Peristiwa Utama</span>
        <CurationBadge tier={event.tier as any} size="sm" />
      </div>
      <h1 class="text-xl md:text-2xl font-extrabold text-text-primary mt-1">{event.title}</h1>
    </div>

    <!-- Dual Date Badges -->
    <a href="/time/1h" class="flex gap-2 hover:scale-[1.02] transition-transform">
      <div class="px-4 py-2 rounded-xl bg-gold-500/10 border border-gold-500/20 text-center min-w-[70px]">
        <div class="text-[10px] text-gold-500 font-bold">HIJRIAH</div>
        <div class="text-sm font-extrabold text-gold-400">{event.hijriYear}</div>
      </div>
      <div class="px-4 py-2 rounded-xl bg-navy-950/60 border border-border/10 text-center min-w-[70px]">
        <div class="text-[10px] text-text-secondary font-bold">GREGORIAN</div>
        <div class="text-sm font-extrabold text-text-primary">{event.gregorianYear}</div>
      </div>
    </a>
  </div>

  <!-- Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Description & Sources -->
    <div class="lg:col-span-2 flex flex-col gap-6">
      <!-- Description -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-3">
        <h2 class="text-sm font-bold text-gold-400">Deskripsi Narasi Sejarah</h2>
        <p class="text-xs text-text-secondary leading-relaxed font-light">
          {event.description}
        </p>
      </div>

      <!-- Expansion Hook (Global Connection) -->
      {#if event.globalHook.is_connected_to_global}
        <div class="glass p-5 rounded-2xl border border-emerald-500/20 bg-emerald-950/10 flex gap-4">
          <div class="text-xl">🌍</div>
          <div>
            <h3 class="text-xs font-bold text-emerald-400">Hubungan Sejarah Global (Pivot Sumbu)</h3>
            <p class="text-xs text-text-secondary mt-1 leading-relaxed">
              Kategori Keterhubungan: <span class="text-gold-400 font-bold">{event.globalHook.global_pivot_category}</span>
            </p>
          </div>
        </div>
      {/if}

      <!-- Sources / References -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Kredibilitas Sumber & Pembuktian (Dimension 4)</h2>
        <div class="flex flex-col gap-3">
          {#each event.sources as src}
            <a href="/source/{src.id}" class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 hover:border-violet-500/20 hover:bg-violet-500/5 transition-all flex flex-col md:flex-row md:justify-between md:items-center gap-3 group">
              <div>
                <span class="text-[10px] font-bold text-violet-400 bg-violet-500/10 px-2 py-0.5 rounded">
                  {src.domain}
                </span>
                <p class="text-xs text-text-primary mt-2 font-medium group-hover:text-violet-400 transition-colors">{src.text}</p>
              </div>
              <div class="text-right flex-shrink-0">
                <span class="text-[10px] text-text-secondary block">Reliability Score</span>
                <span class="text-xs font-extrabold text-emerald-400">{(src.score * 100).toFixed(0)}% Match</span>
              </div>
            </a>
          {/each}
        </div>
      </div>
    </div>

    <!-- Linked Actors & Locations Sidebar -->
    <div class="flex flex-col gap-6">
      <!-- Linked Actors -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Aktor Terkait</h2>
        <div class="flex flex-col gap-3">
          {#each event.actors as actor}
            <a href="/actor/{encodeURIComponent(actor.name)}" class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 hover:border-emerald-500/20 hover:bg-emerald-500/5 transition-all flex justify-between items-center group">
              <div>
                <h4 class="text-xs font-bold text-text-primary group-hover:text-emerald-400 transition-colors">{actor.name}</h4>
                <p class="text-[10px] text-text-muted mt-0.5">{actor.role}</p>
              </div>
              <span class="text-xs text-emerald-500 group-hover:translate-x-1 transition-transform">→</span>
            </a>
          {/each}
        </div>
      </div>

      <!-- Linked Locations -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Lokasi Terkait</h2>
        <div class="flex flex-col gap-3">
          {#each event.locations as loc}
            <a href="/location/{encodeURIComponent(loc.name)}" class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 hover:border-amber-500/20 hover:bg-amber-500/5 transition-all flex flex-col gap-1 group">
              <h4 class="text-xs font-bold text-text-primary group-hover:text-amber-400 transition-colors">{loc.name}</h4>
              <p class="text-[10px] text-text-muted">{loc.type}</p>
              <p class="text-[9px] text-text-muted mt-1 font-mono">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</p>
            </a>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
