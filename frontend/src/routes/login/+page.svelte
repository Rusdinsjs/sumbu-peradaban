<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth.svelte';
  import { gql } from '$lib/graphql/client';

  let username = $state('');
  let password = $state('');
  let isRegister = $state(false);
  let errorMsg = $state('');
  let isLoading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    errorMsg = '';
    isLoading = true;

    const mutation = isRegister 
      ? `mutation { register(input: { username: "${username}", password: "${password}" }) { token user { id username role } } }`
      : `mutation { login(input: { username: "${username}", password: "${password}" }) { token user { id username role } } }`;

    try {
      const result: any = await gql(mutation);
      const data = isRegister ? result.register : result.login;
      
      auth.login(data.token, {
        id: data.user.id,
        username: data.user.username,
        role: data.user.role
      });
      
      goto('/factory');
    } catch (err: any) {
      errorMsg = err.message || 'Terjadi kesalahan saat otentikasi.';
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center p-6 relative overflow-hidden bg-background">
  <!-- Dynamic Orbs Background (Consistent with other pages) -->
  <div class="absolute -top-1/4 -right-1/4 w-[80vw] h-[80vw] max-w-3xl max-h-3xl rounded-full bg-gold-600/10 blur-[120px] mix-blend-screen pointer-events-none"></div>
  <div class="absolute -bottom-1/4 -left-1/4 w-[60vw] h-[60vw] max-w-2xl max-h-2xl rounded-full bg-verdigris-600/10 blur-[100px] mix-blend-screen pointer-events-none"></div>

  <div class="glass p-8 md:p-10 rounded-3xl w-full max-w-md border border-border/20 shadow-2xl relative z-10 animate-fade-in-up">
    <div class="text-center mb-8">
      <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gold-500/10 border border-gold-500/30 text-3xl mb-4 text-gold-400">
        🛡️
      </div>
      <h1 class="text-2xl font-bold text-text-primary tracking-tight">
        {isRegister ? 'Daftar Akses' : 'Otentikasi Sistem'}
      </h1>
      <p class="text-sm text-text-muted mt-2">
        {isRegister ? 'Registrasi identitas ke dalam Sumbu Peradaban.' : 'Silakan masukkan kredensial Anda.'}
      </p>
    </div>

    {#if errorMsg}
      <div class="mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-sm flex items-start gap-3">
        <span class="mt-0.5">⚠️</span>
        <p>{errorMsg}</p>
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="flex flex-col gap-5">
      <div class="flex flex-col gap-1.5">
        <label for="username" class="text-xs font-bold text-text-secondary uppercase tracking-wider pl-1">ID Pengguna</label>
        <input 
          id="username"
          type="text" 
          bind:value={username} 
          required 
          class="w-full bg-surface-lighter/50 border border-border/30 rounded-xl px-4 py-3 text-sm text-text-primary focus:outline-none focus:border-gold-500/50 focus:ring-1 focus:ring-gold-500/50 transition-all placeholder:text-text-muted/50"
          placeholder="Masukkan username..."
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <label for="password" class="text-xs font-bold text-text-secondary uppercase tracking-wider pl-1">Kata Sandi</label>
        <input 
          id="password"
          type="password" 
          bind:value={password} 
          required 
          class="w-full bg-surface-lighter/50 border border-border/30 rounded-xl px-4 py-3 text-sm text-text-primary focus:outline-none focus:border-gold-500/50 focus:ring-1 focus:ring-gold-500/50 transition-all placeholder:text-text-muted/50"
          placeholder="••••••••"
        />
      </div>

      <button 
        type="submit" 
        disabled={isLoading}
        class="mt-2 w-full bg-gold-600 hover:bg-gold-500 text-iron-950 font-bold py-3.5 px-6 rounded-xl transition-all shadow-[0_0_20px_rgba(212,175,55,0.3)] hover:shadow-[0_0_30px_rgba(212,175,55,0.5)] flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if isLoading}
          <span class="w-4 h-4 border-2 border-iron-950/30 border-t-iron-950 rounded-full animate-spin"></span>
          Memproses...
        {:else}
          {isRegister ? 'Daftar' : 'Masuk (Login)'}
        {/if}
      </button>
    </form>

    <div class="mt-8 text-center">
      <button 
        type="button"
        class="text-xs text-text-muted hover:text-gold-400 transition-colors"
        onclick={() => { isRegister = !isRegister; errorMsg = ''; }}
      >
        {isRegister 
          ? 'Sudah memiliki akses? Kembali ke Login' 
          : 'Belum terdaftar? Minta akses kontributor (Visitor)'}
      </button>
    </div>
  </div>
</div>
