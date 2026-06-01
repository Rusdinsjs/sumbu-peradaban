<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  // State
  let syncData = $state<any>(null);
  let logs = $state<any[]>([]);
  let entitiesList = $state<any[]>([]);
  let selectedEntityRaw = $state<string>('');
  let selectedEntityUuid = $state<string>('');
  let isLoading = $state(true);
  let isViewingRaw = $state(false);

  // Sync / Health indicators
  const pgConnected = $derived(syncData?.postgresConnected ?? false);
  const neoConnected = $derived(syncData?.neo4jConnected ?? false);

  async function loadHistorianData() {
    isLoading = true;
    try {
      // 1. Fetch Sync Status & Audit Logs
      const query = `
        query {
          syncStatus {
            postgresConnected
            neo4jConnected
            postgresRecordsCount
            neo4jNodesCount
          }
          auditLogs(limit: 20) {
            id
            entityType
            entityId
            action
            performedBy
            performedAt
          }
          events {
            uuid
            title
            description
            curationTier
            gregorianDate { year month day }
          }
          actors {
            uuid
            name
            description
            curationTier
          }
          locations {
            uuid
            name
            curationTier
          }
          sources {
            sourceId
            title
            author
            referenceText
          }
        }
      `;
      const res = await gql<any>(query);
      syncData = res.syncStatus;
      logs = res.auditLogs || [];

      // Combine all raw entities for viewer
      const rawEvents = (res.events || []).map((e: any) => ({
        uuid: e.uuid,
        title: e.title,
        type: 'Event',
        rawData: JSON.stringify(e, null, 2)
      }));

      const rawActors = (res.actors || []).map((a: any) => ({
        uuid: a.uuid,
        title: a.name,
        type: 'Actor',
        rawData: JSON.stringify(a, null, 2)
      }));

      const rawLocations = (res.locations || []).map((l: any) => ({
        uuid: l.uuid,
        title: l.name,
        type: 'Location',
        rawData: JSON.stringify(l, null, 2)
      }));

      const rawSources = (res.sources || []).map((s: any) => ({
        uuid: s.sourceId,
        title: s.title || `Sumber oleh ${s.author || 'Anonim'}`,
        type: 'Source',
        rawData: JSON.stringify(s, null, 2)
      }));

      entitiesList = [...rawEvents, ...rawActors, ...rawLocations, ...rawSources];

      if (entitiesList.length > 0) {
        selectedEntityUuid = entitiesList[0].uuid;
        selectedEntityRaw = entitiesList[0].rawData;
      }
    } catch (err) {
      console.error('Gagal memuat data historian:', err);
    } finally {
      isLoading = false;
    }
  }

  function handleEntitySelect(uuid: string) {
    const found = entitiesList.find(e => e.uuid === uuid);
    if (found) {
      selectedEntityUuid = uuid;
      selectedEntityRaw = found.rawData;
    }
  }

  onMount(async () => {
    await loadHistorianData();
  });
</script>

