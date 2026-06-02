<script lang="ts">
  import { page } from "$app/state";
  import { slide } from "svelte/transition";
  import { auth } from "$lib/stores/auth.svelte";
  import { 
    Compass, 
    Network, 
    Landmark, 
    Users, 
    Map as MapIcon, 
    Clock, 
    BookOpen, 
    Anvil, 
    LineChart, 
    Bot, 
    UserCog, 
    MapPin, 
    Library, 
    ScrollText, 
    Shield, 
    PenTool 
  } from "lucide-svelte";

  let currentPath = $derived(page.url.pathname);
  let { collapsed = $bindable(false) } = $props<{ collapsed?: boolean }>();

  let entitasOpen = $state(false);
  let factoryOpen = $state(false);

  function isActive(href: string): boolean {
    if (href === "/") return currentPath === "/";
    return currentPath.startsWith(href);
  }

  function toggle() {
    collapsed = !collapsed;
  }

  function handleEntitasClick(e: Event) {
    if (collapsed) {
      collapsed = false;
      entitasOpen = true;
    } else {
      entitasOpen = !entitasOpen;
    }
  }

  function handleFactoryClick(e: Event) {
    if (collapsed) {
      collapsed = false;
      factoryOpen = true;
    } else {
      factoryOpen = !factoryOpen;
    }
  }

  $effect(() => {
    if (collapsed) {
      entitasOpen = false;
      factoryOpen = false;
    }
  });
</script>

