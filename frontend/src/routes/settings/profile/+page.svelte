<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import { auth } from '$lib/stores/auth.svelte';

  let isLoading = $state(true);
  let isSaving = $state(false);
  let errorMsg = $state('');
  let successMsg = $state('');

  let formData = $state({
    fullName: '',
    email: '',
    avatarUrl: '',
    password: '',
  });

  async function loadProfile() {
    try {
      const res = await gql(`query { me { id username email fullName avatarUrl role } }`) as any;
      const me = res.me;
      formData.fullName = me.fullName || '';
      formData.email = me.email || '';
      formData.avatarUrl = me.avatarUrl || '';
    } catch (e: any) {
      errorMsg = e.message || 'Gagal memuat profil.';
    } finally {
      isLoading = false;
    }
  }

  async function saveProfile() {
    isSaving = true;
    errorMsg = '';
    successMsg = '';
    try {
      const mutation = `
        mutation UpdateProfile($input: UpdateProfileInput!) {
          updateProfile(input: $input) { id }
        }
      `;
      const vars = {
        input: {
          fullName: formData.fullName || null,
          email: formData.email || null,
          avatarUrl: formData.avatarUrl || null,
          password: formData.password || null,
        }
      };
      await gql(mutation, vars);
      successMsg = 'Profil berhasil diperbarui!';
      formData.password = ''; // clear password field
    } catch (e: any) {
      errorMsg = e.message || 'Gagal memperbarui profil.';
    } finally {
      isSaving = false;
    }
  }

  async function handleAvatarUpload(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;

    if (!file.type.startsWith('image/')) {
      errorMsg = 'File harus berupa gambar.';
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement('canvas');
        const MAX_SIZE = 256;
        let width = img.width;
        let height = img.height;

        if (width > height) {
          if (width > MAX_SIZE) {
            height *= MAX_SIZE / width;
            width = MAX_SIZE;
          }
        } else {
          if (height > MAX_SIZE) {
            width *= MAX_SIZE / height;
            height = MAX_SIZE;
          }
        }
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext('2d');
        ctx?.drawImage(img, 0, 0, width, height);
        
        // Convert to Base64 (WebP or JPEG)
        const dataUrl = canvas.toDataURL('image/jpeg', 0.8);
        formData.avatarUrl = dataUrl;
      };
      img.src = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  onMount(() => loadProfile());
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-4xl mx-auto relative">
  <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-900/20 via-navy-950 to-navy-950 -z-10 pointer-events-none"></div>

  <div class="flex flex-col gap-2">
    <h1 class="text-3xl font-black text-text-primary tracking-tight">My Profile</h1>
    <p class="text-sm text-text-secondary leading-relaxed max-w-2xl">
      Kelola informasi profil, avatar, dan kata sandi Anda.
    </p>
  </div>

  {#if errorMsg}
    <div class="p-4 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm font-bold">
      {errorMsg}
    </div>
  {/if}

  {#if successMsg}
    <div class="p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-lg text-emerald-400 text-sm font-bold">
      {successMsg}
    </div>
  {/if}

  <div class="bg-navy-900/40 backdrop-blur-md border border-border/10 rounded-xl shadow-2xl p-8 flex flex-col gap-6">
    {#if isLoading}
      <div class="text-text-secondary text-sm">Memuat profil...</div>
    {:else}
      <div class="flex flex-col md:flex-row gap-8 items-start">
        <div class="flex-shrink-0 flex flex-col items-center gap-4">
          <div class="relative w-32 h-32 rounded-full bg-navy-950 border border-border/20 overflow-hidden flex items-center justify-center group">
            {#if formData.avatarUrl}
              <img src={formData.avatarUrl} alt="Avatar" class="w-full h-full object-cover" />
            {:else}
              <span class="text-4xl">👤</span>
            {/if}
            <label class="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 flex items-center justify-center cursor-pointer transition-opacity">
              <span class="text-xs font-bold text-white">Ubah Foto</span>
              <input type="file" accept="image/*" onchange={handleAvatarUpload} class="hidden" />
            </label>
          </div>
          <span class="text-xs font-bold text-text-secondary px-3 py-1 bg-navy-950/50 rounded-full border border-border/10">
            {auth.isAdmin ? 'Admin' : (auth.isEditor ? 'Editor' : 'Visitor')}
          </span>
        </div>

        <div class="flex-1 grid grid-cols-1 gap-6 w-full">
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-bold text-text-secondary">Nama Lengkap</label>
            <input type="text" bind:value={formData.fullName} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Fulan bin Fulan">
          </div>
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-bold text-text-secondary">Email</label>
            <input type="email" bind:value={formData.email} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="fulan@example.com">
          </div>
          <div class="flex flex-col gap-1.5 pt-4 border-t border-border/10">
            <label class="text-xs font-bold text-text-secondary">Ganti Kata Sandi (Kosongkan jika tidak ingin ganti)</label>
            <input type="password" bind:value={formData.password} class="bg-navy-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-blue-500/50 outline-none transition-colors" placeholder="Kata Sandi Baru">
          </div>
          <div class="pt-4 flex justify-end">
            <button onclick={saveProfile} disabled={isSaving} class="px-6 py-3 bg-blue-600 hover:bg-blue-500 text-white font-bold rounded-lg transition-colors text-sm disabled:opacity-50">
              {isSaving ? 'Menyimpan...' : 'Simpan Profil'}
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
