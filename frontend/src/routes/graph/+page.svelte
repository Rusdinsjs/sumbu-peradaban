<script lang="ts">
  import GraphExplorer from '$lib/components/GraphExplorer.svelte';

  let selectedNode = $state<any>(null);

  function handleNodeSelect(nodeData: any) {
    selectedNode = nodeData;
  }
</script>

<div class="w-full h-[calc(100vh-80px)] flex gap-6 animate-fade-in">
  <!-- Left panel: Controls and Info -->
  <div class="w-80 flex flex-col gap-5 flex-shrink-0 h-full">
    <!-- Graph Info/Legend -->
    <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col gap-4">
      <h2 class="text-base font-extrabold text-gold-400">Peta Jaringan Peradaban</h2>
      <p class="text-xs text-text-secondary leading-relaxed">
        Visualisasi graf pengetahuan yang menghubungkan Peristiwa, Tokoh (Aktor), dan Lokasi Sejarah.
      </p>

      <!-- Legend -->
      <div class="flex flex-col gap-2.5 border-t border-border/10 pt-3">
        <div class="flex items-center gap-3">
          <span class="w-3.5 h-3.5 bg-gold-500 rounded border border-surface shadow-[0_0_6px_rgba(212,168,83,0.4)]"></span>
          <span class="text-xs text-text-primary font-medium">Peristiwa Sejarah</span>
        </div>
        <div class="flex items-center gap-3">
          <span class="w-3.5 h-3.5 bg-emerald-500 rounded-full border border-surface shadow-[0_0_6px_rgba(16,185,129,0.4)]"></span>
          <span class="text-xs text-text-primary font-medium">Tokoh / Aktor</span>
        </div>
        <div class="flex items-center gap-3">
          <span class="w-3.5 h-3.5 bg-amber-500 transform rotate-45 border border-surface shadow-[0_0_6px_rgba(245,158,11,0.4)]"></span>
          <span class="text-xs text-text-primary font-medium">Lokasi Geografis</span>
        </div>
      </div>
    </div>

    <!-- Node Detail Inspector -->
    <div class="glass p-5 rounded-2xl border border-border/10 flex-1 overflow-y-auto">
      {#if selectedNode}
        <div class="flex flex-col gap-4 animate-fade-in">
          <div>
            <span class="text-[10px] uppercase font-bold tracking-wider text-gold-500">
              {selectedNode.type}
            </span>
            <h3 class="text-base font-bold text-text-primary mt-1">
              {selectedNode.label}
            </h3>
          </div>

          <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 flex flex-col gap-2">
            <div class="flex justify-between text-xs">
              <span class="text-text-secondary">Status Curation:</span>
              <span class="text-gold-400 capitalize font-bold">{selectedNode.tier}</span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-text-secondary">Sumbu Pivot:</span>
              <span class="text-emerald-400 font-bold">Dunia Islam</span>
            </div>
          </div>

          <div>
            <span class="text-xs font-semibold text-text-secondary block mb-1">Deskripsi Ringkas:</span>
            <p class="text-xs text-text-muted leading-relaxed">
              Tokoh atau peristiwa ini terhubung dalam Center-Outward Architecture Sumbu Peradaban, memperlihatkan pengaruh global sejarah Islam.
            </p>
          </div>

          <a 
            href="/event/{selectedNode.id}" 
            class="mt-2 w-full py-2.5 rounded-xl gradient-gold text-surface text-xs font-bold text-center hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all"
          >
            Lihat Analisis Detail →
          </a>
        </div>
      {:else}
        <div class="h-full flex flex-col items-center justify-center text-center p-4">
          <span class="text-3xl text-border mb-3">🕸️</span>
          <p class="text-xs text-text-secondary font-medium">Pilih salah satu node pada graf untuk melakukan inspeksi data.</p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Right area: Full Graph Visualization -->
  <div class="flex-1 h-full glass rounded-2xl border border-border/10 overflow-hidden relative p-4">
    <GraphExplorer onNodeSelect={handleNodeSelect} />
  </div>
</div>
