<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import cytoscape from 'cytoscape';

  interface NodeData {
    id: string;
    label: string;
    type: 'event' | 'actor' | 'location';
    tier: 'draft' | 'verified' | 'reviewed' | 'canonical';
  }

  interface EdgeData {
    source: string;
    target: string;
    relationship: string;
  }

  let { 
    elements = $bindable([]),
    onNodeSelect = null
  } = $props<{
    elements?: any[];
    onNodeSelect?: ((node: any) => void) | null;
  }>();

  let container: HTMLDivElement;
  let cy: any;
  let currentLayout = $state('cose');

  // Islamic-inspired dark theme colors
  const colors = {
    bg: '#0a0f1e',
    event: '#d4a853',      // Gold
    actor: '#10b981',      // Emerald
    location: '#f59e0b',   // Amber
    edge: '#4a5568',       // Muted slate
    text: '#f1f5f9',
    draft: '#6b7280',
    verified: '#3b82f6',
    reviewed: '#d4a853',
    canonical: '#10b981'
  };

  // Demo elements representing the center-outward narrative pivot
  const defaultElements = [
    // Nodes
    { data: { id: 'prophet', label: 'Nabi Muhammad ﷺ', type: 'actor', tier: 'canonical' } },
    { data: { id: 'abubakar', label: 'Abu Bakar As-Siddiq', type: 'actor', tier: 'reviewed' } },
    { data: { id: 'umar', label: 'Umar bin Khattab', type: 'actor', tier: 'reviewed' } },
    { data: { id: 'hijrah', label: 'Hijrah ke Madinah (622 M)', type: 'event', tier: 'canonical' } },
    { data: { id: 'fathu', label: 'Fathu Makkah (630 M)', type: 'event', tier: 'canonical' } },
    { data: { id: 'makkah', label: 'Makkah', type: 'location', tier: 'canonical' } },
    { data: { id: 'madinah', label: 'Madinah', type: 'location', tier: 'canonical' } },
    { data: { id: 'alquds', label: 'Baitul Maqdis', type: 'location', tier: 'canonical' } },

    // Edges
    { data: { source: 'prophet', target: 'hijrah', relationship: 'PARTICIPATED_IN' } },
    { data: { source: 'abubakar', target: 'hijrah', relationship: 'PARTICIPATED_IN' } },
    { data: { source: 'prophet', target: 'fathu', relationship: 'PARTICIPATED_IN' } },
    { data: { source: 'umar', target: 'fathu', relationship: 'PARTICIPATED_IN' } },
    { data: { source: 'hijrah', target: 'madinah', relationship: 'OCCURRED_AT' } },
    { data: { source: 'fathu', target: 'makkah', relationship: 'OCCURRED_AT' } },
    { data: { source: 'umar', target: 'alquds', relationship: 'INFLUENCED' } }
  ];

  function initCytoscape() {
    const elms = elements.length > 0 ? elements : defaultElements;

    cy = cytoscape({
      container,
      elements: elms,
      style: [
        {
          selector: 'node',
          style: {
            'background-color': (ele: any) => {
              const type = ele.data('type');
              if (type === 'event') return colors.event;
              if (type === 'actor') return colors.actor;
              return colors.location;
            },
            'label': 'data(label)',
            'color': colors.text,
            'font-size': '12px',
            'font-family': 'Inter, sans-serif',
            'text-valign': 'center',
            'text-halign': 'center',
            'width': '60px',
            'height': '60px',
            'border-width': '2px',
            'border-color': '#1f2937',
            'text-wrap': 'wrap',
            'text-max-width': '80px'
          }
        },
        {
          selector: 'node[type="event"]',
          style: {
            'shape': 'hexagon',
          }
        },
        {
          selector: 'node[type="location"]',
          style: {
            'shape': 'diamond',
          }
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': colors.edge,
            'target-arrow-color': colors.edge,
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier',
            'label': 'data(relationship)',
            'font-size': '8px',
            'color': '#94a3b8',
            'text-background-opacity': 0.7,
            'text-background-color': '#0a0f1e',
            'text-background-padding': '3px',
            'text-background-shape': 'roundrectangle'
          }
        }
      ],
      layout: {
        name: currentLayout,
        animate: true
      }
    });

    cy.on('tap', 'node', (evt: any) => {
      const node = evt.target;
      if (onNodeSelect) {
        onNodeSelect(node.data());
      }
    });
  }

  function updateLayout(layoutName: string) {
    currentLayout = layoutName;
    if (cy) {
      cy.layout({ name: layoutName, animate: true }).run();
    }
  }

  onMount(() => {
    initCytoscape();
  });

  onDestroy(() => {
    if (cy) {
      cy.destroy();
    }
  });
</script>

<div class="relative w-full h-full flex flex-col min-h-[400px]">
  <!-- Control Bar -->
  <div class="absolute top-4 left-4 z-10 flex gap-2 glass px-3 py-2 rounded-xl">
    <button 
      class="px-2 py-1 text-xs rounded transition-all {currentLayout === 'cose' ? 'bg-gold-500 text-surface' : 'text-text-secondary hover:bg-navy-800'}"
      onclick={() => updateLayout('cose')}
    >
      Cose
    </button>
    <button 
      class="px-2 py-1 text-xs rounded transition-all {currentLayout === 'circle' ? 'bg-gold-500 text-surface' : 'text-text-secondary hover:bg-navy-800'}"
      onclick={() => updateLayout('circle')}
    >
      Circle
    </button>
    <button 
      class="px-2 py-1 text-xs rounded transition-all {currentLayout === 'grid' ? 'bg-gold-500 text-surface' : 'text-text-secondary hover:bg-navy-800'}"
      onclick={() => updateLayout('grid')}
    >
      Grid
    </button>
  </div>

  <!-- Map/Graph Container -->
  <div bind:this={container} class="w-full flex-1 rounded-2xl border border-border/10 bg-surface/50 overflow-hidden"></div>
</div>

<style>
  :global(.cy-node-hover) {
    cursor: pointer;
  }
</style>
