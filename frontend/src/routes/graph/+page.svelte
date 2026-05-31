<script lang="ts">
  import GraphExplorer from "$lib/components/GraphExplorer.svelte";
  import Graph3D from "$lib/components/Graph3D.svelte";
  import { 
    deriveActors, 
    deriveLocations
  } from "$lib/data/world-events";

  let { data } = $props<{ data: any }>();

  let selectedNode = $state<any>(null);
  let viewMode = $state<'3d' | '2d'>('3d'); // Make 3D space-time default
  let searchQuery = $state('');
  let showDropdown = $state(false);

  // ─── SEARCH SUGGESTIONS ──────────────────────────────────────
  let searchResults = $derived.by(() => {
    if (!searchQuery.trim() || searchQuery.trim().length < 2) return [];
    
    const q = searchQuery.toLowerCase();
    const results: any[] = [];
    
    // 1. Search Events
    realWorldEvents.forEach(e => {
      if (e.title.toLowerCase().includes(q)) {
        results.push({ id: e.uuid, label: e.title, type: 'event', tier: e.tier, subtitle: e.year });
      }
    });
    
    // 2. Search Actors
    const actors = deriveActors(realWorldEvents);
    actors.forEach(a => {
      if (a.name.toLowerCase().includes(q)) {
        results.push({ id: a.name, label: a.name, type: 'actor', tier: 'canonical', subtitle: 'Tokoh Sejarah' });
      }
    });

    // 3. Search Locations
    const locations = deriveLocations(realWorldEvents);
    locations.forEach(l => {
      if (l.name.toLowerCase().includes(q)) {
        results.push({ id: l.name, label: l.name, type: 'location', tier: 'canonical', subtitle: 'Lokasi Geografis' });
      }
    });
    
    return results.slice(0, 6); // Limit to top 6 results
  });

  // ─── RECONSTRUCT REAL WORLD EVENTS FROM GRAPH API ──────────────
  let realWorldEvents = $derived.by(() => {
    const apiData = data.fullGraphData || { nodes: [], edges: [] };
    const locData = data.locationsData || [];
    const eventsMap = new Map();

    // 1. Gather all events
    apiData.nodes.forEach((n: any) => {
      if (n.data.type === 'event') {
        eventsMap.set(n.data.id, {
          uuid: n.data.id,
          title: n.data.label,
          year: "622 M", // Fallback, would need API extension for real year
          yearSort: 622,
          description: "Data ditarik dari database Sumbu Peradaban (Neo4j)",
          tier: n.data.tier,
          actors: [],
          locations: []
        });
      }
    });

    // 2. Map Actors & Locations via edges
    apiData.edges.forEach((e: any) => {
      const sourceId = e.data.source;
      const targetId = e.data.target;
      const rel = e.data.relationship;

      let ev = eventsMap.get(sourceId);
      let otherId = targetId;
      if (!ev) {
        ev = eventsMap.get(targetId);
        otherId = sourceId;
      }

      if (ev) {
        const otherNode = apiData.nodes.find((n: any) => n.data.id === otherId);
        if (otherNode) {
          if (otherNode.data.type === 'actor') {
            ev.actors.push({ name: otherNode.data.label, role: rel });
          } else if (otherNode.data.type === 'location') {
            const locDetails = locData.find((l: any) => l.uuid === otherId);
            ev.locations.push({ 
              name: otherNode.data.label, 
              type: rel,
              lat: locDetails?.lat,
              lng: locDetails?.lng
            });
          }
        }
      }
    });

    return Array.from(eventsMap.values());
  });

  // ─── SEARCH FILTER ───────────────────────────────────────────
  let filteredEvents = $derived.by(() => {
    if (!searchQuery.trim()) return realWorldEvents;
    const q = searchQuery.toLowerCase();
    return realWorldEvents.filter((ev: any) => 
      ev.title.toLowerCase().includes(q) || 
      ev.description.toLowerCase().includes(q) ||
      (ev.actors || []).some((a: any) => a.name.toLowerCase().includes(q)) ||
      (ev.locations || []).some((l: any) => l.name.toLowerCase().includes(q))
    );
  });

  // ─── AUTO-SELECT SEARCH RESULT ───────────────────────────────
  $effect(() => {
    if (searchQuery.trim().length > 1) {
      const q = searchQuery.toLowerCase();
      
      // 1. Try matching Event titles
      const matchedEvent = realWorldEvents.find((e: any) => e.title.toLowerCase().includes(q));
      if (matchedEvent) {
        selectedNode = { id: matchedEvent.uuid, label: matchedEvent.title, type: 'event', tier: matchedEvent.tier };
        return;
      }
      
      // 2. Try matching Actor names
      const actors = deriveActors(realWorldEvents);
      const matchedActor = actors.find(a => a.name.toLowerCase().includes(q));
      if (matchedActor) {
        selectedNode = { id: matchedActor.name, label: matchedActor.name, type: 'actor', tier: 'canonical' };
        return;
      }

      // 3. Try matching Location names
      const locations = deriveLocations(realWorldEvents);
      const matchedLoc = locations.find(l => l.name.toLowerCase().includes(q));
      if (matchedLoc) {
        selectedNode = { id: matchedLoc.name, label: matchedLoc.name, type: 'location', tier: 'canonical' };
        return;
      }
      
      // 4. Fallback: just show the first filtered event if query matched inside description
      if (filteredEvents.length > 0) {
        const ev = filteredEvents[0];
        selectedNode = { id: ev.uuid, label: ev.title, type: 'event', tier: ev.tier };
      }
    }
  });

  function handleNodeSelect(nodeData: any) {
    selectedNode = nodeData;
  }

  // ─── 🕸️ DYNAMIC GRAPH BUILDER ENGINE (FOR 2D VIEW) ───────────
  let graphElements = $derived.by(() => {
    const realGraphData = data.fullGraphData || { nodes: [], edges: [] };

    if (!searchQuery.trim() || searchQuery.trim().length < 2) {
      return [...realGraphData.nodes, ...realGraphData.edges];
    }

    const q = searchQuery.toLowerCase();
    
    // 1. Find matching nodes in real graph data
    const matchedNodes = realGraphData.nodes.filter((n: any) => 
      (n.data.label || '').toLowerCase().includes(q) ||
      (n.data.type || '').toLowerCase().includes(q)
    );
    const matchedNodeIds = new Set(matchedNodes.map((n: any) => n.data.id));

    // 2. Find edges where at least one node is matched
    const filteredEdges = realGraphData.edges.filter((e: any) => 
      matchedNodeIds.has(e.data.source) || matchedNodeIds.has(e.data.target)
    );

    // 3. Ensure all connected nodes are also included to complete the graph
    filteredEdges.forEach((e: any) => {
      matchedNodeIds.add(e.data.source);
      matchedNodeIds.add(e.data.target);
    });

    const finalNodes = realGraphData.nodes.filter((n: any) => matchedNodeIds.has(n.data.id));

    return [...finalNodes, ...filteredEdges];
  });

  // ─── DYNAMIC DATA CORRESPONDENCE ENGINE ───────────────────────
  let entityDetails = $derived.by(() => {
    if (!selectedNode) return null;
    const label = selectedNode.label.toLowerCase();
    
    // 1. Match Event
    if (selectedNode.type === 'event') {
      const match = realWorldEvents.find((e: any) => 
        e.uuid === selectedNode.id || 
        e.title.toLowerCase().includes(label) || 
        label.includes(e.title.toLowerCase())
      );
      if (match) {
        return {
          description: match.description,
          curationTier: match.tier,
          subtitle: match.year,
          stats: `${match.actors?.length || 0} Tokoh, ${match.locations?.length || 0} Lokasi`,
          link: `/event/${match.uuid}`
        };
      }
    }
    
    // 2. Match Actor (Tokoh)
    if (selectedNode.type === 'actor') {
      const actors = deriveActors(realWorldEvents);
      const match = actors.find(a => 
        a.name.toLowerCase().includes(label) || 
        label.includes(a.name.toLowerCase())
      );
      if (match) {
        return {
          description: `Peran utama: ${match.role}. Tokoh sejarah ini terbukti terlibat aktif dalam ${match.events.length} klaster peristiwa besar di Sumbu Peradaban dunia Islam dan peradaban sinkronik global.`,
          curationTier: 'canonical' as const,
          subtitle: match.role,
          stats: `${match.events.length} Peristiwa Sejarah`,
          link: `/actor/${selectedNode.id}`
        };
      } else {
        // Fallback if not in realWorldEvents mock
        return {
          description: `Tokoh sejarah ini terlibat aktif dalam peradaban sinkronik global.`,
          curationTier: 'canonical' as const,
          subtitle: 'Aktor Sejarah',
          stats: `1 Peristiwa Terdaftar`,
          link: `/actor/${selectedNode.id}`
        };
      }
    }
    
    // 3. Match Location (Lokasi)
    if (selectedNode.type === 'location') {
      const locations = deriveLocations(realWorldEvents);
      const match = locations.find(l => 
        l.name.toLowerCase().includes(label) || 
        label.includes(l.name.toLowerCase())
      );
      if (match) {
        return {
          description: `Klasifikasi wilayah: ${match.type || 'Titik Strategis'}. Lokasi geospasial klasik ini menjadi panggung utama berlangsungnya ${match.events.length} peristiwa penting dalam sejarah dunia.`,
          curationTier: 'canonical' as const,
          subtitle: match.type || 'Wilayah Strategis',
          stats: `${match.events.length} Peristiwa Terdaftar`,
          link: `/location/${selectedNode.id}`
        };
      } else {
        return {
          description: `Lokasi geospasial klasik ini menjadi panggung utama berlangsungnya peristiwa penting dalam sejarah dunia.`,
          curationTier: 'canonical' as const,
          subtitle: 'Wilayah Strategis',
          stats: `1 Peristiwa Terdaftar`,
          link: `/location/${selectedNode.id}`
        };
      }
    }
    
    // 4. Match Source (Kitab Rujukan)
    if (selectedNode.type === 'source') {
      const realGraphData = data.fullGraphData || { nodes: [], edges: [] };
      const match = realGraphData.nodes.find((n: any) => 
        (n.data.type === 'source') && 
        (n.data.id === selectedNode.id || (n.data.label || '').toLowerCase().includes(label))
      );
      if (match) {
        return {
          description: `Kitab rujukan atau sanad yang mendasari pembuktian historis di dalam sistem.`,
          curationTier: match.data.tier || 'verified',
          subtitle: match.data.label,
          stats: `Otoritas: ${(match.data.tier || 'verified').toUpperCase()}`,
          link: `/source/${match.data.id}`
        };
      }
    }

    return null;
  });

  // Dynamic reactive fallbacks
  let displayDescription = $derived(entityDetails?.description || 'Entitas sejarah terdaftar dalam sistem visualisasi Sumbu Peradaban.');
  let displayTier = $derived(entityDetails?.curationTier || selectedNode?.tier || 'draft');
  let displaySubtitle = $derived(entityDetails?.subtitle || (selectedNode?.type === 'actor' ? 'Tokoh Sejarah' : selectedNode?.type === 'location' ? 'Lokasi Geografis' : 'Entitas Utama'));
  let displayStats = $derived(entityDetails?.stats || 'Tersinkronisasi');
  let displayLink = $derived(entityDetails?.link || (
    selectedNode?.type === 'actor' 
      ? `/actor/${encodeURIComponent(selectedNode.label)}`
      : selectedNode?.type === 'location'
      ? `/location/${encodeURIComponent(selectedNode.label)}`
      : selectedNode?.type === 'source'
      ? `/source/${selectedNode.id}`
      : `/event/${selectedNode.id}`
  ));
