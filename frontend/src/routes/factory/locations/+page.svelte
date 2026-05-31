<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import { fade } from 'svelte/transition';

  let locations = $state<any[]>([]);
  let isLoading = $state(true);
  
  let showForm = $state(false);
  let formMode = $state('create'); // 'create' | 'edit'
  let notification = $state<{type: 'success' | 'error', message: string} | null>(null);

  function showNotification(type: 'success' | 'error', message: string) {
    notification = { type, message };
    setTimeout(() => { notification = null; }, 4000);
  }

  let formData = $state({
    uuid: '',
    name: '',
    precision: 'POINT',
    isTranscendental: false,
    lat: '',
    lng: '',
    geographyClimate: '',
    demographics: '',
    socioCultural: '',
    historicalRole: '',
    mediaLinks: [] as { mediaType: string; url: string; title: string }[]
  });

  let newMedia = $state({
    mediaType: 'image',
    url: '',
    title: ''
  });

  let submitError = $state('');
  let isSaving = $state(false);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    isLoading = true;
    try {
      const res = await gql(`
        query { 
          locations { 
            uuid 
            name 
            lat 
            lng 
            precision 
            isTranscendental 
            geographyClimate 
            demographics 
            socioCultural 
            historicalRole 
            mediaLinks {
              mediaType
              url
              title
            }
          } 
        }
      `) as any;
      locations = res.locations || [];
    } catch (err) {
      console.error('Failed to load locations', err);
    } finally {
      isLoading = false;
    }
  }

  function openCreateForm() {
    formMode = 'create';
    resetForm();
    showForm = true;
  }

  function openEditForm(loc: any) {
    formMode = 'edit';
    formData = {
      uuid: loc.uuid,
      name: loc.name,
      precision: loc.precision || 'POINT',
      isTranscendental: loc.isTranscendental || false,
      lat: loc.lat?.toString() || '',
      lng: loc.lng?.toString() || '',
      geographyClimate: loc.geographyClimate || '',
      demographics: loc.demographics || '',
      socioCultural: loc.socioCultural || '',
      historicalRole: loc.historicalRole || '',
      mediaLinks: loc.mediaLinks ? loc.mediaLinks.map((ml: any) => ({
        mediaType: ml.mediaType,
        url: ml.url,
        title: ml.title || ''
      })) : []
    };
    submitError = '';
    showForm = true;
  }

  function resetForm() {
    formData = {
      uuid: '',
      name: '',
      precision: 'POINT',
      isTranscendental: false,
      lat: '',
      lng: '',
      geographyClimate: '',
      demographics: '',
      socioCultural: '',
      historicalRole: '',
      mediaLinks: []
    };
    newMedia = { mediaType: 'image', url: '', title: '' };
    submitError = '';
  }

  function addMediaLink() {
    if (newMedia.url.trim()) {
      formData.mediaLinks = [...formData.mediaLinks, {
        mediaType: newMedia.mediaType,
        url: newMedia.url.trim(),
        title: newMedia.title.trim()
      }];
      newMedia = { mediaType: 'image', url: '', title: '' };
    }
  }

  function removeMediaLink(index: number) {
    formData.mediaLinks = formData.mediaLinks.filter((_, i) => i !== index);
  }

  async function saveLocation() {
    if (!formData.name) {
      submitError = 'Nama lokasi wajib diisi.';
      return;
    }
    isSaving = true;
    submitError = '';

    try {
      if (formMode === 'create') {
        const createMutation = `
          mutation CreateLocation($input: CreateLocationInput!) {
            createLocation(input: $input) { uuid }
          }
        `;
        const createVars = {
          input: {
            name: formData.name,
            precision: formData.precision,
            isTranscendental: formData.isTranscendental,
            lat: formData.lat ? parseFloat(formData.lat) : null,
            lng: formData.lng ? parseFloat(formData.lng) : null,
            geographyClimate: formData.geographyClimate || null,
            demographics: formData.demographics || null,
            socioCultural: formData.socioCultural || null,
            historicalRole: formData.historicalRole || null,
            mediaLinks: formData.mediaLinks.map(ml => ({
              mediaType: ml.mediaType,
              url: ml.url,
              title: ml.title || null
            }))
          }
        };
        await gql(createMutation, createVars) as any;
      } else {
        const updateMutation = `
          mutation UpdateLocation($uuid: UUID!, $input: UpdateLocationInput!) {
            updateLocation(uuid: $uuid, input: $input) { uuid }
          }
        `;
        const updateVars = {
          uuid: formData.uuid,
          input: {
            name: formData.name,
            precision: formData.precision,
            isTranscendental: formData.isTranscendental,
            lat: formData.lat ? parseFloat(formData.lat) : null,
            lng: formData.lng ? parseFloat(formData.lng) : null,
            geographyClimate: formData.geographyClimate || null,
            demographics: formData.demographics || null,
            socioCultural: formData.socioCultural || null,
            historicalRole: formData.historicalRole || null,
            mediaLinks: formData.mediaLinks.map(ml => ({
              mediaType: ml.mediaType,
              url: ml.url,
              title: ml.title || null
            }))
          }
        };
        await gql(updateMutation, updateVars) as any;
      }

      showForm = false;
      showNotification('success', formMode === 'create' ? 'Lokasi berhasil ditambahkan ke Knowledge Graph!' : 'Data lokasi berhasil diperbarui!');
      await loadData();
    } catch (err: any) {
      submitError = err.message || 'Gagal menyimpan lokasi.';
      showNotification('error', submitError);
    } finally {
      isSaving = false;
    }
  }

  async function deleteLocation(uuid: string) {
    if (!confirm('Apakah Anda yakin ingin menghapus lokasi ini dari sistem?')) return;
    try {
      const delMutation = `mutation { deleteLocation(uuid: "${uuid}") }`;
      await gql(delMutation) as any;
      showNotification('success', 'Lokasi berhasil dihapus.');
      await loadData();
    } catch (err: any) {
      showNotification('error', err.message || 'Gagal menghapus lokasi.');
    }
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto relative">
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
      <h1 class="text-2xl font-extrabold text-emerald-400">Manajemen Lokasi (Location)</h1>
      <p class="text-xs text-text-secondary mt-1">Kelola entitas spasial / tempat bersejarah dalam Knowledge Graph.</p>
    </div>
    <button onclick={openCreateForm} class="px-5 py-2.5 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 text-xs font-bold rounded-xl border border-emerald-500/20 transition-all flex items-center gap-2">
      <span>➕</span> Tambah Lokasi
    </button>
  </div>

  {#if showForm}
    <div class="glass p-8 rounded-2xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.05)] relative" transition:fade>
      <button onclick={() => showForm = false} class="absolute top-6 right-6 text-text-muted hover:text-red-400 transition-colors text-xl">✕</button>
      
      <h2 class="text-lg font-bold text-text-primary mb-6">{formMode === 'create' ? 'Tambah Lokasi Baru' : 'Edit Data Lokasi'}</h2>
      
      {#if submitError}
        <div class="bg-red-950/50 border border-red-500/50 text-red-400 p-3 rounded-lg text-xs mb-6 font-bold">{submitError}</div>
      {/if}

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Nama Tempat / Lokasi *</label>
          <input type="text" bind:value={formData.name} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Misal: Makkah Al-Mukarramah">
        </div>
        
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Presisi Lokasi</label>
          <select bind:value={formData.precision} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors">
            <option value="POINT">Titik Koordinat Pasti (Point)</option>
            <option value="REGION">Wilayah Secara Umum (Region)</option>
            <option value="CONCEPTUAL">Hanya Konseptual (Conceptual)</option>
          </select>
        </div>

        <div class="flex items-center gap-2 pt-6">
          <input type="checkbox" bind:checked={formData.isTranscendental} class="w-4 h-4 rounded border-border/10 text-emerald-500 focus:ring-emerald-500 bg-navy-950/60" id="transcendental">
          <label for="transcendental" class="text-xs font-bold text-text-secondary">Lokasi Gaib / Transendental (Contoh: Sidratul Muntaha)</label>
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Latitude (Garis Lintang)</label>
          <input type="number" step="any" bind:value={formData.lat} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Misal: 21.4225">
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Longitude (Garis Bujur)</label>
          <input type="number" step="any" bind:value={formData.lng} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Misal: 39.8262">
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Geografis & Iklim</label>
          <textarea bind:value={formData.geographyClimate} rows="3" class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Deskripsikan letak geografis, bentang alam, serta iklim wilayah ini secara historis..."></textarea>
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Demografi Kependudukan</label>
          <textarea bind:value={formData.demographics} rows="3" class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Kondisi kependudukan suku bangsa, kabilah, jumlah populasi dalam lintasan zaman..."></textarea>
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Kehidupan Sosial Kultural</label>
          <textarea bind:value={formData.socioCultural} rows="3" class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Budaya, sistem nilai, tradisi, serta pola relasi sosial masyarakat setempat..."></textarea>
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Peran & Signifikansi Sejarah</label>
          <textarea bind:value={formData.historicalRole} rows="3" class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-emerald-500/50 outline-none transition-colors" placeholder="Uraikan posisi strategis, fungsi taktis, serta peristiwa-peristiwa agung yang melekat erat pada lokasi ini..."></textarea>
        </div>

        <!-- Media Links Section -->
        <div class="flex flex-col gap-3 md:col-span-2 border-t border-border/5 pt-6">
          <label class="text-xs font-bold text-emerald-400 flex items-center gap-2">
            <span>🎥</span> Berkas Media Pendukung (Gambar, Audio, Video, Naskah Teks)
          </label>
          
          <div class="grid grid-cols-1 sm:grid-cols-4 gap-3 bg-navy-950/40 p-3 rounded-lg border border-border/5">
            <div class="flex flex-col gap-1">
              <label class="text-[10px] font-bold text-text-secondary">Tipe Media</label>
              <select bind:value={newMedia.mediaType} class="bg-navy-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-emerald-500/50 outline-none">
                <option value="image">Gambar / Foto (Image)</option>
                <option value="audio">Rekaman Suara (Audio)</option>
                <option value="video">Dokumenter Video (Video)</option>
                <option value="text">Naskah Asli / Artikel (Text)</option>
              </select>
            </div>
            
            <div class="flex flex-col gap-1 sm:col-span-2">
              <label class="text-[10px] font-bold text-text-secondary">URL Media / Berkas</label>
              <input type="text" bind:value={newMedia.url} class="bg-navy-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-emerald-500/50 outline-none" placeholder="https://host.com/peta-kuno.png">
            </div>
            
            <div class="flex flex-col gap-1">
              <label class="text-[10px] font-bold text-text-secondary">Label / Judul Media</label>
              <div class="flex gap-2">
                <input type="text" bind:value={newMedia.title} class="flex-1 bg-navy-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-emerald-500/50 outline-none" placeholder="Peta Rute Hijrah Kuno" onkeydown={(e) => e.key === 'Enter' && addMediaLink()}>
                <button type="button" onclick={addMediaLink} class="px-3 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 text-xs font-bold rounded border border-emerald-500/25 transition-all">Add</button>
              </div>
            </div>
          </div>

          {#if formData.mediaLinks.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2">
              {#each formData.mediaLinks as media, idx}
                <div class="flex items-center justify-between p-2.5 bg-navy-900/60 border border-border/10 rounded-lg text-xs gap-3">
                  <div class="flex items-center gap-2 overflow-hidden">
                    <span class="px-2 py-0.5 bg-navy-950 rounded text-[9px] uppercase border border-border/10 text-emerald-400 font-mono">
                      {media.mediaType}
                    </span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="font-bold text-text-primary truncate">{media.title || 'Media Link'}</span>
                      <a href={media.url} target="_blank" rel="noopener noreferrer" class="text-[10px] text-blue-400 hover:underline truncate">{media.url}</a>
                    </div>
                  </div>
                  <button type="button" onclick={() => removeMediaLink(idx)} class="text-red-400 hover:text-red-300 font-bold px-2 py-1 hover:bg-red-500/10 rounded-md transition-all">✕</button>
                </div>
              {/each}
            </div>
          {:else}
            <span class="text-[11px] text-text-muted">Belum ada berkas media terhubung.</span>
          {/if}
        </div>
      </div>

      <div class="flex justify-end gap-3 mt-8">
        <button onclick={() => showForm = false} class="px-5 py-2.5 text-xs font-bold text-text-secondary hover:text-text-primary transition-colors">Batal</button>
        <button onclick={saveLocation} disabled={isSaving} class="px-6 py-2.5 bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-400 hover:to-emerald-500 text-navy-950 text-xs font-extrabold rounded-xl transition-all shadow-lg shadow-emerald-500/20 disabled:opacity-50">
          {isSaving ? 'Menyimpan...' : 'Simpan Lokasi'}
        </button>
      </div>
    </div>
  {/if}

  <!-- Data Table -->
  <div class="glass rounded-2xl border border-border/10 overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead class="bg-navy-950/40 border-b border-border/10">
          <tr>
            <th class="px-6 py-4 font-bold text-text-secondary">Nama Lokasi</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Presisi</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Status & Koordinat</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Karakteristik & Media</th>
            <th class="px-6 py-4 font-bold text-text-secondary text-right">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/5">
          {#if isLoading}
            <tr><td colspan="5" class="px-6 py-8 text-center text-text-muted">Memuat data...</td></tr>
          {:else if locations.length === 0}
            <tr><td colspan="5" class="px-6 py-8 text-center text-text-muted">Belum ada data lokasi.</td></tr>
          {:else}
            {#each locations as loc}
              <tr class="hover:bg-navy-950/20 transition-colors">
                <td class="px-6 py-4">
                  <div class="font-bold text-text-primary">{loc.name}</div>
                  {#if loc.historicalRole}
                    <div class="text-[10px] text-text-muted truncate max-w-[200px] mt-0.5">{loc.historicalRole}</div>
                  {/if}
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <span class="px-2 py-1 bg-navy-950/50 rounded text-[10px] uppercase border border-border/10">{loc.precision}</span>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <div class="flex items-center gap-1.5">
                    <span>{loc.isTranscendental ? '☁️ Gaib' : '🌍 Fisik'}</span>
                  </div>
                  <div class="text-[10px] text-text-muted font-mono mt-0.5">{loc.lat || '?'}, {loc.lng || '?'}</div>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <div class="flex flex-col gap-1">
                    {#if loc.geographyClimate || loc.demographics || loc.socioCultural}
                      <span class="text-[10px] text-emerald-400">✓ Terdeskripsi</span>
                    {/if}
                    <span class="flex items-center gap-1 text-[10px] {loc.mediaLinks && loc.mediaLinks.length > 0 ? 'text-blue-400' : 'text-text-muted'}">
                      🎥 {loc.mediaLinks ? loc.mediaLinks.length : 0} Media
                    </span>
                  </div>
                </td>
                <td class="px-6 py-4 text-right font-medium">
                  <div class="flex items-center justify-end gap-2">
                    <button onclick={() => openEditForm(loc)} class="text-emerald-400 hover:text-emerald-300 font-bold px-3 py-1 bg-emerald-500/10 rounded-lg transition-colors">Edit</button>
                    <button onclick={() => deleteLocation(loc.uuid)} class="text-red-400 hover:text-red-300 font-bold px-3 py-1 bg-red-500/10 rounded-lg transition-colors">Hapus</button>
                  </div>
                </td>
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
    </div>
  </div>
</div>
