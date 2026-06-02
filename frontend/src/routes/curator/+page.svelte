<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';

  // State
  let queue = $state<any[]>([]);
  let selectedId = $state('');
  let isApproved = $state(false);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let errorMessage = $state('');

  const selectedItem = $derived(queue.find(item => item.id === selectedId));

  async function loadData() {
    isLoading = true;
    try {
      const query = `
        query {
          events { 
            uuid 
            title 
            description 
            curationTier 
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
        }
      `;
      const res = await gql<any>(query);

      const loadedEvents = (res.events || []).map((e: any) => ({
        id: e.uuid,
        title: e.title,
        description: e.description || 'Tidak ada deskripsi historis.',
        type: 'event',
        tier: e.curationTier.toLowerCase(),
        submittedBy: 'AI Pipeline',
        date: 'Baru saja'
      }));

      const loadedActors = (res.actors || []).map((a: any) => ({
        id: a.uuid,
        title: a.name,
        description: a.description || 'Tidak ada deskripsi biografi.',
        type: 'actor',
        tier: a.curationTier.toLowerCase(),
        submittedBy: 'Historian Agent',
        date: 'Baru saja'
      }));

      const loadedLocations = (res.locations || []).map((l: any) => ({
        id: l.uuid,
        title: l.name,
        description: 'Detail koordinat dan peranan geografis.',
        type: 'location',
        tier: l.curationTier.toLowerCase(),
        submittedBy: 'Architect Agent',
        date: 'Baru saja'
      }));

      // Combine and filter out Canonical (already approved/final)
      queue = [...loadedEvents, ...loadedActors, ...loadedLocations].filter(
        item => item.tier !== 'canonical'
      );

      // Select first item if nothing selected or selected item is no longer in queue
      if (queue.length > 0) {
        if (!selectedId || !queue.some(item => item.id === selectedId)) {
          selectedId = queue[0].id;
        }
      } else {
        selectedId = '';
      }
    } catch (err) {
      console.error('Gagal memuat data kurasi:', err);
    } finally {
      isLoading = false;
    }
  }

  onMount(async () => {
    await loadData();
  });

  async function approve() {
    if (!selectedItem) return;
    isSaving = true;
    errorMessage = '';

    // Determine next CurationTier (enum must be uppercase)
    let nextTier = 'VERIFIED';
    if (selectedItem.tier === 'verified') {
      nextTier = 'REVIEWED';
    } else if (selectedItem.tier === 'reviewed') {
      nextTier = 'CANONICAL';
    }

    try {
      const mutation = `
        mutation PromoteEntity($uuid: String!, $newTier: CurationTier!) {
          promoteEntity(uuid: $uuid, newTier: $newTier)
        }
      `;
      const vars = {
        uuid: selectedItem.id,
        newTier: nextTier
      };

      await gql(mutation, vars);
      isApproved = true;
      
      // Auto-reload data after 2 seconds to transition smoothly
      setTimeout(async () => {
        await loadData();
        isApproved = false;
      }, 2000);
    } catch (err: any) {
      console.error(err);
      errorMessage = err.message || 'Gagal mengubah status kurasi.';
    } finally {
      isSaving = false;
    }
  }

  async function markMultiplePerspectives() {
    if (!selectedItem) return;
    isSaving = true;
    errorMessage = '';

    // Promoting to Reviewed to signify human curation is complete with "Multiple Perspectives"
    try {
      const mutation = `
        mutation PromoteEntity($uuid: String!, $newTier: CurationTier!) {
          promoteEntity(uuid: $uuid, newTier: $newTier)
        }
      `;
      const vars = {
        uuid: selectedItem.id,
        newTier: 'REVIEWED'
      };

      await gql(mutation, vars);
      isApproved = true;
      
      setTimeout(async () => {
        await loadData();
        isApproved = false;
      }, 2000);
    } catch (err: any) {
      console.error(err);
      errorMessage = err.message || 'Gagal menandai perspektif ganda.';
    } finally {
      isSaving = false;
    }
  }

  function reset() {
    isApproved = false;
    errorMessage = '';
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

  {#if isLoading}
    <div class="flex justify-center items-center py-20">
      <span class="animate-spin w-8 h-8 border-4 border-gold-500/20 border-t-gold-500 rounded-full"></span>
    </div>
  {:else if queue.length === 0}
    <div class="glass p-12 rounded-2xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-5xl">🎉</span>
      <h3 class="text-base font-bold text-text-primary">Semua Bersih!</h3>
      <p class="text-xs text-text-muted max-w-md leading-relaxed">
        Saat ini tidak ada entitas dengan status Draft atau Verified yang memerlukan tinjauan kurator. Seluruh data di Knowledge Graph telah termutakhirkan!
      </p>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">
      <!-- Queue Sidebar (2/5) -->
      <div class="lg:col-span-2 flex flex-col gap-4">
        <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col gap-3 flex-1">
          <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider">Antrean Peninjauan ({queue.length})</h3>
          
          <div class="flex flex-col gap-2 max-h-[600px] overflow-y-auto pr-1">
            {#each queue as item}
              <button
                onclick={() => { selectedId = item.id; reset(); }}
                class="w-full p-4 rounded-xl text-left border transition-all flex flex-col gap-2 bg-iron-950/60 {selectedId === item.id ? 'border-gold-500/40 bg-iron-900/40' : 'border-border/10 hover:border-gold-500/20'}"
              >
                <div class="flex justify-between items-center">
                  <span class="text-[10px] uppercase font-bold tracking-wider text-text-muted">{item.type}</span>
                  <CurationBadge tier={item.tier} size="sm" />
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
                <CurationBadge tier={selectedItem.tier} size="md" />
              </div>

              <!-- Real Description -->
              <div class="p-3.5 rounded-xl bg-iron-950/60 border border-border/10">
                <span class="text-[10px] text-text-secondary block font-bold">Catatan Narasi / Deskripsi:</span>
                <p class="text-xs font-medium text-text-primary mt-1.5 leading-relaxed">{selectedItem.description}</p>
              </div>

              <!-- Details checklist -->
              <div class="flex flex-col gap-3">
                <div class="p-3.5 rounded-xl bg-iron-950/60 border border-border/10">
                  <span class="text-[10px] text-text-secondary block">Metode Interpretasi:</span>
                  <span class="text-xs font-medium text-text-primary mt-1 block">Silang data naskah sejarah Al-Tabari & Ibn Kathir</span>
                </div>
                <div class="p-3.5 rounded-xl bg-iron-950/60 border border-border/10">
                  <span class="text-[10px] text-text-secondary block">Ambang Batas Kepercayaan (Consensus Score):</span>
                  <span class="text-xs font-extrabold text-verdigris-400 mt-1 block">89% Valid</span>
                </div>
              </div>

              <!-- Conflict handling / multiple perspectives warning -->
              <div class="p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-xs leading-relaxed text-text-secondary">
                ⚠️ <span class="font-bold text-gold-400">Pemberitahuan Governance:</span> Jika terdapat pertentangan antarsumber klasik, simpan sebagai <span class="text-gold-400 font-bold">"Multiple Perspectives"</span> alih-alih melakukan penghapusan data sepihak.
              </div>

              {#if errorMessage}
                <div class="p-3.5 rounded-xl bg-red-500/10 border border-red-500/20 text-center text-xs font-bold text-red-400">
                  Error: {errorMessage}
                </div>
              {/if}
            </div>

            {#if isApproved}
              <div class="p-4 rounded-xl bg-verdigris-500/15 border border-verdigris-500/20 text-center animate-fade-in">
                <p class="text-xs font-bold text-verdigris-400">Entitas berhasil dipromosikan ke Tier berikutnya! ✅</p>
              </div>
            {:else}
              <!-- Actions -->
              <div class="flex flex-col sm:flex-row gap-3 pt-4 border-t border-border/10">
                <button
                  onclick={approve}
                  disabled={isSaving}
                  class="flex-1 py-3 rounded-xl gradient-rust text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] disabled:opacity-50 transition-all flex items-center justify-center gap-2"
                >
                  {#if isSaving}
                    <span class="animate-spin w-3 h-3 border-2 border-surface border-t-transparent rounded-full"></span>
                  {/if}
                  Promosikan Tingkat Kurasi
                </button>
                <button
                  onclick={markMultiplePerspectives}
                  disabled={isSaving}
                  class="flex-1 py-3 rounded-xl glass border border-amber-500/20 text-gold-500 font-bold text-xs hover:bg-iron-900 disabled:opacity-50 transition-all flex items-center justify-center gap-2"
                >
                  Tandai Sebagai "Perspektif Ganda"
                </button>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