<aside class="sidebar" class:collapsed>
  <!-- Toggle Button -->
  <button
    onclick={toggle}
    class="toggle-btn"
    title={collapsed ? "Buka sidebar" : "Tutup sidebar"}
  >
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="toggle-icon"
      class:rotated={collapsed}
    >
      <polyline points="15 18 9 12 15 6"></polyline>
    </svg>
  </button>

  <!-- Logo Area -->
  <div class="logo-area">
    <a href="/" class="logo-link group">
      <span class="logo-glyph">{collapsed ? "𝌭" : "𝌭"}</span>
      {#if !collapsed}
        <div class="logo-text">
          <h1 class="logo-title">Sumbu Peradaban</h1>
          <p class="logo-subtitle">World Knowledge Graph</p>
        </div>
      {/if}
    </a>
  </div>

  <!-- Islamic Geometric Pattern Divider -->
  {#if !collapsed}
    <div class="divider">
      <svg width="200" height="12" viewBox="0 0 200 12" fill="none">
        <path
          d="M0 6 L20 0 L40 6 L60 0 L80 6 L100 0 L120 6 L140 0 L160 6 L180 0 L200 6"
          stroke="#e4891b"
          stroke-width="0.5"
          fill="none"
        />
        <path
          d="M0 6 L20 12 L40 6 L60 12 L80 6 L100 12 L120 6 L140 12 L160 6 L180 12 L200 6"
          stroke="#e4891b"
          stroke-width="0.5"
          fill="none"
        />
      </svg>
    </div>
  {/if}

  <!-- Navigation -->
  <nav class="nav-area">
    <!-- Dashboard -->
    <a
      href="/"
      class="nav-item"
      class:active={isActive('/')}
      title={collapsed ? "Dashboard" : ""}
    >
      <Compass size={20} class="nav-icon nav-icon-relief" />
      {#if !collapsed}
        <span class="nav-label">Dashboard</span>
      {/if}
      {#if isActive('/')}
        <div class="active-dot"></div>
      {/if}
    </a>

    <!-- Graph Explorer -->
    <a
      href="/graph"
      class="nav-item"
      class:active={isActive('/graph')}
      title={collapsed ? "Graph Explorer" : ""}
    >
      <Network size={20} class="nav-icon nav-icon-relief" />
      {#if !collapsed}
        <span class="nav-label">Graph Explorer</span>
      {/if}
      {#if isActive('/graph')}
        <div class="active-dot"></div>
      {/if}
    </a>

    <!-- Entitas Sejarah Dropdown -->
    <div class="dropdown-container">
      <button
        onclick={handleEntitasClick}
        class="nav-item dropdown-trigger"
        class:active={isActive('/actor') || isActive('/map') || isActive('/timeline') || isActive('/source')}
        title={collapsed ? "Entitas Sejarah" : ""}
      >
        <Landmark size={20} class="nav-icon nav-icon-relief" />
        {#if !collapsed}
          <span class="nav-label">Entitas Sejarah</span>
          <span class="dropdown-arrow" class:rotated={entitasOpen}>▼</span>
        {/if}
        {#if (isActive('/actor') || isActive('/map') || isActive('/timeline') || isActive('/source')) && collapsed}
          <div class="active-dot"></div>
        {/if}
      </button>

      {#if entitasOpen && !collapsed}
        <div class="dropdown-menu" transition:slide={{ duration: 250 }}>
          <a
            href="/actor"
            class="nav-sub-item"
            class:active={isActive('/actor')}
          >
            <Users size={16} class="nav-sub-icon nav-sub-icon-relief" />
            <span class="nav-sub-label">Pelaku Sejarah</span>
          </a>
          <a
            href="/map"
            class="nav-sub-item"
            class:active={isActive('/map')}
          >
            <MapIcon size={16} class="nav-sub-icon nav-sub-icon-relief" />
            <span class="nav-sub-label">Peta Sejarah</span>
          </a>
          <a
            href="/timeline"
            class="nav-sub-item"
            class:active={isActive('/timeline')}
          >
            <Clock size={16} class="nav-sub-icon nav-sub-icon-relief" />
            <span class="nav-sub-label">Timeline</span>
          </a>
          <a
            href="/source"
            class="nav-sub-item"
            class:active={isActive('/source')}
          >
            <BookOpen size={16} class="nav-sub-icon nav-sub-icon-relief" />
            <span class="nav-sub-label">Kitab Rujukan</span>
          </a>
        </div>
      {/if}
    </div>

    <!-- Data Factory Dropdown -->
    {#if auth.isEditor}
      <div class="dropdown-container">
        <button
          onclick={handleFactoryClick}
          class="nav-item dropdown-trigger"
          class:active={isActive('/factory')}
          title={collapsed ? "Data Factory" : ""}
        >
          <Anvil size={20} class="nav-icon nav-icon-relief" />
          {#if !collapsed}
            <span class="nav-label">Data Factory</span>
            <span class="dropdown-arrow" class:rotated={factoryOpen}>▼</span>
          {/if}
          {#if isActive('/factory') && collapsed}
            <div class="active-dot"></div>
          {/if}
        </button>

        {#if factoryOpen && !collapsed}
          <div class="dropdown-menu" transition:slide={{ duration: 250 }}>
            <a
              href="/factory"
              class="nav-sub-item"
              class:active={currentPath === '/factory'}
            >
              <LineChart size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">Dashboard Kurasi</span>
            </a>
            <a
              href="/factory/historian"
              class="nav-sub-item"
              class:active={isActive('/factory/historian')}
            >
              <Bot size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">AI Historian (Med)</span>
            </a>
            <a
              href="/factory/actors"
              class="nav-sub-item"
              class:active={isActive('/factory/actors')}
            >
              <UserCog size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">Manajemen Tokoh</span>
            </a>
            <a
              href="/factory/locations"
              class="nav-sub-item"
              class:active={isActive('/factory/locations')}
            >
              <MapPin size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">Manajemen Lokasi</span>
            </a>
            <a
              href="/factory/sources"
              class="nav-sub-item"
              class:active={isActive('/factory/sources')}
            >
              <Library size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">Manajemen Rujukan</span>
            </a>
            <a
              href="/factory/events"
              class="nav-sub-item"
              class:active={isActive('/factory/events')}
            >
              <ScrollText size={16} class="nav-sub-icon nav-sub-icon-relief" />
              <span class="nav-sub-label">Manajemen Peristiwa</span>
            </a>
            
            <div class="mt-2 pt-2 border-t border-white/5 pl-2 mb-1">
              <span class="text-[9px] font-bold text-red-400 uppercase tracking-widest pl-1">Akun & Sistem</span>
            </div>
            <a
              href="/settings/profile"
              class="nav-sub-item"
              style="color: rgba(248, 113, 113, 0.8);"
              class:active={isActive('/settings/profile')}
            >
              <Shield size={16} class="nav-sub-icon nav-sub-icon-relief" style="color: rgba(248, 113, 113, 0.8);" />
              <span class="nav-sub-label">My Profile</span>
            </a>
            {#if auth.isAdmin}
              <a
                href="/factory/users"
                class="nav-sub-item"
                style="color: rgba(248, 113, 113, 0.8);"
                class:active={isActive('/factory/users')}
              >
                <Users size={16} class="nav-sub-icon nav-sub-icon-relief" style="color: rgba(248, 113, 113, 0.8);" />
                <span class="nav-sub-label">Manajemen User</span>
              </a>
            {/if}
          </div>
        {/if}
      </div>
    {/if}


    <!-- Curator -->
    <a
      href="/curator"
      class="nav-item"
      class:active={isActive('/curator')}
      title={collapsed ? "Curator" : ""}
    >
      <PenTool size={20} class="nav-icon nav-icon-relief" />
      {#if !collapsed}
        <span class="nav-label">Curator</span>
      {/if}
      {#if isActive('/curator')}
        <div class="active-dot"></div>
      {/if}
    </a>
  </nav>

  <!-- Bottom Section -->
  <div class="bottom-section">
    {#if !collapsed}
      <div class="bottom-star">
        <svg width="40" height="40" viewBox="0 0 40 40" fill="none">
          <polygon
            points="20,2 26,14 38,16 29,25 31,38 20,32 9,38 11,25 2,16 14,14"
            stroke="#e4891b"
            stroke-width="0.5"
            fill="none"
          />
          <polygon
            points="20,8 24,16 32,17 26,23 28,32 20,28 12,32 14,23 8,17 16,16"
            stroke="#e4891b"
            stroke-width="0.3"
            fill="none"
          />
        </svg>
      </div>
      <p class="version-text">v0.1.0 — Alpha</p>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    z-index: 50;
    display: flex;
    flex-direction: column;
    border-right: 1px solid rgba(158, 74, 43, 0.15); /* Rust border */
    background: rgba(28, 24, 21, 0.85); /* Iron surface elevated */
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    width: 260px;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .sidebar.collapsed {
    width: 68px;
  }

  .toggle-btn {
    position: absolute;
    top: 28px;
    right: -14px;
    z-index: 60;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: rgba(28, 24, 21, 0.95);
    border: 1px solid rgba(158, 74, 43, 0.25);
    color: #e4891b;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .toggle-btn:hover {
    background: rgba(158, 74, 43, 0.15);
    border-color: #e4891b;
    transform: scale(1.1);
  }

  .toggle-icon {
    transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .toggle-icon.rotated {
    transform: rotate(180deg);
  }

  .logo-area {
    padding: 24px 16px 20px;
    border-bottom: 1px solid rgba(158, 74, 43, 0.15);
    overflow: hidden;
  }

  .logo-link {
    display: flex;
    align-items: center;
    gap: 12px;
    text-decoration: none;
  }

  .logo-glyph {
    font-size: 1.5rem;
    transition: transform 0.3s;
    flex-shrink: 0;
  }

  .logo-link:hover .logo-glyph {
    transform: rotate(12deg);
  }

  .logo-text {
    white-space: nowrap;
    overflow: hidden;
  }

  .logo-title {
    font-size: 1.1rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    background: linear-gradient(135deg, #e4891b, #f5d78e, #e4891b);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .logo-subtitle {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.3);
    letter-spacing: 0.2em;
    text-transform: uppercase;
  }

  .divider {
    display: flex;
    justify-content: center;
    padding: 8px 0;
    opacity: 0.2;
  }

  .nav-area {
    flex: 1;
    padding: 16px 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.5);
    text-decoration: none;
    transition: all 0.2s ease;
    position: relative;
    border-left: 3px solid transparent;
    white-space: nowrap;
    overflow: hidden;
  }

  .nav-item:hover {
    color: rgba(255, 255, 255, 0.8);
    background: rgba(158, 74, 43, 0.05); /* rust */
  }

  .nav-item.active {
    color: #e4891b;
    background: rgba(158, 74, 43, 0.1);
    border-left-color: #e4891b;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: 12px 0;
    border-left: none;
  }

  .nav-icon {
    font-size: 1.1rem;
    flex-shrink: 0;
    transition: transform 0.2s;
  }

  .nav-item:hover .nav-icon {
    transform: scale(1.1);
  }

  .nav-label {
    position: relative;
    z-index: 1;
  }

  .active-dot {
    position: absolute;
    right: 12px;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #e4891b;
    box-shadow: 0 0 8px rgba(228, 137, 27, 0.6);
  }

  .sidebar.collapsed .active-dot {
    right: auto;
    bottom: 4px;
    left: 50%;
    transform: translateX(-50%);
    width: 4px;
    height: 4px;
  }

  .bottom-section {
    padding: 16px 24px;
    border-top: 1px solid rgba(158, 74, 43, 0.15);
  }

  .bottom-star {
    display: flex;
    justify-content: center;
    margin-bottom: 12px;
    opacity: 0.15;
  }

  .version-text {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.2);
    text-align: center;
    letter-spacing: 0.15em;
    text-transform: uppercase;
  }

  /* Dropdown sub-menu styling */
  .dropdown-container {
    display: flex;
    flex-direction: column;
  }

  .dropdown-trigger {
    width: 100%;
    background: transparent;
    border: none;
    cursor: pointer;
    text-align: left;
  }

  .dropdown-arrow {
    margin-left: auto;
    font-size: 8px;
    opacity: 0.5;
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .dropdown-arrow.rotated {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-left: 20px;
    margin-top: 2px;
    margin-bottom: 4px;
    border-left: 1px solid rgba(158, 74, 43, 0.15);
    margin-left: 28px;
  }

  .nav-sub-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 0.75rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.4);
    text-decoration: none;
    transition: all 0.2s ease;
  }

  .nav-sub-item:hover {
    color: rgba(255, 255, 255, 0.8);
    background: rgba(158, 74, 43, 0.05);
  }

  .nav-sub-item.active {
    color: #e4891b;
    background: rgba(158, 74, 43, 0.1);
    font-weight: 600;
  }

  .nav-sub-icon {
    font-size: 0.95rem;
    flex-shrink: 0;
  }

  /* Relief/Engraved Styling for SVG Icons */
  :global(.nav-icon-relief) {
    color: #a3725b; /* Muted rust */
    filter: drop-shadow(1px 1px 0px rgba(255, 255, 255, 0.05)) drop-shadow(-1px -1px 0px rgba(0, 0, 0, 0.9));
    transition: all 0.2s ease;
  }

  .nav-item:hover :global(.nav-icon-relief) {
    color: #d88942;
    filter: drop-shadow(0 0 5px rgba(228, 137, 27, 0.3)) drop-shadow(1px 1px 0px rgba(255, 255, 255, 0.05)) drop-shadow(-1px -1px 0px rgba(0, 0, 0, 0.9));
  }

  .nav-item.active :global(.nav-icon-relief) {
    color: #e4891b;
    filter: drop-shadow(0 0 8px rgba(228, 137, 27, 0.5)) drop-shadow(1px 1px 0px rgba(255, 255, 255, 0.05)) drop-shadow(-1px -1px 0px rgba(0, 0, 0, 0.9));
  }

  :global(.nav-sub-icon-relief) {
    color: rgba(255, 255, 255, 0.25);
    filter: drop-shadow(1px 1px 0px rgba(255, 255, 255, 0.05)) drop-shadow(-1px -1px 0px rgba(0, 0, 0, 0.9));
    transition: all 0.2s ease;
  }

  .nav-sub-item:hover :global(.nav-sub-icon-relief) {
    color: rgba(255, 255, 255, 0.8);
  }

  .nav-sub-item.active :global(.nav-sub-icon-relief) {
    color: #e4891b;
    filter: drop-shadow(0 0 5px rgba(228, 137, 27, 0.4)) drop-shadow(1px 1px 0px rgba(255, 255, 255, 0.05)) drop-shadow(-1px -1px 0px rgba(0, 0, 0, 0.9));
  }
</style>
