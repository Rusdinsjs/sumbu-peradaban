<script lang="ts">
  import CurationBadge from './CurationBadge.svelte';
  import type { TimelineEvent } from '$lib/types/timeline';

  let { 
    events = [] 
  } = $props<{
    events?: TimelineEvent[];
  }>();

  // Group events by year for synchronic clustering
  interface YearCluster {
    year: string;
    yearSort: number;
    events: TimelineEvent[];
  }

  let clusters = $derived.by(() => {
    const map = new Map<string, YearCluster>();
    
    for (const ev of events) {
      if (!map.has(ev.year)) {
        map.set(ev.year, { year: ev.year, yearSort: ev.yearSort, events: [] });
      }
      map.get(ev.year)!.events.push(ev);
    }

    return Array.from(map.values()).sort((a, b) => a.yearSort - b.yearSort);
  });
</script>

<div class="timeline-container">
  {#each clusters as cluster, i}
    <div class="year-cluster">
      <!-- Year Marker -->
      <div class="year-marker">
        <div class="year-dot"></div>
        <span class="year-label">{cluster.year}</span>
        {#if i < clusters.length - 1}
          <div class="year-connector"></div>
        {/if}
      </div>

      <!-- Events in this year -->
      <div class="events-column">
        {#each cluster.events as event}
          <div class="event-card" class:synchronic={event.isSynchrnic}>
            <!-- Header -->
            <div class="event-header">
              <div class="event-title-area">
                {#if event.isSynchrnic && event.civilization}
                  <span class="sync-badge">{event.civilization}</span>
                {/if}
                <h4 class="event-title">{event.title}</h4>
              </div>
              <CurationBadge tier={event.tier} size="sm" />
            </div>

            <!-- Description -->
            <p class="event-desc">{event.description}</p>

            <!-- Dimension Chips -->
            <div class="chips-area">
              {#if event.locations && event.locations.length > 0}
                {#each event.locations as loc}
                  <a href="/location/{encodeURIComponent(loc.name)}" class="chip chip-location" title={loc.type || 'Lokasi'}>
                    <span class="chip-icon">📍</span>
                    <span>{loc.name}</span>
                  </a>
                {/each}
              {/if}

              {#if event.actors && event.actors.length > 0}
                {#each event.actors as actor}
                  <a href="/actor/{encodeURIComponent(actor.name)}" class="chip chip-actor" title={actor.role || 'Pelaku'}>
                    <span class="chip-icon">👤</span>
                    <span>{actor.name}</span>
                  </a>
                {/each}
              {/if}

              {#if event.sources && event.sources.length > 0}
                {#each event.sources as src}
                  <a href="/source/{src.id}" class="chip chip-source" title="Rujukan">
                    <span class="chip-icon">📄</span>
                    <span>{src.title}</span>
                  </a>
                {/each}
              {/if}
            </div>

            <!-- Footer -->
            {#if event.uuid}
              <div class="event-footer">
                <a href="/event/{event.uuid}" class="detail-link">
                  Lihat Detail Peristiwa →
                </a>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <span class="empty-icon">🕰️</span>
      <p>Belum ada peristiwa untuk ditampilkan pada era ini.</p>
    </div>
  {/each}
</div>

<style>
  .timeline-container {
    display: flex;
    flex-direction: column;
    gap: 0;
    position: relative;
  }

  .year-cluster {
    display: flex;
    gap: 24px;
    position: relative;
    min-height: 60px;
  }

  /* Year Marker (left axis) */
  .year-marker {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 120px;
    flex-shrink: 0;
    position: relative;
    padding-top: 4px;
  }

  .year-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #d4a853;
    border: 3px solid rgba(13, 17, 23, 0.9);
    box-shadow: 0 0 10px rgba(212, 168, 83, 0.5);
    z-index: 2;
    flex-shrink: 0;
  }

  .year-label {
    font-size: 11px;
    font-weight: 800;
    color: #d4a853;
    white-space: nowrap;
    margin-top: 6px;
    text-align: center;
    letter-spacing: 0.02em;
  }

  .year-connector {
    position: absolute;
    top: 16px;
    left: 50%;
    transform: translateX(-50%);
    width: 2px;
    height: calc(100% + 0px);
    background: linear-gradient(180deg, rgba(212, 168, 83, 0.4), rgba(212, 168, 83, 0.08));
    z-index: 1;
  }

  /* Events Column */
  .events-column {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding-bottom: 28px;
  }

  /* Event Card */
  .event-card {
    padding: 16px 20px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.06);
    background: rgba(255, 255, 255, 0.02);
    backdrop-filter: blur(12px);
    display: flex;
    flex-direction: column;
    gap: 10px;
    transition: all 0.25s ease;
  }

  .event-card:hover {
    border-color: rgba(212, 168, 83, 0.25);
    background: rgba(255, 255, 255, 0.035);
    transform: translateX(4px);
  }

  .event-card.synchronic {
    border-left: 3px solid rgba(34, 211, 238, 0.35);
    background: rgba(34, 211, 238, 0.02);
  }

  .event-card.synchronic:hover {
    border-color: rgba(34, 211, 238, 0.4);
    border-left-color: rgba(34, 211, 238, 0.6);
  }

  /* Header */
  .event-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 8px;
  }

  .event-title-area {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .sync-badge {
    font-size: 9px;
    font-weight: 700;
    color: #22d3ee;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 2px 8px;
    background: rgba(34, 211, 238, 0.08);
    border: 1px solid rgba(34, 211, 238, 0.15);
    border-radius: 4px;
    width: fit-content;
  }

  .event-title {
    font-size: 13px;
    font-weight: 800;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.3;
    margin: 0;
  }

  .event-desc {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.45);
    line-height: 1.6;
    margin: 0;
  }

  /* Dimension Chips */
  .chips-area {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 10px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 600;
    text-decoration: none;
    transition: all 0.2s ease;
    border: 1px solid transparent;
  }

  .chip-icon {
    font-size: 11px;
  }

  .chip-location {
    color: #f59e0b;
    background: rgba(245, 158, 11, 0.06);
    border-color: rgba(245, 158, 11, 0.12);
  }
  .chip-location:hover {
    background: rgba(245, 158, 11, 0.12);
    border-color: rgba(245, 158, 11, 0.3);
  }

  .chip-actor {
    color: #10b981;
    background: rgba(16, 185, 129, 0.06);
    border-color: rgba(16, 185, 129, 0.12);
  }
  .chip-actor:hover {
    background: rgba(16, 185, 129, 0.12);
    border-color: rgba(16, 185, 129, 0.3);
  }

  .chip-source {
    color: #8b5cf6;
    background: rgba(139, 92, 246, 0.06);
    border-color: rgba(139, 92, 246, 0.12);
  }
  .chip-source:hover {
    background: rgba(139, 92, 246, 0.12);
    border-color: rgba(139, 92, 246, 0.3);
  }

  /* Footer */
  .event-footer {
    padding-top: 8px;
    border-top: 1px solid rgba(255, 255, 255, 0.04);
  }

  .detail-link {
    font-size: 10px;
    font-weight: 700;
    color: #d4a853;
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .detail-link:hover {
    color: #e8c068;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 48px 24px;
    text-align: center;
  }

  .empty-icon {
    font-size: 32px;
    opacity: 0.3;
    margin-bottom: 12px;
  }

  .empty-state p {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.35);
    font-weight: 500;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .year-marker {
      width: 72px;
    }
    .year-label {
      font-size: 9px;
    }
    .year-cluster {
      gap: 12px;
    }
    .event-card {
      padding: 12px 14px;
    }
  }
</style>
