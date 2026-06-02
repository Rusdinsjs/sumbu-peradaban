<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import cytoscape from "cytoscape";
  import { gql } from "$lib/graphql/client";

  interface NodeData {
    id: string;
    label: string;
    type: "event" | "actor" | "location";
    tier: "draft" | "verified" | "reviewed" | "canonical";
  }

  interface EdgeData {
    source: string;
    target: string;
    relationship: string;
  }

  let { elements = $bindable([]), onNodeSelect = null } = $props<{
    elements?: any[];
    onNodeSelect?: ((node: any) => void) | null;
  }>();

  let container: HTMLDivElement;
  let cy: any;
  let currentLayout = $state("cose");
  let resizeObserver: ResizeObserver;

  // Search state
  let searchQuery = $state("");
  let searchCategory = $state<
    "all" | "event" | "actor" | "location" | "source"
  >("all");
  let searchResults = $state<any[]>([]);
  let isSearching = $state(false);
  let showResults = $state(false);
  let searchTimeout: ReturnType<typeof setTimeout>;

  // Islamic-inspired dark theme colors
  const colors = {
    bg: "#0a0f1e",
    event: "#e4891b", // Gold
    actor: "#5d8f8a", // Emerald
    location: "#f59e0b", // Amber
    source: "#8b5cf6", // Violet
    edge: "#4a5568", // Muted slate
    text: "#f1f5f9",
    draft: "#6b7280",
    verified: "#3b82f6",
    reviewed: "#e4891b",
    canonical: "#5d8f8a",
  };

  const categoryLabels: Record<
    string,
    { label: string; color: string; icon: string }
  > = {
    all: { label: "Semua", color: "#94a3b8", icon: "🔍" },
    event: { label: "Peristiwa", color: colors.event, icon: "⬡" },
    actor: { label: "Tokoh", color: colors.actor, icon: "●" },
    location: { label: "Lokasi", color: colors.location, icon: "◆" },
    source: { label: "Sumber", color: colors.source, icon: "📄" },
  };

  // Demo elements representing the center-outward narrative pivot
  const defaultElements = [
    // Nodes
    {
      data: {
        id: "prophet",
        label: "Nabi Muhammad ﷺ",
        type: "actor",
        tier: "canonical",
      },
    },
    {
      data: {
        id: "abubakar",
        label: "Abu Bakar As-Siddiq",
        type: "actor",
        tier: "reviewed",
      },
    },
    {
      data: {
        id: "umar",
        label: "Umar bin Khattab",
        type: "actor",
        tier: "reviewed",
      },
    },
    {
      data: {
        id: "hijrah",
        label: "Hijrah ke Madinah (622 M)",
        type: "event",
        tier: "canonical",
      },
    },
    {
      data: {
        id: "fathu",
        label: "Fathu Makkah (630 M)",
        type: "event",
        tier: "canonical",
      },
    },
    {
      data: {
        id: "makkah",
        label: "Makkah",
        type: "location",
        tier: "canonical",
      },
    },
    {
      data: {
        id: "madinah",
        label: "Madinah",
        type: "location",
        tier: "canonical",
      },
    },
    {
      data: {
        id: "alquds",
        label: "Baitul Maqdis",
        type: "location",
        tier: "canonical",
      },
    },

    // Edges
    {
      data: {
        source: "prophet",
        target: "hijrah",
        relationship: "PARTICIPATED_IN",
      },
    },
    {
      data: {
        source: "abubakar",
        target: "hijrah",
        relationship: "PARTICIPATED_IN",
      },
    },
    {
      data: {
        source: "prophet",
        target: "fathu",
        relationship: "PARTICIPATED_IN",
      },
    },
    {
      data: {
        source: "umar",
        target: "fathu",
        relationship: "PARTICIPATED_IN",
      },
    },
    {
      data: {
        source: "hijrah",
        target: "madinah",
        relationship: "OCCURRED_AT",
      },
    },
    {
      data: { source: "fathu", target: "makkah", relationship: "OCCURRED_AT" },
    },
    { data: { source: "umar", target: "alquds", relationship: "INFLUENCED" } },
  ];

  async function performSearch(q: string) {
    if (q.trim().length < 2) {
      searchResults = [];
      showResults = false;
      return;
    }

    isSearching = true;
    showResults = true;
    const results: any[] = [];
    const seen = new Set<string>();
    const qLower = q.toLowerCase();

    // 1. Search LOCAL graph nodes first (always available, instant)
    if (cy) {
      cy.nodes().forEach((node: any) => {
        const data = node.data();
        const label = (data.label || "").toLowerCase();
        const matchesCategory =
          searchCategory === "all" || searchCategory === data.type;
        if (matchesCategory && label.includes(qLower)) {
          const key = `local:${data.id}`;
          if (!seen.has(key)) {
            seen.add(key);
            results.push({
              id: data.id,
              label: data.label,
              type: data.type || "event",
              tier: data.tier || "draft",
              source: "local",
            });
          }
        }
      });
    }

    // 2. Search API (database) for real persisted entities
    try {
      if (searchCategory === "all" || searchCategory === "event") {
        const eventData = await gql<any>(
          `
          query SearchEvents($limit: Int) {
            events(limit: $limit) { uuid, title, curationTier }
          }
        `,
          { limit: 50 },
        );
        (eventData.events || [])
          .filter((e: any) => e.title.toLowerCase().includes(qLower))
          .forEach((e: any) => {
            const key = `api:${e.uuid}`;
            if (!seen.has(key)) {
              seen.add(key);
              results.push({
                id: e.uuid,
                label: e.title,
                type: "event",
                tier: e.curationTier?.toLowerCase() || "draft",
                source: "api",
              });
            }
          });
      }

      if (searchCategory === "all" || searchCategory === "actor") {
        const actorData = await gql<any>(
          `
          query SearchActors($limit: Int) {
            actors(limit: $limit) { uuid, name, curationTier }
          }
        `,
          { limit: 50 },
        );
        (actorData.actors || [])
          .filter((a: any) => a.name.toLowerCase().includes(qLower))
          .forEach((a: any) => {
            const key = `api:${a.uuid}`;
            if (!seen.has(key)) {
              seen.add(key);
              results.push({
                id: a.uuid,
                label: a.name,
                type: "actor",
                tier: a.curationTier?.toLowerCase() || "draft",
                source: "api",
              });
            }
          });
      }

      if (searchCategory === "all" || searchCategory === "location") {
        const locData = await gql<any>(
          `
          query SearchLocations($limit: Int) {
            locations(limit: $limit) { uuid, name, curationTier }
          }
        `,
          { limit: 50 },
        );
        (locData.locations || [])
          .filter((l: any) => l.name.toLowerCase().includes(qLower))
          .forEach((l: any) => {
            const key = `api:${l.uuid}`;
            if (!seen.has(key)) {
              seen.add(key);
              results.push({
                id: l.uuid,
                label: l.name,
                type: "location",
                tier: l.curationTier?.toLowerCase() || "draft",
                source: "api",
              });
            }
          });
      }

      if (searchCategory === "all" || searchCategory === "source") {
        const srcData = await gql<any>(
          `
          query SearchSources($limit: Int) {
            sources(limit: $limit) { sourceId, domain, referenceText }
          }
        `,
          { limit: 50 },
        );
        (srcData.sources || [])
          .filter(
            (s: any) =>
              (s.domain || "").toLowerCase().includes(qLower) ||
              (s.referenceText || "").toLowerCase().includes(qLower),
          )
          .forEach((s: any) => {
            const key = `api:${s.sourceId}`;
            if (!seen.has(key)) {
              seen.add(key);
              results.push({
                id: s.sourceId,
                label: s.referenceText || s.domain,
                type: "source",
                tier: "draft",
                source: "api",
              });
            }
          });
      }
    } catch (err) {
      console.error("Search failed:", err);
    }

    searchResults = results;
    isSearching = false;
  }

  function handleSearchInput() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      performSearch(searchQuery);
    }, 300);
  }

  function selectSearchResult(result: any) {
    if (!cy) return;

    let node = cy.getElementById(result.id);

    // If node not in graph yet, add it dynamically
    if (!node || node.length === 0) {
      cy.add({
        data: {
          id: result.id,
          label: result.label,
          type: result.type,
          tier: result.tier,
        },
      });
      node = cy.getElementById(result.id);
    }

    // Remove previous highlights
    cy.nodes().removeClass("highlighted");

    // Center on the node and highlight it
    if (node && node.length > 0) {
      node.addClass("highlighted");
      cy.animate({ center: { eles: node }, zoom: 2 }, { duration: 600 });
      // Remove highlight after 3s
      setTimeout(() => node.removeClass("highlighted"), 3000);
    }

    // Notify parent
    if (onNodeSelect) {
      onNodeSelect(result);
    }

    showResults = false;
    searchQuery = "";
  }

  function initCytoscape() {
    const elms = elements.length > 0 ? elements : defaultElements;

    cy = cytoscape({
      container,
      elements: elms,
      style: [
        {
          selector: "node",
          style: {
            "background-color": (ele: any) => {
              const type = ele.data("type");
              if (type === "event") return colors.event;
              if (type === "actor") return colors.actor;
              return colors.location;
            },
            label: "data(label)",
            color: colors.text,
            "font-size": "12px",
            "font-family": "Inter, sans-serif",
            "text-valign": "center",
            "text-halign": "center",
            width: "60px",
            height: "60px",
            "border-width": "2px",
            "border-color": "#1f2937",
            "text-wrap": "wrap",
            "text-max-width": "80px",
          },
        },
        {
          selector: 'node[type="event"]',
          style: {
            shape: "hexagon",
          },
        },
        {
          selector: 'node[type="location"]',
          style: {
            shape: "diamond",
          },
        },
        {
          selector: "node.highlighted",
          style: {
            "border-width": "4px",
            "border-color": "#D4A853",
            "background-color": "#D4A853",
            width: "80px",
            height: "80px",
          },
        },
        {
          selector: "edge",
          style: {
            width: 2,
            "line-color": colors.edge,
            "target-arrow-color": colors.edge,
            "target-arrow-shape": "triangle",
            "curve-style": "bezier",
            label: "data(relationship)",
            "font-size": "8px",
            color: "#94a3b8",
            "text-background-opacity": 0.7,
            "text-background-color": "#0a0f1e",
            "text-background-padding": "3px",
            "text-background-shape": "roundrectangle",
          },
        },
      ],
      layout: {
        name: currentLayout,
        animate: true,
      } as any,
    });

    cy.on("tap", "node", (evt: any) => {
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

    if (container) {
      resizeObserver = new ResizeObserver(() => {
        if (cy) cy.resize();
      });
      resizeObserver.observe(container);
    }
  });

  // Reactive effect to update Cytoscape when elements prop changes
  $effect(() => {
    if (cy && elements && elements.length > 0) {
      cy.elements().remove();
      cy.add(elements);
      cy.layout({ name: currentLayout, animate: true }).run();
    }
  });

  onDestroy(() => {
    if (resizeObserver) resizeObserver.disconnect();
    if (cy) {
      cy.destroy();
    }
  });
</script>

<div class="relative w-full h-full flex flex-col min-h-[400px]">
  <!-- Control Bar (top-left) -->
  <div class="absolute top-4 left-4 z-10 flex gap-2 glass px-3 py-2 rounded-xl">
    <button
      class="px-2 py-1 text-xs rounded transition-all {currentLayout === 'cose'
        ? 'bg-gold-500 text-surface'
        : 'text-text-secondary hover:bg-iron-800'}"
      onclick={() => updateLayout("cose")}
    >
      Cose
    </button>
    <button
      class="px-2 py-1 text-xs rounded transition-all {currentLayout ===
      'circle'
        ? 'bg-gold-500 text-surface'
        : 'text-text-secondary hover:bg-iron-800'}"
      onclick={() => updateLayout("circle")}
    >
      Circle
    </button>
    <button
      class="px-2 py-1 text-xs rounded transition-all {currentLayout === 'grid'
        ? 'bg-gold-500 text-surface'
        : 'text-text-secondary hover:bg-iron-800'}"
      onclick={() => updateLayout("grid")}
    >
      Grid
    </button>
  </div>

  <!-- Search Bar (top-right) -->
  <div class="absolute top-4 right-4 z-10 flex flex-col items-end gap-1">
    <div
      class="glass rounded-xl border border-border/15 flex items-center gap-1 px-2 py-1.5 w-80"
    >
      <!-- Category selector -->
      <div class="flex gap-0.5 mr-1 border-r border-border/15 pr-2">
        {#each Object.entries(categoryLabels) as [key, meta]}
          <button
            onclick={() => {
              searchCategory = key as any;
              if (searchQuery) handleSearchInput();
            }}
            class="px-1.5 py-0.5 text-[10px] rounded transition-all font-bold"
            style={searchCategory === key
              ? `background: ${meta.color}22; color: ${meta.color};`
              : "color: #6b7280;"}
            title={meta.label}
          >
            {meta.icon}
          </button>
        {/each}
      </div>

      <!-- Search input -->
      <svg
        class="w-3.5 h-3.5 text-text-muted flex-shrink-0"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
      <input
        type="text"
        bind:value={searchQuery}
        oninput={handleSearchInput}
        onfocus={() => {
          if (searchResults.length > 0) showResults = true;
        }}
        placeholder="Cari entitas graf..."
        class="bg-transparent text-xs text-text-primary placeholder-text-muted outline-none flex-1 min-w-0"
      />
      {#if isSearching}
        <span
          class="w-3.5 h-3.5 border-2 border-gold-500 border-t-transparent rounded-full animate-spin flex-shrink-0"
        ></span>
      {/if}
      {#if searchQuery}
        <button
          onclick={() => {
            searchQuery = "";
            searchResults = [];
            showResults = false;
          }}
          class="text-text-muted hover:text-text-primary text-xs px-1">✕</button
        >
      {/if}
    </div>

    <!-- Search Results Dropdown -->
    {#if showResults && searchResults.length > 0}
      <div
        class="glass rounded-xl border border-border/15 w-80 max-h-64 overflow-y-auto shadow-2xl"
      >
        <div class="p-2 border-b border-border/10">
          <span
            class="text-[10px] text-text-muted font-bold uppercase tracking-wider"
            >{searchResults.length} hasil ditemukan</span
          >
        </div>
        {#each searchResults as result}
          <button
            onclick={() => selectSearchResult(result)}
            class="w-full text-left px-3 py-2.5 hover:bg-iron-800/60 transition-colors flex items-center gap-3 border-b border-border/5 last:border-0"
          >
            <span
              class="w-6 h-6 rounded flex items-center justify-center text-[10px] font-bold flex-shrink-0"
              style="background: {categoryLabels[result.type]?.color ||
                '#6b7280'}22; color: {categoryLabels[result.type]?.color ||
                '#6b7280'};"
            >
              {categoryLabels[result.type]?.icon || "?"}
            </span>
            <div class="flex-1 min-w-0">
              <p class="text-xs font-semibold text-text-primary truncate">
                {result.label}
              </p>
              <span
                class="text-[10px] capitalize"
                style="color: {categoryLabels[result.type]?.color ||
                  '#6b7280'};"
                >{categoryLabels[result.type]?.label || result.type}</span
              >
            </div>
            <span class="text-[10px] text-text-muted capitalize"
              >{result.tier}</span
            >
          </button>
        {/each}
      </div>
    {:else if showResults && !isSearching && searchQuery.length >= 2}
      <div
        class="glass rounded-xl border border-border/15 w-80 p-4 text-center"
      >
        <p class="text-xs text-text-muted">
          Tidak ada hasil untuk "<strong class="text-text-primary"
            >{searchQuery}</strong
          >"
        </p>
      </div>
    {/if}
  </div>

  <!-- Map/Graph Container -->
  <div
    bind:this={container}
    class="w-full flex-1 rounded-2xl border border-border/10 bg-surface/50 overflow-hidden"
  ></div>
</div>

<!-- Click outside to close results -->
<svelte:window
  onclick={() => {
    showResults = false;
  }}
/>

<style>
  :global(.cy-node-hover) {
    cursor: pointer;
  }
</style>
