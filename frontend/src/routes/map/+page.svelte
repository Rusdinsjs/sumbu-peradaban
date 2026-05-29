<script lang="ts">
  import MapView from '$lib/components/MapView.svelte';
</script>

<div class="w-full h-[calc(100vh-80px)] flex gap-6 animate-fade-in">
  <!-- Left Panel: Historical Locations list -->
  <div class="w-80 flex flex-col gap-5 flex-shrink-0 h-full">
    <div class="glass p-5 rounded-2xl border border-border/10 flex flex-col gap-4">
      <h2 class="text-base font-extrabold text-gold-400">Peta Geografis Peradaban</h2>
      <p class="text-xs text-text-secondary leading-relaxed">
        Visualisasi 3 Dimensi geospasial lokasi-lokasi penting sejarah Islam dan keterhubungannya dengan konseptual transendental.
      </p>
    </div>

    <!-- Location list -->
    <div class="glass p-5 rounded-2xl border border-border/10 flex-1 overflow-y-auto flex flex-col gap-3">
      <h3 class="text-xs font-bold text-text-secondary uppercase tracking-wider">Lokasi Utama</h3>
      
      <div class="flex flex-col gap-2">
        <div class="p-3 rounded-xl bg-navy-950/60 border border-border/10 hover:border-gold-500/20 transition-all cursor-pointer">
          <h4 class="text-xs font-bold text-text-primary">Makkah Al-Mukarramah</h4>
          <span class="text-[10px] text-gold-500 mt-1 block">Sumbu Utama Peradaban</span>
        </div>
        <div class="p-3 rounded-xl bg-navy-950/60 border border-border/10 hover:border-gold-500/20 transition-all cursor-pointer">
          <h4 class="text-xs font-bold text-text-primary">Madinah Al-Munawwarah</h4>
          <span class="text-[10px] text-emerald-500 mt-1 block">Pusat Madani/Pemerintahan</span>
        </div>
        <div class="p-3 rounded-xl bg-navy-950/60 border border-border/10 hover:border-gold-500/20 transition-all cursor-pointer">
          <h4 class="text-xs font-bold text-text-primary">Baitul Maqdis (Al-Quds)</h4>
          <span class="text-[10px] text-amber-500 mt-1 block">Transendental Isra\' Mi\'raj</span>
        </div>
      </div>
    </div>
  </div>

  <!-- Map container -->
  <div class="flex-1 h-full glass rounded-2xl border border-border/10 overflow-hidden p-4">
    <MapView />
  </div>
</div>
