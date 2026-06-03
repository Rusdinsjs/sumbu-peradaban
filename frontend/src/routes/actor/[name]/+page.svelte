<script lang="ts">
  import CurationBadge from '$lib/components/CurationBadge.svelte';

  let { data } = $props<{ data: { actor: any } }>();
  let actor = $derived(data.actor);
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Back link -->
  <a href="/actor" class="text-xs text-verdigris-400 hover:text-verdigris-300 transition-colors font-bold flex items-center gap-1.5 self-start">
    ← Kembali ke Direktori Tokoh
  </a>

  {#if !actor}
    <div class="glass p-12 rounded-3xl border border-border/10 text-center flex flex-col items-center justify-center gap-4">
      <span class="text-4xl">👤</span>
      <h2 class="text-lg font-bold text-text-primary">Data Tokoh Tidak Ditemukan</h2>
      <p class="text-xs text-text-muted">Entitas tokoh sejarah tidak terdaftar atau telah dihapus.</p>
      <a href="/actor" class="mt-2 px-5 py-2.5 bg-verdigris-500/10 hover:bg-verdigris-500/20 text-verdigris-400 text-xs font-bold rounded-xl border border-verdigris-500/20 transition-all">
        Kembali ke Direktori
      </a>
    </div>
  {:else}
    <!-- CV-Style Document Container -->
    <div class="glass p-8 lg:p-12 rounded-3xl border border-verdigris-500/10 flex flex-col gap-10 relative overflow-hidden">
      <!-- Ambient light effect -->
      <div class="absolute -top-24 -left-24 w-96 h-96 bg-verdigris-500/10 rounded-full blur-3xl pointer-events-none"></div>
      
      <!-- Header Section -->
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-8 relative z-10 border-b border-border/5 pb-8">
        <div class="flex items-start sm:items-center gap-6">
          <div class="w-32 h-32 sm:w-40 sm:h-40 rounded-full bg-iron-950 border-4 border-gold-500/20 flex items-center justify-center text-6xl shadow-[0_0_30px_rgba(212,168,83,0.15)] overflow-hidden relative group shrink-0">
            {#if actor.mediaLinks && actor.mediaLinks.some((m: any) => m.mediaType === 'image')}
              <img src={actor.mediaLinks.find((m: any) => m.mediaType === 'image').url} alt={actor.name} class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" />
            {:else}
              <span class="relative z-10 filter drop-shadow-md group-hover:scale-110 transition-transform duration-500">
                {actor.actorType === 'Group' || actor.actorType === 'Dynasty' ? '👥' : '👤'}
              </span>
            {/if}
          </div>
          
          <div class="flex flex-col gap-1.5">
            <div class="flex flex-wrap items-center gap-2">
              <span class="px-2.5 py-0.5 bg-verdigris-500/15 text-verdigris-400 border border-verdigris-500/20 text-[10px] font-extrabold uppercase rounded tracking-widest">
                {actor.actorType}
              </span>
              <CurationBadge tier={actor.curationTier} size="sm" />
            </div>
            
            <h1 class="text-2xl sm:text-4xl font-black text-text-primary leading-tight mt-1 tracking-tight">
              {actor.name}
            </h1>
            
            <p class="text-xs sm:text-sm text-text-secondary font-medium mt-1 flex items-center gap-2">
              <span>Wilayah Pengaruh:</span>
              <span class="text-verdigris-400 font-bold">{actor.culturalSphere || '-'}</span>
            </p>
          </div>
        </div>

        <!-- Life Span Minimalist Info -->
        <div class="flex flex-col items-start md:items-end gap-1">
          <span class="text-[9px] text-text-muted font-bold uppercase tracking-widest">Masa Hidup</span>
          <span class="text-lg sm:text-xl font-black text-verdigris-400 font-mono">
            {actor.birthYear !== null ? `${actor.birthYear} H` : '?'} — {actor.deathYear !== null ? `${actor.deathYear} H` : '?'}
          </span>
        </div>
      </div>

      <!-- Seamless Content Grid (CV Style) -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 relative z-10">
        
        <!-- Main Column: Biography, Timeline, Works -->
        <div class="lg:col-span-2 flex flex-col gap-12">
          
          <!-- Biography -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-verdigris-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">📜</span> Biografi Singkat
            </h2>
            <p class="text-[13px] text-text-secondary leading-loose whitespace-pre-wrap font-normal">
              {actor.description || 'Tidak ada uraian biografi tertulis untuk tokoh sejarah ini.'}
            </p>
          </div>

          <!-- Works & Roles -->
          <div class="flex flex-col gap-6">
            <h2 class="text-[11px] font-bold text-verdigris-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">💼</span> Portofolio & Kontribusi
            </h2>
            
            <div class="flex flex-col sm:flex-row gap-8">
              <!-- Roles -->
              <div class="flex-1 flex flex-col gap-3">
                <span class="text-[10px] text-text-muted font-bold uppercase tracking-widest">Jabatan / Gelar Utama</span>
                {#if actor.roles && actor.roles.length > 0}
                  <ul class="flex flex-col gap-2">
                    {#each actor.roles as role}
                      <li class="flex items-center gap-2 text-xs text-text-primary">
                        <span class="w-1.5 h-1.5 rounded-full bg-verdigris-500/50"></span>
                        {role}
                      </li>
                    {/each}
                  </ul>
                {:else}
                  <p class="text-xs text-text-muted italic">Belum ada catatan peran/jabatan.</p>
                {/if}
              </div>

              <!-- Works -->
              <div class="flex-1 flex flex-col gap-3">
                <span class="text-[10px] text-text-muted font-bold uppercase tracking-widest">Karya Tulis / Mahakarya</span>
                {#if actor.works && actor.works.length > 0}
                  <ul class="flex flex-col gap-2">
                    {#each actor.works as work}
                      <li class="flex items-center gap-2 text-xs text-gold-400 italic">
                        <span class="w-1.5 h-1.5 rounded-full bg-gold-500/50"></span>
                        {work}
                      </li>
                    {/each}
                  </ul>
                {:else}
                  <p class="text-xs text-text-muted italic">Belum ada karya terdaftar.</p>
                {/if}
              </div>
            </div>
          </div>

          <!-- Timeline -->
          <div class="flex flex-col gap-6 mt-4">
            <h2 class="text-[11px] font-bold text-verdigris-400 uppercase tracking-widest flex items-center gap-2">
              <span class="text-base">📅</span> Linimasa Keterlibatan Peristiwa
            </h2>

            {#if actor.timeline && actor.timeline.length > 0}
              <div class="relative border-l border-border/10 pl-6 ml-2 space-y-8 mt-2">
                {#each actor.timeline as ev}
                  <div class="relative group">
                    <span class="absolute -left-[29px] top-1.5 w-2.5 h-2.5 bg-verdigris-500 rounded-full shadow-[0_0_8px_rgba(16,185,129,0.6)] group-hover:scale-125 transition-transform"></span>
                    
                    <div class="flex flex-col gap-1">
                      <span class="text-[10px] font-bold text-verdigris-400 font-mono tracking-wider">
                        {ev.gregorianDate.year} M
                      </span>
                      <a href="/event/{ev.uuid}" class="text-sm font-bold text-text-primary hover:text-verdigris-400 transition-colors inline-block mt-0.5">
                        {ev.title}
                      </a>
                      {#if ev.description}
                        <p class="text-xs text-text-secondary mt-1 leading-relaxed">{ev.description}</p>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-xs text-text-muted italic">Tidak ada catatan peristiwa yang terhubung.</p>
            {/if}
          </div>

        </div>

        <!-- Sidebar Column: Network, Location, Source -->
        <div class="flex flex-col gap-10 border-t lg:border-t-0 lg:border-l border-border/5 pt-8 lg:pt-0 lg:pl-10">
          
          <!-- Network -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>🕸️</span> Koneksi & Jejaring
            </h2>
            {#if actor.relatedActors && actor.relatedActors.length > 0}
              <div class="flex flex-col gap-3">
                {#each actor.relatedActors as rel}
                  <a href="/actor/{rel.actor.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-verdigris-500/30 transition-colors">👤</div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-verdigris-400 transition-colors">{rel.actor.name}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada jejaring tercatat.</p>
            {/if}
          </div>

          <!-- Locations -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>🌍</span> Jejak Geografis
            </h2>
            {#if actor.visitedLocations && actor.visitedLocations.length > 0}
              <div class="flex flex-col gap-3">
                {#each actor.visitedLocations as rel}
                  <a href="/location/{rel.location.uuid}" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-verdigris-500/30 transition-colors">📍</div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-verdigris-400 transition-colors">{rel.location.name}</span>
                      <span class="text-[9px] text-text-muted uppercase tracking-widest">{rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada jejak lokasi.</p>
            {/if}
          </div>

          <!-- Sources -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>📚</span> Daftar Rujukan (Referensi)
            </h2>
            {#if actor.sources && actor.sources.length > 0}
              <div class="flex flex-col gap-4">
                {#each actor.sources as rel}
                  <a href="/source/{rel.source.sourceId}" class="group flex flex-col gap-1">
                    <span class="text-xs font-bold text-text-primary group-hover:text-verdigris-400 transition-colors leading-snug">{rel.source.title || 'Manuskrip Sejarah'}</span>
                    <div class="flex items-center gap-2">
                      <span class="text-[9px] text-text-muted">{rel.source.author || 'Anonim'}</span>
                      <span class="text-[9px] text-verdigris-400 uppercase tracking-widest">• {rel.relationshipType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada rujukan sumber.</p>
            {/if}
          </div>

          <!-- Media -->
          <div class="flex flex-col gap-4">
            <h2 class="text-[11px] font-bold text-text-muted uppercase tracking-widest flex items-center gap-2">
              <span>🎥</span> Lampiran Media
            </h2>
            {#if actor.mediaLinks && actor.mediaLinks.length > 0}
              <div class="flex flex-col gap-3">
                {#each actor.mediaLinks as media}
                  <a href={media.url} target="_blank" rel="noopener noreferrer" class="group flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-iron-950/60 border border-border/10 flex items-center justify-center text-xs group-hover:border-verdigris-500/30 transition-colors">
                      {media.mediaType === 'image' ? '🖼️' : media.mediaType === 'audio' ? '🎙️' : '📄'}
                    </div>
                    <div class="flex flex-col">
                      <span class="text-xs font-bold text-text-primary group-hover:text-verdigris-400 transition-colors truncate max-w-[150px]">{media.title || 'Lampiran'}</span>
                      <span class="text-[9px] text-text-muted uppercase tracking-widest">{media.mediaType}</span>
                    </div>
                  </a>
                {/each}
              </div>
            {:else}
              <p class="text-[11px] text-text-muted italic">Tidak ada media lampiran.</p>
            {/if}
          </div>

        </div>
      </div>
    </div>
  {/if}
</div>
