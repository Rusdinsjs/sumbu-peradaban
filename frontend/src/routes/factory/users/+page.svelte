<script lang="ts">
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';
  import { auth } from '$lib/stores/auth.svelte';

  let users: any[] = $state([]);
  let isLoading = $state(true);
  let errorMsg = $state('');
  
  let isFormOpen = $state(false);
  let isSaving = $state(false);
  let isEditMode = $state(false);
  
  let formData = $state({
    id: '',
    username: '',
    email: '',
    fullName: '',
    role: 'Visitor',
    password: ''
  });

  async function loadUsers() {
    try {
      const res = await gql(`query { users { id username email fullName role } }`) as any;
      users = res.users || [];
    } catch (e: any) {
      errorMsg = e.message || 'Gagal memuat user.';
    } finally {
      isLoading = false;
    }
  }

  function openCreateForm() {
    formData = { id: '', username: '', email: '', fullName: '', role: 'Visitor', password: '' };
    isEditMode = false;
    isFormOpen = true;
    errorMsg = '';
  }

  function openEditForm(user: any) {
    formData = {
      id: user.id,
      username: user.username,
      email: user.email || '',
      fullName: user.fullName || '',
      role: user.role,
      password: '' // Only filled if they want to change it
    };
    isEditMode = true;
    isFormOpen = true;
    errorMsg = '';
  }

  async function saveUser() {
    isSaving = true;
    errorMsg = '';
    try {
      if (isEditMode) {
        const mutation = `
          mutation AdminUpdateUser($userId: UUID!, $input: AdminUpdateUserInput!) {
            adminUpdateUser(userId: $userId, input: $input) { id }
          }
        `;
        const vars = {
          userId: formData.id,
          input: {
            username: formData.username,
            email: formData.email || null,
            fullName: formData.fullName || null,
            role: formData.role,
            password: formData.password || null
          }
        };
        await gql(mutation, vars);
      } else {
        if (!formData.password) throw new Error("Kata sandi wajib diisi untuk pengguna baru!");
        const mutation = `
          mutation AdminCreateUser($input: AdminCreateUserInput!) {
            adminCreateUser(input: $input) { id }
          }
        `;
        const vars = {
          input: {
            username: formData.username,
            email: formData.email || null,
            fullName: formData.fullName || null,
            role: formData.role,
            password: formData.password
          }
        };
        await gql(mutation, vars);
      }
      isFormOpen = false;
      await loadUsers();
    } catch (e: any) {
      errorMsg = e.message || 'Gagal menyimpan user.';
    } finally {
      isSaving = false;
    }
  }

  async function deleteUser(userId: string) {
    if (userId === auth.user?.id) {
      alert("Anda tidak bisa menghapus akun Anda sendiri!");
      return;
    }
    if (!confirm('Apakah Anda yakin ingin menghapus user ini secara permanen?')) return;
    try {
      const mutation = `mutation { deleteUser(userId: "${userId}") }`;
      await gql(mutation) as any;
      await loadUsers();
    } catch (e: any) {
      alert(e.message || 'Gagal menghapus user');
    }
  }

  onMount(() => loadUsers());
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-6xl mx-auto relative pt-16 sm:pt-20">
  <div class="flex flex-col gap-2">
    <div class="flex justify-between items-center">
      <h1 class="text-3xl font-black text-red-400 tracking-tight">Manajemen User</h1>
      <button onclick={openCreateForm} class="px-4 py-2 bg-red-600 hover:bg-red-500 text-white font-bold rounded-lg shadow-lg shadow-red-500/20 transition-all flex items-center gap-2">
        <span>+</span> Tambah User
      </button>
    </div>
    <p class="text-sm text-text-secondary leading-relaxed max-w-2xl">
      Pengaturan hak akses dan administrasi pengguna Sumbu Peradaban.
    </p>
  </div>

  {#if errorMsg && !isFormOpen}
    <div class="p-4 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm font-bold">
      {errorMsg}
    </div>
  {/if}

  <div class="bg-iron-900/40 backdrop-blur-md border border-red-500/10 rounded-xl shadow-2xl overflow-hidden">
    <div class="overflow-x-auto">
      <table class="w-full text-left text-sm">
        <thead class="bg-iron-950/40 border-b border-red-500/10 text-red-400/80">
          <tr>
            <th class="px-6 py-4 font-bold">Username</th>
            <th class="px-6 py-4 font-bold">Nama Lengkap</th>
            <th class="px-6 py-4 font-bold">Email</th>
            <th class="px-6 py-4 font-bold">Role Aktif</th>
            <th class="px-6 py-4 font-bold text-right">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-red-500/5">
          {#if isLoading}
            <tr>
              <td colspan="5" class="px-6 py-8 text-center text-text-secondary">Memuat data...</td>
            </tr>
          {:else if users.length === 0}
            <tr>
              <td colspan="5" class="px-6 py-8 text-center text-text-secondary">Belum ada user.</td>
            </tr>
          {:else}
            {#each users as user}
              <tr class="hover:bg-iron-950/20 transition-colors">
                <td class="px-6 py-4 font-bold text-text-primary">{user.username}</td>
                <td class="px-6 py-4 text-text-secondary">{user.fullName || '-'}</td>
                <td class="px-6 py-4 text-text-secondary">{user.email || '-'}</td>
                <td class="px-6 py-4 text-text-secondary">
                  <span class="px-2 py-1 bg-iron-950/50 rounded text-xs font-bold uppercase border border-border/10">
                    {user.role}
                  </span>
                </td>
                <td class="px-6 py-4 text-right">
                  <div class="flex items-center justify-end gap-2">
                    <button onclick={() => openEditForm(user)} class="text-blue-400 hover:text-blue-300 font-bold px-3 py-1 bg-blue-500/10 rounded-lg transition-colors">Edit</button>
                    <button onclick={() => deleteUser(user.id)} class="text-red-400 hover:text-red-300 font-bold px-3 py-1 bg-red-500/10 rounded-lg transition-colors disabled:opacity-50" disabled={user.id === auth.user?.id}>Hapus</button>
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

{#if isFormOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <div class="absolute inset-0 bg-iron-950/80 backdrop-blur-sm" onclick={() => isFormOpen = false}></div>
    <div class="bg-iron-900 border border-red-500/20 rounded-2xl shadow-2xl w-full max-w-lg relative z-10 flex flex-col max-h-[90vh]">
      <div class="p-6 border-b border-white/5 flex justify-between items-center flex-shrink-0">
        <h2 class="text-xl font-black text-red-400">{isEditMode ? 'Edit User' : 'Tambah User Baru'}</h2>
        <button onclick={() => isFormOpen = false} class="text-text-secondary hover:text-white transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>
      
      <div class="p-6 overflow-y-auto flex-1 flex flex-col gap-4">
        {#if errorMsg}
          <div class="p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-400 text-sm font-bold">
            {errorMsg}
          </div>
        {/if}

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Username *</label>
          <input type="text" bind:value={formData.username} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-red-500/50 outline-none" placeholder="Contoh: ahmad123">
        </div>
        
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Nama Lengkap</label>
          <input type="text" bind:value={formData.fullName} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-red-500/50 outline-none" placeholder="Contoh: Ahmad Abdullah">
        </div>
        
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Email</label>
          <input type="email" bind:value={formData.email} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-red-500/50 outline-none" placeholder="ahmad@example.com">
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">Role *</label>
          <select bind:value={formData.role} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-red-500/50 outline-none">
            <option value="Visitor">Visitor</option>
            <option value="Contributor">Contributor</option>
            <option value="Editor">Editor</option>
            <option value="Reviewer">Reviewer</option>
            <option value="Admin">Admin</option>
          </select>
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-text-secondary">{isEditMode ? 'Kata Sandi (Kosongkan jika tidak diubah)' : 'Kata Sandi *'}</label>
          <input type="password" bind:value={formData.password} class="bg-iron-950/60 border border-border/10 rounded-lg p-3 text-sm text-text-primary focus:border-red-500/50 outline-none" placeholder="********">
        </div>
      </div>

      <div class="p-6 border-t border-white/5 bg-iron-950/50 flex justify-end gap-3 flex-shrink-0">
        <button onclick={() => isFormOpen = false} class="px-4 py-2 text-sm font-bold text-text-secondary hover:text-white transition-colors">
          Batal
        </button>
        <button onclick={saveUser} disabled={isSaving || !formData.username || (!isEditMode && !formData.password)} class="px-6 py-2 bg-red-600 hover:bg-red-500 text-white text-sm font-bold rounded-lg transition-colors shadow-lg shadow-red-500/20 disabled:opacity-50">
          {isSaving ? 'Menyimpan...' : 'Simpan'}
        </button>
      </div>
    </div>
  </div>
{/if}
