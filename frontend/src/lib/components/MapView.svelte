<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  interface LocationMarker {
    uuid?: string;
    id?: string;
    name: string;
    lat: number;
    lng: number;
    type: string;
    description?: string;
  }

  let { 
    locations = [],
    center = null,
    zoom = null
  } = $props<{
    locations?: LocationMarker[];
    center?: [number, number] | null;
    zoom?: number | null;
  }>();

  let mapContainer: HTMLDivElement | undefined = $state();
  let map: any = $state.raw(null);
  let mapInstance: any = $state.raw(null);
  // We no longer need a token for MapLibre
  let isLoaded = $state(false);

  // Islamic History Demo Locations
  const demoLocations: LocationMarker[] = [
    { name: 'Makkah Al-Mukarramah', lat: 21.4225, lng: 39.8262, type: 'Kota Suci', description: 'Tempat kelahiran Islam & Ka\'bah' },
    { name: 'Madinah Al-Munawwarah', lat: 24.4672, lng: 39.6112, type: 'Pusat Pemerintahan', description: 'Kota tujuan Hijrah Rasulullah ﷺ' },
    { name: 'Baitul Maqdis (Al-Quds)', lat: 31.7780, lng: 35.2354, type: 'Tempat Suci', description: 'Kiblat pertama & tempat Isra\' Mi\'raj' },
    { name: 'Baghdad', lat: 33.3152, lng: 44.3661, type: 'Pusat Keilmuan', description: 'Ibu kota Kekhalifahan Abbasiyah' },
    { name: 'Cordoba (Qurtuba)', lat: 37.8882, lng: -4.7794, type: 'Peradaban Barat', description: 'Pusat kemajuan Islam di Andalusia' },
  ];

  const activeLocations = $derived(locations.length > 0 ? locations : demoLocations);

  let markers: any[] = [];

  export function flyTo(lng: number, lat: number) {
    if (map) {
      map.flyTo({ center: [lng, lat], zoom: 8, pitch: 45, duration: 2000 });
      // Open the popup of the matching marker
      markers.forEach(m => {
        const lngLat = m.getLngLat();
        if (Math.abs(lngLat.lng - lng) < 0.01 && Math.abs(lngLat.lat - lat) < 0.01) {
          m.togglePopup();
        }
      });
    }
  }

  onMount(async () => {
    try {
      const maplibregl = (await import('maplibre-gl')).default;
      await import('maplibre-gl/dist/maplibre-gl.css');

      const freeDarkStyle = {
        version: 8,
        sources: {
          'carto-dark': {
            type: 'raster',
            tiles: [
              'https://a.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png',
              'https://b.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png',
              'https://c.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png',
              'https://d.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png'
            ],
            tileSize: 256,
            attribution: '&copy; OpenStreetMap contributors &copy; CARTO'
          }
        },
        layers: [
          {
            id: 'carto-dark-layer',
            type: 'raster',
            source: 'carto-dark',
            minzoom: 0,
            maxzoom: 20
          }
        ]
      };

      if (mapContainer) {
        let initialCenter: [number, number] = [39.8262, 21.4225];
        let initialZoom = 3.5;

        if (center) {
          initialCenter = center;
        } else if (locations.length === 1) {
          initialCenter = [Number(locations[0].lng), Number(locations[0].lat)];
        }

        if (zoom !== null) {
          initialZoom = zoom;
        } else if (locations.length === 1) {
          initialZoom = 6;
        }

        const newMap = new maplibregl.Map({
          container: mapContainer,
          style: freeDarkStyle as any,
          center: initialCenter,
          zoom: initialZoom,
          pitch: 30
        });
        
        newMap.on('load', () => {
          mapInstance = maplibregl;
          map = newMap;
          isLoaded = true;
          
          // Force initial marker render
          renderMarkers();
        });
      }
    } catch (err) {
      console.error('MapLibre failed to load:', err);
    }
  });

  function renderMarkers() {
    if (!map || !mapInstance || !activeLocations || activeLocations.length === 0) return;
    
    // Clear existing
    markers.forEach(m => m.remove());
    markers = [];

    activeLocations.forEach((loc: LocationMarker) => {
      const popupHTML = `
        <div class="p-3 text-surface dark:text-text-primary bg-surface-elevated rounded flex flex-col gap-2">
          <div>
            <h3 class="font-bold text-gold-500 text-sm leading-tight">${loc.name}</h3>
            <p class="text-[10px] text-text-secondary mt-0.5">${loc.type}</p>
          </div>
          ${loc.description ? `<p class="text-xs border-t border-border/10 pt-1.5 text-text-muted leading-relaxed">${loc.description}</p>` : ''}
          <a href="/location/${loc.uuid || loc.id || encodeURIComponent(loc.name)}" data-sveltekit-reload class="mt-1 w-full py-1.5 rounded bg-verdigris-500/10 hover:bg-verdigris-500/20 border border-verdigris-500/30 text-verdigris-400 text-[10px] font-bold text-center transition-colors block">
            Lihat Dimensi Lokasi →
          </a>
        </div>
      `;

      const marker = new mapInstance.Marker({ color: '#D4A853' })
        .setLngLat([Number(loc.lng), Number(loc.lat)])
        .setPopup(new mapInstance.Popup({ offset: 25 }).setHTML(popupHTML))
        .addTo(map);
        
      markers.push(marker);
    });
  }

  $effect(() => {
    // This will trigger when activeLocations changes
    if (activeLocations) {
      renderMarkers();
    }
  });

  onDestroy(() => {
    if (map) {
      map.remove();
    }
  });
</script>

<div class="relative w-full h-full min-h-[400px] flex flex-col rounded-2xl overflow-hidden border border-border/15">
  <div bind:this={mapContainer} class="w-full flex-1 relative z-0"></div>
  
  {#if !isLoaded}
    <div class="absolute inset-0 z-10 flex flex-col items-center justify-center bg-iron-950/80 backdrop-blur-sm animate-pulse">
      <span class="w-8 h-8 border-4 border-gold-500 border-t-transparent rounded-full animate-spin"></span>
      <p class="text-xs text-gold-400 mt-4 font-bold tracking-widest uppercase">Memuat Peta Global...</p>
    </div>
  {/if}

</div>

<style>
  :global(.maplibregl-popup-content) {
    background: #1a1d24 !important;
    color: #e5e7eb !important;
    border: 1px solid rgba(212, 168, 83, 0.3);
    border-radius: 8px;
    padding: 0 !important;
    box-shadow: 0 4px 20px rgba(0,0,0,0.5);
  }
  :global(.maplibregl-popup-tip) {
    border-top-color: #1a1d24 !important;
  }
  :global(.maplibregl-popup-close-button) {
    color: #D4A853 !important;
    font-size: 16px;
  }
</style>
