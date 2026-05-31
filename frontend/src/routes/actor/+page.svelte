<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: { actors: any[] } }>();

  let searchQuery = $state('');
  let viewMode = $state<'grid' | 'list'>('list');

  const filteredActors = $derived(
    (data.actors || []).filter(actor =>
      actor.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (actor.roles || []).some((r: string) => r.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (actor.works || []).some((w: string) => w.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto">
  <!-- Header Title -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
    <div>
      <h1 class="text-xl md:text-2xl font-extrabold text-emerald-400">Direktori Pelaku Sejarah</h1>
      <p class="text-xs text-text-secondary mt-1">Poros dimensi Aktor/Tokoh — terdaftar secara kronologis dengan jejaring sanad & karya intelektual di Sumbu Peradaban.</p>
    </div>
  </div>

  <!-- Search and Controls -->
  <div class="glass p-4 rounded-xl border border-border/10 flex flex-col md:flex-row gap-4 items-center justify-between">
    <div class="relative w-full md:w-80">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Cari pelaku sejarah, peran, atau karya..."
        class="w-full bg-navy-950/60 border border-border/15 rounded-xl px-4 py-2.5 text-xs text-text-primary placeholder-text-muted outline-none focus:border-emerald-500/40 transition-colors"
      />
    </div>
    <div class="flex items-center gap-4 w-full md:w-auto justify-between md:justify-end">
      <span class="text-xs text-text-muted">{filteredActors.length} tokoh terdaftar</span>
      <div class="flex items-center bg-navy-950/60 border border-border/10 rounded-lg p-1">
        <button 
          class="p-1.5 rounded-md text-xs transition-colors {viewMode === 'grid' ? 'bg-emerald-500/20 text-emerald-400' : 'text-text-muted hover:text-text-primary'}"
          onclick={() => viewMode = 'grid'}
          title="Tampilan Kanban (Grid)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect></svg>
        </button>
        <button 
          class="p-1.5 rounded-md text-xs transition-colors {viewMode === 'list' ? 'bg-emerald-500/20 text-emerald-400' : 'text-text-muted hover:text-text-primary'}"
          onclick={() => viewMode = 'list'}
          title="Tampilan Daftar (List)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line></svg>
        </button>
      </div>
    </div>
  </div>

  <!-- Actors List / Grid -->
  {#if filteredActors.length === 0}
    <div class="glass p-12 rounded-2xl border border-border/10 text-center text-xs text-text-muted">
      Tidak ada tokoh sejarah yang cocok dengan kriteria pencarian.
    </div>
  {:else if viewMode === 'grid'}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each filteredActors as actor}
        <a 
          href="/actor/{actor.uuid}"
          class="glass p-5 rounded-2xl border border-border/10 hover:border-emerald-500/30 hover:shadow-[0_0_20px_rgba(16,185,129,0.05)] transition-all flex flex-col gap-4 group"
        >
          <div class="flex justify-between items-start">
            <div class="w-12 h-12 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-xl text-emerald-400 group-hover:scale-105 transition-transform">
              👤
            </div>
            <CurationBadge tier={actor.curationTier} size="sm" />
          </div>

          <div>
            <h3 class="text-sm font-bold text-text-primary group-hover:text-emerald-400 transition-colors">{actor.name}</h3>
            <p class="text-xs text-text-secondary mt-0.5">{actor.culturalSphere} • {actor.actorType}</p>
          </div>

          {#if actor.description}
            <p class="text-[11px] text-text-muted line-clamp-2 leading-relaxed">{actor.description}</p>
          {/if}

          <!-- Works and Roles tags -->
          <div class="flex flex-wrap gap-1 mt-auto">
            {#each (actor.roles || []).slice(0, 2) as role}
              <span class="px-2 py-0.5 rounded bg-navy-950/60 border border-border/10 text-emerald-400 text-[9px] font-mono">{role}</span>
            {/each}
            {#each (actor.works || []).slice(0, 1) as work}
              <span class="px-2 py-0.5 rounded bg-navy-950/60 border border-border/10 text-gold-400 text-[9px] italic">📖 {work}</span>
            {/each}
          </div>
        </a>
      {/each}
    </div>
  {:else}
    <div class="flex flex-col gap-3">
      {#each filteredActors as actor}
        <a 
          href="/actor/{actor.uuid}"
          class="glass p-4 rounded-xl border border-border/10 hover:border-emerald-500/30 hover:shadow-[0_0_20px_rgba(16,185,129,0.05)] transition-all flex flex-col md:flex-row md:items-center justify-between gap-4 group"
        >
          <div class="flex items-center gap-4">
            <div class="w-10 h-10 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-lg text-emerald-400 group-hover:scale-105 transition-transform flex-shrink-0">
              👤
            </div>
            <div>
              <div class="flex items-center gap-2">
                <h3 class="text-sm font-bold text-text-primary group-hover:text-emerald-400 transition-colors">{actor.name}</h3>
                <CurationBadge tier={actor.curationTier} size="sm" />
              </div>
              <p class="text-[10px] text-text-secondary mt-0.5">{actor.culturalSphere} • {actor.actorType}</p>
            </div>
          </div>

          <div class="flex items-center gap-4 md:gap-6 flex-wrap text-[10px] justify-between md:justify-end">
            {#if actor.birthYear || actor.deathYear}
              <div class="flex flex-col text-right">
                <span class="text-text-muted">Masa Hidup</span>
                <span class="text-gold-400 font-bold mt-0.5 font-mono">
                  {actor.birthYear !== null ? `${actor.birthYear} H` : '?'} - {actor.deathYear !== null ? `${actor.deathYear} H` : '?'}
                </span>
              </div>
            {/if}
            
            <div class="flex flex-wrap gap-1.5 max-w-md">
              {#each (actor.roles || []).slice(0, 3) as role}
                <span class="px-2 py-0.5 rounded bg-navy-950/60 border border-border/10 text-text-muted text-[9px]">{role}</span>
              {/each}
              {#if (actor.roles || []).length > 3}
                <span class="px-2 py-0.5 rounded bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-[9px]">+{actor.roles.length - 3}</span>
              {/if}
            </div>
          </div>
        </a>
      {/each}
    </div>
  {/if}
</div>
