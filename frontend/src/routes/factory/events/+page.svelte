<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';

  let activeTab = $state('manual'); // 'manual' | 'ai'

  // --- Data States ---
  let events: any[] = $state([]);
  let actors: any[] = $state([]);
  let locations: any[] = $state([]);
  let sources: any[] = $state([]);
  let isLoading = $state(true);

  // --- UI States ---
  let showForm = $state(false);
  let formMode = $state('create'); // 'create' | 'edit'
  let notification = $state<{type: 'success' | 'error', message: string} | null>(null);

  function showNotification(type: 'success' | 'error', message: string) {
    notification = { type, message };
    setTimeout(() => {
      notification = null;
    }, 4000);
  }

  // --- AI Agent State ---
  let textInput = $state('');
  let isProcessingAI = $state(false);
  let processStep = $state(0); 

  // --- Unified Form State (Event + Relational Entities) ---
  let formData = $state({
    uuid: '',
    title: '',
    description: '',
    hijriYear: '',
    gregorianYear: '',
    precision: 'EXACT',
    isPivot: false,
    selectedActors: [] as string[],
    selectedLocations: [] as string[],
    selectedSources: [] as string[]
  });
  let submitError = $state('');
  let isSaving = $state(false);

  async function deleteEvent(uuid: string) {
    if (!confirm('Apakah Anda yakin ingin menghapus peristiwa ini? Seluruh relasi juga akan terhapus.')) return;
    try {
      const delMutation = `mutation { deleteEvent(uuid: "${uuid}") }`;
      await gql(delMutation) as any;
      showNotification('success', 'Peristiwa berhasil dihapus.');
      await loadData();
    } catch (err: any) {
      showNotification('error', err.message || 'Gagal menghapus peristiwa.');
    }
  }

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    isLoading = true;
    try {
      const query = `
        query {
          events { 
            uuid 
            title 
            description 
            gregorianDate { year } 
            islamicDate { year } 
            precision 
            actors { uuid }
            locations { uuid }
            sources { sourceId }
          }
          actors { uuid name }
          locations { uuid name }
          sources { sourceId referenceText }
        }
      `;
      const res = await gql(query) as any;
      events = res.events || [];
      actors = res.actors || [];
      locations = res.locations || [];
      sources = res.sources || [];
    } catch (err) {
      console.error('Failed to load data', err);
    } finally {
      isLoading = false;
    }
  }

  function openCreateForm() {
    formMode = 'create';
    resetForm();
    showForm = true;
  }

  function openEditForm(evt: any) {
    formMode = 'edit';
    formData = {
      uuid: evt.uuid,
      title: evt.title,
      description: evt.description || '',
      hijriYear: evt.islamicDate?.year?.toString() || '',
      gregorianYear: evt.gregorianDate?.year?.toString() || '',
      precision: evt.precision || 'EXACT',
      isPivot: false,
      selectedActors: (evt.actors || []).map((a: any) => a.uuid),
      selectedLocations: (evt.locations || []).map((l: any) => l.uuid),
      selectedSources: (evt.sources || []).map((s: any) => s.sourceId)
    };
    submitError = '';
    showForm = true;
  }

  function resetForm() {
    formData = {
      uuid: '', title: '', description: '', hijriYear: '', gregorianYear: '', 
      precision: 'EXACT', isPivot: false, selectedActors: [], selectedLocations: [], selectedSources: []
    };
    submitError = '';
  }

  async function saveEvent() {
    isSaving = true;
    submitError = '';
    
    try {
      let eventUuid = formData.uuid;

      if (formMode === 'create') {
        const createMutation = `
          mutation CreateExtractedEvent($input: CreateEventInput!) {
            createEvent(input: $input) { uuid title }
          }
        `;
        
        const createVars = {
          input: {
            title: formData.title,
            description: formData.description,
            islamicDate: { year: parseInt(formData.hijriYear) || 0 },
            gregorianDate: { year: parseInt(formData.gregorianYear) || 0 },
            precision: formData.precision,
            isConnectedToGlobal: formData.isPivot,
            globalPivotCategory: formData.isPivot ? "Dunia Islam" : null
          }
        };

        const createRes = await gql(createMutation, createVars) as any;
        eventUuid = createRes.createEvent.uuid;
      } else {
        const updateMutation = `
          mutation UpdateExtractedEvent($uuid: String!, $input: UpdateEventInput!) {
            updateEvent(uuid: $uuid, input: $input) { uuid title }
          }
        `;
        
        const updateVars = {
          uuid: eventUuid,
          input: {
            title: formData.title,
            description: formData.description,
            isConnectedToGlobal: formData.isPivot,
            globalPivotCategory: formData.isPivot ? "Dunia Islam" : null
          }
        };

        await gql(updateMutation, updateVars);
        
        // Clear old relations before writing new ones
        await gql(`mutation { clearEventRelations(eventUuid: "${eventUuid}") }`);
      }

      // 2. Link Actors
      for (const actorUuid of formData.selectedActors) {
        await gql(`mutation { linkActorToEvent(actorUuid: "${actorUuid}", eventUuid: "${eventUuid}", role: "PARTICIPANT") }`);
      }

      // 3. Link Locations
      for (const locUuid of formData.selectedLocations) {
        await gql(`mutation { linkEventToLocation(eventUuid: "${eventUuid}", locationUuid: "${locUuid}") }`);
      }

      // 4. Link Sources
      for (const sourceId of formData.selectedSources) {
        await gql(`mutation { linkEventToSource(eventUuid: "${eventUuid}", sourceId: "${sourceId}") }`);
      }

      showForm = false;
      showNotification('success', formMode === 'create' ? 'Data Peristiwa berhasil direkam secara permanen ke Knowledge Graph!' : 'Perubahan peristiwa berhasil diperbarui!');
      await loadData();
    } catch (err: any) {
      submitError = err.message || 'Gagal menyimpan peristiwa.';
      showNotification('error', submitError);
    } finally {
      isSaving = false;
    }
  }

  // --- AI Mock logic (unchanged) ---
  function startProcessing() {
    if (!textInput.trim()) return;
    isProcessingAI = true;
    processStep = 1;
    setTimeout(() => {
      formData.title = 'Hijrah ke Madinah';
      formData.description = textInput.substring(0, 150) + '...';
      formData.hijriYear = '1';
      formData.gregorianYear = '622';
      processStep = 2;
      isProcessingAI = false;
    }, 2500);
  }
  
  function resetAI() {
    textInput = ''; processStep = 0; resetForm();
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto relative">
  <!-- Toast Notification -->
  {#if notification}
    <div class="fixed top-8 right-8 z-[100] animate-fade-in flex items-center gap-4 px-6 py-4 rounded-xl shadow-2xl glass {notification.type === 'success' ? 'border-emerald-500/50 text-emerald-400 bg-emerald-950/50' : 'border-red-500/50 text-red-400 bg-red-950/50'}">
      <span class="text-2xl">{notification.type === 'success' ? '✅' : '❌'}</span>
      <div>
        <p class="font-bold text-sm text-text-primary">{notification.type === 'success' ? 'Sukses' : 'Gagal'}</p>
        <p class="text-xs">{notification.message}</p>
      </div>
      <button onclick={() => notification = null} class="ml-4 hover:opacity-70 transition-opacity">✕</button>
    </div>
  {/if}

  <!-- Header -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
    <div>
      <h1 class="text-2xl font-extrabold text-gold-400 flex items-center gap-3">
        <span>📜</span> Manajemen Peristiwa
      </h1>
      <p class="text-xs text-text-secondary mt-1 max-w-lg">
        Pusat kurasi narasi sejarah. Hubungkan 4 Entitas Utama (Peristiwa, Tokoh, Lokasi, Sumber) untuk graf pengetahuan yang kaya.
      </p>
    </div>
    
    <div class="flex bg-navy-950/50 p-1.5 rounded-xl border border-border/10">
      <button class="px-5 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'manual' ? 'bg-gold-500/20 text-gold-400 shadow-sm' : 'text-text-muted hover:text-text-primary'}" onclick={() => {activeTab = 'manual'; showForm=false;}}>📝 Data Master (CRUD)</button>
      <button class="px-5 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'ai' ? 'bg-gold-500/20 text-gold-400 shadow-sm' : 'text-text-muted hover:text-text-primary'}" onclick={() => activeTab = 'ai'}>🤖 AI Extractor</button>
    </div>
  </div>

  {#if activeTab === 'manual'}
    {#if !showForm}
      <div class="flex justify-between items-center mb-2 mt-2">
        <h2 class="text-lg font-bold text-text-primary">Daftar Peristiwa ({events.length})</h2>
        <button onclick={openCreateForm} class="px-5 py-2 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all flex items-center gap-2">
          <span>+</span> Tambah Peristiwa
        </button>
      </div>

      {#if isLoading}
        <div class="flex justify-center p-12"><span class="animate-spin w-8 h-8 border-4 border-gold-500/20 border-t-gold-500 rounded-full"></span></div>
      {:else}
        <div class="glass rounded-2xl border border-border/10 overflow-hidden">
          {#if events.length === 0}
            <div class="p-12 text-center flex flex-col items-center justify-center">
              <div class="text-5xl mb-4 opacity-50">📂</div>
              <h3 class="text-base font-bold text-text-primary mb-2">Belum Ada Data Tersimpan</h3>
              <p class="text-xs text-text-secondary max-w-sm">Klik tombol Tambah Peristiwa di atas untuk memulai input data.</p>
            </div>
          {:else}
            <table class="w-full text-left text-sm text-text-secondary">
              <thead class="bg-surface-lighter/50 text-xs uppercase text-text-muted">
                <tr>
                  <th class="px-6 py-4 font-bold tracking-wider">Peristiwa</th>
                  <th class="px-6 py-4 font-bold tracking-wider">Tahun (M/H)</th>
                  <th class="px-6 py-4 font-bold tracking-wider">Presisi</th>
                  <th class="px-6 py-4 font-bold tracking-wider text-right">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border/10">
                {#each events as evt}
                  <tr class="hover:bg-surface-lighter/20 transition-colors">
                    <td class="px-6 py-4">
                      <div class="font-bold text-text-primary">{evt.title}</div>
                      <div class="text-[10px] truncate max-w-xs">{evt.description}</div>
                    </td>
                    <td class="px-6 py-4 text-xs">
                      {evt.gregorianDate?.year ? `${evt.gregorianDate.year} M` : '-'} / {evt.islamicDate?.year ? `${evt.islamicDate.year} H` : '-'}
                    </td>
                    <td class="px-6 py-4 text-xs font-mono bg-navy-950/30 inline-block mt-3 ml-6 rounded px-2">{evt.precision}</td>
                    <td class="px-6 py-4 text-right">
                      <div class="flex items-center justify-end gap-2">
                        <button onclick={() => openEditForm(evt)} class="text-gold-400 hover:text-gold-300 font-bold px-3 py-1 bg-gold-500/10 rounded-lg transition-colors">Edit</button>
                        <button onclick={() => deleteEvent(evt.uuid)} class="text-red-400 hover:text-red-300 font-bold px-3 py-1 bg-red-500/10 rounded-lg transition-colors">Hapus</button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      {/if}
    {:else}
      <!-- CREATE / EDIT FORM -->
      <div class="glass p-8 rounded-2xl border border-border/10">
        <h2 class="text-lg font-bold text-text-primary mb-6 border-b border-border/10 pb-4">Formulir Perekaman Peristiwa</h2>
        
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <!-- Kolom 1: Peristiwa Dasar -->
          <div class="flex flex-col gap-5">
            <h3 class="text-sm font-bold text-gold-400">1. Data Dasar Peristiwa</h3>
            
            <div class="flex flex-col gap-1.5">
              <label for="f-title" class="text-xs font-bold text-text-secondary">Judul Peristiwa *</label>
              <input id="f-title" bind:value={formData.title} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none" />
            </div>

            <div class="flex flex-col gap-1.5">
              <label for="f-desc" class="text-xs font-bold text-text-secondary">Deskripsi Historis *</label>
              <textarea id="f-desc" bind:value={formData.description} rows="4" class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none"></textarea>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="flex flex-col gap-1.5">
                <label for="f-masehi" class="text-xs font-bold text-text-secondary">Tahun Masehi</label>
                <input id="f-masehi" type="number" bind:value={formData.gregorianYear} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary" />
              </div>
              <div class="flex flex-col gap-1.5">
                <label for="f-hijri" class="text-xs font-bold text-text-secondary">Tahun Hijriah</label>
                <input id="f-hijri" type="number" bind:value={formData.hijriYear} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary" />
              </div>
            </div>
            
            <div class="flex flex-col gap-1.5">
              <label for="f-prec" class="text-xs font-bold text-text-secondary">Presisi Waktu</label>
              <select id="f-prec" bind:value={formData.precision} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary">
                <option value="EXACT">Tepat Waktu (Exact)</option>
                <option value="YEAR">Kira-kira Tahun (Year)</option>
                <option value="DECADE">Kira-kira Dekade (Decade)</option>
              </select>
            </div>
          </div>

          <!-- Kolom 2: Relasi 3 Entitas Lain -->
          <div class="flex flex-col gap-5 border-l border-border/10 pl-0 lg:pl-8">
            <h3 class="text-sm font-bold text-emerald-400">2. Relasi Graf Historis</h3>
            
            <!-- Tokoh -->
            <div class="flex flex-col gap-1.5">
              <div class="flex justify-between items-end">
                <label for="f-actors" class="text-xs font-bold text-text-secondary flex items-center gap-2"><span>👤</span> Pelaku/Tokoh Terlibat</label>
                <a href="/factory/actors" target="_blank" class="text-[10px] text-gold-400 hover:text-gold-300 font-bold flex items-center gap-1">Kelola Tokoh ↗</a>
              </div>
              <!-- Custom Multi-Select UI for Actors -->
              <div class="flex flex-col gap-2">
                {#if formData.selectedActors.length > 0}
                  <div class="flex flex-wrap gap-2 p-2 bg-navy-950/30 border border-border/5 rounded-lg min-h-[40px] items-center">
                    {#each formData.selectedActors as actorUuid}
                      {@const actor = actors.find(a => a.uuid === actorUuid)}
                      {#if actor}
                        <span class="inline-flex items-center gap-1 px-3 py-1 bg-gold-500/10 border border-gold-500/30 text-gold-400 rounded-full text-[10px] font-bold animate-fade-in">
                          {actor.name}
                          <button type="button" onclick={() => formData.selectedActors = formData.selectedActors.filter(id => id !== actorUuid)} class="hover:text-red-400 transition-colors ml-1" title="Hapus">✕</button>
                        </span>
                      {/if}
                    {/each}
                  </div>
                {/if}
                <select 
                  class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full"
                  onchange={(e) => {
                    const val = (e.target as HTMLSelectElement).value;
                    if (val && !formData.selectedActors.includes(val)) {
                      formData.selectedActors = [...formData.selectedActors, val];
                      (e.target as HTMLSelectElement).value = "";
                    }
                  }}
                >
                  <option value="">-- + Pilih Tokoh untuk Ditambahkan --</option>
                  {#each actors.filter(a => !formData.selectedActors.includes(a.uuid)) as actor}
                    <option value={actor.uuid}>{actor.name}</option>
                  {/each}
                </select>
              </div>
            </div>

            <!-- Lokasi -->
            <div class="flex flex-col gap-1.5">
              <div class="flex justify-between items-end">
                <label for="f-locs" class="text-xs font-bold text-text-secondary flex items-center gap-2"><span>🗺️</span> Lokasi Kejadian Utama</label>
                <a href="/factory/locations" target="_blank" class="text-[10px] text-emerald-400 hover:text-emerald-300 font-bold flex items-center gap-1">Kelola Lokasi ↗</a>
              </div>
              <!-- Custom Multi-Select UI for Locations -->
              <div class="flex flex-col gap-2">
                {#if formData.selectedLocations.length > 0}
                  <div class="flex flex-wrap gap-2 p-2 bg-navy-950/30 border border-border/5 rounded-lg min-h-[40px] items-center">
                    {#each formData.selectedLocations as locUuid}
                      {@const loc = locations.find(l => l.uuid === locUuid)}
                      {#if loc}
                        <span class="inline-flex items-center gap-1 px-3 py-1 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 rounded-full text-[10px] font-bold animate-fade-in">
                          {loc.name}
                          <button type="button" onclick={() => formData.selectedLocations = formData.selectedLocations.filter(id => id !== locUuid)} class="hover:text-red-400 transition-colors ml-1" title="Hapus">✕</button>
                        </span>
                      {/if}
                    {/each}
                  </div>
                {/if}
                <select 
                  class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full"
                  onchange={(e) => {
                    const val = (e.target as HTMLSelectElement).value;
                    if (val && !formData.selectedLocations.includes(val)) {
                      formData.selectedLocations = [...formData.selectedLocations, val];
                      (e.target as HTMLSelectElement).value = "";
                    }
                  }}
                >
                  <option value="">-- + Pilih Lokasi untuk Ditambahkan --</option>
                  {#each locations.filter(l => !formData.selectedLocations.includes(l.uuid)) as loc}
                    <option value={loc.uuid}>{loc.name}</option>
                  {/each}
                </select>
              </div>
            </div>

            <!-- Rujukan -->
            <div class="flex flex-col gap-1.5">
              <div class="flex justify-between items-end">
                <label for="f-srcs" class="text-xs font-bold text-text-secondary flex items-center gap-2"><span>📄</span> Sumber / Kitab Rujukan</label>
                <a href="/factory/sources" target="_blank" class="text-[10px] text-blue-400 hover:text-blue-300 font-bold flex items-center gap-1">Daftar Rujukan ↗</a>
              </div>
              <!-- Custom Multi-Select UI for Sources -->
              <div class="flex flex-col gap-2">
                {#if formData.selectedSources.length > 0}
                  <div class="flex flex-wrap gap-2 p-2 bg-navy-950/30 border border-border/5 rounded-lg min-h-[40px] items-center">
                    {#each formData.selectedSources as sourceId}
                      {@const src = sources.find(s => s.sourceId === sourceId)}
                      {#if src}
                        <span class="inline-flex items-center gap-1 px-3 py-1 bg-blue-500/10 border border-blue-500/30 text-blue-400 rounded-full text-[10px] font-bold animate-fade-in">
                          {src.referenceText.substring(0, 30)}{src.referenceText.length > 30 ? '...' : ''}
                          <button type="button" onclick={() => formData.selectedSources = formData.selectedSources.filter(id => id !== sourceId)} class="hover:text-red-400 transition-colors ml-1" title="Hapus">✕</button>
                        </span>
                      {/if}
                    {/each}
                  </div>
                {/if}
                <select 
                  class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full"
                  onchange={(e) => {
                    const val = (e.target as HTMLSelectElement).value;
                    if (val && !formData.selectedSources.includes(val)) {
                      formData.selectedSources = [...formData.selectedSources, val];
                      (e.target as HTMLSelectElement).value = "";
                    }
                  }}
                >
                  <option value="">-- + Tambahkan Kitab / Rujukan --</option>
                  {#each sources.filter(s => !formData.selectedSources.includes(s.sourceId)) as src}
                    <option value={src.sourceId}>{src.referenceText}</option>
                  {/each}
                </select>
              </div>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="mt-8 pt-6 border-t border-border/10 flex justify-end gap-4">
          <button onclick={() => showForm = false} class="px-6 py-2.5 rounded-xl border border-border/10 text-text-primary font-bold text-xs hover:bg-surface-lighter transition-all">Batal</button>
          <button onclick={saveEvent} disabled={isSaving || !formData.title} class="px-8 py-2.5 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] disabled:opacity-50 flex items-center gap-2">
            {#if isSaving} <span class="animate-spin w-3 h-3 border-2 border-surface border-t-transparent rounded-full"></span> {/if}
            Simpan Peristiwa
          </button>
        </div>
      </div>
    {/if}
  {:else}
    <!-- AI TAB -->
    <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
      <div class="flex justify-between items-center">
        <label for="h-input" class="text-xs font-bold text-text-secondary uppercase">Naskah Sejarah Mentah</label>
      </div>
      <textarea id="h-input" bind:value={textInput} rows="6" class="w-full bg-navy-950/60 border border-border/10 rounded-xl p-4 text-xs text-text-primary" placeholder="Teks catatan sejarah..."></textarea>
      <button onclick={startProcessing} disabled={isProcessingAI || !textInput} class="self-end px-8 py-3 rounded-xl gradient-gold text-surface font-bold text-xs">Jalankan AI Konsensus</button>
    </div>
  {/if}
</div>
