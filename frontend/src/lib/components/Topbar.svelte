<script lang="ts">
  import { auth } from '$lib/stores/auth.svelte';
  import { slide } from 'svelte/transition';
  import { onMount } from 'svelte';
  import { gql } from '$lib/graphql/client';

  let dropdownOpen = $state(false);
  let userAvatar = $state('');

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }

  // Optional: close dropdown when clicking outside
  function handleOutsideClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.profile-menu-container')) {
      dropdownOpen = false;
    }
  }

  function handleLogout() {
    auth.logout();
    dropdownOpen = false;
    window.location.href = '/';
  }

  onMount(async () => {
    document.addEventListener('click', handleOutsideClick);
    
    if (auth.isAuthenticated) {
      try {
        const res = await gql(`query { me { avatarUrl } }`) as any;
        if (res.me && res.me.avatarUrl) {
          userAvatar = res.me.avatarUrl;
        }
      } catch (e) {
        // ignore
      }
    }

    return () => {
      document.removeEventListener('click', handleOutsideClick);
    };
  });
</script>

{#if auth.isAuthenticated}
  <div class="profile-menu-container fixed top-4 right-4 z-50">
    <button 
      onclick={toggleDropdown}
      class="flex items-center gap-3 bg-navy-900/60 backdrop-blur-md border border-border/10 rounded-full pl-2 pr-4 py-1.5 shadow-lg hover:bg-navy-900/80 transition-all group"
    >
      <div class="w-8 h-8 rounded-full bg-navy-950 border border-border/20 overflow-hidden flex items-center justify-center flex-shrink-0">
        {#if userAvatar}
          <img src={userAvatar} alt="Avatar" class="w-full h-full object-cover" />
        {:else}
          <span class="text-sm">👤</span>
        {/if}
      </div>
      <div class="flex flex-col items-start hidden sm:flex">
        <span class="text-xs font-bold text-text-primary leading-tight">{auth.user?.username || 'User'}</span>
        <span class="text-[9px] text-gold-400 uppercase tracking-widest font-bold">{auth.user?.role || 'VISITOR'}</span>
      </div>
      <svg class="w-4 h-4 text-text-secondary group-hover:text-text-primary transition-colors ml-1" class:rotate-180={dropdownOpen} fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    {#if dropdownOpen}
      <div 
        transition:slide={{ duration: 200, axis: 'y' }}
        class="absolute right-0 top-full mt-2 w-48 bg-navy-900/90 backdrop-blur-xl border border-border/10 rounded-xl shadow-2xl py-2 overflow-hidden"
      >
        <div class="px-4 py-2 border-b border-border/10 mb-1 sm:hidden">
          <p class="text-sm font-bold text-text-primary">{auth.user?.username || 'User'}</p>
          <p class="text-xs text-gold-400">{auth.user?.role || 'VISITOR'}</p>
        </div>

        <a 
          href="/settings/profile" 
          onclick={() => dropdownOpen = false}
          class="flex items-center gap-2 px-4 py-2 text-sm text-text-secondary hover:text-text-primary hover:bg-white/5 transition-colors"
        >
          <span>🛡️</span> My Profile
        </a>
        
        {#if auth.isAdmin}
          <a 
            href="/factory/users" 
            onclick={() => dropdownOpen = false}
            class="flex items-center gap-2 px-4 py-2 text-sm text-red-400/80 hover:text-red-400 hover:bg-white/5 transition-colors"
          >
            <span>👥</span> Manajemen User
          </a>
        {/if}

        <div class="h-px bg-border/10 my-1"></div>

        <button 
          onclick={handleLogout}
          class="w-full flex items-center gap-2 px-4 py-2 text-sm text-red-400/80 hover:text-red-400 hover:bg-white/5 transition-colors text-left"
        >
          <span>🚪</span> Logout
        </button>
      </div>
    {/if}
  </div>
{/if}
