<script>
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';

  let { children } = $props();
</script>

<div class="app-layout">
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    min-height: 100vh;
    background-color: var(--color-navy);
  }

  .main-content {
    flex: 1;
    margin-left: 260px;
    padding: 24px 32px;
    overflow-y: auto;
    max-height: 100vh;
  }

  @media (max-width: 768px) {
    .main-content {
      margin-left: 0;
      padding: 16px;
      padding-top: 72px;
    }
  }
</style>
