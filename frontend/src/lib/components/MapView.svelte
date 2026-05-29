<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { PUBLIC_MAPBOX_TOKEN } from '$env/static/public';

  interface LocationMarker {
    name: string;
    lat: number;
    lng: number;
    type: string;
    description?: string;
  }

  let { 
    locations = [] 
  } = $props<{
    locations?: LocationMarker[];
  }>();

  let mapContainer: HTMLDivElement;
  let map: any = null;
  let hasToken = $state(!!PUBLIC_MAPBOX_TOKEN && PUBLIC_MAPBOX_TOKEN !== 'pk.your_mapbox_token_here');

  // Islamic History Demo Locations
  const demoLocations: LocationMarker[] = [
    { name: 'Makkah Al-Mukarramah', lat: 21.4225, lng: 39.8262, type: 'Kota Suci', description: 'Tempat kelahiran Islam & Ka\'bah' },
    { name: 'Madinah Al-Munawwarah', lat: 24.4672, lng: 39.6112, type: 'Pusat Pemerintahan', description: 'Kota tujuan Hijrah Rasulullah ﷺ' },
    { name: 'Baitul Maqdis (Al-Quds)', lat: 31.7780, lng: 35.2354, type: 'Tempat Suci', description: 'Kiblat pertama & tempat Isra\' Mi\'raj' },
    { name: 'Baghdad', lat: 33.3152, lng: 44.3661, type: 'Pusat Keilmuan', description: 'Ibu kota Kekhalifahan Abbasiyah' },
    { name: 'Cordoba (Qurtuba)', lat: 37.8882, lng: -4.7794, type: 'Peradaban Barat', description: 'Pusat kemajuan Islam di Andalusia' },
  ];

  const activeLocations = $derived(locations.length > 0 ? locations : demoLocations);

  onMount(async () => {
    if (hasToken) {
      try {
        const mapboxgl = (await import('mapbox-gl')).default;
        import('mapbox-gl/dist/mapbox-gl.css');

        mapboxgl.accessToken = PUBLIC_MAPBOX_TOKEN;
        map = new mapboxgl.Map({
          container: mapContainer,
          style: 'mapbox://styles/mapbox/dark-v11',
          center: [39.8262, 21.4225], // Centered around Arabian Peninsula
          zoom: 3.5,
          pitch: 30
        });

        // Add markers
        activeLocations.forEach(loc => {
          const el = document.createElement('div');
          el.className = 'custom-marker';
          
          new mapboxgl.Marker(el)
            .setLngLat([loc.lng, loc.lat])
            .setPopup(
              new mapboxgl.Popup({ offset: 25 })
                .setHTML(`
                  <div class="p-2 text-surface dark:text-text-primary bg-surface-elevated rounded">
                    <h3 class="font-bold text-gold-500">${loc.name}</h3>
                    <p class="text-xs text-text-secondary">${loc.type}</p>
                    ${loc.description ? `<p class="text-xs mt-1 border-t border-border/10 pt-1">${loc.description}</p>` : ''}
                  </div>
                `)
            )
            .addTo(map);
        });
      } catch (err) {
        console.error('Mapbox failed to load:', err);
        hasToken = false;
      }
    }
  });

  onDestroy(() => {
    if (map) {
      map.remove();
    }
  });
</script>

<div class="relative w-full h-full min-h-[400px] flex flex-col rounded-2xl overflow-hidden border border-border/15">
  {#if hasToken}
    <div bind:this={mapContainer} class="w-full flex-1"></div>
  {:else}
    <!-- Gorgeous Custom Styled Islamic Map Canvas/SVG Placeholder -->
    <div class="flex-1 w-full bg-navy-950 pattern-islamic flex flex-col items-center justify-center p-8 text-center relative">
      <div class="absolute inset-0 bg-gradient-to-b from-transparent to-navy-950/90 pointer-events-none"></div>
      
      <!-- Interactive Decorative Map Graphic -->
      <div class="glass max-w-lg p-8 rounded-2xl relative z-10 border border-gold-500/20 shadow-2xl animate-fade-in">
        <span class="text-4xl">🗺️</span>
        <h2 class="text-xl font-bold text-gold-400 mt-4 mb-2">Peta Interaktif Sumbu Peradaban</h2>
        <p class="text-xs text-text-secondary mb-6 leading-relaxed">
          Visualisasi pemetaan 3 Dimensi Geospasial Dunia Islam sebagai Pivot Narasi Sejarah Dunia.
        </p>

        <!-- Mock Map Locations -->
        <div class="grid grid-cols-2 gap-3 text-left">
          {#each activeLocations as loc}
            <div class="p-3 rounded-lg bg-navy-900/60 border border-border/10 hover:border-gold-500/20 transition-all">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-gold-500 animate-ping"></span>
                <span class="text-xs font-bold text-text-primary">{loc.name}</span>
              </div>
              <p class="text-[10px] text-text-muted mt-1">{loc.type}</p>
            </div>
          {/each}
        </div>

        <div class="mt-6 text-[10px] text-text-muted italic border-t border-border/10 pt-4">
          💡 Mapbox GL JS Token tidak terdeteksi. Set <code class="text-gold-300 font-mono">PUBLIC_MAPBOX_TOKEN</code> di file <code class="font-mono text-gold-300">.env</code> untuk melihat peta satelit beresolusi tinggi.
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  :global(.custom-marker) {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background-color: var(--color-gold-500);
    border: 2px solid var(--color-surface);
    box-shadow: 0 0 10px var(--color-gold-500);
    cursor: pointer;
    transition: transform 0.2s ease;
  }
  :global(.custom-marker:hover) {
    transform: scale(1.3);
  }
</style>
