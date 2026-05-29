<script lang="ts">
  let { onSearch = (_q: string) => {} }: { onSearch?: (query: string) => void } = $props();

  let query = $state('');
  let isFocused = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  $effect(() => {
    const q = query;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (q.trim()) {
        onSearch(q.trim());
      }
    }, 300);

    return () => {
      if (debounceTimer) clearTimeout(debounceTimer);
    };
  });
</script>

<div
  class="relative group w-full"
>
  <!-- Glow effect on focus -->
  {#if isFocused}
    <div
      class="absolute -inset-0.5 rounded-2xl opacity-30 blur-sm transition-opacity duration-500"
      style="background: linear-gradient(135deg, #d4a853, #f59e0b, #d4a853);"
    ></div>
  {/if}

  <div
    class="relative flex items-center gap-3 px-5 py-3.5 rounded-xl transition-all duration-300"
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
      style="color: {isFocused ? '#d4a853' : 'rgba(255,255,255,0.3)'};"
    >🔍</span>

    <!-- Input -->
    <input
      type="text"
      bind:value={query}
      onfocus={() => isFocused = true}
      onblur={() => isFocused = false}
      placeholder="Cari peristiwa, tokoh, atau lokasi..."
      class="flex-1 bg-transparent outline-none text-sm text-white/90 placeholder-white/25"
    />

    <!-- Clear button -->
    {#if query}
      <button
        onclick={() => { query = ''; }}
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
</div>
