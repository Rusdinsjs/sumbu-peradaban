<script lang="ts">
  import { auth } from '$lib/stores/auth.svelte';
  
  // This is a placeholder for the Approval Dashboard
  // In a full implementation, this will fetch events/actors where curationTier === 'DRAFT'
</script>

<div class="p-8 max-w-6xl mx-auto animate-fade-in">
  <header class="mb-8">
    <h1 class="text-3xl font-extrabold text-text-primary">Dashboard Kurasi</h1>
    <p class="text-sm text-text-muted mt-2">Pusat kontrol persetujuan data Sumbu Peradaban.</p>
  </header>

  <!-- Stats Grid -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
    <div class="glass p-6 rounded-2xl border border-gold-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">📜</div>
      <h3 class="text-xs font-bold text-gold-400 uppercase tracking-wider mb-1">Menunggu Persetujuan</h3>
      <div class="text-4xl font-extrabold text-text-primary">0</div>
      <p class="text-[10px] text-text-muted mt-2">Peristiwa status Draft</p>
    </div>
    
    <div class="glass p-6 rounded-2xl border border-emerald-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">👤</div>
      <h3 class="text-xs font-bold text-emerald-400 uppercase tracking-wider mb-1">Tokoh Baru</h3>
      <div class="text-4xl font-extrabold text-text-primary">0</div>
      <p class="text-[10px] text-text-muted mt-2">Menunggu validasi historis</p>
    </div>

    <div class="glass p-6 rounded-2xl border border-blue-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">✅</div>
      <h3 class="text-xs font-bold text-blue-400 uppercase tracking-wider mb-1">Data Terverifikasi</h3>
      <div class="text-4xl font-extrabold text-text-primary">0</div>
      <p class="text-[10px] text-text-muted mt-2">Total entitas di database</p>
    </div>
  </div>

  <!-- Approval Table -->
  <div class="glass rounded-2xl border border-border/10 overflow-hidden">
    <div class="p-6 border-b border-border/10 flex justify-between items-center">
      <h2 class="text-lg font-bold text-text-primary">Antrean Validasi (Drafts)</h2>
      
      {#if auth.isAdmin}
        <span class="px-3 py-1 bg-red-500/10 border border-red-500/30 text-red-400 text-xs font-bold rounded-full">
          Mode Admin Aktif
        </span>
      {:else}
        <span class="px-3 py-1 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-bold rounded-full">
          Mode Editor Aktif
        </span>
      {/if}
    </div>

    <div class="p-8 text-center">
      <div class="text-4xl mb-4">📭</div>
      <h3 class="text-sm font-bold text-text-primary">Tidak Ada Data Tertunda</h3>
      <p class="text-xs text-text-muted mt-1 max-w-md mx-auto">
        Semua data yang dimasukkan oleh editor telah disetujui. Silakan gunakan menu di samping untuk mengelola entitas.
      </p>
    </div>
  </div>
</div>
