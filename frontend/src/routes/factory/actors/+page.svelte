<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import { fade } from 'svelte/transition';

  let actors = $state<any[]>([]);
  let isLoading = $state(true);
  
  let searchQuery = $state('');
  let filterType = $state('all');
  let sortBy = $state('name-asc');

  let filteredActors = $derived(actors.filter(actor => {
    const searchLower = searchQuery.toLowerCase();
    const matchesSearch = actor.name.toLowerCase().includes(searchLower) || (actor.culturalSphere && actor.culturalSphere.toLowerCase().includes(searchLower));
    const matchesFilter = filterType === 'all' || actor.actorType === filterType;
    return matchesSearch && matchesFilter;
  }).sort((a, b) => {
    if (sortBy === 'name-asc') return a.name.localeCompare(b.name);
    if (sortBy === 'name-desc') return b.name.localeCompare(a.name);
    return 0;
  }));
  
  let showForm = $state(false);
  let formMode = $state('create'); // 'create' | 'edit'
  let notification = $state<{type: 'success' | 'error', message: string} | null>(null);

  function showNotification(type: 'success' | 'error', message: string) {
    notification = { type, message };
    setTimeout(() => { notification = null; }, 4000);
  }

  let formData = $state<{
    uuid: string;
    name: string;
    actorType: string;
    culturalSphere: string;
    birthYear: string;
    deathYear: string;
    description: string;
    works: string[];
    roles: string[];
    mediaLinks: { mediaType: string; url: string; title: string }[];
  }>({
    uuid: '',
    name: '',
    actorType: 'Individual',
    culturalSphere: '',
    birthYear: '',
    deathYear: '',
    description: '',
    works: [],
    roles: [],
    mediaLinks: []
  });

  let newWork = $state('');
  let newRole = $state('');
  
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
          actors {
            uuid
            name
            actorType
            culturalSphere
            birthYear
            deathYear
            description
            works
            roles
            mediaLinks {
              mediaType
              url
              title
            }
          }
        }
      `) as any;
      actors = res.actors || [];
    } catch (err) {
      console.error('Failed to load actors', err);
    } finally {
      isLoading = false;
    }
  }

  function openCreateForm() {
    formMode = 'create';
    resetForm();
    showForm = true;
  }

  function openEditForm(act: any) {
    formMode = 'edit';
    formData = {
      uuid: act.uuid,
      name: act.name,
      actorType: act.actorType || 'Individual',
      culturalSphere: act.culturalSphere || '',
      birthYear: act.birthYear?.toString() || '',
      deathYear: act.deathYear?.toString() || '',
      description: act.description || '',
      works: act.works ? [...act.works] : [],
      roles: act.roles ? [...act.roles] : [],
      mediaLinks: act.mediaLinks ? act.mediaLinks.map((ml: any) => ({
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
      actorType: 'Individual',
      culturalSphere: '',
      birthYear: '',
      deathYear: '',
      description: '',
      works: [],
      roles: [],
      mediaLinks: []
    };
    newWork = '';
    newRole = '';
    newMedia = { mediaType: 'image', url: '', title: '' };
    submitError = '';
  }

  function addWork() {
    if (newWork.trim()) {
      formData.works = [...formData.works, newWork.trim()];
      newWork = '';
    }
  }

  function removeWork(index: number) {
    formData.works = formData.works.filter((_, i) => i !== index);
  }

  function addRole() {
    if (newRole.trim()) {
      formData.roles = [...formData.roles, newRole.trim()];
      newRole = '';
    }
  }

  function removeRole(index: number) {
    formData.roles = formData.roles.filter((_, i) => i !== index);
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

  async function saveActor() {
    if (!formData.name) {
      submitError = 'Nama tokoh wajib diisi.';
      return;
    }
    isSaving = true;
    submitError = '';

    try {
      if (formMode === 'create') {
        const createMutation = `
          mutation CreateActor($input: CreateActorInput!) {
            createActor(input: $input) { uuid }
          }
        `;
        const createVars = {
          input: {
            name: formData.name,
            actorType: formData.actorType,
            culturalSphere: formData.culturalSphere || null,
            birthYear: formData.birthYear ? parseInt(formData.birthYear) : null,
            deathYear: formData.deathYear ? parseInt(formData.deathYear) : null,
            description: formData.description || null,
            works: formData.works.length > 0 ? formData.works : null,
            roles: formData.roles.length > 0 ? formData.roles : null,
            mediaLinks: formData.mediaLinks.length > 0 ? formData.mediaLinks.map(ml => ({
              mediaType: ml.mediaType,
              url: ml.url,
              title: ml.title || null
            })) : null
          }
        };
        await gql(createMutation, createVars) as any;
      } else {
        const updateMutation = `
          mutation UpdateActor($uuid: UUID!, $input: UpdateActorInput!) {
            updateActor(uuid: $uuid, input: $input) { uuid }
          }
        `;
        const updateVars = {
          uuid: formData.uuid,
          input: {
            name: formData.name,
            actorType: formData.actorType,
            culturalSphere: formData.culturalSphere || null,
            birthYear: formData.birthYear ? parseInt(formData.birthYear) : null,
            deathYear: formData.deathYear ? parseInt(formData.deathYear) : null,
            description: formData.description || null,
            works: formData.works,
            roles: formData.roles,
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
      showNotification('success', formMode === 'create' ? 'Tokoh berhasil ditambahkan ke Knowledge Graph!' : 'Data tokoh berhasil diperbarui!');
      await loadData();
    } catch (err: any) {
      submitError = err.message || 'Gagal menyimpan tokoh.';
      showNotification('error', submitError);
    } finally {
      isSaving = false;
    }
  }

  async function deleteActor(uuid: string) {
    if (!confirm('Apakah Anda yakin ingin menghapus tokoh ini dari sistem?')) return;
    try {
      const delMutation = `mutation { deleteActor(uuid: "${uuid}") }`;
      await gql(delMutation) as any;
      showNotification('success', 'Tokoh berhasil dihapus.');
      await loadData();
    } catch (err: any) {
      showNotification('error', err.message || 'Gagal menghapus tokoh.');
    }
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto relative animate-fade-in">
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

  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
    <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
      <div>
        <h1 class="text-2xl font-extrabold text-gold-400">Manajemen Tokoh (Actor)</h1>
        <p class="text-xs text-text-secondary mt-1">Kelola entitas pelaku sejarah dan hubungkan pustaka karya serta berkas media pendukung.</p>
      </div>
      <button onclick={openCreateForm} class="px-5 py-2.5 bg-gold-500/10 hover:bg-gold-500/20 text-gold-400 text-xs font-bold rounded-xl border border-gold-500/20 transition-all flex items-center gap-2 whitespace-nowrap">
        <span>➕</span> Tambah Tokoh
      </button>
    </div>

    <div class="flex flex-col md:flex-row gap-3 pt-4 border-t border-border/10">
      <input type="text" bind:value={searchQuery} placeholder="Cari nama atau kultural..." class="flex-1 bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none" />
      <select bind:value={filterType} class="bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none min-w-[150px]">
        <option value="all">Semua Tipe</option>
        <option value="Individual">Individu</option>
        <option value="Group">Kelompok</option>
        <option value="Dynasty">Dinasti</option>
      </select>
      <select bind:value={sortBy} class="bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none min-w-[150px]">
        <option value="name-asc">Nama (A-Z)</option>
        <option value="name-desc">Nama (Z-A)</option>
      </select>
    </div>
  </div>

  {#if showForm}
    <div class="glass p-8 rounded-2xl border border-gold-500/30 shadow-[0_0_30px_rgba(212,168,83,0.05)] relative" transition:fade>
      <button onclick={() => showForm = false} class="absolute top-6 right-6 text-text-muted hover:text-red-400 transition-colors text-xl">✕</button>
      
      <h2 class="text-lg font-bold text-text-primary mb-6">{formMode === 'create' ? 'Tambah Tokoh Baru' : 'Edit Data Tokoh'}</h2>
      
      {#if submitError}
        <div class="bg-red-950/50 border border-red-500/50 text-red-400 p-3 rounded-lg text-xs mb-6 font-bold">{submitError}</div>
      {/if}

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Nama Tokoh / Kelompok Sejarah *</label>
          <input type="text" bind:value={formData.name} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors" placeholder="Misal: Ibnu Sina / Kaum Muhajirin">
        </div>
        
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Tipe Entitas</label>
          <select bind:value={formData.actorType} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors">
            <option value="Individual">Individu (Tokoh)</option>
            <option value="Group">Kelompok / Kaum</option>
            <option value="Dynasty">Dinasti / Kerajaan</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Lingkup Kultural</label>
          <input type="text" bind:value={formData.culturalSphere} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors" placeholder="Misal: Arab Islam, Romawi Timur, Persia Sasanid">
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Tahun Lahir / Berdiri (Masehi)</label>
          <input type="number" bind:value={formData.birthYear} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors" placeholder="-500 untuk SM">
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Tahun Wafat / Runtuh (Masehi)</label>
          <input type="number" bind:value={formData.deathYear} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors" placeholder="Kosongkan jika masih ada/aktif">
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label class="text-xs font-bold text-text-secondary">Deskripsi & Biografi Singkat</label>
          <textarea bind:value={formData.description} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-gold-500/50 outline-none transition-colors min-h-[80px]" placeholder="Tulis ringkasan riwayat hidup atau catatan sejarah penting..."></textarea>
        </div>

        <!-- Avatar / Foto Tokoh -->
        <div class="flex flex-col gap-2 md:col-span-2 glass p-4 rounded-xl border border-border/5 bg-iron-950/20">
          <label class="text-xs font-bold text-gold-400 flex items-center gap-2">
            <span>🖼️</span> Avatar / Foto Profil Tokoh
          </label>
          <div class="flex gap-4 items-center">
            <div class="w-16 h-16 sm:w-20 sm:h-20 rounded-full bg-iron-900 border-2 border-border/10 overflow-hidden flex-shrink-0 flex items-center justify-center text-3xl">
              {#if formData.mediaLinks.some((m: any) => m.mediaType === 'image')}
                <img src={formData.mediaLinks.find((m: any) => m.mediaType === 'image')?.url} class="w-full h-full object-cover" alt="Avatar" />
              {:else}
                👤
              {/if}
            </div>
            <div class="flex flex-col gap-2 flex-1">
              <input type="text" 
                value={formData.mediaLinks.find((m: any) => m.mediaType === 'image')?.url || ''} 
                oninput={(e) => {
                  const url = e.currentTarget.value;
                  const idx = formData.mediaLinks.findIndex((m: any) => m.mediaType === 'image');
                  if (url) {
                    if (idx >= 0) formData.mediaLinks[idx].url = url;
                    else formData.mediaLinks.push({ mediaType: 'image', url, title: 'Avatar' });
                  } else {
                    if (idx >= 0) formData.mediaLinks.splice(idx, 1);
                  }
                }}
                class="bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none w-full" 
                placeholder="URL Gambar Avatar..." 
              />
              <div class="flex gap-2 items-center">
                <input type="file" accept="image/*" class="hidden" id="avatarUploadActor" onchange={async (e) => {
                  const file = (e.target as HTMLInputElement).files?.[0];
                  if (!file) return;
                  const fd = new FormData();
                  fd.append('file', file);
                  try {
                    const res = await fetch('/internal/upload', { method: 'POST', body: fd });
                    const data = await res.json();
                    if (data.url) {
                      const idx = formData.mediaLinks.findIndex((m: any) => m.mediaType === 'image');
                      if (idx >= 0) {
                        formData.mediaLinks[idx].url = data.url;
                      } else {
                        formData.mediaLinks.push({ mediaType: 'image', url: data.url, title: 'Avatar' });
                      }
                    } else alert(data.error || 'Upload gagal');
                  } catch (err) {
                    alert('Upload gagal');
                  }
                }}>
                <label for="avatarUploadActor" class="px-4 py-2 bg-gold-500/10 hover:bg-gold-500/20 text-gold-400 text-xs font-bold rounded-lg border border-gold-500/20 cursor-pointer transition-all">
                  📁 Upload File Gambar
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Works -->
        <div class="flex flex-col gap-2 glass p-4 rounded-xl border border-border/5">
          <label class="text-xs font-bold text-gold-400">Karya Tulis / Mahakarya (Works)</label>
          <div class="flex gap-2">
            <input type="text" bind:value={newWork} class="flex-1 bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none" placeholder="Contoh: Kitab Al-Qanun fi al-Tibb" onkeydown={(e) => e.key === 'Enter' && addWork()}>
            <button type="button" onclick={addWork} class="px-4 bg-gold-500/20 hover:bg-gold-500/30 text-gold-400 text-xs font-bold rounded-lg border border-gold-500/25 transition-all">Tambah</button>
          </div>
          {#if formData.works.length > 0}
            <div class="flex flex-wrap gap-1.5 mt-2">
              {#each formData.works as work, idx}
                <span class="inline-flex items-center gap-1 px-2.5 py-1 bg-iron-900 border border-border/10 text-text-secondary text-[11px] rounded-lg">
                  {work}
                  <button type="button" onclick={() => removeWork(idx)} class="text-red-400 hover:text-red-300 ml-1 text-xs">✕</button>
                </span>
              {/each}
            </div>
          {:else}
            <span class="text-[11px] text-text-muted">Belum ada karya terdaftar.</span>
          {/if}
        </div>

        <!-- Roles -->
        <div class="flex flex-col gap-2 glass p-4 rounded-xl border border-border/5">
          <label class="text-xs font-bold text-gold-400">Peran / Profesi / Gelar (Roles)</label>
          <div class="flex gap-2">
            <input type="text" bind:value={newRole} class="flex-1 bg-iron-950/60 border border-border/10 rounded-lg p-2.5 text-xs text-text-primary focus:border-gold-500/50 outline-none" placeholder="Contoh: Tabib Istana / Khalifah" onkeydown={(e) => e.key === 'Enter' && addRole()}>
            <button type="button" onclick={addRole} class="px-4 bg-gold-500/20 hover:bg-gold-500/30 text-gold-400 text-xs font-bold rounded-lg border border-gold-500/25 transition-all">Tambah</button>
          </div>
          {#if formData.roles.length > 0}
            <div class="flex flex-wrap gap-1.5 mt-2">
              {#each formData.roles as role, idx}
                <span class="inline-flex items-center gap-1 px-2.5 py-1 bg-iron-900 border border-border/10 text-text-secondary text-[11px] rounded-lg">
                  {role}
                  <button type="button" onclick={() => removeRole(idx)} class="text-red-400 hover:text-red-300 ml-1 text-xs">✕</button>
                </span>
              {/each}
            </div>
          {:else}
            <span class="text-[11px] text-text-muted">Belum ada peran terdaftar.</span>
          {/if}
        </div>

        <!-- Media Links -->
        <div class="flex flex-col gap-3 md:col-span-2 glass p-4 rounded-xl border border-border/5">
          <label class="text-xs font-bold text-gold-400 flex items-center gap-2">
            <span>🎥</span> Berkas Media Pendukung (Gambar, Audio, Video, Naskah Teks)
          </label>
          
          <div class="grid grid-cols-1 sm:grid-cols-4 gap-3 bg-iron-950/40 p-3 rounded-lg border border-border/5">
            <div class="flex flex-col gap-1">
              <label class="text-[10px] font-bold text-text-secondary">Tipe Media</label>
              <select bind:value={newMedia.mediaType} class="bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-gold-500/50 outline-none">
                <option value="image">Gambar / Foto (Image)</option>
                <option value="audio">Rekaman Suara (Audio)</option>
                <option value="video">Dokumenter Video (Video)</option>
                <option value="text">Naskah Asli / Artikel (Text)</option>
              </select>
            </div>
            
            <div class="flex flex-col gap-1 sm:col-span-2">
              <label class="text-[10px] font-bold text-text-secondary">URL Media / Berkas (atau Upload)</label>
              <div class="flex gap-2 items-center">
                <input type="text" bind:value={newMedia.url} class="flex-1 bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-gold-500/50 outline-none" placeholder="https://host.com/gambar-tokoh.png">
                <input type="file" accept="image/*,audio/*,video/*,.pdf,.txt" class="hidden" id="fileUploadActor" onchange={async (e) => {
                  const file = (e.target as HTMLInputElement).files?.[0];
                  if (!file) return;
                  const fd = new FormData();
                  fd.append('file', file);
                  try {
                    const res = await fetch('/internal/upload', { method: 'POST', body: fd });
                    const data = await res.json();
                    if (data.url) {
                      newMedia.url = data.url;
                      if (!newMedia.title) newMedia.title = data.title;
                    } else alert(data.error || 'Upload gagal');
                  } catch (err) {
                    alert('Upload gagal');
                  }
                }}>
                <label for="fileUploadActor" class="px-3 py-2 bg-iron-900 hover:bg-iron-800 border border-border/10 rounded text-xs cursor-pointer text-text-secondary transition-colors whitespace-nowrap font-bold">
                  📁 Upload
                </label>
              </div>
            </div>
            
            <div class="flex flex-col gap-1">
              <label class="text-[10px] font-bold text-text-secondary">Label / Judul Media</label>
              <div class="flex gap-2">
                <input type="text" bind:value={newMedia.title} class="flex-1 bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-gold-500/50 outline-none" placeholder="Foto Artefak Makam" onkeydown={(e) => e.key === 'Enter' && addMediaLink()}>
                <button type="button" onclick={addMediaLink} class="px-3 bg-gold-500/20 hover:bg-gold-500/30 text-gold-400 text-xs font-bold rounded border border-gold-500/25 transition-all">Add</button>
              </div>
            </div>
          </div>

          {#if formData.mediaLinks.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2">
              {#each formData.mediaLinks as media, idx}
                <div class="flex items-center justify-between p-2.5 bg-iron-900/60 border border-border/10 rounded-lg text-xs gap-3">
                  <div class="flex items-center gap-2 overflow-hidden">
                    <span class="px-2 py-0.5 bg-iron-950 rounded text-[9px] uppercase border border-border/10 text-gold-500 font-mono">
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
        <button onclick={saveActor} disabled={isSaving} class="px-6 py-2.5 bg-gradient-to-r from-gold-500 to-gold-600 hover:from-gold-400 hover:to-gold-500 text-iron-950 text-xs font-extrabold rounded-xl transition-all shadow-lg shadow-gold-500/20 disabled:opacity-50">
          {isSaving ? 'Menyimpan...' : 'Simpan Tokoh'}
        </button>
      </div>
    </div>
  {/if}

  <!-- Data Table -->
  <div class="glass rounded-2xl border border-border/10 overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-left text-xs">
        <thead class="bg-iron-950/40 border-b border-border/10">
          <tr>
            <th class="px-6 py-4 font-bold text-text-secondary">Nama Tokoh</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Tipe</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Kultural & Peran</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Metadata Sejarah</th>
            <th class="px-6 py-4 font-bold text-text-secondary text-right">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/5">
          {#if isLoading}
            <tr><td colspan="5" class="px-6 py-8 text-center text-text-muted">Memuat data...</td></tr>
          {:else if filteredActors.length === 0}
            <tr><td colspan="5" class="px-6 py-8 text-center text-text-muted">Tidak ada data tokoh ditemukan.</td></tr>
          {:else}
            {#each filteredActors as actor}
              <tr class="hover:bg-iron-950/20 transition-colors">
                <td class="px-6 py-4">
                  <div class="flex items-center gap-4">
                    {#if actor.mediaLinks && actor.mediaLinks.some((m: any) => m.mediaType === 'image')}
                      <div class="w-10 h-10 rounded-full overflow-hidden border border-border/10 flex-shrink-0">
                        <img src={actor.mediaLinks.find((m: any) => m.mediaType === 'image').url} alt={actor.name} class="w-full h-full object-cover" />
                      </div>
                    {:else}
                      <div class="w-10 h-10 rounded-full bg-iron-900 flex items-center justify-center text-xl border border-border/10 flex-shrink-0">
                        👤
                      </div>
                    {/if}
                    <div class="flex flex-col gap-0.5">
                      <span class="font-bold text-text-primary text-sm">{actor.name}</span>
                      {#if actor.description}
                        <p class="text-[11px] text-text-muted max-w-sm line-clamp-1 italic">{actor.description}</p>
                      {/if}
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <span class="px-2 py-1 bg-iron-950/50 rounded text-[10px] uppercase border border-border/10 font-bold">{actor.actorType}</span>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <div class="flex flex-col gap-1">
                    <span class="font-semibold text-text-primary">{actor.culturalSphere || '-'}</span>
                    {#if actor.roles && actor.roles.length > 0}
                      <div class="flex flex-wrap gap-1">
                        {#each actor.roles as role}
                          <span class="px-1.5 py-0.5 bg-gold-500/10 text-gold-400 border border-gold-500/20 rounded text-[9px]">{role}</span>
                        {/each}
                      </div>
                    {/if}
                  </div>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <div class="flex flex-col gap-1.5 font-mono text-[10px]">
                    <div>Masa: <span class="text-text-primary font-bold">{actor.birthYear || '?'} — {actor.deathYear || '?'}</span></div>
                    <div class="flex gap-2">
                      <span class="flex items-center gap-1 text-[10px] {actor.works && actor.works.length > 0 ? 'text-verdigris-400' : 'text-text-muted'}">
                        📚 {actor.works ? actor.works.length : 0} Karya
                      </span>
                      <span class="flex items-center gap-1 text-[10px] {actor.mediaLinks && actor.mediaLinks.length > 0 ? 'text-blue-400' : 'text-text-muted'}">
                        🎥 {actor.mediaLinks ? actor.mediaLinks.length : 0} Media
                      </span>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 text-right">
                  <div class="flex items-center justify-end gap-2">
                    <a href="/actor/{actor.uuid}" target="_blank" class="text-verdigris-400 hover:text-verdigris-300 font-bold px-3 py-1 bg-verdigris-500/10 rounded-lg transition-colors flex items-center gap-1">
                      <span class="text-sm">👁️</span> View
                    </a>
                    <button onclick={() => openEditForm(actor)} class="text-gold-400 hover:text-gold-300 font-bold px-3 py-1 bg-gold-500/10 rounded-lg transition-colors">Edit</button>
                    <button onclick={() => deleteActor(actor.uuid)} class="text-red-400 hover:text-red-300 font-bold px-3 py-1 bg-red-500/10 rounded-lg transition-colors">Hapus</button>
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
