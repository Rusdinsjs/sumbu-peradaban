<script lang="ts">
  import { page } from '$app/state';
  import CurationBadge from '$lib/components/CurationBadge.svelte';
  import MapView from '$lib/components/MapView.svelte';
  import GraphExplorer from '$lib/components/GraphExplorer.svelte';

  const uuid = $derived(page.params.uuid);

  // Mock Event details matching Sumbu Peradaban structure
  const event = {
    title: 'Peristiwa Hijrah ke Madinah',
    hijriYear: '1 H',
    gregorianYear: '622 M',
    description: 'Peristiwa penting kepindahan umat Islam dan Rasulullah ﷺ dari Makkah ke Madinah. Peristiwa ini melambangkan titik awal pembentukan peradaban Islam yang mandiri, peletakan sendi-sendi Daulah Madinah, serta dijadikan jangkar penomoran tahun kalender Hijriah.',
    tier: 'canonical',
    precision: 'Exact',
    globalHook: {
      is_connected_to_global: true,
      global_pivot_category: 'Migrasi Peradaban & Kelahiran Kosmopolitan Baru'
    },
    actors: [
      { name: 'Nabi Muhammad ﷺ', role: 'Pemimpin Utama' },
      { name: 'Abu Bakar As-Siddiq', role: 'Sahabat Utama / Pendamping' }
    ],
    locations: [
      { name: 'Makkah', type: 'Titik Keberangkatan', lat: 21.4225, lng: 39.8262 },
      { name: 'Madinah', type: 'Titik Tujuan', lat: 24.4672, lng: 39.6112 }
    ],
    sources: [
      { id: '1', domain: 'Hadith Sahih', text: 'Shahih al-Bukhari, Bab Al-Hijrah, Hadits No. 3905.', score: 0.98 },
      { id: '2', domain: 'Historiografi Klasik', text: 'Sirah Nabawiyyah Ibnu Hisyam, Vol. 2.', score: 0.95 }
    ]
  };
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back Link -->
  <a href="/timeline" class="text-xs text-gold-500 hover:text-gold-400 transition-colors font-bold flex items-center gap-1">
    ← Kembali ke Timeline
  </a>

  <!-- Header Card -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
    <div>
      <div class="flex items-center gap-3">
        <span class="text-[10px] uppercase font-bold tracking-wider text-gold-500">Peristiwa Utama</span>
        <CurationBadge tier={event.tier as any} size="sm" />
      </div>
      <h1 class="text-xl md:text-2xl font-extrabold text-text-primary mt-1">{event.title}</h1>
    </div>

    <!-- Dual Date Badges -->
    <div class="flex gap-2">
      <div class="px-4 py-2 rounded-xl bg-gold-500/10 border border-gold-500/20 text-center min-w-[70px]">
        <div class="text-[10px] text-gold-500 font-bold">HIJRIAH</div>
        <div class="text-sm font-extrabold text-gold-400">{event.hijriYear}</div>
      </div>
      <div class="px-4 py-2 rounded-xl bg-navy-950/60 border border-border/10 text-center min-w-[70px]">
        <div class="text-[10px] text-text-secondary font-bold">GREGORIAN</div>
        <div class="text-sm font-extrabold text-text-primary">{event.gregorianYear}</div>
      </div>
    </div>
  </div>

  <!-- Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Description & Sources -->
    <div class="lg:col-span-2 flex flex-col gap-6">
      <!-- Description -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-3">
        <h2 class="text-sm font-bold text-gold-400">Deskripsi Narasi Sejarah</h2>
        <p class="text-xs text-text-secondary leading-relaxed font-light">
          {event.description}
        </p>
      </div>

      <!-- Expansion Hook (Global Connection) -->
      {#if event.globalHook.is_connected_to_global}
        <div class="glass p-5 rounded-2xl border border-emerald-500/20 bg-emerald-950/10 flex gap-4">
          <div class="text-xl">🌍</div>
          <div>
            <h3 class="text-xs font-bold text-emerald-400">Hubungan Sejarah Global (Pivot Sumbu)</h3>
            <p class="text-xs text-text-secondary mt-1 leading-relaxed">
              Kategori Keterhubungan: <span class="text-gold-400 font-bold">{event.globalHook.global_pivot_category}</span>
            </p>
          </div>
        </div>
      {/if}

      <!-- Sources / References -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Kredibilitas Sumber & Pembuktian (Dimension 4)</h2>
        <div class="flex flex-col gap-3">
          {#each event.sources as src}
            <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 flex flex-col md:flex-row md:justify-between md:items-center gap-3">
              <div>
                <span class="text-[10px] font-bold text-gold-500 bg-gold-500/10 px-2 py-0.5 rounded">
                  {src.domain}
                </span>
                <p class="text-xs text-text-primary mt-2 font-medium">{src.text}</p>
              </div>
              <div class="text-right flex-shrink-0">
                <span class="text-[10px] text-text-secondary block">Reliability Score</span>
                <span class="text-xs font-extrabold text-emerald-400">{(src.score * 100).toFixed(0)}% Match</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Linked Actors & Locations Sidebar -->
    <div class="flex flex-col gap-6">
      <!-- Linked Actors -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Aktor Terkait</h2>
        <div class="flex flex-col gap-3">
          {#each event.actors as actor}
            <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 hover:border-gold-500/20 transition-all flex justify-between items-center">
              <div>
                <h4 class="text-xs font-bold text-text-primary">{actor.name}</h4>
                <p class="text-[10px] text-text-muted mt-0.5">{actor.role}</p>
              </div>
              <span class="text-xs text-gold-500">→</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- Linked Locations -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400">Lokasi Terkait</h2>
        <div class="flex flex-col gap-3">
          {#each event.locations as loc}
            <div class="p-3.5 rounded-xl bg-navy-950/60 border border-border/10 flex flex-col gap-1">
              <h4 class="text-xs font-bold text-text-primary">{loc.name}</h4>
              <p class="text-[10px] text-text-muted">{loc.type}</p>
              <p class="text-[9px] text-text-muted mt-1 font-mono">{loc.lat.toFixed(4)}° N, {loc.lng.toFixed(4)}° E</p>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
