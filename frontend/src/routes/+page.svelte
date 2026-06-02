<script lang="ts">
  import SearchBar from '$lib/components/SearchBar.svelte';
  import StatCard from '$lib/components/StatCard.svelte';
  import CurationBadge from '$lib/components/CurationBadge.svelte';
  import GraphExplorer from '$lib/components/GraphExplorer.svelte';
  import { onMount } from 'svelte';

  let { data } = $props<{ data: any }>();
  
  let visible = $state(false);
  let heroVisible = $state(false);
  let statsVisible = $state(false);
  let activityVisible = $state(false);
  let graphVisible = $state(false);

  let dbData = $derived(data.dashboardData || { events: [], actors: [], locations: [], sources: [] });

  let stats = $derived([
    { icon: '📜', label: 'Total Peristiwa', value: dbData.events ? dbData.events.length.toString() : '0', trend: 'Live Data' },
    { icon: '👥', label: 'Total Tokoh', value: dbData.actors ? dbData.actors.length.toString() : '0', trend: 'Live Data' },
    { icon: '🌍', label: 'Total Lokasi', value: dbData.locations ? dbData.locations.length.toString() : '0', trend: 'Live Data' },
    { icon: '📚', label: 'Total Sumber', value: dbData.sources ? dbData.sources.length.toString() : '0', trend: 'Live Data' }
  ]);
  
  // Mix real data with some mock time for presentation purposes since audit log isn't fetched yet
  let recentActivity = $derived([
    ...(dbData.events || []).map((e: any, i: number) => ({
      id: `e-${i}`, time: 'Baru saja', action: 'Data Historis', entity: e.title, type: 'event' as const, tier: (e.curationTier?.toLowerCase() || 'draft') as any
    })),
    ...(dbData.actors || []).map((a: any, i: number) => ({
      id: `a-${i}`, time: '1 jam lalu', action: 'Tokoh', entity: a.name, type: 'actor' as const, tier: (a.curationTier?.toLowerCase() || 'draft') as any
    })),
    ...(dbData.locations || []).map((l: any, i: number) => ({
      id: `l-${i}`, time: 'Hari ini', action: 'Lokasi', entity: l.name, type: 'location' as const, tier: (l.curationTier?.toLowerCase() || 'draft') as any
    }))
  ].slice(0, 5));

  // Fallback if no real data
  let finalActivity = $derived(recentActivity.length > 0 ? recentActivity : [
    { id: 'd1', time: '2 menit lalu', action: 'Ditambahkan', entity: 'Perang Badar', type: 'event' as const, tier: 'verified' as any },
    { id: 'd2', time: '15 menit lalu', action: 'Diperbarui', entity: 'Khalid bin Walid', type: 'actor' as const, tier: 'reviewed' as any }
  ]);

  const typeIcons: Record<string, string> = {
    event: '📜',
    actor: '👤',
    location: '📍'
  };

  onMount(() => {
    // Staggered entrance animations
    setTimeout(() => { heroVisible = true; }, 100);
    setTimeout(() => { statsVisible = true; }, 400);
    setTimeout(() => { activityVisible = true; }, 700);
    setTimeout(() => { graphVisible = true; }, 1000);
    visible = true;
  });
</script>

<svelte:head>
  <title>Sumbu Peradaban — Peta Pengetahuan Dunia</title>
  <meta name="description" content="Membaca Dunia dari Sumbu Peradaban. Visualisasi graf pengetahuan sejarah Islam dan dunia." />
</svelte:head>

