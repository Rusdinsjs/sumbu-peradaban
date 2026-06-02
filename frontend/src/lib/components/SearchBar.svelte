<script lang="ts">
  import { gql } from '$lib/graphql/client';

  let { onSearch = (_q: string) => {} }: { onSearch?: (query: string) => void } = $props();

  let query = $state('');
  let isFocused = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  
  let results = $state<any[]>([]);
  let isSearching = $state(false);

  async function performSearch(q: string) {
    if (!q.trim()) {
      results = [];
      return;
    }
    isSearching = true;
    try {
      const queryStr = `
        query SearchAll($query: String!) {
          searchAll(query: $query) {
            uuid
            title
            description
            entityType
          }
        }
      `;
      const res = await gql<any>(queryStr, { query: q });
      results = res.searchAll || [];
    } catch (err) {
      console.error('Cross-entity search failed:', err);
    } finally {
      isSearching = false;
    }
  }

  $effect(() => {
    const q = query;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (q.trim()) {
        performSearch(q);
        onSearch(q.trim());
      } else {
        results = [];
      }
    }, 300);

    return () => {
      if (debounceTimer) clearTimeout(debounceTimer);
    };
  });

  function getUrl(item: any) {
    if (item.entityType === 'event') return `/event/${item.uuid}`;
    if (item.entityType === 'actor') return `/actor/${encodeURIComponent(item.title)}`;
    if (item.entityType === 'location') return `/location/${encodeURIComponent(item.title)}`;
    if (item.entityType === 'source') return `/source/${item.uuid}`;
    return '#';
  }
</script>

<div
  class="relative group w-full"
>
  <!-- Glow effect on focus -->
  {#if isFocused}
    <div
      class="absolute -inset-0.5 rounded-2xl opacity-30 blur-sm transition-opacity duration-500"
      style="background: linear-gradient(135deg, #e4891b, #f59e0b, #e4891b);"
    ></div>
  {/if}

  <div
    class="relative flex items-center gap-3 px-5 py-3.5 rounded-xl transition-all duration-300 animate-fade-in"
    style="
      background: rgba(255, 255, 255, 0.03);
      backdrop-filter: blur(16px);
      -webkit-backdrop-filter: blur(16px);
      border: 1px solid {isFocused ? 'rgba(212, 168, 83, 0.4)' : 'rgba(255, 255, 255, 0.06)'};
      box-shadow: {isFocused ? '0 0 20px rgba(212, 168, 83, 0.08)' : '0 2px 8px rgba(0,0,0,0.2)'};
    "
  >
    <!-- Search Icon -->
    <span
      class="text-lg transition-all duration-300"
      style="color: {isFocused ? '#e4891b' : 'rgba(255,255,255,0.3)'};"
    >🔍</span>

    <!-- Input -->
    <input
      type="text"
      bind:value={query}
      onfocus={() => isFocused = true}
      onblur={() => setTimeout(() => { isFocused = false; }, 200)}
      placeholder="Cari peristiwa, tokoh, lokasi, atau sumber..."
      class="flex-1 bg-transparent outline-none text-sm text-white/90 placeholder-white/25"
    />

    <!-- Clear button -->
    {#if query}
      <button
        onclick={() => { query = ''; results = []; }}
        class="text-white/30 hover:text-white/60 transition-colors duration-200 text-lg leading-none"
        aria-label="Clear search"
      >
        ✕
      </button>
    {/if}

    <!-- Shortcut hint -->
    {#if !isFocused && !query}
      <kbd
        class="hidden sm:inline-flex items-center px-2 py-0.5 rounded text-[10px] text-white/20 border border-white/10"
        style="background: rgba(255,255,255,0.03);"
      >
        Ctrl+K
      </kbd>
    {/if}
  </div>

  <!-- Search Results Dropdown -->
  {#if query.trim() && isFocused}
    <div
      class="absolute left-0 right-0 top-full mt-2 glass border border-border/15 rounded-xl overflow-hidden shadow-2xl z-50 animate-fade-in max-h-[350px] overflow-y-auto"
      style="background: rgba(10, 15, 30, 0.95); backdrop-filter: blur(20px);"
    >
      {#if isSearching}
        <div class="p-5 text-center text-xs text-text-muted flex items-center justify-center gap-2">
          <span class="animate-spin w-4 h-4 border-2 border-gold-500/20 border-t-gold-500 rounded-full"></span>
          Mencari lintas entitas...
        </div>
      {:else if results.length === 0}
        <div class="p-5 text-center text-xs text-text-muted">
          Tidak ada entitas ditemukan untuk "{query}"
        </div>
      {:else}
        <div class="p-2 flex flex-col gap-1">
          {#each results as item}
            <a
              href={getUrl(item)}
              class="flex items-center justify-between p-3 rounded-lg hover:bg-white/5 transition-colors group/item"
            >
              <div class="flex flex-col gap-0.5 text-left">
                <span class="text-xs font-bold text-white group-hover/item:text-gold-400 transition-colors">
                  {item.title}
                </span>
                {#if item.description}
                  <span class="text-[10px] text-text-muted line-clamp-1 max-w-[400px]">
                    {item.description}
                  </span>
                {/if}
              </div>
              
              <!-- Tag with Entity Type -->
              <span class="text-[9px] uppercase tracking-wider font-extrabold px-2 py-0.5 rounded bg-iron-950/60 text-gold-400 border border-gold-500/20 whitespace-nowrap">
                {item.entityType === 'source' ? 'Sumber' : item.entityType === 'actor' ? 'Tokoh' : item.entityType === 'event' ? 'Peristiwa' : 'Lokasi'}
              </span>
            </a>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
