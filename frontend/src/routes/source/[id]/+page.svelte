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
        badgeClass: 'border-verdigris-500/30 text-verdigris-400 bg-verdigris-950/20',
        shield: '🛡️',
        desc: 'Sumber historis kredibel dengan bias minimal dan didukung oleh konsensus historiografi utama.'
      };
    }
    if (score >= 0.70) {
      return {
        label: 'Dapat Diterima / Kredibilitas Cukup',
        tier: 'verified',
        badgeClass: 'border-gold-500/30 text-gold-400 bg-gold-950/20',
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

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back Link -->
  <a href="/source" class="text-xs text-gold-400 hover:text-gold-300 transition-colors font-bold flex items-center gap-1.5 self-start">
    ← Kembali ke Direktori Rujukan
  </a>

  {#if !src}
    <div class="glass p-12 rounded-3xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-4xl">📚</span>
      <h2 class="text-lg font-bold text-text-primary">Data Rujukan Tidak Ditemukan</h2>
      <p class="text-xs text-text-muted">Rujukan dengan ID tersebut tidak terdaftar di sistem atau telah dihapus.</p>
      <a href="/source" class="mt-2 px-5 py-2.5 bg-gold-500/10 hover:bg-gold-500/20 text-gold-400 text-xs font-bold rounded-xl border border-gold-500/20 transition-all">
        Kembali ke Direktori
      </a>
    </div>
  {:else}
    <!-- Document-Style Container -->
    <div class="glass p-8 lg:p-12 rounded-3xl border border-gold-500/10 flex flex-col gap-10 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-24 -left-24 w-96 h-96 bg-gold-500/10 rounded-full blur-3xl pointer-events-none"></div>
      
      <!-- Header Section -->
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-8 relative z-10 border-b border-border/5 pb-8">
        <div class="flex items-start sm:items-center gap-6">
          <!-- Book Cover Representation -->
          {#if src.mediaLinks && src.mediaLinks.some((m: any) => m.mediaType === 'image')}
            <div class="w-24 h-32 rounded-r-xl rounded-l-sm overflow-hidden border-r border-gold-500/20 shadow-[0_0_30px_rgba(59,130,246,0.15)] flex-shrink-0 relative">
              <div class="absolute left-0 top-0 bottom-0 w-2 bg-black/40 z-10"></div>
              <div class="absolute left-2 top-0 bottom-0 w-[1px] bg-white/20 z-10"></div>
              <img src={src.mediaLinks.find((m: any) => m.mediaType === 'image').url} alt={src.title} class="w-full h-full object-cover" />
            </div>
          {:else}
            <div class="w-24 h-32 rounded-r-xl rounded-l-sm bg-gradient-to-br from-gold-900/40 to-iron-950 border-r border-gold-500/20 flex flex-col items-center justify-center relative shadow-[0_0_30px_rgba(59,130,246,0.15)] flex-shrink-0">
              <div class="absolute left-0 top-0 bottom-0 w-2 bg-black/40"></div>
              <div class="absolute left-2 top-0 bottom-0 w-[1px] bg-white/10"></div>
              <span class="text-5xl relative z-10 filter drop-shadow-md">
                {#if src.domain === 'Teks Suci'}
                  📖
                {:else}
                  📄
                {/if}
              </span>
            </div>
          {/if}
          
          <div class="flex flex-col gap-1.5">
            <div class="flex flex-wrap items-center gap-2">
              <span class="px-2.5 py-0.5 bg-gold-500/15 text-gold-400 border border-gold-500/20 text-[10px] font-extrabold uppercase rounded tracking-widest">
                {src.domain}
              </span>
              {#if rel}
                <CurationBadge tier={rel.tier} size="sm" />
              {/if}
            </div>
            
            <h1 class="text-2xl sm:text-4xl font-black text-text-primary leading-tight mt-1 tracking-tight">
              {src.title || 'Manuskrip Sejarah'}
            </h1>
            
            <div class="text-xs sm:text-sm text-text-secondary font-medium flex flex-wrap items-center gap-x-2 gap-y-1 mt-1">
              <span>Ditulis oleh:</span> 
              <span class="text-gold-400 font-bold">{src.author || 'Penyusun Anonim'}</span>
              <span class="text-text-muted">•</span>
              <span class="text-text-muted italic">{src.publicationEra || 'Era Sejarah'}</span>
            </div>
          </div>
        </div>

        <!-- Kredibilitas Minimalist Info -->
        <div class="flex flex-col items-start md:items-end gap-1">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-widest">Kredibilitas</span>
          <span class="text-lg sm:text-2xl font-black text-gold-400 font-mono flex items-center gap-2">
            <span>{rel?.shield}</span>
            {src.reliabilityScore !== null ? `${(src.reliabilityScore * 100).toFixed(0)}%` : 'TBD'}
          </span>
        </div>
      </div>

      <!-- Seamless Content Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 relative z-10">
        
        <!-- Main Column: Primary Text and Assessment -->
        <div class="lg:col-span-2 flex flex-col gap-12">
          
          <!-- Kutipan Primer -->
          <div class="flex flex-col gap-4 relative">
            <span class="absolute -top-4 -left-4 text-7xl text-gold-500/10 font-serif select-none pointer-events-none">“</span>
            
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">📜</span> Kutipan Naskah Primer
            </h2>
            
            <div class="font-serif text-sm sm:text-[15px] text-text-primary italic leading-loose whitespace-pre-wrap pl-4 border-l-2 border-gold-500/20">
              "{src.referenceText}"
            </div>

            {#if src.interpretationMethod}
              <div class="flex items-center gap-2 mt-2">
                <span class="text-[10px] text-text-muted uppercase tracking-widest">Metodologi Interpretasi:</span>
                <span class="text-[11px] text-text-secondary font-mono">
                  {src.interpretationMethod}
                </span>
              </div>
            {/if}
          </div>

          <!-- Analisis Kesahihan -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">🔬</span> Analisis Kritik Intern & Ekstern
            </h2>

            {#if src.reliabilityAssessment}
              <p class="text-[13px] text-text-secondary leading-loose font-normal whitespace-pre-wrap">
                {src.reliabilityAssessment}
              </p>
            {:else}
              <p class="text-xs text-text-muted italic leading-relaxed">
                Belum ada deskripsi analisis kritik akademis.
              </p>
            {/if}
          </div>
          
        </div>

        <!-- Sidebar Column: Scorecard, Media, Connections -->
        <div class="flex flex-col gap-10 border-t lg:border-t-0 lg:border-l border-border/5 pt-8 lg:pt-0 lg:pl-10">
          
          <!-- Scorecard -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest">
              Authenticity Scorecard
            </h2>

            {#if rel}
              <div class="flex flex-col gap-3">
                <div class="flex flex-col gap-0.5">
                  <span class="text-[10px] font-bold uppercase tracking-widest text-text-muted">Tingkat Kesahihan</span>
                  <span class="text-xs font-extrabold {rel.badgeClass.split(' ')[1]}">{rel.label}</span>
                </div>
                <p class="text-[11px] text-text-secondary leading-relaxed">
                  {rel.desc}
                </p>
              </div>
            {/if}

            {#if src.reliabilityScore !== null}
              <div class="flex flex-col gap-2 mt-2 pt-4 border-t border-border/5">
                <div class="flex justify-between text-[10px] font-bold text-text-secondary uppercase tracking-widest">
                  <span>Skor Validitas</span>
                  <span class="text-gold-400 font-mono">{(src.reliabilityScore * 100).toFixed(0)}%</span>
                </div>
                <div class="h-1.5 w-full bg-iron-950 rounded-full overflow-hidden">
                  <div class="h-full bg-gold-500 rounded-full" style="width: {src.reliabilityScore * 100}%"></div>
                </div>
              </div>
            {/if}
          </div>

          <!-- Media Scans -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>🎥</span> Lampiran Media Berkas
            </h2>

            {#if src.mediaLinks && src.mediaLinks.length > 0}
              <div class="flex flex-col gap-3">
                {#each src.mediaLinks as media}
                  <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors">
                      {media.mediaType === 'image' ? '🖼️' : '📄'}
                    </div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-gold-400 transition-colors truncate max-w-[150px]">{media.title || 'Unduh Berkas'}</span>
                      <span class="text-[9px] text-text-muted uppercase tracking-widest">{media.mediaType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada scan manuskrip dilampirkan.</p>
            {/if}
          </div>

        </div>
      </div>
      
      <!-- Dimension Connections Section -->
      <div class="border-t border-border/5 pt-10 mt-2 flex flex-col gap-8">
        <h2 class="text-[11px] font-bold text-gold-400 uppercase tracking-widest flex items-center gap-2">
          <span class="text-base">🌐</span> Peta Hubungan Multi-Dimensi Sejarah
        </h2>
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
          
          <!-- a. Tokoh -->
          <div class="flex flex-col gap-4">
            <h3 class="text-[10px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>👤</span> Tokoh Sejarah Terkait
            </h3>
            {#if src.actors && src.actors.length > 0}
              <div class="flex flex-col gap-3">
                {#each src.actors as rel}
                  <a href="/actor/{rel.actor.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors">👤</div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-gold-400 transition-colors">{rel.actor.name}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada tokoh yang dicatat.</p>
            {/if}
          </div>

          <!-- b. Lokasi -->
          <div class="flex flex-col gap-4">
            <h3 class="text-[10px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>📍</span> Tempat Geografis
            </h3>
            {#if src.locations && src.locations.length > 0}
              <div class="flex flex-col gap-3">
                {#each src.locations as rel}
                  <a href="/location/{rel.location.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors">📍</div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-gold-400 transition-colors">{rel.location.name}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada lokasi disebut.</p>
            {/if}
          </div>

          <!-- c. Peristiwa -->
          <div class="flex flex-col gap-4">
            <h3 class="text-[10px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>📅</span> Peristiwa Diulas
            </h3>
            {#if src.events && src.events.length > 0}
              <div class="flex flex-col gap-3">
                {#each src.events as ev}
                  <a href="/event/{ev.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-gold-500/30 transition-colors font-mono tracking-widest text-gold-400">
                      {ev.gregorianDate?.year ? String(ev.gregorianDate.year).slice(-2) : 'Ev'}
                    </div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-gold-400 transition-colors line-clamp-1">{ev.title}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest font-mono">{ev.gregorianDate?.year ? `${ev.gregorianDate.year} M` : 'Kronologi Sejarah'}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada peristiwa yang diulas.</p>
            {/if}
          </div>

        </div>
      </div>
    </div>
  {/if}
</div>
