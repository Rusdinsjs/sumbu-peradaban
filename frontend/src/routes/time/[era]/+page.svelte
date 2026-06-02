<script lang="ts">
  import { page } from '$app/state';
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  const eraParam = $derived(page.params.era);
  const formattedEra = $derived(decodeURIComponent(eraParam || '').toUpperCase());

  const getEraData = (eraCode: string) => {
    return {
      hijriYear: '1 H',
      gregorianYear: '622 M',
      title: 'Tahun Hijrah & Konsolidasi Madinah',
      globalContext: 'Kekaisaran Romawi Timur (Bizantium) sedang terlibat pertempuran hidup-mati dengan Kekaisaran Persia Sassanid di Levant. Sementara itu di jazirah Arab, peta aliansi kesukuan mengalami goncangan besar dengan lahirnya daulah baru di Yatrib (Madinah).',
      scaleStats: {
        activeActors: '34 Tokoh',
        newAlliances: '12 Kabilah',
        expansionSpeed: '300 km/tahun'
      },
      regionalEvents: [
        { region: 'Madinah (Yatrib)', event: 'Deklarasi Piagam Madinah', desc: 'Peletakan dasar konstitusi pluralitas masyarakat pertama.' },
        { region: 'Makkah', event: 'Konsolidasi Quraish', desc: 'Pemulihan wibawa ekonomi klan pasca eksodus umat Islam.' },
        { region: 'Levant (Syam)', event: 'Konflik Romawi-Persia', desc: 'Kekaisaran Heraklius merebut kembali Yerusalem dari tangan dinasti Sassanid.' }
      ]
    };
  };

  const era = $derived(getEraData(formattedEra));
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back link -->
  <a href="/graph" class="text-xs text-gold-500 hover:text-gold-400 transition-colors font-bold flex items-center gap-1">
    ← Kembali ke Graph Explorer
  </a>

  <!-- Header Card (Waktu/Era Style) -->
  <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
    <div class="flex items-center gap-5">
      <div class="w-16 h-16 rounded-2xl bg-gold-500/10 border border-gold-500/30 flex items-center justify-center text-3xl text-gold-400 shadow-[0_0_15px_rgba(212,168,83,0.2)]">
        ⏳
      </div>
      <div>
        <div class="flex items-center gap-3">
          <span class="text-[10px] uppercase font-bold tracking-wider text-gold-500">Dimensi Kronologis Sinkronik</span>
          <CurationBadge tier="canonical" size="sm" />
        </div>
        <h1 class="text-xl md:text-2xl font-extrabold text-text-primary mt-1">Era {era.hijriYear} / {era.gregorianYear}</h1>
        <p class="text-xs text-text-secondary font-medium">Poros Waktu: <span class="text-gold-400">{era.title}</span></p>
      </div>
    </div>

    <!-- Statistics -->
    <div class="flex gap-2">
      <div class="px-4 py-2 rounded-xl bg-iron-950/60 border border-border/10 text-center min-w-[80px]">
        <div class="text-[9px] text-text-secondary font-bold uppercase tracking-wider">Aktor Aktif</div>
        <div class="text-xs font-bold text-text-primary mt-0.5">{era.scaleStats.activeActors}</div>
      </div>
      <div class="px-4 py-2 rounded-xl bg-iron-950/60 border border-border/10 text-center min-w-[80px]">
        <div class="text-[9px] text-text-secondary font-bold uppercase tracking-wider">Aliansi Baru</div>
        <div class="text-xs font-bold text-text-primary mt-0.5">{era.scaleStats.newAlliances}</div>
      </div>
    </div>
  </div>

  <!-- Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Synchronic Analysis (Grid 1 & 2) -->
    <div class="lg:col-span-2 flex flex-col gap-6">
      <!-- Global Geopolitics Context -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-3">
        <h2 class="text-sm font-bold text-gold-400 flex items-center gap-2">
          <span>🌍</span> Konteks Geopolitik Dunia (Global Synchronicity)
        </h2>
        <p class="text-xs text-text-secondary leading-relaxed font-light">
          {era.globalContext}
        </p>
      </div>

      <!-- Sync Regional Timeline Comparison -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-5">
        <h2 class="text-sm font-bold text-gold-400 flex items-center gap-2">
          <span>🌐</span> Perbandingan Peristiwa Antar-Wilayah (Regional Sync)
        </h2>

        <div class="flex flex-col gap-4">
          {#each era.regionalEvents as re}
            <div class="p-4 rounded-xl bg-iron-950/60 border border-border/10 hover:border-gold-500/10 transition-all flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div class="flex-1">
                <span class="text-[10px] font-bold text-gold-400 bg-gold-500/10 px-2 py-0.5 rounded">
                  {re.region}
                </span>
                <h3 class="text-xs font-bold text-text-primary mt-2">{re.event}</h3>
                <p class="text-xs text-text-muted mt-1 leading-relaxed font-light">{re.desc}</p>
              </div>
              <span class="text-xs text-gold-500 hidden md:block">→</span>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Recommendations / Pivot Scale (Grid 3) -->
    <div class="flex flex-col gap-6">
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
        <h2 class="text-sm font-bold text-gold-400 flex items-center gap-2">
          <span>🧩</span> Hubungan Dengan Sumbu Utama
        </h2>
        <div class="text-xs text-text-secondary leading-relaxed font-light">
          Dimensi waktu sinkronis membuktikan bahwa lahirnya peradaban baru di Madinah tidak terisolasi dari krisis peradaban global, melainkan bertindak sebagai resolusi spiritual-sosial dari ketidakstabilan kekaisaran besar pada abad ke-7 M.
        </div>
      </div>
    </div>
  </div>
</div>
