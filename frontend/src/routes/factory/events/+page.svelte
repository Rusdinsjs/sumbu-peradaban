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
    selectedSources: [] as string[],
    subReferences: {} as Record<string, Array<{ section: string, point: string, note: string }>>
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
            sources { sourceId subReferences }
          }
          actors { uuid name }
          locations { uuid name }
          sources { sourceId title referenceText }
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
    const subRefsMap: Record<string, any[]> = {};
    (evt.sources || []).forEach((s: any) => {
      let parsed = [];
      if (s.subReferences) {
        try {
          parsed = JSON.parse(s.subReferences);
        } catch (e) {
          console.error('Failed to parse subReferences JSON:', e);
        }
      }
      subRefsMap[s.sourceId] = parsed;
    });

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
      selectedSources: (evt.sources || []).map((s: any) => s.sourceId),
      subReferences: subRefsMap
    };
    submitError = '';
    showForm = true;
  }

  function resetForm() {
    formData = {
      uuid: '', title: '', description: '', hijriYear: '', gregorianYear: '', 
      precision: 'EXACT', isPivot: false, selectedActors: [], selectedLocations: [], selectedSources: [],
      subReferences: {}
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
            islamicDate: { year: parseInt(formData.hijriYear) || 0 },
            gregorianDate: { year: parseInt(formData.gregorianYear) || 0 },
            precision: formData.precision,
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
        const subRefsList = formData.subReferences[sourceId] || [];
        const serialized = subRefsList.length > 0 ? JSON.stringify(subRefsList) : null;
        const linkMutation = `
          mutation LinkSource($eventUuid: String!, $sourceId: String!, $subRefs: String) {
            linkEventToSource(eventUuid: $eventUuid, sourceId: $sourceId, subReferences: $subRefs)
          }
        `;
        await gql(linkMutation, { eventUuid, sourceId, subRefs: serialized });
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
    <div class="fixed top-8 right-8 z-[100] animate-fade-in flex items-center gap-4 px-6 py-4 rounded-xl shadow-2xl glass {notification.type === 'success' ? 'border-verdigris-500/50 text-verdigris-400 bg-verdigris-950/50' : 'border-red-500/50 text-red-400 bg-red-950/50'}">
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
    
    <div class="flex bg-iron-950/50 p-1.5 rounded-xl border border-border/10">
      <button class="px-5 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'manual' ? 'bg-gold-500/20 text-gold-400 shadow-sm' : 'text-text-muted hover:text-text-primary'}" onclick={() => {activeTab = 'manual'; showForm=false;}}>📝 Data Master (CRUD)</button>
      <button class="px-5 py-2 rounded-lg text-xs font-bold transition-all {activeTab === 'ai' ? 'bg-gold-500/20 text-gold-400 shadow-sm' : 'text-text-muted hover:text-text-primary'}" onclick={() => activeTab = 'ai'}>🤖 AI Extractor</button>
    </div>
  </div>

  {#if activeTab === 'manual'}
    {#if !showForm}
      <div class="flex justify-between items-center mb-2 mt-2">
        <h2 class="text-lg font-bold text-text-primary">Daftar Peristiwa ({events.length})</h2>
        <button onclick={openCreateForm} class="px-5 py-2 rounded-xl gradient-rust text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all flex items-center gap-2">
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
                    <td class="px-6 py-4 text-xs font-mono bg-iron-950/30 inline-block mt-3 ml-6 rounded px-2">{evt.precision}</td>
                    <td class="px-6 py-4 text-right">
                      <div class="flex items-center justify-end gap-2">
                        <a href="/event/{evt.uuid}" target="_blank" class="text-verdigris-400 hover:text-verdigris-300 font-bold px-3 py-1 bg-verdigris-500/10 rounded-lg transition-colors flex items-center gap-1">
                          <span class="text-sm">👁️</span> View
                        </a>
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
        
        <div class="flex flex-col gap-6">
          <!-- Group 1: Data Dasar Peristiwa (Full Width) -->
          <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-5 bg-iron-950/20">
            <h3 class="text-sm font-bold text-gold-400 flex items-center gap-2 border-b border-border/5 pb-2.5">
              <span>🗂️</span> 1. Data Dasar Peristiwa
            </h3>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-1.5">
                  <label for="f-title" class="text-xs font-bold text-text-secondary">Judul Peristiwa *</label>
                  <input id="f-title" bind:value={formData.title} placeholder="Contoh: Pembukaan Kota Makkah (Fathu Makkah)" class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full" />
                </div>

                <div class="flex flex-col gap-1.5">
                  <label for="f-desc" class="text-xs font-bold text-text-secondary">Deskripsi Historis *</label>
                  <textarea id="f-desc" bind:value={formData.description} rows="5" placeholder="Tuliskan kronologi singkat, konteks sosio-politik, dan dampak peradaban..." class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full resize-y"></textarea>
                </div>
              </div>

              <div class="flex flex-col gap-4">
                <div class="grid grid-cols-2 gap-4">
                  <div class="flex flex-col gap-1.5">
                    <label for="f-masehi" class="text-xs font-bold text-text-secondary">Tahun Masehi</label>
                    <input id="f-masehi" type="number" bind:value={formData.gregorianYear} placeholder="Contoh: 630" class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none" />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <label for="f-hijri" class="text-xs font-bold text-text-secondary">Tahun Hijriah</label>
                    <input id="f-hijri" type="number" bind:value={formData.hijriYear} placeholder="Contoh: 8" class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none" />
                  </div>
                </div>
                
                <div class="flex flex-col gap-1.5">
                  <label for="f-prec" class="text-xs font-bold text-text-secondary">Presisi Waktu</label>
                  <select id="f-prec" bind:value={formData.precision} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none">
                    <option value="EXACT">Tepat Waktu (Exact)</option>
                    <option value="YEAR">Kira-kira Tahun (Year)</option>
                    <option value="DECADE">Kira-kira Dekade (Decade)</option>
                  </select>
                </div>

                <!-- Global Connection / Pivot Toggle -->
                <div class="flex flex-col gap-1.5 mt-2">
                  <label class="text-xs font-bold text-text-secondary">Status Hubungan Global</label>
                  <label class="flex items-center gap-3 p-3 rounded-lg bg-iron-950/40 border border-border/5 hover:border-gold-500/20 transition-all cursor-pointer">
                    <input type="checkbox" bind:checked={formData.isPivot} class="rounded text-gold-500 focus:ring-gold-500/30 bg-iron-950 border-border/10" />
                    <div>
                      <span class="text-xs font-bold text-text-primary block">Tandai sebagai Pivot Peradaban Global</span>
                      <span class="text-[10px] text-text-muted">Peristiwa ini terhubung dengan dinamika sosial-politik lintas benua di luar dunia Islam.</span>
                    </div>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- Group 2: Relasi Tokoh Terkait (Actors) - Full Width -->
          <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4 bg-iron-950/20">
            <div class="flex justify-between items-center border-b border-border/5 pb-2.5">
              <h3 class="text-sm font-bold text-gold-400 flex items-center gap-2">
                <span>👤</span> 2. Tokoh / Pelaku Sejarah Terlibat
              </h3>
              <a href="/factory/actors" target="_blank" class="text-[10px] text-gold-400 hover:text-gold-300 font-bold flex items-center gap-1">Kelola Tokoh ↗</a>
            </div>
            
            <div class="flex flex-col gap-3">
              <!-- Selection dropdown -->
              <select 
                class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full"
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

              <!-- Badges list -->
              {#if formData.selectedActors.length > 0}
                <div class="flex flex-wrap gap-2 p-2.5 bg-iron-950/30 border border-border/5 rounded-xl min-h-[45px] items-center">
                  {#each formData.selectedActors as actorUuid}
                    {@const actor = actors.find(a => a.uuid === actorUuid)}
                    {#if actor}
                      <span class="inline-flex items-center gap-1.5 px-3 py-1 bg-gold-500/10 border border-gold-500/30 text-gold-400 rounded-full text-xs font-bold animate-fade-in">
                        {actor.name}
                        <button type="button" onclick={() => formData.selectedActors = formData.selectedActors.filter(id => id !== actorUuid)} class="hover:text-red-400 transition-colors ml-1 font-bold" title="Hapus">✕</button>
                      </span>
                    {/if}
                  {/each}
                </div>
              {:else}
                <p class="text-[11px] text-text-muted italic p-3 text-center border border-dashed border-border/10 rounded-xl bg-iron-950/10">Belum ada tokoh yang dihubungkan ke peristiwa ini.</p>
              {/if}
            </div>
          </div>

          <!-- Group 3: Relasi Lokasi Kejadian (Locations) - Full Width -->
          <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4 bg-iron-950/20">
            <div class="flex justify-between items-center border-b border-border/5 pb-2.5">
              <h3 class="text-sm font-bold text-verdigris-400 flex items-center gap-2">
                <span>🗺️</span> 3. Lokasi Geografis Utama
              </h3>
              <a href="/factory/locations" target="_blank" class="text-[10px] text-verdigris-400 hover:text-verdigris-300 font-bold flex items-center gap-1">Kelola Lokasi ↗</a>
            </div>
            
            <div class="flex flex-col gap-3">
              <select 
                class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full"
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

              {#if formData.selectedLocations.length > 0}
                <div class="flex flex-wrap gap-2 p-2.5 bg-iron-950/30 border border-border/5 rounded-xl min-h-[45px] items-center">
                  {#each formData.selectedLocations as locUuid}
                    {@const loc = locations.find(l => l.uuid === locUuid)}
                    {#if loc}
                      <span class="inline-flex items-center gap-1.5 px-3 py-1 bg-verdigris-500/10 border border-verdigris-500/30 text-verdigris-400 rounded-full text-xs font-bold animate-fade-in">
                        {loc.name}
                        <button type="button" onclick={() => formData.selectedLocations = formData.selectedLocations.filter(id => id !== locUuid)} class="hover:text-red-400 transition-colors ml-1 font-bold" title="Hapus">✕</button>
                      </span>
                    {/if}
                  {/each}
                </div>
              {:else}
                <p class="text-[11px] text-text-muted italic p-3 text-center border border-dashed border-border/10 rounded-xl bg-iron-950/10">Belum ada lokasi kejadian yang ditentukan.</p>
              {/if}
            </div>
          </div>

          <!-- Group 4: Rujukan & Bukti Validitas (Sources & Sub-References) - Full Width -->
          <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4 bg-iron-950/20">
            <div class="flex justify-between items-center border-b border-border/5 pb-2.5">
              <h3 class="text-sm font-bold text-gold-400 flex items-center gap-2">
                <span>📄</span> 4. Rujukan Sumber Kitab & Sub-Rujukan Pembuktian
              </h3>
              <a href="/factory/sources" target="_blank" class="text-[10px] text-gold-400 hover:text-gold-300 font-bold flex items-center gap-1">Daftar Rujukan ↗</a>
            </div>
            
            <div class="flex flex-col gap-4">
              <!-- Select Dropdown -->
              <select 
                class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-xs text-text-primary focus:border-gold-500/30 outline-none w-full animate-fade-in"
                onchange={(e) => {
                  const val = (e.target as HTMLSelectElement).value;
                  if (val && !formData.selectedSources.includes(val)) {
                    formData.selectedSources = [...formData.selectedSources, val];
                    if (!formData.subReferences[val]) {
                      formData.subReferences[val] = [];
                    }
                    (e.target as HTMLSelectElement).value = "";
                  }
                }}
              >
                <option value="">-- + Hubungkan Rujukan Kitab / Sumber --</option>
                {#each sources.filter(s => !formData.selectedSources.includes(s.sourceId)) as src}
                  <option value={src.sourceId}>{src.title || 'Tanpa Judul'} - {src.referenceText.substring(0, 50)}...</option>
                {/each}
              </select>

              <!-- Selected Sources with dynamic Sub-References List -->
              {#if formData.selectedSources.length > 0}
                <div class="flex flex-col gap-6 mt-2">
                  {#each formData.selectedSources as sourceId}
                    {@const src = sources.find(s => s.sourceId === sourceId)}
                    {#if src}
                      <div class="glass p-5 rounded-xl border border-border/10 bg-iron-950/40 flex flex-col gap-4 animate-fade-in">
                        <!-- Source Card Header -->
                        <div class="flex flex-col md:flex-row md:justify-between md:items-center pb-3 border-b border-border/5 gap-2">
                          <span class="text-xs font-extrabold text-gold-400 flex items-center gap-2">
                            📖 Rujukan Utama: <span class="text-text-primary font-normal">{src.title || 'Tanpa Judul'}</span>
                          </span>
                          <button 
                            type="button" 
                            onclick={() => {
                              formData.selectedSources = formData.selectedSources.filter(id => id !== sourceId);
                              delete formData.subReferences[sourceId];
                            }} 
                            class="text-xs font-bold text-red-400 hover:text-red-300 transition-colors flex items-center gap-1 cursor-pointer self-end md:self-auto"
                          >
                            ✕ Hapus Rujukan
                          </button>
                        </div>

                        <!-- Sub-References List / Table -->
                        <div class="flex flex-col gap-3">
                          <span class="text-[10px] font-bold text-text-secondary uppercase tracking-wider pl-1 block">
                            📌 Rincian Sub-Rujukan Pembuktian (Bab & Ayat)
                          </span>
                          
                          {#if (formData.subReferences[sourceId] || []).length > 0}
                            <div class="overflow-x-auto rounded-xl border border-border/10 bg-iron-950/60">
                              <table class="w-full text-left border-collapse text-xs">
                                <thead>
                                  <tr class="bg-iron-950/90 border-b border-border/10 text-gold-400/80 font-bold uppercase tracking-wider text-[9px]">
                                    <th class="px-4 py-3 w-1/4">Bab / Surat / Volume</th>
                                    <th class="px-4 py-3 w-1/5">Halaman / Ayat / Nomor</th>
                                    <th class="px-4 py-3">Kutipan & Catatan Ulasan / Tafsir</th>
                                    <th class="px-4 py-3 text-center w-12">Aksi</th>
                                  </tr>
                                </thead>
                                <tbody class="divide-y divide-border/5">
                                  {#each formData.subReferences[sourceId] as sub, idx}
                                    <tr class="hover:bg-iron-900/10 transition-colors">
                                      <!-- Col 1: Section (Bab/Surat) -->
                                      <td class="p-3">
                                        <input 
                                          type="text" 
                                          bind:value={sub.section}
                                          placeholder="Misal: Surat Al-Baqarah"
                                          class="w-full bg-surface/30 border border-border/10 rounded px-2.5 py-1.5 text-xs text-text-primary focus:outline-none focus:border-gold-500/30 transition-all"
                                        />
                                      </td>
                                      <!-- Col 2: Point (Ayat/Halaman) -->
                                      <td class="p-3">
                                        <input 
                                          type="text" 
                                          bind:value={sub.point}
                                          placeholder="Misal: Ayat 25 atau Hal. 12"
                                          class="w-full bg-surface/30 border border-border/10 rounded px-2.5 py-1.5 text-xs text-text-primary focus:outline-none focus:border-gold-500/30 transition-all font-mono"
                                        />
                                      </td>
                                      <!-- Col 3: Note (Kutipan/Ulasan) -->
                                      <td class="p-3">
                                        <textarea 
                                          bind:value={sub.note}
                                          placeholder="Tulis tafsir, naskah kutipan, atau keterangan pendukung..."
                                          rows="1"
                                          class="w-full bg-surface/30 border border-border/10 rounded px-2.5 py-1.5 text-xs text-text-primary focus:outline-none focus:border-gold-500/30 transition-all resize-y leading-relaxed"
                                        ></textarea>
                                      </td>
                                      <!-- Col 4: Action (Remove) -->
                                      <td class="p-3 text-center">
                                        <button 
                                          type="button" 
                                          onclick={() => {
                                            formData.subReferences[sourceId] = formData.subReferences[sourceId].filter((_, i) => i !== idx);
                                          }}
                                          class="w-8 h-8 rounded bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/20 flex items-center justify-center hover:text-red-300 transition-all cursor-pointer mx-auto"
                                          title="Hapus baris sub-rujukan"
                                        >
                                          ✕
                                        </button>
                                      </td>
                                    </tr>
                                  {/each}
                                </tbody>
                              </table>
                            </div>
                          {:else}
                            <p class="text-[11px] text-text-muted italic p-4 text-center border border-dashed border-border/10 rounded-xl bg-iron-950/20">Belum ada rincian sub-rujukan yang ditambahkan. Silakan klik tombol di bawah untuk menambah rincian.</p>
                          {/if}

                          <!-- Add Row Button -->
                          <button 
                            type="button" 
                            onclick={() => {
                              formData.subReferences[sourceId] = [
                                ...(formData.subReferences[sourceId] || []),
                                { section: '', point: '', note: '' }
                              ];
                            }}
                            class="self-start text-[10px] font-bold text-gold-400 hover:text-gold-300 transition-colors flex items-center gap-1 mt-1 cursor-pointer bg-gold-500/5 hover:bg-gold-500/10 border border-gold-500/20 px-3 py-1.5 rounded-lg"
                          >
                            ➕ Tambah Sub Rujukan (Bab/Ayat)
                          </button>
                        </div>
                      </div>
                    {/if}
                  {/each}
                </div>
              {/if}
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="mt-8 pt-6 border-t border-border/10 flex justify-end gap-4">
          <button onclick={() => showForm = false} class="px-6 py-2.5 rounded-xl border border-border/10 text-text-primary font-bold text-xs hover:bg-surface-lighter transition-all">Batal</button>
          <button onclick={saveEvent} disabled={isSaving || !formData.title} class="px-8 py-2.5 rounded-xl gradient-rust text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] disabled:opacity-50 flex items-center gap-2">
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
      <textarea id="h-input" bind:value={textInput} rows="6" class="w-full bg-iron-950/60 border border-border/10 rounded-xl p-4 text-xs text-text-primary" placeholder="Teks catatan sejarah..."></textarea>
      <button onclick={startProcessing} disabled={isProcessingAI || !textInput} class="self-end px-8 py-3 rounded-xl gradient-rust text-surface font-bold text-xs">Jalankan AI Konsensus</button>
    </div>
  {/if}
</div>
