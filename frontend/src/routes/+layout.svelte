<script>
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import Topbar from '$lib/components/Topbar.svelte';

  let { children } = $props();
  let sidebarCollapsed = $state(false);
</script>

<div class="app-layout relative">
  <Sidebar bind:collapsed={sidebarCollapsed} />
  <Topbar />
  <main class="main-content" style="margin-left: {sidebarCollapsed ? '68px' : '260px'};">
    {@render children()}
  </main>
</div>

<style>
  .app-layout {
    min-height: 100vh;
    background-color: transparent;
  }

  .main-content {
    margin-left: var(--sidebar-width, 260px);
    padding: 24px 32px;
    overflow-y: auto;
    max-height: 100vh;
    transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  @media (max-width: 768px) {
    .main-content {
      margin-left: 0 !important;
      padding: 16px;
      padding-top: 72px;
    }
  }
</style>
