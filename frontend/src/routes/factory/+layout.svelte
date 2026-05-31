<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth.svelte';

  let { children } = $props();

  onMount(() => {
    // If not logged in, go to login
    if (!auth.isAuthenticated) {
      goto('/login');
      return;
    }

    // Only Editor and Admin can access Factory
    if (!auth.isEditor) {
      goto('/'); // redirect visitors away
    }
  });
</script>

{#if auth.isAuthenticated && auth.isEditor}
  <div class="animate-fade-in w-full h-full">
    {@render children()}
  </div>
{/if}
