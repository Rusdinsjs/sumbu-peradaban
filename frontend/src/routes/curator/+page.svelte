<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  // Mock curator review queue
  let selectedId = $state('1');
  const queue = [
    { id: '1', title: 'Perang Yarmuk', type: 'event', tier: 'draft', submittedBy: 'AI Pipeline', date: '29 Mei 2026' },
    { id: '2', title: 'Khalifah Harun al-Rasyid', type: 'actor', tier: 'verified', submittedBy: 'Historian Agent', date: '29 Mei 2026' },
    { id: '3', title: 'Baitul Hikmah', type: 'location', tier: 'draft', submittedBy: 'Architect Agent', date: '28 Mei 2026' },
  ];

  const selectedItem = $derived(queue.find(item => item.id === selectedId));
  let isApproved = $state(false);

  function approve() {
    isApproved = true;
  }

  function reset() {
    isApproved = false;
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Page Header -->
  <div class="glass p-6 rounded-2xl border border-border/10">
    <h1 class="text-xl font-extrabold text-gold-400">Curator Panel</h1>
    <p class="text-xs text-text-secondary leading-relaxed mt-1">
      Pusat peninjauan data oleh Kurator Manusia (BozzQ) untuk verifikasi, peningkatan derajat kurasi (Curation Tier), dan penanganan pertentangan sejarah.
    </p>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">
    <!-- Queue Sidebar (2/5) -->
    <div class="lg:col-span-2 flex flex-col gap-4">
      <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col gap-3 flex-1">
        <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider">Antrean Peninjauan</h3>
        
        <div class="flex flex-col gap-2">
          {#each queue as item}
            <button
              onclick={() => { selectedId = item.id; reset(); }}
              class="w-full p-4 rounded-xl text-left border transition-all flex flex-col gap-2 bg-navy-950/60 {selectedId === item.id ? 'border-gold-500/40 bg-navy-900/40' : 'border-border/10 hover:border-gold-500/20'}"
            >
              <div class="flex justify-between items-center">
                <span class="text-[10px] uppercase font-bold tracking-wider text-text-muted">{item.type}</span>
                <CurationBadge tier={item.tier as any} size="sm" />
              </div>
              <h4 class="text-xs font-bold text-text-primary">{item.title}</h4>
              <div class="flex justify-between text-[9px] text-text-muted mt-2 border-t border-border/5 pt-2">
                <span>Oleh: {item.submittedBy}</span>
                <span>{item.date}</span>
              </div>
            </button>
          {/each}
        </div>
      </div>
    </div>

    <!-- Inspector Details (3/5) -->
    <div class="lg:col-span-3">
      {#if selectedItem}
        <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-5 h-full justify-between">
          <div class="flex flex-col gap-4">
            <div class="flex justify-between items-center border-b border-border/10 pb-4">
              <div>
                <span class="text-[10px] uppercase font-bold tracking-wider text-gold-500">Pemeriksaan Entitas</span>
                <h2 class="text-lg font-bold text-text-primary mt-1">{selectedItem.title}</h2>
              </div>
              <CurationBadge tier={selectedItem.tier as any} size="md" />
            </div>

            <!-- Details checklist -->
            <div class="flex flex-col gap-3">
              <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10">
                <span class="text-[10px] text-text-secondary block">Metode Interpretasi:</span>
                <span class="text-xs font-medium text-text-primary mt-1 block">Silang data naskah sejarah Al-Tabari & Ibn Kathir</span>
              </div>
              <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10">
                <span class="text-[10px] text-text-secondary block">Ambang Batas Kepercayaan (Consensus Score):</span>
                <span class="text-xs font-extrabold text-emerald-400 mt-1 block">89% Valid</span>
              </div>
            </div>

            <!-- Conflict handling / multiple perspectives warning -->
            <div class="p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-xs leading-relaxed text-text-secondary">
              ⚠️ <span class="font-bold text-gold-400">Pemberitahuan Governance:</span> Jika terdapat pertentangan antarsumber klasik, simpan sebagai <span class="text-gold-400 font-bold">"Multiple Perspectives"</span> alih-alih melakukan penghapusan data sepihak.
            </div>
          </div>

          {#if isApproved}
            <div class="p-4 rounded-xl bg-emerald-500/15 border border-emerald-500/20 text-center animate-fade-in">
              <p class="text-xs font-bold text-emerald-400">Entitas berhasil dipromosikan ke Tier berikutnya! ✅</p>
            </div>
          {:else}
            <!-- Actions -->
            <div class="flex flex-col sm:flex-row gap-3 pt-4 border-t border-border/10">
              <button
                onclick={approve}
                class="flex-1 py-3 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all"
              >
                Promosikan Tingkat Kurasi
              </button>
              <button
                class="flex-1 py-3 rounded-xl glass border border-amber-500/20 text-gold-500 font-bold text-xs hover:bg-navy-900 transition-all"
              >
                Tandai Sebagai "Perspektif Ganda"
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>