<div class="dashboard">
  <!-- Hero Section -->
  <section class="hero" class:animate-fade-in={heroVisible}>
    <div class="hero-pattern"></div>
    <div class="hero-content">
      <p class="hero-arabic">محور الحضارة</p>
      <h1 class="hero-title text-gradient-gold">Sumbu Peradaban</h1>
      <p class="hero-subtitle">Membaca Dunia dari Sumbu Peradaban</p>
      <p class="hero-description">
        Visualisasi graf pengetahuan yang menghubungkan peristiwa, tokoh, dan lokasi
        sepanjang sejarah peradaban — dengan sejarah Islam sebagai sumbu utama.
      </p>
    </div>
    <div class="hero-glow"></div>
  </section>

  <!-- Search Section -->
  <section class="search-section" class:animate-fade-in={heroVisible}>
    <SearchBar />
  </section>

  <!-- Stats Row -->
  <section class="stats-grid" class:animate-fade-in={statsVisible}>
    {#each stats as stat}
      <StatCard
        icon={stat.icon}
        label={stat.label}
        value={stat.value}
        trend={stat.trend}
      />
    {/each}
  </section>

  <!-- Recent Activity -->
  <section class="activity-section" class:animate-fade-in={activityVisible}>
    <div class="section-header">
      <h2 class="section-title">Aktivitas Terbaru</h2>
      <span class="section-badge">Live</span>
    </div>
    <div class="glass activity-card">
      {#each finalActivity as item (item.id)}
        <div class="activity-item">
          <span class="activity-icon">{typeIcons[item.type]}</span>
          <div class="activity-info">
            <div class="activity-main">
              <span class="activity-action">{item.action}</span>
              <span class="activity-entity">{item.entity}</span>
              <CurationBadge tier={item.tier} size="sm" />
            </div>
            <span class="activity-time">{item.time}</span>
          </div>
        </div>
      {/each}
    </div>
  </section>

  <!-- Quick Graph Preview -->
  <section class="graph-preview-section" class:animate-fade-in={graphVisible}>
    <div class="section-header">
      <h2 class="section-title">Peta Pengetahuan</h2>
      <a href="/graph" class="section-link gradient-rust">
        Lihat Selengkapnya →
      </a>
    </div>
    <div class="glass graph-preview-card">
      <div class="graph-container">
        <GraphExplorer />
      </div>
    </div>
  </section>
</div>

<style>
  .dashboard {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 32px;
    padding-bottom: 48px;
  }

  /* Hero */
  .hero {
    position: relative;
    text-align: center;
    padding: 64px 24px 48px;
    border-radius: 20px;
    overflow: hidden;
    background: radial-gradient(circle at top, rgba(42, 36, 31, 0.9) 0%, rgba(18, 14, 12, 0.95) 100%);
    border: 1px solid var(--color-border);
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .hero.animate-fade-in {
    opacity: 1;
    transform: translateY(0);
  }

  .hero-pattern {
    position: absolute;
    inset: 0;
    background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23d4a853' fill-opacity='0.04'%3E%3Cpath d='M30 0L60 30L30 60L0 30z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    pointer-events: none;
  }

  .hero-glow {
    position: absolute;
    top: -50%;
    left: 50%;
    transform: translateX(-50%);
    width: 600px;
    height: 600px;
    background: radial-gradient(circle, rgba(212, 168, 83, 0.08) 0%, transparent 70%);
    pointer-events: none;
  }

  .hero-content {
    position: relative;
    z-index: 1;
  }

  .hero-arabic {
    font-family: 'Amiri', serif;
    font-size: 2rem;
    color: rgba(212, 168, 83, 0.4);
    margin-bottom: 8px;
    letter-spacing: 4px;
  }

  .hero-title {
    font-size: 3.5rem;
    font-weight: 800;
    margin-bottom: 12px;
    letter-spacing: -1px;
  }

  .hero-subtitle {
    font-size: 1.25rem;
    color: rgba(255, 255, 255, 0.6);
    margin-bottom: 16px;
    font-style: italic;
  }

  .hero-description {
    font-size: 0.95rem;
    color: rgba(255, 255, 255, 0.45);
    max-width: 600px;
    margin: 0 auto;
    line-height: 1.7;
  }

  /* Search Section */
  .search-section {
    max-width: 640px;
    margin: -16px auto 0;
    width: 100%;
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
    transition-delay: 0.15s;
  }

  .search-section.animate-fade-in {
    opacity: 1;
    transform: translateY(0);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20px;
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .stats-grid.animate-fade-in {
    opacity: 1;
    transform: translateY(0);
  }

  @media (max-width: 1024px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }
  }

  /* Section Headers */
  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.9);
  }

  .section-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    border-radius: 999px;
    background: rgba(16, 185, 129, 0.15);
    color: #5d8f8a;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .section-badge::before {
    content: '';
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #5d8f8a;
    animation: pulse-dot 2s infinite;
  }

  @keyframes pulse-dot {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }

  .section-link {
    font-size: 0.875rem;
    font-weight: 600;
    text-decoration: none;
    padding: 6px 16px;
    border-radius: 8px;
    transition: all 0.2s;
  }

  .section-link:hover {
    transform: translateX(4px);
  }

  /* Activity */
  .activity-section {
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .activity-section.animate-fade-in {
    opacity: 1;
    transform: translateY(0);
  }

  .activity-card {
    padding: 0;
    overflow: hidden;
  }

  .activity-item {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    transition: background 0.2s;
  }

  .activity-item:last-child {
    border-bottom: none;
  }

  .activity-item:hover {
    background: rgba(255, 255, 255, 0.03);
  }

  .activity-icon {
    font-size: 1.25rem;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.05);
    flex-shrink: 0;
  }

  .activity-info {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .activity-main {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .activity-action {
    color: rgba(255, 255, 255, 0.5);
    font-size: 0.875rem;
  }

  .activity-entity {
    color: rgba(255, 255, 255, 0.9);
    font-weight: 600;
    font-size: 0.875rem;
  }

  .activity-time {
    color: rgba(255, 255, 255, 0.3);
    font-size: 0.8rem;
    white-space: nowrap;
  }

  /* Graph Preview */
  .graph-preview-section {
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .graph-preview-section.animate-fade-in {
    opacity: 1;
    transform: translateY(0);
  }

  .graph-preview-card {
    padding: 0;
    overflow: hidden;
  }

  .graph-container {
    height: 300px;
    position: relative;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .hero-title {
      font-size: 2.25rem;
    }

    .hero-arabic {
      font-size: 1.5rem;
    }

    .hero {
      padding: 40px 16px 32px;
    }

    .activity-info {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