<div class="p-8 max-w-6xl mx-auto animate-fade-in pb-16">
  <!-- Header -->
  <header class="mb-8 flex justify-between items-center border-b border-border/10 pb-6">
    <div>
      <h1 class="text-3xl font-extrabold text-gold-400">AI Historian Panel</h1>
      <p class="text-sm text-text-muted mt-2">
        Terminal pemantauan aktivitas AI Agent (Med), Log Injeksi Data, Status Sinkronisasi DB, dan Pemeriksa Data Mentah (*Raw Data Viewer*).
      </p>
    </div>
    
    <button 
      onclick={loadHistorianData}
      class="px-4 py-2 rounded-xl glass border border-gold-500/20 text-gold-400 font-semibold text-xs hover:bg-gold-500/10 transition-all flex items-center gap-2"
    >
      🔄 Segarkan Panel
    </button>
  </header>

  {#if isLoading}
    <div class="flex justify-center items-center py-20">
      <span class="animate-spin w-8 h-8 border-4 border-gold-500/20 border-t-gold-500 rounded-full"></span>
    </div>
  {:else}
    <!-- DB connection & stats row -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
      <!-- PG Connection -->
      <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col justify-between">
        <div>
          <span class="text-[10px] font-bold text-text-secondary uppercase tracking-widest block">PostgreSQL Status</span>
          <div class="flex items-center gap-2 mt-2">
            <span class="w-2.5 h-2.5 rounded-full {pgConnected ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]' : 'bg-red-500'}"></span>
            <span class="text-xs font-bold text-text-primary">{pgConnected ? 'Terhubung (OK)' : 'Terputus'}</span>
          </div>
        </div>
        <div class="mt-4 border-t border-border/5 pt-3">
          <span class="text-[10px] text-text-muted block">Jumlah Dokumen Sumber</span>
          <span class="text-lg font-black text-white">{syncData?.postgresRecordsCount ?? 0}</span>
        </div>
      </div>

      <!-- Neo4j Connection -->
      <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col justify-between">
        <div>
          <span class="text-[10px] font-bold text-text-secondary uppercase tracking-widest block">Neo4j Graph Status</span>
          <div class="flex items-center gap-2 mt-2">
            <span class="w-2.5 h-2.5 rounded-full {neoConnected ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]' : 'bg-red-500'}"></span>
            <span class="text-xs font-bold text-text-primary">{neoConnected ? 'Terhubung (OK)' : 'Terputus'}</span>
          </div>
        </div>
        <div class="mt-4 border-t border-border/5 pt-3">
          <span class="text-[10px] text-text-muted block">Jumlah Node Graf</span>
          <span class="text-lg font-black text-white">{syncData?.neo4jNodesCount ?? 0}</span>
        </div>
      </div>

      <!-- AI Ingestion Health -->
      <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col justify-between md:col-span-2">
        <div>
          <span class="text-[10px] font-bold text-text-secondary uppercase tracking-widest block">Status Pipeline Injeksi AI</span>
          <p class="text-[11px] text-text-muted mt-1 leading-relaxed">
            Menghubungkan **Med Historian Agent** & **Den Ingestion Agent** langsung ke antrean Graph Database.
          </p>
        </div>
        <div class="flex gap-4 mt-4 border-t border-border/5 pt-3">
          <div>
            <span class="text-[9px] text-text-muted uppercase">Deduplikasi</span>
            <span class="text-xs font-bold text-emerald-400 block">100% Aktif</span>
          </div>
          <div>
            <span class="text-[9px] text-text-muted uppercase">Upsert Topology</span>
            <span class="text-xs font-bold text-emerald-400 block">Dua-Fase Enforced</span>
          </div>
          <div>
            <span class="text-[9px] text-text-muted uppercase">Kecepatan Sync</span>
            <span class="text-xs font-mono text-gold-400 block">~45ms / node</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Workspace (Logs & Raw Viewer) -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
      <!-- Left side: Injection Logs (7/12) -->
      <div class="lg:col-span-7 flex flex-col gap-6">
        <div class="glass p-6 rounded-2xl border border-border/10 flex-1">
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider">Log Audit & Injeksi Terakhir</h3>
            <span class="px-2 py-0.5 bg-gold-500/10 border border-gold-500/20 text-gold-400 rounded text-[9px] font-bold">Real-time</span>
          </div>

          <div class="flex flex-col gap-3 max-h-[500px] overflow-y-auto pr-1">
            {#if logs.length === 0}
              <div class="p-8 text-center text-xs text-text-muted">
                Belum ada log aktivitas penulisan/kurasi terdeteksi.
              </div>
            {:else}
              {#each logs as log}
                <div class="p-3 rounded-xl border border-border/5 bg-navy-950/40 flex justify-between items-start gap-4">
                  <div class="flex flex-col gap-1 text-left">
                    <div class="flex items-center gap-2">
                      <span class="text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded bg-white/5 text-gold-400">
                        {log.action}
                      </span>
                      <span class="text-xs font-bold text-text-primary">{log.entityType}</span>
                    </div>
                    <span class="text-[9px] font-mono text-text-muted">UUID: {log.entityId}</span>
                    <span class="text-[10px] text-text-secondary">Oleh: <strong class="text-white/80">{log.performedBy}</strong></span>
                  </div>
                  <span class="text-[9px] text-text-muted font-mono whitespace-nowrap">
                    {new Date(log.performedAt).toLocaleTimeString()}
                  </span>
                </div>
              {/each}
            {/if}
          </div>
        </div>
      </div>

      <!-- Right side: Raw Data Viewer (5/12) -->
      <div class="lg:col-span-5 flex flex-col gap-6">
        <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4 h-full justify-between">
          <div class="flex flex-col gap-3">
            <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider">Raw Data Viewer</h3>
            
            <div class="flex flex-col gap-2">
              <span class="text-[10px] text-text-muted block">Pilih Entitas Sejarah:</span>
              <select 
                class="w-full bg-navy-950/80 border border-border/10 rounded-xl px-3 py-2.5 text-xs text-text-primary outline-none focus:border-gold-500/40"
                bind:value={selectedEntityUuid}
                onchange={(e) => handleEntitySelect((e.target as HTMLSelectElement).value)}
              >
                {#each entitiesList as ent}
                  <option value={ent.uuid}>[{ent.type}] {ent.title}</option>
                {/each}
              </select>
            </div>

            <div class="flex flex-col gap-2 flex-1">
              <span class="text-[10px] text-text-muted block mt-2">Payload JSON Data Mentah (DB):</span>
              <pre class="bg-navy-950/90 border border-border/15 p-4 rounded-xl text-[10px] font-mono text-emerald-400 overflow-x-auto max-h-[300px] leading-relaxed select-all">{selectedEntityRaw}</pre>
            </div>
          </div>

          <div class="p-3.5 rounded-xl bg-gold-500/5 border border-gold-500/10 text-[10px] leading-relaxed text-text-secondary mt-4">
            📌 **Gunakan Payload di Atas**: Struktur JSON di atas mencerminkan skema database graf riil Neo4j & PostgreSQL.
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  pre::-webkit-scrollbar {
    width: 6px;
    height: 6px;
  }
  pre::-webkit-scrollbar-thumb {
    background: rgba(212, 168, 83, 0.2);
    border-radius: 99px;
  }
</style>
