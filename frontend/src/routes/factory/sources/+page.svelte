<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import { fade } from 'svelte/transition';

  let sources = $state<any[]>([]);
  let isLoading = $state(true);
  
  let showForm = $state(false);
  let formMode = $state('create'); // 'create' | 'edit'
  let notification = $state<{type: 'success' | 'error', message: string} | null>(null);

  function showNotification(type: 'success' | 'error', message: string) {
    notification = { type, message };
    setTimeout(() => { notification = null; }, 4000);
  }

  let formData = $state({
    sourceId: '',
    domain: 'Sejarah',
    title: '',
    author: '',
    publicationEra: '',
    referenceText: '',
    interpretationMethod: '',
    reliabilityScore: '',
    reliabilityAssessment: '',
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
          sources {
            sourceId
            domain
            title
            author
            publicationEra
            referenceText
            interpretationMethod
            reliabilityScore
            reliabilityAssessment
            mediaLinks {
              mediaType
              url
              title
            }
          }
        }
      `) as any;
      sources = res.sources || [];
    } catch (err) {
      console.error('Failed to load sources', err);
    } finally {
      isLoading = false;
    }
  }

  function openCreateForm() {
    formMode = 'create';
    resetForm();
    showForm = true;
  }

  function openEditForm(src: any) {
    formMode = 'edit';
    formData = {
      sourceId: src.sourceId,
      domain: src.domain || 'Sejarah',
      title: src.title || '',
      author: src.author || '',
      publicationEra: src.publicationEra || '',
      referenceText: src.referenceText || '',
      interpretationMethod: src.interpretationMethod || '',
      reliabilityScore: src.reliabilityScore?.toString() || '',
      reliabilityAssessment: src.reliabilityAssessment || '',
      mediaLinks: src.mediaLinks ? src.mediaLinks.map((ml: any) => ({
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
      sourceId: '',
      domain: 'Sejarah',
      title: '',
      author: '',
      publicationEra: '',
      referenceText: '',
      interpretationMethod: '',
      reliabilityScore: '',
      reliabilityAssessment: '',
      mediaLinks: []
    };
    submitError = '';
  }

  function addMediaLink() {
    if (!newMedia.url) return;
    formData.mediaLinks = [...formData.mediaLinks, {
      mediaType: newMedia.mediaType,
      url: newMedia.url,
      title: newMedia.title || 'Berkas Rujukan'
    }];
    newMedia.url = '';
    newMedia.title = '';
  }

  function removeMediaLink(index: number) {
    formData.mediaLinks = formData.mediaLinks.filter((_, i) => i !== index);
  }

  async function saveSource() {
    if (!formData.referenceText) {
      submitError = 'Kutipan / teks referensi primer wajib diisi.';
      return;
    }
    isSaving = true;
    submitError = '';

    try {
      if (formMode === 'create') {
        const createMutation = `
          mutation CreateSource($input: CreateSourceInput!) {
            createSource(input: $input) { sourceId }
          }
        `;
        const createVars = {
          input: {
            domain: formData.domain,
            title: formData.title || null,
            author: formData.author || null,
            publicationEra: formData.publicationEra || null,
            referenceText: formData.referenceText,
            interpretationMethod: formData.interpretationMethod || null,
            reliabilityScore: formData.reliabilityScore ? parseFloat(formData.reliabilityScore) : null,
            reliabilityAssessment: formData.reliabilityAssessment || null,
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
          mutation UpdateSource($sourceId: UUID!, $input: UpdateSourceInput!) {
            updateSource(sourceId: $sourceId, input: $input) { sourceId }
          }
        `;
        const updateVars = {
          sourceId: formData.sourceId,
          input: {
            domain: formData.domain,
            title: formData.title || null,
            author: formData.author || null,
            publicationEra: formData.publicationEra || null,
            referenceText: formData.referenceText,
            interpretationMethod: formData.interpretationMethod || null,
            reliabilityScore: formData.reliabilityScore ? parseFloat(formData.reliabilityScore) : null,
            reliabilityAssessment: formData.reliabilityAssessment || null,
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
      showNotification('success', formMode === 'create' ? 'Rujukan berhasil ditambahkan!' : 'Data rujukan berhasil diperbarui!');
      await loadData();
    } catch (err: any) {
      submitError = err.message || 'Gagal menyimpan rujukan.';
      showNotification('error', submitError);
    } finally {
      isSaving = false;
    }
  }

  async function deleteSource(uuid: string) {
    if (!confirm('Apakah Anda yakin ingin menghapus rujukan ini dari sistem?')) return;
    try {
      const delMutation = `mutation { deleteSource(sourceId: "${uuid}") }`;
      await gql(delMutation) as any;
      showNotification('success', 'Rujukan berhasil dihapus.');
      await loadData();
    } catch (err: any) {
      showNotification('error', err.message || 'Gagal menghapus rujukan.');
    }
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto relative">
  {#if notification}
    <div class="fixed top-8 right-8 z-[100] animate-fade-in flex items-center gap-4 px-6 py-4 rounded-xl shadow-2xl glass {notification.type === 'success' ? 'border-blue-500/50 text-blue-400 bg-blue-950/50' : 'border-red-500/50 text-red-400 bg-red-950/50'}">
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
      <h1 class="text-2xl font-extrabold text-blue-400">Manajemen Rujukan (Source)</h1>
      <p class="text-xs text-text-secondary mt-1">Kelola daftar pustaka, kitab rujukan sejarah, penilaian kesahihan sanad, dan manuskrip historiografi kuno.</p>
    </div>
    <button onclick={openCreateForm} class="px-5 py-2.5 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 text-xs font-bold rounded-xl border border-blue-500/20 transition-all flex items-center gap-2">
      <span>➕</span> Tambah Rujukan
    </button>
  </div>

  {#if showForm}
    <div class="glass p-8 rounded-2xl border border-blue-500/30 shadow-[0_0_30px_rgba(59,130,246,0.05)] relative" transition:fade>
      <button onclick={() => showForm = false} class="absolute top-6 right-6 text-text-muted hover:text-red-400 transition-colors text-xl">✕</button>
      
      <h2 class="text-lg font-bold text-text-primary mb-6">{formMode === 'create' ? 'Tambah Rujukan Baru' : 'Edit Data Rujukan'}</h2>
      
      {#if submitError}
        <div class="bg-red-950/50 border border-red-500/50 text-red-400 p-3 rounded-lg text-xs mb-6 font-bold">{submitError}</div>
      {/if}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="flex flex-col gap-1.5">
          <label for="sourceTitle" class="text-xs font-bold text-text-secondary">Judul Kitab / Buku</label>
          <input id="sourceTitle" type="text" bind:value={formData.title} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Misal: Ar-Rahiq Al-Makhtum">
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="sourceAuthor" class="text-xs font-bold text-text-secondary">Nama Pengarang / Penulis</label>
          <input id="sourceAuthor" type="text" bind:value={formData.author} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Misal: Syeikh Safiyjurrahman Al-Mubarakfuri">
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="sourcePublicationEra" class="text-xs font-bold text-text-secondary">Era Publikasi / Penulisan</label>
          <input id="sourcePublicationEra" type="text" bind:value={formData.publicationEra} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Misal: Abad 14 Hijriyah / Modern">
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="sourceDomain" class="text-xs font-bold text-text-secondary">Domain Studi</label>
          <select id="sourceDomain" bind:value={formData.domain} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors">
            <option value="Sejarah">Sejarah Umum (Tarikh)</option>
            <option value="Teks Suci">Teks Suci / Wahyu</option>
            <option value="Hadits">Hadits / Sanad</option>
            <option value="Arkeologi">Arkeologi / Epigrafi</option>
            <option value="Sains">Jurnal / Riset Sains</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="sourceInterpretationMethod" class="text-xs font-bold text-text-secondary">Metode Interpretasi</label>
          <input id="sourceInterpretationMethod" type="text" bind:value={formData.interpretationMethod} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Misal: Kritik Historis, Analisis Sanad">
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="sourceReliabilityScore" class="text-xs font-bold text-text-secondary">Skor Keandalan (0.00 - 1.00) - {formData.reliabilityScore || '0.85'}</label>
          <div class="flex items-center gap-3">
            <input id="sourceReliabilityScore" type="range" min="0" max="1" step="0.05" bind:value={formData.reliabilityScore} class="flex-1 accent-blue-400">
            <input aria-label="Numeric Reliability Score" type="number" step="0.01" min="0" max="1" bind:value={formData.reliabilityScore} class="w-20 bg-iron-950/60 border border-border/10 rounded-lg p-2 text-center text-xs text-text-primary focus:border-blue-500/50 outline-none">
          </div>
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="sourceReferenceText" class="text-xs font-bold text-text-secondary">Teks Kutipan / Referensi Primer *</label>
          <textarea id="sourceReferenceText" bind:value={formData.referenceText} rows="3" class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors font-serif" placeholder="Tuliskan kutipan naskah sejarah asli, terjemahan, atau bait kronik sejarah yang dikutip..."></textarea>
        </div>

        <div class="flex flex-col gap-1.5 md:col-span-2">
          <label for="sourceReliabilityAssessment" class="text-xs font-bold text-text-secondary">Analisis Kritik Intern & Ekstern (Kesahihan Kualitatif)</label>
          <textarea id="sourceReliabilityAssessment" bind:value={formData.reliabilityAssessment} rows="3" class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Berikan tinjauan ilmiah mengenai keakuratan sanad, ketersambungan periwayatan, serta orisinalitas naskah ini..."></textarea>
        </div>

        <!-- Media Links Section -->
        <div class="flex flex-col gap-3 md:col-span-2 border-t border-border/5 pt-6">
          <span class="text-xs font-bold text-blue-400 flex items-center gap-2">
            <span>🎥</span> Berkas Scan Naskah & Media Pendukung (Gambar, Rekaman, PDF, Dokumen)
          </span>
          
          <div class="grid grid-cols-1 sm:grid-cols-4 gap-3 bg-iron-950/40 p-3 rounded-lg border border-border/5">
            <div class="flex flex-col gap-1">
              <label for="newMediaType" class="text-[10px] font-bold text-text-secondary">Tipe Berkas</label>
              <select id="newMediaType" bind:value={newMedia.mediaType} class="bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-blue-500/50 outline-none">
                <option value="image">Gambar / Scan (Image)</option>
                <option value="audio">Rekaman / Audio (Audio)</option>
                <option value="video">Dokumenter Video (Video)</option>
                <option value="text">Transkrip Teks / PDF (Text)</option>
              </select>
            </div>
            
            <div class="flex flex-col gap-1 sm:col-span-2">
              <label for="newMediaUrl" class="text-[10px] font-bold text-text-secondary">Tautan URL Berkas</label>
              <input id="newMediaUrl" type="text" bind:value={newMedia.url} class="bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-blue-500/50 outline-none" placeholder="https://host.com/manuskrip-kuno.jpg">
            </div>
            
            <div class="flex flex-col gap-1">
              <label for="newMediaTitle" class="text-[10px] font-bold text-text-secondary">Label / Judul</label>
              <div class="flex gap-2">
                <input id="newMediaTitle" type="text" bind:value={newMedia.title} class="flex-1 bg-iron-900 border border-border/10 rounded p-2 text-xs text-text-primary focus:border-blue-500/50 outline-none" placeholder="Lembaran Kitab Asli" onkeydown={(e) => e.key === 'Enter' && addMediaLink()}>
                <button type="button" onclick={addMediaLink} class="px-3 bg-blue-500/20 hover:bg-blue-500/30 text-blue-400 text-xs font-bold rounded border border-blue-500/25 transition-all">Sematkan</button>
              </div>
            </div>
          </div>
          {#if formData.mediaLinks.length > 0}
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mt-2">
              {#each formData.mediaLinks as media, idx}
                <div class="flex items-center justify-between p-2.5 bg-iron-900/60 border border-border/10 rounded-lg text-xs gap-3" transition:fade>
                  <div class="flex items-center gap-2 overflow-hidden">
                    <span class="px-2 py-0.5 bg-iron-950 rounded text-[9px] uppercase border border-border/10 text-blue-400 font-mono">
                      {media.mediaType}
                    </span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="font-bold text-text-primary truncate">{media.title || 'Scan Rujukan'}</span>
                      <a href={media.url} target="_blank" rel="noopener noreferrer" class="text-[10px] text-blue-400 hover:underline truncate">{media.url}</a>
                    </div>
                  </div>
                  <button type="button" onclick={() => removeMediaLink(idx)} class="text-red-400 hover:text-red-300 font-bold px-2 py-1 hover:bg-red-500/10 rounded-md transition-all">✕</button>
                </div>
              {/each}
            </div>
          {:else}
            <span class="text-[11px] text-text-muted">Belum ada scan naskah / berkas media pendukung yang disematkan.</span>
          {/if}
        </div>
      </div>

      <div class="flex justify-end gap-3 mt-8">
        <button onclick={() => showForm = false} class="px-5 py-2.5 text-xs font-bold text-text-secondary hover:text-text-primary transition-colors">Batal</button>
        <button onclick={saveSource} disabled={isSaving} class="px-6 py-2.5 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-400 hover:to-blue-500 text-iron-950 text-xs font-extrabold rounded-xl transition-all shadow-lg shadow-blue-500/20 disabled:opacity-50">
          {isSaving ? 'Menyimpan...' : 'Simpan Rujukan'}
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
            <th class="px-6 py-4 font-bold text-text-secondary">Kitab / Rujukan Utama</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Domain</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Penulis / Era</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Metodologi</th>
            <th class="px-6 py-4 font-bold text-text-secondary">Keandalan</th>
            <th class="px-6 py-4 font-bold text-text-secondary text-right">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/5">
          {#if isLoading}
            <tr><td colspan="6" class="px-6 py-8 text-center text-text-muted">Memuat data...</td></tr>
          {:else if sources.length === 0}
            <tr><td colspan="6" class="px-6 py-8 text-center text-text-muted">Belum ada data rujukan.</td></tr>
          {:else}
            {#each sources as src}
              <tr class="hover:bg-iron-950/20 transition-colors">
                <td class="px-6 py-4 font-bold text-text-primary max-w-xs" title={src.title || src.referenceText}>
                  <div class="flex flex-col">
                    <span class="text-blue-400 font-bold">{src.title || 'Tanpa Judul'}</span>
                    <span class="text-[10px] text-text-muted italic truncate max-w-xs">"{src.referenceText}"</span>
                  </div>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <span class="px-2 py-1 bg-iron-950/50 rounded text-[10px] uppercase border border-border/10">{src.domain}</span>
                </td>
                <td class="px-6 py-4 text-text-secondary">
                  <div class="flex flex-col">
                    <span>{src.author || '-'}</span>
                    <span class="text-[10px] text-text-muted">{src.publicationEra || '-'}</span>
                  </div>
                </td>
                <td class="px-6 py-4 text-text-secondary">{src.interpretationMethod || '-'}</td>
                <td class="px-6 py-4 text-text-secondary font-mono">
                  {#if src.reliabilityScore !== null}
                    <div class="flex items-center gap-1.5">
                      <div class="h-2 w-12 bg-iron-900 rounded-full overflow-hidden border border-border/5">
                        <div class="h-full bg-blue-500" style="width: {src.reliabilityScore * 100}%"></div>
                      </div>
                      <span>{(src.reliabilityScore * 100).toFixed(0)}%</span>
                    </div>
                  {:else}
                    <span class="text-text-muted">-</span>
                  {/if}
                </td>
                <td class="px-6 py-4 text-right">
                  <div class="flex items-center justify-end gap-2">
                    <a href="/source/{src.sourceId}" target="_blank" class="text-violet-400 hover:text-violet-300 font-bold px-3 py-1 bg-violet-500/10 rounded-lg transition-colors flex items-center gap-1">
                      <span class="text-sm">👁️</span> View
                    </a>
                    <button onclick={() => openEditForm(src)} class="text-blue-400 hover:text-blue-300 font-bold px-3 py-1 bg-blue-500/10 rounded-lg transition-colors">Edit</button>
                    <button onclick={() => deleteSource(src.sourceId)} class="text-red-400 hover:text-red-300 font-bold px-3 py-1 bg-red-500/10 rounded-lg transition-colors">Hapus</button>
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
