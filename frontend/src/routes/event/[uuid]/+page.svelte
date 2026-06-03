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
    
    // Find all edges connected to this event UUID (for Actors and Locations)
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
            actors.push({ uuid: relatedNode.data.id, name: relatedNode.data.label, role: relationship });
          } else if (relatedNode.data.type === 'location') {
            locations.push({ uuid: relatedNode.data.id, name: relatedNode.data.label, type: relatedNode.data.tier || 'Titik Kritis', lat: 21.0, lng: 39.0 });
          }
        }
      }
    });

    // Directly parse sources and subReferences from DB response
    const rawSources = dbEvent.sources || [];
    const sources = rawSources.map((s: any) => {
      let subRefs = [];
      if (s.subReferences) {
        try {
          subRefs = JSON.parse(s.subReferences);
        } catch (e) {
          console.error('Failed to parse sub-references JSON:', e);
        }
      }
      return {
        id: s.sourceId,
        domain: s.domain || 'Verified',
        title: s.title || 'Tanpa Judul',
        author: s.author || 'Tanpa Penulis',
        text: s.referenceText,
        score: s.reliabilityScore || 0.95,
        subReferences: subRefs
      };
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
      sources,
      mediaLinks: dbEvent.mediaLinks || []
    };
  });
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back Link -->
  <a href="/timeline" class="text-xs text-gold-500 hover:text-gold-400 transition-colors font-bold flex items-center gap-1">
    ← Kembali ke Timeline
  </a>

  <!-- Document-Style Container -->
  <div class="glass p-8 lg:p-12 rounded-3xl border border-gold-500/10 flex flex-col gap-10 relative overflow-hidden">
    <!-- Ambient light effect -->
    <div class="absolute -top-24 -right-24 w-96 h-96 bg-gold-500/10 rounded-full blur-3xl pointer-events-none"></div>

    {#if event.mediaLinks && event.mediaLinks.some((m: any) => m.mediaType === 'image')}
      <!-- Jumbotron with Integrated Header -->
      <div class="w-full h-80 md:h-[450px] rounded-2xl overflow-hidden relative shadow-2xl border border-border/10 mb-2 mt-[-1rem]">
        <img src={event.mediaLinks.find((m: any) => m.mediaType === 'image').url} alt={event.title} class="w-full h-full object-cover object-center" />
        <div class="absolute inset-0 bg-gradient-to-t from-iron-950 via-iron-950/50 to-transparent"></div>
        
        <div class="absolute bottom-0 left-0 right-0 p-8 flex flex-col md:flex-row justify-between items-end gap-8 z-10">
          <div class="flex flex-col gap-2">
            <div class="flex items-center gap-3">
              <span class="text-[10px] uppercase font-bold tracking-widest text-gold-500 drop-shadow-md">Peristiwa Utama</span>
              <CurationBadge tier={event.tier as any} size="sm" />
            </div>
            <h1 class="text-3xl md:text-5xl font-black text-white mt-1 tracking-tight drop-shadow-lg">{event.title}</h1>
          </div>

          <!-- Dual Date Badges Minimalist (Jumbotron Overlay) -->
          <a href="/time/1h" class="flex gap-4 items-center group/time bg-iron-950/40 backdrop-blur-md px-5 py-3 rounded-xl border border-gold-500/20">
            <div class="flex flex-col items-end">
              <div class="text-[9px] text-gold-500 font-bold uppercase tracking-widest drop-shadow-sm">Hijriah</div>
              <div class="text-xl md:text-2xl font-extrabold text-gold-400 font-mono group-hover/time:text-gold-300 transition-colors drop-shadow-md">{event.hijriYear}</div>
            </div>
            <div class="h-8 w-px bg-border/40"></div>
            <div class="flex flex-col items-start">
              <div class="text-[9px] text-gray-300 font-bold uppercase tracking-widest drop-shadow-sm">Gregorian</div>
              <div class="text-xl md:text-2xl font-extrabold text-white font-mono group-hover/time:text-gray-200 transition-colors drop-shadow-md">{event.gregorianYear}</div>
            </div>
          </a>
        </div>
      </div>
    {:else}
      <!-- Header Section Fallback (Without Jumbotron) -->
      <div class="flex flex-col md:flex-row justify-between items-start gap-8 relative z-10 border-b border-border/5 pb-8">
        <div class="flex flex-col gap-2">
          <div class="flex items-center gap-3">
            <span class="text-[10px] uppercase font-bold tracking-widest text-gold-500">Peristiwa Utama</span>
            <CurationBadge tier={event.tier as any} size="sm" />
          </div>
          <h1 class="text-3xl md:text-5xl font-black text-text-primary mt-1 tracking-tight">{event.title}</h1>
        </div>

        <!-- Dual Date Badges Minimalist -->
        <a href="/time/1h" class="flex gap-4 items-center group/time">
          <div class="flex flex-col items-end">
            <div class="text-[9px] text-gold-500 font-bold uppercase tracking-widest">Hijriah</div>
            <div class="text-xl md:text-2xl font-extrabold text-gold-400 font-mono group-hover/time:text-gold-300 transition-colors">{event.hijriYear}</div>
          </div>
          <div class="h-8 w-px bg-border/20"></div>
          <div class="flex flex-col items-start">
            <div class="text-[9px] text-text-secondary font-bold uppercase tracking-widest">Gregorian</div>
            <div class="text-xl md:text-2xl font-extrabold text-text-primary font-mono group-hover/time:text-white transition-colors">{event.gregorianYear}</div>
          </div>
        </a>
      </div>
    {/if}

    <!-- Seamless Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 relative z-10">
      <!-- Main Column: Description & Sources -->
      <div class="lg:col-span-2 flex flex-col gap-12">
        <!-- Description -->
        <div class="flex flex-col gap-4 relative">
          <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
            <span class="text-base">📜</span> Deskripsi Narasi Sejarah
          </h2>
          <p class="text-[13px] text-text-secondary leading-loose font-normal pl-4 border-l-2 border-gold-500/20 whitespace-pre-wrap">
            {event.description}
          </p>
        </div>

        <!-- Expansion Hook (Global Connection) -->
        {#if event.globalHook.is_connected_to_global}
          <div class="p-5 rounded-xl border-l-4 border-verdigris-500/40 bg-verdigris-950/10 flex gap-4">
            <div class="text-xl">🌍</div>
            <div class="flex flex-col gap-1">
              <h3 class="text-[11px] font-bold text-verdigris-400 uppercase tracking-widest">Pivot Sumbu & Hubungan Global</h3>
              <p class="text-[12px] text-text-secondary leading-relaxed">
                Kategori Keterhubungan: <span class="text-gold-400 font-bold tracking-wide">{event.globalHook.global_pivot_category}</span>
              </p>
            </div>
          </div>
        {/if}

        <!-- Sources / References -->
        <div class="flex flex-col gap-6">
          <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
            <span class="text-base">⚖️</span> Kredibilitas Sumber & Pembuktian
          </h2>
          
          {#if event.sources && event.sources.length > 0}
            <div class="flex flex-col gap-6">
              {#each event.sources as src}
                <div class="flex flex-col gap-3 relative pl-6 before:absolute before:left-0 before:top-2 before:bottom-0 before:w-px before:bg-border/10">
                  <span class="absolute -left-[5px] top-1.5 w-2.5 h-2.5 rounded-full bg-gold-500/50 border-2 border-iron-950"></span>
                  <div class="flex flex-col md:flex-row md:justify-between md:items-start gap-3">
                    <div>
                      <div class="flex items-center gap-2">
                        <span class="text-[9px] font-bold text-violet-400 bg-violet-500/10 px-2 py-0.5 rounded tracking-widest uppercase">
                          {src.domain}
                        </span>
                        <a href="/source/{src.id}" class="text-[11px] font-extrabold text-gold-400 hover:text-gold-300 hover:underline transition-all tracking-wide">
                          Lihat Rujukan Lengkap ↗
                        </a>
                      </div>
                      <p class="text-[13px] text-text-primary mt-3 font-normal leading-loose italic text-justify">{src.text}</p>
                    </div>
                    <div class="text-right flex-shrink-0 md:pl-4">
                      <span class="text-[9px] text-text-secondary block uppercase tracking-widest">Reliability</span>
                      <span class="text-sm font-extrabold text-verdigris-400 font-mono">{(src.score * 100).toFixed(0)}%</span>
                    </div>
                  </div>

                  <!-- Sub-References Perkamen Table -->
                  {#if src.subReferences && src.subReferences.length > 0}
                    <div class="mt-4 border-t border-border/5 pt-4">
                      <span class="text-[9px] font-bold text-text-secondary uppercase tracking-widest mb-3 block">
                        Rincian Sub-Rujukan Pembuktian
                      </span>
                      <div class="overflow-x-auto rounded-xl border border-border/10">
                        <table class="w-full text-left border-collapse text-xs">
                          <thead>
                            <tr class="bg-iron-950/40 border-b border-border/10 text-gold-400/80 font-bold uppercase tracking-widest text-[9px]">
                              <th class="px-4 py-3 w-1/3">Bab / Surat</th>
                              <th class="px-4 py-3 w-1/4">Halaman / Ayat</th>
                              <th class="px-4 py-3">Ulasan / Tafsir</th>
                            </tr>
                          </thead>
                          <tbody class="divide-y divide-border/5">
                            {#each src.subReferences as sub}
                              <tr class="hover:bg-iron-900/20 transition-colors">
                                <td class="px-4 py-3 font-bold text-text-primary font-serif">{sub.section || '-'}</td>
                                <td class="px-4 py-3 font-mono text-verdigris-400 font-bold">{sub.point || '-'}</td>
                                <td class="px-4 py-3 text-text-secondary leading-relaxed font-light italic">{sub.note || '-'}</td>
                              </tr>
                            {/each}
                          </tbody>
                        </table>
                      </div>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-[12px] text-text-muted italic">Belum ada rujukan sumber historis yang ditautkan.</p>
          {/if}
        </div>
      </div>

      <!-- Right Column: Linked Actors & Locations Sidebar -->
      <div class="flex flex-col gap-10 border-t lg:border-t-0 lg:border-l border-border/5 pt-8 lg:pt-0 lg:pl-10">
        <!-- Linked Actors -->
        <div class="flex flex-col gap-4">
          <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
            <span>👥</span> Aktor Terlibat
          </h2>
          {#if event.actors && event.actors.length > 0}
            <div class="flex flex-col gap-3">
              {#each event.actors as actor}
                <a href="/actor/{actor.uuid}" class="group flex items-center gap-3">
                  <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-verdigris-500/30 transition-colors">👤</div>
                  <div class="flex flex-col flex-1">
                    <span class="text-[12px] font-bold text-text-primary group-hover:text-verdigris-400 transition-colors">{actor.name}</span>
                    <span class="text-[9px] text-text-muted uppercase tracking-widest">{actor.role}</span>
                  </div>
                  <span class="text-xs text-verdigris-500 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada aktor yang tertaut langsung.</p>
          {/if}
        </div>

        <!-- Linked Locations -->
        <div class="flex flex-col gap-4">
          <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
            <span>🌍</span> Lokasi Terkait
          </h2>
          {#if event.locations && event.locations.length > 0}
            <div class="flex flex-col gap-3">
              {#each event.locations as loc}
                <a href="/location/{loc.uuid}" class="group flex items-center gap-3">
                  <div class="w-8 h-8 rounded-lg bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-amber-500/30 transition-colors">📍</div>
                  <div class="flex flex-col flex-1">
                    <span class="text-[12px] font-bold text-text-primary group-hover:text-amber-400 transition-colors">{loc.name}</span>
                    <span class="text-[9px] text-text-muted uppercase tracking-widest">{loc.type}</span>
                  </div>
                  <span class="text-[9px] text-text-muted font-mono">{loc.lat.toFixed(2)}° N</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada lokasi yang tertaut.</p>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
