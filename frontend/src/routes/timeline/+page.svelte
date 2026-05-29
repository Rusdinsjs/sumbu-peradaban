<script lang="ts">
  import Timeline from '$lib/components/Timeline.svelte';
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in">
  <!-- Page Header -->
  <div class="glass p-6 rounded-2xl border border-border/10">
    <h1 class="text-xl font-extrabold text-gold-400">Garis Waktu Sejarah Dunia</h1>
    <p class="text-xs text-text-secondary leading-relaxed mt-1">
      Navigasi kronologis dua kalender (Hijriah & Gregorian) yang memperlihatkan integrasi peristiwa sejarah Islam sebagai jangkar narasi peradaban.
    </p>
  </div>

  <!-- Eras Quick Navigation -->
  <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-gold-400 hover:border-gold-500/30 transition-all text-center">
      Masa Nubuwwah
    </button>
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-text-secondary hover:border-gold-500/30 transition-all text-center">
      Khulafaur Rasyidin
    </button>
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-text-secondary hover:border-gold-500/30 transition-all text-center">
      Daulah Umayyah
    </button>
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-text-secondary hover:border-gold-500/30 transition-all text-center">
      Daulah Abbasiyah
    </button>
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-text-secondary hover:border-gold-500/30 transition-all text-center">
      Daulah Utsmaniyyah
    </button>
    <button class="glass p-3 rounded-xl border border-border/10 text-xs font-bold text-text-secondary hover:border-gold-500/30 transition-all text-center">
      Peradaban Modern
    </button>
  </div>

  <!-- Main Timeline component -->
  <div class="glass p-6 rounded-2xl border border-border/10 min-h-[400px]">
    <Timeline />
  </div>
</div>
