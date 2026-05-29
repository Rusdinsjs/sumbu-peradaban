<script lang="ts">
  import { page } from '$app/state';

  const navItems = [
    { icon: '🏠', label: 'Dashboard', href: '/' },
    { icon: '🕸️', label: 'Graph Explorer', href: '/graph' },
    { icon: '🗺️', label: 'Peta Sejarah', href: '/map' },
    { icon: '📅', label: 'Timeline', href: '/timeline' },
    { icon: '⚙️', label: 'Data Factory', href: '/factory' },
    { icon: '👤', label: 'Curator', href: '/curator' },
  ];

  let currentPath = $derived(page.url.pathname);
  let collapsed = $state(false);

  function isActive(href: string): boolean {
    if (href === '/') return currentPath === '/';
    return currentPath.startsWith(href);
  }
</script>

<aside
  class="fixed left-0 top-0 h-screen z-50 flex flex-col border-r border-white/5"
  style="width: 260px; background: rgba(10, 15, 30, 0.85); backdrop-filter: blur(24px); -webkit-backdrop-filter: blur(24px);"
>
  <!-- Logo Area -->
  <div class="px-6 py-7 border-b border-white/5">
    <a href="/" class="flex items-center gap-3 group">
      <span class="text-2xl transition-transform duration-300 group-hover:rotate-12">𝌭</span>
      <div>
        <h1
          class="text-lg font-bold tracking-wide"
          style="background: linear-gradient(135deg, #d4a853, #f5d78e, #d4a853); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;"
        >
          Sumbu Peradaban
        </h1>
        <p class="text-[10px] text-white/30 tracking-[0.2em] uppercase">World Knowledge Graph</p>
      </div>
    </a>
  </div>

  <!-- Islamic Geometric Pattern Divider -->
  <div class="flex justify-center py-2 opacity-20">
    <svg width="200" height="12" viewBox="0 0 200 12" fill="none">
      <path d="M0 6 L20 0 L40 6 L60 0 L80 6 L100 0 L120 6 L140 0 L160 6 L180 0 L200 6" stroke="#d4a853" stroke-width="0.5" fill="none"/>
      <path d="M0 6 L20 12 L40 6 L60 12 L80 6 L100 12 L120 6 L140 12 L160 6 L180 12 L200 6" stroke="#d4a853" stroke-width="0.5" fill="none"/>
    </svg>
  </div>

  <!-- Navigation -->
  <nav class="flex-1 px-3 py-4 space-y-1 overflow-y-auto">
    {#each navItems as item}
      {@const active = isActive(item.href)}
      <a
        href={item.href}
        class="group flex items-center gap-3 px-4 py-3 rounded-lg text-sm font-medium transition-all duration-300 relative overflow-hidden {!active ? 'text-white/50 hover:text-white/80' : ''}"
        style={active
          ? 'color: #d4a853; background: rgba(212, 168, 83, 0.08); border-left: 3px solid #d4a853;'
          : 'border-left: 3px solid transparent;'}
      >
        <!-- Hover glow effect -->
        {#if !active}
          <div
            class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
            style="background: linear-gradient(90deg, rgba(212, 168, 83, 0.03), transparent);"
          ></div>
        {/if}

        <span class="text-lg relative z-10 transition-transform duration-200 group-hover:scale-110">{item.icon}</span>
        <span class="relative z-10">{item.label}</span>

        {#if active}
          <div
            class="absolute right-3 w-1.5 h-1.5 rounded-full"
            style="background: #d4a853; box-shadow: 0 0 8px rgba(212, 168, 83, 0.6);"
          ></div>
        {/if}
      </a>
    {/each}
  </nav>

  <!-- Bottom Section -->
  <div class="px-6 py-4 border-t border-white/5">
    <!-- Decorative element -->
    <div class="flex justify-center mb-3 opacity-15">
      <svg width="40" height="40" viewBox="0 0 40 40" fill="none">
        <polygon points="20,2 26,14 38,16 29,25 31,38 20,32 9,38 11,25 2,16 14,14" stroke="#d4a853" stroke-width="0.5" fill="none"/>
        <polygon points="20,8 24,16 32,17 26,23 28,32 20,28 12,32 14,23 8,17 16,16" stroke="#d4a853" stroke-width="0.3" fill="none"/>
      </svg>
    </div>
    <p class="text-[10px] text-white/20 text-center tracking-widest uppercase">v0.1.0 — Alpha</p>
  </div>
</aside>
