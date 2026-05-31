<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: { actor: any } }>();
  let actor = $derived(data.actor);
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12 p-8 max-w-5xl mx-auto">
  <!-- Back link -->
  <a href="/actor" class="text-xs text-emerald-400 hover:text-emerald-300 transition-colors font-bold flex items-center gap-1.5 self-start">
    ← Kembali ke Direktori Tokoh
  </a>

  {#if !actor}
    <div class="glass p-12 rounded-3xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-4xl">👤</span>
      <h2 class="text-lg font-bold text-text-primary">Data Tokoh Tidak Ditemukan</h2>
      <p class="text-xs text-text-muted">Entitas tokoh sejarah tidak terdaftar atau telah dihapus.</p>
      <a href="/actor" class="mt-2 px-5 py-2.5 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 text-xs font-bold rounded-xl border border-emerald-500/20 transition-all">
        Kembali ke Direktori
      </a>
    </div>
  {:else}
    <!-- Main Card Header -->
    <div class="glass p-8 rounded-3xl border border-emerald-500/10 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-12 -left-12 w-48 h-48 bg-emerald-500/10 rounded-full blur-3xl"></div>
      
      <div class="flex items-start sm:items-center gap-5 relative z-10">
        <div class="w-20 h-20 rounded-2xl bg-emerald-500/10 border border-emerald-500/25 flex items-center justify-center text-4xl shadow-[0_0_20px_rgba(16,185,129,0.15)] flex-shrink-0">
          👤
        </div>
        
        <div class="flex flex-col gap-1">
          <div class="flex flex-wrap items-center gap-2">
            <span class="px-2 py-0.5 bg-emerald-500/15 text-emerald-400 border border-emerald-500/20 text-[9px] font-extrabold uppercase rounded tracking-wider">
              {actor.actorType}
            </span>
            <CurationBadge tier={actor.curationTier} size="sm" />
          </div>
          
          <h1 class="text-xl sm:text-2xl font-black text-text-primary leading-tight mt-1">
            {actor.name}
          </h1>
          
          <div class="text-xs text-text-secondary font-medium flex flex-wrap items-center gap-x-2 gap-y-1 mt-0.5">
            <span>Afiliasi Kebudayaan:</span> 
            <span class="text-emerald-400 font-bold">{actor.culturalSphere}</span>
          </div>
        </div>
      </div>

      <!-- Life Span Badge -->
      <div class="px-5 py-3 rounded-2xl bg-navy-950/60 border border-border/10 flex flex-row md:flex-col items-center gap-3 md:gap-1 text-center min-w-[130px] self-stretch md:self-auto relative z-10 justify-between sm:justify-center">
        <span class="text-2xl">⏳</span>
        <div class="flex flex-col md:items-center">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-wider">Masa Hidup</span>
          <span class="text-xs font-black text-emerald-400 font-mono">
            {actor.birthYear !== null ? `${actor.birthYear} H` : '?'} - {actor.deathYear !== null ? `${actor.deathYear} H` : '?'}
          </span>
        </div>
      </div>
    </div>

    <!-- Details Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Column 1 & 2: Biography, Timeline, and Media -->
      <div class="lg:col-span-2 flex flex-col gap-6">
        
        <!-- Biography Description -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>📜</span> Biografi & Deskripsi Tokoh
          </h2>
          <p class="text-xs text-text-secondary leading-relaxed whitespace-pre-wrap font-normal">
            {actor.description || 'Tidak ada uraian biografi tertulis untuk tokoh sejarah ini.'}
          </p>
        </div>

        <!-- Works and Contributions -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-5">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider">
            💼 Peran & Karya Intelektual (Mahakarya)
          </h2>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
            <!-- Roles -->
            <div class="flex flex-col gap-2.5">
              <span class="text-[10px] text-text-muted font-bold uppercase tracking-wider">Peran Utama</span>
              {#if actor.roles && actor.roles.length > 0}
                <div class="flex flex-wrap gap-1.5">
                  {#each actor.roles as role}
                    <span class="px-2.5 py-1 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-300 text-xs">{role}</span>
                  {/each}
                </div>
              {:else}
                <p class="text-xs text-text-muted italic">Belum ada peran terdaftar untuk tokoh ini.</p>
              {/if}
            </div>

            <!-- Works -->
            <div class="flex flex-col gap-2.5">
              <span class="text-[10px] text-text-muted font-bold uppercase tracking-wider">Karya Kitab / Tulisan (Mahakarya)</span>
              {#if actor.works && actor.works.length > 0}
                <div class="flex flex-wrap gap-1.5">
                  {#each actor.works as work}
                    <span class="px-2.5 py-1 rounded-xl bg-gold-500/10 border border-gold-500/20 text-gold-300 text-xs italic">📖 {work}</span>
                  {/each}
                </div>
              {:else}
                <p class="text-xs text-text-muted italic">Belum ada mahakarya/kitab terdaftar untuk tokoh ini.</p>
              {/if}
            </div>
          </div>
        </div>

        <!-- Timeline of Events -->
        <div class="glass p-8 rounded-3xl border border-border/10 flex flex-col gap-6">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>📅</span> Linimasa Keterlibatan Peristiwa Sejarah
          </h2>

          {#if actor.timeline && actor.timeline.length > 0}
            <div class="relative border-l border-border/10 pl-6 ml-3 space-y-6">
              {#each actor.timeline as ev}
                <div class="relative group">
                  <!-- Node Dot -->
                  <span class="absolute -left-[31px] top-1.5 w-3 h-3 bg-emerald-500 rounded-full border-2 border-surface shadow-[0_0_6px_rgba(16,185,129,0.8)] group-hover:scale-110 transition-transform"></span>
                  
                  <div>
                    <span class="text-[9px] font-bold text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded font-mono">
                      {ev.gregorianDate.year} M
                    </span>
                    <a href="/event/{ev.uuid}" class="block text-xs font-bold text-text-primary hover:text-emerald-400 mt-2 transition-colors">
                      {ev.title}
                    </a>
                    {#if ev.description}
                      <p class="text-[11px] text-text-muted mt-1 leading-relaxed line-clamp-2">{ev.description}</p>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-xs text-text-muted italic">Tidak ada catatan peristiwa sejarah yang terhubung langsung dengan tokoh ini.</p>
          {/if}
        </div>
      </div>

      <!-- Column 3: Network, Locations, and Sources -->
      <div class="flex flex-col gap-6">
        
        <!-- Companions Network -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>🕸️</span> Jejaring Tokoh Terkait
          </h2>

          {#if actor.relatedActors && actor.relatedActors.length > 0}
            <div class="flex flex-col gap-2">
              {#each actor.relatedActors as rel}
                <a href="/actor/{rel.actor.uuid}" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-emerald-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-base">👤</span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="text-[11px] font-bold text-text-primary group-hover:text-emerald-400 transition-colors truncate">
                        {rel.actor.name}
                      </span>
                      <span class="text-[9px] text-emerald-400 font-bold uppercase tracking-wider mt-0.5">
                        {rel.relationshipType}
                      </span>
                    </div>
                  </div>
                  <span class="text-[10px] text-emerald-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Belum ada jejaring tokoh terhubung.</p>
          {/if}
        </div>

        <!-- Visited Locations -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>🌍</span> Tempat yang Pernah Disinggahi
          </h2>

          {#if actor.visitedLocations && actor.visitedLocations.length > 0}
            <div class="flex flex-col gap-2">
              {#each actor.visitedLocations as loc}
                <a href="/location/{loc.uuid}" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-emerald-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-base">📍</span>
                    <span class="text-[11px] font-bold text-text-primary group-hover:text-emerald-400 transition-colors truncate">
                      {loc.name}
                    </span>
                  </div>
                  <span class="text-[10px] text-emerald-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada catatan riwayat singgah di lokasi sejarah.</p>
          {/if}
        </div>

        <!-- Sources validation -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>📚</span> Sumber & Validasi Rujukan
          </h2>

          {#if actor.sources && actor.sources.length > 0}
            <div class="flex flex-col gap-3">
              {#each actor.sources as src}
                <a href="/source/{src.sourceId}" class="group block p-3.5 bg-navy-950/60 hover:bg-navy-900 border border-border/5 rounded-2xl transition-all">
                  <div class="flex justify-between items-start gap-2">
                    <span class="text-[11px] font-bold text-text-primary group-hover:text-emerald-400 transition-colors line-clamp-1">{src.title || 'Manuskrip Sejarah'}</span>
                    {#if src.reliabilityScore !== null}
                      <span class="text-[9px] font-bold text-emerald-400 font-mono">{(src.reliabilityScore * 100).toFixed(0)}%</span>
                    {/if}
                  </div>
                  <span class="text-[9px] text-text-muted block mt-1">{src.author || 'Penyusun Anonim'}</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Belum ada rujukan kitab yang terhubung langsung dengan riwayat tokoh ini.</p>
          {/if}
        </div>

        <!-- Media Scans -->
        <div class="glass p-6 rounded-3xl border border-border/10 flex flex-col gap-4">
          <h2 class="text-xs font-bold text-emerald-400 uppercase tracking-wider flex items-center gap-2">
            <span>🎥</span> Berkas Media Pendukung
          </h2>

          {#if actor.mediaLinks && actor.mediaLinks.length > 0}
            <div class="flex flex-col gap-2">
              {#each actor.mediaLinks as media}
                <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center justify-between p-3 bg-navy-950/60 hover:bg-navy-900 border border-border/5 hover:border-emerald-500/20 rounded-2xl transition-all gap-3">
                  <div class="flex items-center gap-2.5 overflow-hidden">
                    <span class="text-lg">
                      {#if media.mediaType === 'image'}
                        🖼️
                      {:else if media.mediaType === 'audio'}
                        🎙️
                      {:else}
                        📄
                      {/if}
                    </span>
                    <div class="flex flex-col overflow-hidden">
                      <span class="text-[11px] font-bold text-text-primary group-hover:text-emerald-400 transition-colors truncate">
                        {media.title || 'Lihat Berkas'}
                      </span>
                      <span class="text-[9px] text-text-muted uppercase tracking-wider mt-0.5">
                        {media.mediaType}
                      </span>
                    </div>
                  </div>
                  <span class="text-[10px] text-emerald-400 opacity-0 group-hover:opacity-100 transition-opacity">↗</span>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-[11px] text-text-muted italic">Tidak ada berkas media gambar/audio pendukung yang dilampirkan.</p>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
