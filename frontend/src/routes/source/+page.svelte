<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: { sources: any[] } }>();

  let searchQuery = $state('');
  let viewMode = $state<'grid' | 'list'>('list');

  const sources = $derived.by(() => {
    return (data.sources || []).map((s: any) => {
      const displayTitle = s.title || 
        (s.referenceText.length > 70 
          ? s.referenceText.substring(0, 70) + '...' 
          : s.referenceText);
          
      return {
        id: s.sourceId,
        title: displayTitle,
        author: s.author || s.interpretationMethod || "Penyusun Anonim",
        type: s.domain,
        period: s.publicationEra || "Periode Klasik",
        tier: s.reliabilityScore >= 0.9 ? 'canonical' : (s.reliabilityScore >= 0.75 ? 'reviewed' : 'verified'),
        reliability: `${((s.reliabilityScore || 0.8) * 100).toFixed(0)}%`,
        avatar: s.domain === 'Teks Suci' ? '📖' : (s.domain === 'Arkeologi' ? '🪨' : '📄'),
        referenceText: s.referenceText
      };
    });
  });

  const filteredSources = $derived(
    sources.filter((source: any) =>
      source.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      source.author.toLowerCase().includes(searchQuery.toLowerCase()) ||
      source.type.toLowerCase().includes(searchQuery.toLowerCase()) ||
      source.referenceText.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto">
  <!-- Header Title -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
    <div>
      <h1 class="text-xl md:text-2xl font-extrabold text-gold-400">Direktori Kitab Rujukan & Sanad</h1>
      <p class="text-xs text-text-secondary mt-1">Poros dimensi pembuktian historis — memetakan tingkat keabsahan & reliabilitas naskah rujukan di Sumbu Peradaban.</p>
    </div>
  </div>

  <!-- Search and Controls -->
  <div class="glass p-4 rounded-xl border border-border/10 flex flex-col md:flex-row gap-4 items-center justify-between">
    <div class="relative w-full md:w-80">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Cari rujukan sejarah..."
        class="w-full bg-iron-950/60 border border-border/15 rounded-xl px-4 py-2.5 text-xs text-text-primary placeholder-text-muted outline-none focus:border-gold-500/40 transition-colors"
      />
    </div>
    <div class="flex items-center gap-4 w-full md:w-auto justify-between md:justify-end">
      <span class="text-xs text-text-muted">{filteredSources.length} sumber tervalidasi</span>
      <div class="flex items-center bg-iron-950/60 border border-border/10 rounded-lg p-1">
        <button 
          class="p-1.5 rounded-md text-xs transition-colors {viewMode === 'grid' ? 'bg-gold-500/20 text-gold-400' : 'text-text-muted hover:text-text-primary'}"
          onclick={() => viewMode = 'grid'}
          title="Tampilan Kanban (Grid)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect></svg>
        </button>
        <button 
          class="p-1.5 rounded-md text-xs transition-colors {viewMode === 'list' ? 'bg-gold-500/20 text-gold-400' : 'text-text-muted hover:text-text-primary'}"
          onclick={() => viewMode = 'list'}
          title="Tampilan Daftar (List)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line></svg>
        </button>
      </div>
    </div>
  </div>

  <!-- Sources List / Grid -->
  {#if filteredSources.length === 0}
    <div class="glass p-12 rounded-2xl border border-border/10 text-center text-xs text-text-muted">
      Tidak ada naskah rujukan yang cocok dengan kriteria pencarian.
    </div>
  {:else if viewMode === 'grid'}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each filteredSources as source}
        <a 
          href="/source/{source.id}"
          class="rounded-2xl transition-all flex flex-row group overflow-hidden h-full gap-4 hover:bg-iron-950/20"
        >
          <!-- Book Cover Section (25%) -->
          <div class="w-1/4 min-w-[80px] bg-gradient-to-br from-gold-900/40 to-iron-950 border-r border-gold-500/20 flex flex-col items-center justify-center relative shadow-[0_0_20px_rgba(212,168,83,0.05)] group-hover:from-gold-800/40 transition-colors rounded-l-xl rounded-r-sm">
            <div class="absolute left-0 top-0 bottom-0 w-2 bg-black/40"></div> <!-- Spine shadow -->
            <div class="absolute left-2 top-0 bottom-0 w-[1px] bg-white/10"></div> <!-- Spine highlight -->
            <span class="text-3xl sm:text-4xl relative z-10 filter drop-shadow-lg">{source.avatar}</span>
          </div>

          <!-- Book Details Section (75%) -->
          <div class="flex-1 py-4 pr-4 flex flex-col gap-3">
            <div class="flex justify-between items-start gap-2">
              <h3 class="text-sm font-bold text-text-primary group-hover:text-gold-400 transition-colors leading-snug line-clamp-2">{source.title}</h3>
              <CurationBadge tier={source.tier as any} size="sm" />
            </div>

            <p class="text-[11px] text-text-secondary line-clamp-1 italic">{source.author}</p>

            <div class="rounded-xl bg-iron-950/40 border border-border/5 flex flex-col gap-1.5 p-3 text-[10px] mt-auto">
              <div class="flex justify-between">
                <span class="text-text-muted">Domain:</span>
                <span class="text-text-primary font-medium truncate max-w-[100px] text-right">{source.type}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-text-muted">Era:</span>
                <span class="text-gold-400 font-medium">{source.period}</span>
              </div>
              <div class="flex justify-between items-center mt-1.5 pt-1.5 border-t border-border/5">
                <span class="text-text-muted">Validitas:</span>
                <span class="text-verdigris-400 font-bold px-1.5 bg-verdigris-500/10 border border-verdigris-500/20 rounded">{source.reliability}</span>
              </div>
            </div>
          </div>
        </a>
      {/each}
    </div>
  {:else}
    <div class="flex flex-col gap-8">
      {#each filteredSources as source}
        <a 
          href="/source/{source.id}"
          class="transition-all flex flex-row items-stretch group gap-5"
        >
          <!-- Book Cover Section (fixed width for list view) -->
          <div class="w-20 md:w-28 rounded-r-sm rounded-l-xl bg-gradient-to-br from-gold-900/40 to-iron-950 border-r border-gold-500/20 flex flex-col items-center justify-center relative shadow-[0_0_20px_rgba(212,168,83,0.05)] group-hover:from-gold-800/40 transition-colors flex-shrink-0">
            <div class="absolute left-0 top-0 bottom-0 w-1.5 bg-black/40"></div>
            <div class="absolute left-1.5 top-0 bottom-0 w-[1px] bg-white/10"></div>
            <span class="text-2xl md:text-3xl relative z-10 filter drop-shadow-lg">{source.avatar}</span>
          </div>

          <!-- Book Details -->
          <div class="py-2 flex-1 flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/5 pb-6">
            <div class="flex-1">
              <h3 class="text-sm font-bold text-text-primary group-hover:text-gold-400 transition-colors line-clamp-1">{source.title}</h3>
              <p class="text-[10px] text-text-secondary mt-1 italic line-clamp-1">{source.author}</p>
            </div>

            <div class="flex items-center gap-4 md:gap-6 flex-wrap text-[10px] justify-between md:justify-end">
              <div class="flex flex-col text-right hidden sm:flex">
                <span class="text-text-muted">Domain</span>
                <span class="text-text-primary font-medium mt-0.5">{source.type}</span>
              </div>
              <div class="flex flex-col text-right">
                <span class="text-text-muted">Era</span>
                <span class="text-gold-400 font-medium mt-0.5">{source.period}</span>
              </div>
              <div class="flex flex-col text-right">
                <span class="text-text-muted">Keandalan</span>
                <span class="text-verdigris-400 font-bold mt-0.5 font-mono bg-verdigris-500/10 border border-verdigris-500/20 px-1.5 py-0.5 rounded inline-block">{source.reliability}</span>
              </div>
              <div class="flex-shrink-0 ml-auto md:ml-0">
                <CurationBadge tier={source.tier as any} size="sm" />
              </div>
            </div>
          </div>
        </a>
      {/each}
    </div>
  {/if}
</div>
