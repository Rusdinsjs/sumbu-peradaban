<script lang="ts">
  import CurationBadge from './CurationBadge.svelte';

  interface TimelineEvent {
    title: string;
    hijriYear: string;
    gregorianYear: string;
    description: string;
    tier: 'draft' | 'verified' | 'reviewed' | 'canonical';
  }

  let { 
    events = [] 
  } = $props<{
    events?: TimelineEvent[];
  }>();

  // Islamic History Timeline Events
  const defaultEvents: TimelineEvent[] = [
    { title: 'Tahun Kesedihan (\'Am al-Huzn)', hijriYear: '-3 H', gregorianYear: '619 M', tier: 'canonical', description: 'Wafatnya Sayyidah Khadijah & Abu Thalib.' },
    { title: 'Peristiwa Hijrah ke Madinah', hijriYear: '1 H', gregorianYear: '622 M', tier: 'canonical', description: 'Titik awal kalender Hijriah dan berdirinya Daulah Madinah.' },
    { title: 'Perang Badar Al-Kubra', hijriYear: '2 H', gregorianYear: '624 M', tier: 'canonical', description: 'Kemenangan besar pertama umat Islam melawan kaum Quraisy Makkah.' },
    { title: 'Fathu Makkah (Pembebasan)', hijriYear: '8 H', gregorianYear: '630 M', tier: 'canonical', description: 'Rasulullah ﷺ kembali ke Makkah dan membersihkan Ka\'bah dari berhala.' },
    { title: 'Wafatnya Nabi Muhammad ﷺ', hijriYear: '11 H', gregorianYear: '632 M', tier: 'canonical', description: 'Wafatnya Rasulullah dan dimulainya era Khulafaur Rasyidin.' },
    { title: 'Baitul Maqdis Diserahkan', hijriYear: '16 H', gregorianYear: '637 M', tier: 'reviewed', description: 'Khalifah Umar bin Khattab menerima kunci Al-Quds secara damai.' },
    { title: 'Daulah Umayyah Berdiri', hijriYear: '41 H', gregorianYear: '661 M', tier: 'reviewed', description: 'Muawiyah bin Abi Sufyan mendirikan dinasti Islam pertama di Damaskus.' },
  ];

  const activeEvents = $derived(events.length > 0 ? events : defaultEvents);
  let scrollContainer: HTMLDivElement;

  function scroll(direction: 'left' | 'right') {
    if (scrollContainer) {
      const scrollAmount = 300;
      scrollContainer.scrollBy({
        left: direction === 'left' ? -scrollAmount : scrollAmount,
        behavior: 'smooth'
      });
    }
  }
</script>

<div class="relative w-full py-6 flex flex-col">
  <!-- Scroll controls -->
  <div class="flex justify-between items-center mb-4">
    <div class="text-sm font-bold text-gold-500 flex items-center gap-2">
      <span>📅 Kalender Ganda:</span>
      <span class="text-xs text-text-secondary font-normal">(Hijriah ↔ Gregorian)</span>
    </div>
    <div class="flex gap-2">
      <button 
        class="w-8 h-8 rounded-lg glass border border-border/10 hover:border-gold-500/20 text-xs flex items-center justify-center transition-all"
        onclick={() => scroll('left')}
      >
        ←
      </button>
      <button 
        class="w-8 h-8 rounded-lg glass border border-border/10 hover:border-gold-500/20 text-xs flex items-center justify-center transition-all"
        onclick={() => scroll('right')}
      >
        →
      </button>
    </div>
  </div>

  <!-- Timeline scroll wrapper -->
  <div 
    bind:this={scrollContainer}
    class="w-full overflow-x-auto flex gap-6 pb-6 relative scrollbar-none snap-x snap-mandatory"
  >
    <!-- Axis lines -->
    <div class="absolute top-[28px] left-0 right-0 h-0.5 bg-gradient-to-r from-gold-500/10 via-gold-500/40 to-gold-500/10 pointer-events-none"></div>
    <div class="absolute top-[52px] left-0 right-0 h-0.5 bg-gradient-to-r from-navy-500/10 via-navy-500/40 to-navy-500/10 pointer-events-none"></div>

    {#each activeEvents as event}
      <div class="flex-shrink-0 w-80 snap-start flex flex-col relative group">
        <!-- Date Markers -->
        <div class="flex flex-col items-center justify-center mb-6 relative">
          <!-- Hijri Year (Top Axis) -->
          <span class="text-xs font-extrabold text-gold-400 bg-surface px-2 py-0.5 rounded border border-gold-500/20 z-10">
            {event.hijriYear}
          </span>
          
          <!-- Node Dot -->
          <span class="w-3 h-3 rounded-full bg-gold-500 border-2 border-surface my-1 group-hover:scale-125 transition-transform shadow-[0_0_8px_#d4a853]"></span>

          <!-- Gregorian Year (Bottom Axis) -->
          <span class="text-[10px] font-bold text-text-secondary bg-surface px-2 py-0.5 rounded border border-border/10 z-10">
            {event.gregorianYear}
          </span>
        </div>

        <!-- Event Detail Card -->
        <div class="glass p-5 rounded-xl border border-border/10 hover:border-gold-500/30 transition-all duration-300 flex-1 flex flex-col justify-between group-hover:translate-y-[-4px] shadow-lg">
          <div>
            <div class="flex justify-between items-start gap-2 mb-2">
              <h3 class="text-sm font-extrabold text-text-primary group-hover:text-gold-400 transition-colors leading-snug">
                {event.title}
              </h3>
              <CurationBadge tier={event.tier} size="sm" />
            </div>
            <p class="text-xs text-text-secondary leading-relaxed">
              {event.description}
            </p>
          </div>
          <div class="mt-4 border-t border-border/10 pt-3 flex justify-between items-center">
            <span class="text-[10px] text-text-muted">Pivot: Dunia Islam</span>
            <button class="text-[10px] text-gold-500 hover:text-gold-400 transition-colors font-bold">
              Detail Peristiwa →
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  /* Hide scrollbar for Chrome, Safari and Opera */
  .scrollbar-none::-webkit-scrollbar {
    display: none;
  }
  /* Hide scrollbar for IE, Edge and Firefox */
  .scrollbar-none {
    -ms-overflow-style: none;  /* IE and Edge */
    scrollbar-width: none;  /* Firefox */
  }
</style>
