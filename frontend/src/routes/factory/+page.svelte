<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/stores/auth.svelte';
  import { gql } from '$lib/graphql/client';
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let events = $state<any[]>([]);
  let actors = $state<any[]>([]);
  let locations = $state<any[]>([]);
  let isLoading = $state(true);

  // Derived counts
  const draftEventsCount = $derived(events.filter(e => e.curationTier === 'DRAFT').length);
  const draftActorsCount = $derived(actors.filter(a => a.curationTier === 'DRAFT').length);
  const draftLocationsCount = $derived(locations.filter(l => l.curationTier === 'DRAFT').length);
  
  const totalVerifiedCount = $derived(
    events.filter(e => e.curationTier !== 'DRAFT').length +
    actors.filter(a => a.curationTier !== 'DRAFT').length +
    locations.filter(l => l.curationTier !== 'DRAFT').length
  );

  // Drafts queue for the table
  const draftQueue = $derived([
    ...events.filter(e => e.curationTier === 'DRAFT').map(e => ({ id: e.uuid, title: e.title, type: 'Event', date: `${e.gregorianDate?.year || '-'} M`, tier: 'draft' })),
    ...actors.filter(a => a.curationTier === 'DRAFT').map(a => ({ id: a.uuid, title: a.name, type: 'Actor', date: 'N/A', tier: 'draft' })),
    ...locations.filter(l => l.curationTier === 'DRAFT').map(l => ({ id: l.uuid, title: l.name, type: 'Location', date: 'N/A', tier: 'draft' }))
  ]);

  async function loadStats() {
    isLoading = true;
    try {
      const query = `
        query {
          events {
            uuid
            title
            curationTier
            gregorianDate { year }
          }
          actors {
            uuid
            name
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
      events = res.events || [];
      actors = res.actors || [];
      locations = res.locations || [];
    } catch (err) {
      console.error('Failed to load dashboard stats', err);
    } finally {
      isLoading = false;
    }
  }

  onMount(async () => {
    await loadStats();
  });
</script>

<div class="p-8 max-w-6xl mx-auto animate-fade-in">
  <header class="mb-8">
    <h1 class="text-3xl font-extrabold text-text-primary">Dashboard Kurasi</h1>
    <p class="text-sm text-text-muted mt-2">Pusat kontrol persetujuan data Sumbu Peradaban.</p>
  </header>

  <!-- Stats Grid -->
  <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
    <div class="glass p-6 rounded-2xl border border-gold-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">📜</div>
      <h3 class="text-xs font-bold text-gold-400 uppercase tracking-wider mb-1">Peristiwa Tertunda</h3>
      <div class="text-4xl font-extrabold text-text-primary">{draftEventsCount}</div>
      <p class="text-[10px] text-text-muted mt-2">Peristiwa berstatus Draft</p>
    </div>
    
    <div class="glass p-6 rounded-2xl border border-verdigris-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">👤</div>
      <h3 class="text-xs font-bold text-verdigris-400 uppercase tracking-wider mb-1">Tokoh Tertunda</h3>
      <div class="text-4xl font-extrabold text-text-primary">{draftActorsCount}</div>
      <p class="text-[10px] text-text-muted mt-2">Tokoh baru menunggu validasi</p>
    </div>

    <div class="glass p-6 rounded-2xl border border-amber-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">📍</div>
      <h3 class="text-xs font-bold text-amber-400 uppercase tracking-wider mb-1">Lokasi Tertunda</h3>
      <div class="text-4xl font-extrabold text-text-primary">{draftLocationsCount}</div>
      <p class="text-[10px] text-text-muted mt-2">Lokasi baru menunggu kurasi</p>
    </div>
 
    <div class="glass p-6 rounded-2xl border border-blue-500/20 relative overflow-hidden group">
      <div class="absolute -right-4 -bottom-4 text-6xl opacity-10 group-hover:scale-110 transition-transform">✅</div>
      <h3 class="text-xs font-bold text-blue-400 uppercase tracking-wider mb-1">Data Terverifikasi</h3>
      <div class="text-4xl font-extrabold text-text-primary">{totalVerifiedCount}</div>
      <p class="text-[10px] text-text-muted mt-2">Total entitas terverifikasi</p>
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
        <span class="px-3 py-1 bg-verdigris-500/10 border border-verdigris-500/30 text-verdigris-400 text-xs font-bold rounded-full">
          Mode Editor Aktif
        </span>
      {/if}
    </div>

    {#if isLoading}
      <div class="p-12 text-center flex justify-center">
        <span class="animate-spin w-8 h-8 border-4 border-gold-500/20 border-t-gold-500 rounded-full"></span>
      </div>
    {:else if draftQueue.length === 0}
      <div class="p-8 text-center">
        <div class="text-4xl mb-4">📭</div>
        <h3 class="text-sm font-bold text-text-primary">Tidak Ada Data Tertunda</h3>
        <p class="text-xs text-text-muted mt-1 max-w-md mx-auto">
          Semua data yang dimasukkan oleh editor telah disetujui. Silakan gunakan menu di samping untuk mengelola entitas.
        </p>
      </div>
    {:else}
      <table class="w-full text-left text-sm text-text-secondary">
        <thead class="bg-surface-lighter/50 text-xs uppercase text-text-muted">
          <tr>
            <th class="px-6 py-4 font-bold tracking-wider">Nama / Judul Entitas</th>
            <th class="px-6 py-4 font-bold tracking-wider">Tipe</th>
            <th class="px-6 py-4 font-bold tracking-wider">Waktu / Info</th>
            <th class="px-6 py-4 font-bold tracking-wider text-right">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border/10">
          {#each draftQueue as item}
            <tr class="hover:bg-surface-lighter/20 transition-colors">
              <td class="px-6 py-4 font-bold text-text-primary">{item.title}</td>
              <td class="px-6 py-4 text-xs font-semibold uppercase tracking-wider text-gold-400/80">{item.type}</td>
              <td class="px-6 py-4 text-xs font-mono">{item.date}</td>
              <td class="px-6 py-4 text-right">
                <a href="/curator" class="text-gold-400 hover:text-gold-300 font-bold px-3 py-1 bg-gold-500/10 rounded-lg transition-colors inline-block text-xs">
                  Buka Panel Kurasi
                </a>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>