</script>

<div class="w-full h-[calc(100vh-80px)] flex flex-col gap-4 animate-fade-in">
  <!-- Main Area (Full Width) -->
  <div class="flex-1 w-full relative overflow-hidden">
    <!-- Full Graph/3D Cube Visualization -->
    <div
      class="w-full h-full glass rounded-2xl border border-border/10 overflow-hidden flex flex-col transition-all duration-300 min-h-[300px]"
    >
      <!-- Sub-header Menu for Toggle View Engine -->
      <div class="p-3 border-b border-border/10 bg-surface/30 flex justify-between items-center">
        <div class="flex flex-col">
          <h3 class="text-[10px] font-bold text-text-secondary uppercase tracking-wider">
            Visualisasi Sumbu Peradaban
          </h3>
          <span class="text-[8px] text-text-muted mt-0.5">
            {deriveActors(filteredEvents).length} Tokoh Terkoneksi • {filteredEvents.length} Peristiwa
          </span>
        </div>
        
        <!-- Search Input & Suggestions Dropdown -->
        <div class="flex-1 max-w-[240px] relative mx-4 z-50">
          <input 
            type="text" 
            bind:value={searchQuery} 
            onfocus={() => showDropdown = true}
            onblur={() => setTimeout(() => showDropdown = false, 200)}
            placeholder="Cari Tokoh, Lokasi, atau Peristiwa..." 
            class="w-full bg-navy-950/80 border border-border/10 rounded-lg pl-8 pr-3 py-1.5 text-[10px] text-text-primary placeholder:text-text-muted focus:outline-none focus:border-gold-500/50 transition-colors"
          />
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 text-text-muted absolute left-2.5 top-[14px] -translate-y-1/2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>

          <!-- Dropdown List -->
          {#if showDropdown && searchResults.length > 0}
            <div class="absolute top-full left-0 w-full mt-2 bg-navy-950 border border-border/20 rounded-xl shadow-[0_10px_40px_rgba(0,0,0,0.8)] overflow-hidden animate-fade-in backdrop-blur-md">
              {#each searchResults as result}
                <button 
                  class="w-full text-left px-3 py-2.5 hover:bg-white/5 transition-colors border-b border-border/5 last:border-0 flex flex-col gap-1 group cursor-pointer"
                  onclick={() => {
                    selectedNode = { id: result.id, label: result.label, type: result.type, tier: result.tier };
                    searchQuery = result.label;
                    showDropdown = false;
                  }}
                >
                  <div class="flex items-center gap-2">
                    <span class="text-[8px] uppercase font-extrabold px-1.5 py-0.5 rounded {result.type === 'event' ? 'bg-gold-500/20 text-gold-400' : result.type === 'actor' ? 'bg-emerald-500/20 text-emerald-400' : 'bg-amber-500/20 text-amber-400'}">{result.type}</span>
                    <span class="text-xs font-bold text-text-primary group-hover:text-gold-400 transition-colors truncate">{result.label}</span>
                  </div>
                  <span class="text-[9px] text-text-muted pl-[52px] -mt-1 block truncate">{result.subtitle}</span>
                </button>
              {/each}
            </div>
          {/if}
        </div>
        
        <!-- Premium Switch 3D Space-Time vs 2D Relations -->
        <div class="flex items-center bg-navy-950 border border-border/10 rounded-lg p-0.5 shrink-0">
          <button 
            class="px-3 py-1 rounded-md text-[9px] font-extrabold uppercase tracking-wider transition-all cursor-pointer {viewMode === '3d' ? 'bg-gold-500 text-surface shadow-[0_0_10px_rgba(212,168,83,0.3)]' : 'text-text-muted hover:text-text-primary'}"
            onclick={() => viewMode = '3d'}
          >
            3D Space-Time
          </button>
          <button 
            class="px-3 py-1 rounded-md text-[9px] font-extrabold uppercase tracking-wider transition-all cursor-pointer {viewMode === '2d' ? 'bg-gold-500 text-surface shadow-[0_0_10px_rgba(212,168,83,0.3)]' : 'text-text-muted hover:text-text-primary'}"
            onclick={() => viewMode = '2d'}
          >
            2D Relational
          </button>
        </div>
      </div>

      <div class="flex-1 relative p-4 bg-[#060913]/30">
        {#if viewMode === '3d'}
          <!-- 🛸 Gorgeous 3D Spatio-Temporal space-time cube visualizer -->
          <Graph3D 
            events={filteredEvents} 
            onEventSelect={(ev) => handleNodeSelect({ 
              id: ev.uuid, 
              label: ev.title, 
              type: 'event', 
              tier: ev.tier 
            })} 
          />
        {:else}
          <!-- Standard 2D Cytoscape Relational Net -->
          <GraphExplorer elements={graphElements} onNodeSelect={handleNodeSelect} />
        {/if}
      </div>

      <!-- Legend at the bottom of Grid 2 (Only for 2D relation) -->
      {#if viewMode === '2d'}
        <div
          class="p-3 border-t border-border/10 bg-surface/30 flex flex-wrap gap-5 items-center justify-center"
        >
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 bg-gold-500 rounded border border-surface shadow-[0_0_6px_rgba(212,168,83,0.4)]"></span>
            <span class="text-[10px] text-text-primary font-medium uppercase tracking-wider">Peristiwa</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 bg-emerald-500 rounded-full border border-surface shadow-[0_0_6px_rgba(16,185,129,0.4)]"></span>
            <span class="text-[10px] text-text-primary font-medium uppercase tracking-wider">Tokoh</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 bg-amber-500 transform rotate-45 border border-surface shadow-[0_0_6px_rgba(245,158,11,0.4)]"></span>
            <span class="text-[10px] text-text-primary font-medium uppercase tracking-wider">Lokasi</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 bg-violet-500 rounded-sm border border-surface shadow-[0_0_6px_rgba(139,92,246,0.4)]"></span>
            <span class="text-[10px] text-text-primary font-medium uppercase tracking-wider">Sumber</span>
          </div>
        </div>
      {/if}
    </div>

    <!-- Bottom Right Overlay: Node Detail Inspector -->
    {#if selectedNode}
      <div
        class="absolute bottom-6 right-6 w-56 glass p-4 rounded-xl border border-border/10 flex flex-col shadow-[0_10px_40px_rgba(0,0,0,0.8)] animate-fade-in z-50 backdrop-blur-md"
      >
        <!-- Close Button -->
        <button 
          class="absolute top-3 right-3 text-text-muted hover:text-white transition-colors cursor-pointer z-10 text-[10px]"
          onclick={() => selectedNode = null}
          title="Tutup Panel"
        >
          ✕
        </button>

        <h3
          class="text-[8px] font-bold text-text-secondary uppercase tracking-wider mb-2.5 pb-2 border-b border-border/10 pr-4"
        >
          Detail Peninjau Graf
        </h3>

        <div class="flex flex-col gap-3">
          <div>
            <span
              class="text-[7px] uppercase font-extrabold tracking-wider px-1.5 py-0.5 rounded {selectedNode.type === 'event' ? 'bg-gold-500/10 text-gold-400 border border-gold-500/25' : selectedNode.type === 'actor' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/25' : selectedNode.type === 'location' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/25' : 'bg-violet-500/10 text-violet-400 border border-violet-500/25'}"
            >
              {selectedNode.type}
            </span>
            <h2 class="text-xs font-black text-text-primary mt-1.5 leading-tight pr-3 line-clamp-2">
              {selectedNode.label}
            </h2>
            <p class="text-[8px] text-text-muted mt-0.5 truncate">{displaySubtitle}</p>
          </div>

          <!-- Metadata Badges -->
          <div
            class="p-2 rounded-lg bg-navy-950/60 border border-border/10 flex flex-col gap-1.5"
          >
            <div class="flex justify-between items-center text-[9px]">
              <span class="text-text-secondary">Sanad:</span>
              <span class="text-gold-400 capitalize font-bold flex items-center gap-1">
                <span class="w-1.5 h-1.5 rounded-full inline-block {displayTier === 'canonical' ? 'bg-emerald-500' : displayTier === 'reviewed' ? 'bg-gold-500' : 'bg-gray-400'}"></span>
                {displayTier}
              </span>
            </div>
            <div class="flex justify-between items-center text-[9px]">
              <span class="text-text-secondary">Relasi:</span>
              <span class="text-emerald-400 font-bold">{displayStats}</span>
            </div>
          </div>

          <!-- Deep Link Navigation -->
          <a
            href={displayLink}
            class="mt-1 w-full py-2 rounded-lg gradient-gold text-surface text-[10px] font-extrabold text-center hover:shadow-[0_0_15px_rgba(212,168,83,0.35)] transition-all flex items-center justify-center gap-1.5 group cursor-pointer"
          >
            <span class="text-sm leading-none">👁️</span>
            <span>Detail Entitas</span>
          </a>
        </div>
      </div>
    {/if}
  </div>
</div>
