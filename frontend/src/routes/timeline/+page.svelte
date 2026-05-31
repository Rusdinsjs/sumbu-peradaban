<script lang="ts">
  import Timeline from '$lib/components/Timeline.svelte';
  import type { TimelineEvent } from '$lib/types/timeline';
  import { eras } from '$lib/data/world-events';

  let { data } = $props<{ data: any }>();

  let selectedEra = $state('all');
  let selectedEraData = $derived(eras.find(e => e.id === selectedEra) || eras[0]);

  // Merge backend data with demo data
  let backendEvents: TimelineEvent[] = $derived(
    (data.eventsData || []).map((e: any) => ({
      uuid: e.uuid,
      title: e.title,
      year: `${e.islamicDate?.year || '?'} H / ${e.gregorianDate?.year || '?'} M`,
      yearSort: e.gregorianDate?.year || 0,
      description: e.description || '',
      tier: e.curationTier?.toLowerCase() || 'draft',
      actors: [],
      locations: [],
    }))
  );

  let allEvents = $derived(backendEvents);

  // Filter by era
  let filteredEvents = $derived(
    selectedEra === 'all'
      ? allEvents
      : allEvents.filter(ev => ev.yearSort >= selectedEraData.min && ev.yearSort < selectedEraData.max)
  );
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Page Header -->
  <div class="glass p-6 rounded-2xl border border-border/10">
    <h1 class="text-xl font-extrabold text-gold-400">Garis Waktu Sejarah Dunia</h1>
    <p class="text-xs text-text-secondary leading-relaxed mt-1">
      Navigasi kronologis peradaban manusia dengan poros kenabian (dari Adam AS hingga akhir zaman) sebagai sumbu utama — 
      menampilkan peristiwa sinkronik seluruh peradaban dunia di setiap titik waktu.
    </p>
  </div>

  <!-- Era Selector -->
  <div class="flex flex-wrap gap-2">
    {#each eras as era}
      <button
        class="glass px-4 py-2.5 rounded-xl border text-xs font-bold transition-all {selectedEra === era.id ? 'border-gold-500/40 text-gold-400 bg-gold-500/5 shadow-[0_0_12px_rgba(212,168,83,0.1)]' : 'border-border/10 text-text-secondary hover:border-gold-500/20 hover:text-text-primary'}"
        onclick={() => selectedEra = era.id}
      >
        <span class="mr-1.5">{era.icon}</span>
        {era.label}
      </button>
    {/each}
  </div>

  <!-- Stats Bar -->
  <div class="flex items-center gap-6 px-2">
    <span class="text-[10px] text-text-muted uppercase tracking-wider font-bold">
      {filteredEvents.length} peristiwa
      {#if selectedEra !== 'all'}
        di era <span class="text-gold-400">{selectedEraData.label}</span>
      {/if}
    </span>
    <div class="flex items-center gap-3 text-[9px] text-text-muted">
      <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-gold-500 inline-block"></span> Poros Kenabian</span>
      <span class="flex items-center gap-1"><span class="w-2 h-0.5 bg-cyan-400 inline-block"></span> Peradaban Sinkronik</span>
    </div>
  </div>

  <!-- Timeline Component -->
  <div class="glass p-6 rounded-2xl border border-border/10">
    <Timeline events={filteredEvents} />
  </div>
</div>
