<script lang="ts">
  import { fade } from 'svelte/transition';
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: { source: any } }>();
  let src = $derived(data.source);

  // Helper to determine authenticity label, color, and shield icon
  type ReliabilityDetails = {
    label: string;
    tier: 'draft' | 'canonical' | 'reviewed' | 'verified';
    badgeClass: string;
    shield: string;
    desc: string;
  };

  function getReliabilityDetails(score: number | null): ReliabilityDetails {
    if (score === null) {
      return {
        label: 'Kredibilitas Belum Dinilai',
        tier: 'draft',
        badgeClass: 'border-yellow-500/30 text-yellow-400 bg-yellow-950/20',
        shield: '🛡️',
        desc: 'Kurator belum menyelesaikan audit kritik intern & ekstern untuk rujukan ini.'
      };
    }
    if (score >= 0.95) {
      return {
        label: 'Sangat Sahih / Autentik Tinggi',
        tier: 'canonical',
        badgeClass: 'border-amber-500/30 text-amber-400 bg-amber-950/20',
        shield: '🏆',
        desc: 'Sumber primer tingkat satu dengan tingkat kesaksian mata yang teruji secara ekstern dan intern.'
      };
    }
    if (score >= 0.85) {
      return {
        label: 'Sahih / Kredibilitas Tinggi',
        tier: 'reviewed',
        badgeClass: 'border-emerald-500/30 text-emerald-400 bg-emerald-950/20',
        shield: '🛡️',
        desc: 'Sumber historis kredibel dengan bias minimal dan didukung oleh konsensus historiografi utama.'
      };
    }
    if (score >= 0.70) {
      return {
        label: 'Dapat Diterima / Kredibilitas Cukup',
        tier: 'verified',
        badgeClass: 'border-blue-500/30 text-blue-400 bg-blue-950/20',
        shield: '⚙️',
        desc: 'Sumber sekunder atau kompilasi riwayat yang cukup valid, memerlukan komparasi teks pembanding.'
      };
    }
    return {
      label: 'Ditinjau Ulang / Perlu Kritik Intern',
      tier: 'draft',
      badgeClass: 'border-red-500/30 text-red-400 bg-red-950/20',
      shield: '⚠️',
      desc: 'Ditemukan anomali kronologi atau inkonsistensi sanad. Dianjurkan crosscheck dengan naskah pembanding.'
    };
  }

  let rel = $derived(src ? getReliabilityDetails(src.reliabilityScore) : null);
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-5xl mx-auto">
  <!-- Back Link -->
  <a href="/source" class="text-xs text-blue-400 hover:text-blue-300 transition-colors font-bold flex items-center gap-1.5 self-start">
    ← Kembali ke Direktori Rujukan
  </a>

  {#if !src}
    <div class="glass p-12 rounded-3xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-4xl">📚</span>
      <h2 class="text-lg font-bold text-text-primary">Data Rujukan Tidak Ditemukan</h2>
      <p class="text-xs text-text-muted">Rujukan dengan ID tersebut tidak terdaftar di sistem atau telah dihapus.</p>
      <a href="/source" class="mt-2 px-5 py-2.5 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 text-xs font-bold rounded-xl border border-blue-500/20 transition-all">
        Kembali ke Direktori
      </a>
    </div>
  {:else}
    <!-- Main Card Header -->
    <div class="glass p-8 rounded-3xl border border-blue-500/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-12 -left-12 w-48 h-48 bg-blue-500/10 rounded-full blur-3xl"></div>
      
      <div class="flex items-start sm:items-center gap-5 relative z-10">
        <div class="w-20 h-20 rounded-2xl bg-blue-500/10 border border-blue-500/25 flex items-center justify-center text-4xl shadow-[0_0_20px_rgba(59,130,246,0.15)] flex-shrink-0">
          {#if src.domain === 'Teks Suci'}
            📖
          {:else}
            📄
          {/if}
        </div>
        
        <div class="flex flex-col gap-1">
          <div class="flex flex-wrap items-center gap-2">
            <span class="px-2 py-0.5 bg-blue-500/15 text-blue-400 border border-blue-500/20 text-[9px] font-extrabold uppercase rounded tracking-wider">
              {src.domain}
            </span>
            {#if rel}
              <CurationBadge tier={rel.tier} size="sm" />
            {/if}
          </div>
          
          <h1 class="text-xl sm:text-2xl font-black text-text-primary leading-tight mt-1">
            {src.title || 'Manuskrip Sejarah'}
          </h1>
          
          <div class="text-xs text-text-secondary font-medium flex flex-wrap items-center gap-x-2 gap-y-1 mt-0.5">
            <span>Ditulis oleh:</span> 
            <span class="text-blue-400 font-bold">{src.author || 'Penyusun Anonim'}</span>
            <span class="text-text-muted">•</span>
            <span class="text-text-muted italic">{src.publicationEra || 'Era Sejarah'}</span>
          </div>
        </div>
      </div>

      <!-- Kredibilitas Shield Badge -->
      <div class="px-5 py-3 rounded-2xl bg-navy-950/60 border border-border/10 flex flex-row md:flex-col items-center gap-3 md:gap-1 text-center min-w-[130px] self-stretch md:self-auto relative z-10 justify-between sm:justify-center">
        <span class="text-2xl">{rel?.shield}</span>
        <div class="flex flex-col md:items-center">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-wider">Kredibilitas</span>
          <span class="text-base font-black text-blue-400">
            {src.reliabilityScore !== null ? `${(src.reliabilityScore * 100).toFixed(0)}%` : 'TBD'}
          </span>
        </div>
      </div>
    </div>

    <!-- Details Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Column 1 & 2: Primary Text and Assessment -->
      <div class="lg:col-span-2 flex flex-col gap-6">
        
        <!-- Kutipan Primer -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-4 relative">
          <span class="absolute top-6 right-8 text-6xl text-blue-500/5 font-serif select-none pointer-events-none">“</span>
          
          <h2 class="text-xs font-bold text-blue-400 uppercase tracking-wider flex items-center gap-2">
            <span>📜</span> Kutipan Naskah Primer (Reference Text)
          </h2>
          
          <div class="p-6 bg-navy-950/50 border border-border/5 rounded-2xl font-serif text-sm sm:text-base text-text-primary italic leading-relaxed whitespace-pre-wrap">
            "{src.referenceText}"
          </div>

          {#if src.interpretationMethod}
            <div class="flex items-center gap-2 mt-2">
              <span class="text-[10px] text-text-muted">Metodologi Interpretasi:</span>
              <span class="px-2 py-0.5 bg-navy-900 border border-border/10 rounded text-[10px] text-text-secondary font-mono">
                {src.interpretationMethod}
              </span>
            </div>
          {/if}
        </div>

        <!-- Analisis Kesahihan Akademis -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-blue-400 uppercase tracking-wider flex items-center gap-2">
            <span>🔬</span> Analisis Kritik Intern & Ekstern
          </h2>

          {#if src.reliabilityAssessment}
            <p class="text-xs text-text-secondary leading-relaxed font-normal whitespace-pre-wrap">
              {src.reliabilityAssessment}
            </p>
          {:else}
            <p class="text-xs text-text-muted italic leading-relaxed">
              Belum ada deskripsi analisis kritik akademis yang diinput untuk rujukan ini.
            </p>
          {/if}
        </div>
      </div>

      <!-- Column 3: Authenticity Card and Media Scans -->
      <div class="flex flex-col gap-6">
        
        <!-- Scorecard Visual Badge -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-blue-400 uppercase tracking-wider">
            Authenticity Scorecard
          </h2>

          {#if rel}
            <div class="flex flex-col gap-3">
              <div class="p-4 rounded-2xl border {rel.badgeClass} flex flex-col gap-1.5">
                <span class="text-[10px] font-bold uppercase tracking-wider opacity-95">Tingkat Kesahihan</span>
                <span class="text-xs font-extrabold">{rel.label}</span>
              </div>
              <p class="text-[11px] text-text-secondary leading-relaxed">
                {rel.desc}
              </p>
            </div>
          {/if}

          {#if src.reliabilityScore !== null}
            <div class="flex flex-col gap-1.5 mt-2 border-t border-border/5 pt-4">
              <div class="flex justify-between text-[10px] font-bold text-text-secondary">
                <span>Nilai Validitas Kontekstual</span>
                <span>{(src.reliabilityScore * 100).toFixed(0)}%</span>
              </div>
              <div class="h-2 w-full bg-navy-950 rounded-full overflow-hidden border border-border/5">
                <div class="h-full bg-blue-500 rounded-full" style="width: {src.reliabilityScore * 100}%"></div>
              </div>
            </div>
          {/if}
        </div>

        <!-- Berkas Pendukung Scan Media -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-blue-400 uppercase tracking-wider flex items-center gap-2">
            <span>🎥</span> Berkas Pendukung ({src.mediaLinks ? src.mediaLinks.length : 0})
          </h2>

          {#if src.mediaLinks && src.mediaLinks.length > 0}
            <div class="flex flex-col gap-2">
              {#each src.mediaLinks as media}
                <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-blue-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-lg">
                      {#if media.mediaType === 'image'}
                        🖼️
                      {:else}
                        📄
                      {/if}
                    </span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="text-[11px] font-bold text-text-primary group-hover:text-blue-400 transition-colors truncate">
                        {media.title || 'Unduh Berkas'}
                      </span>
                      <span class="text-[9px] text-text-muted uppercase tracking-wider mt-0.5">
                        {media.mediaType}
                      </span>
                    </div>
                  </div>
                  <span class="text-[10px] text-blue-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">
              Tidak ada scan manuskrip kuno atau rekaman media pendukung yang dilampirkan.
            </p>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
