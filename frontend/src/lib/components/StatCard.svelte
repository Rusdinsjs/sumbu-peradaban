<script lang="ts">
  let {
    icon = '📊',
    label = 'Stat',
    value = '0',
    trend = '',
  }: {
    icon?: string;
    label?: string;
    value?: string | number;
    trend?: string;
  } = $props();

  let mounted = $state(false);

  $effect(() => {
    // Trigger fade-in animation on mount
    const timer = setTimeout(() => { mounted = true; }, 50);
    return () => clearTimeout(timer);
  });

  let trendPositive = $derived(trend.startsWith('+') || trend.startsWith('↑'));
  let trendNegative = $derived(trend.startsWith('-') || trend.startsWith('↓'));
</script>

<div
  class="relative rounded-xl p-5 transition-all duration-300 cursor-default group"
  style="
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid rgba(255, 255, 255, 0.06);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
    opacity: {mounted ? 1 : 0};
    transform: translateY({mounted ? 0 : 12}px);
  "
  onmouseenter={(e) => {
    const el = e.currentTarget as HTMLElement;
    el.style.transform = 'translateY(-2px)';
    el.style.boxShadow = '0 8px 32px rgba(0, 0, 0, 0.3), 0 0 20px rgba(212, 168, 83, 0.05)';
    el.style.borderColor = 'rgba(212, 168, 83, 0.15)';
  }}
  onmouseleave={(e) => {
    const el = e.currentTarget as HTMLElement;
    el.style.transform = 'translateY(0)';
    el.style.boxShadow = '0 4px 16px rgba(0, 0, 0, 0.2)';
    el.style.borderColor = 'rgba(255, 255, 255, 0.06)';
  }}
>
  <!-- Subtle corner decoration -->
  <div
    class="absolute top-0 right-0 w-16 h-16 opacity-5"
    style="background: radial-gradient(circle at top right, #e4891b, transparent 70%);"
  ></div>

  <div class="flex items-start gap-4 relative z-10">
    <!-- Icon -->
    <div
      class="flex-shrink-0 w-12 h-12 rounded-lg flex items-center justify-center text-xl transition-transform duration-300 group-hover:scale-110"
      style="background: rgba(212, 168, 83, 0.08); border: 1px solid rgba(212, 168, 83, 0.12);"
    >
      {icon}
    </div>

    <!-- Stats -->
    <div class="flex-1 min-w-0">
      <p class="text-xs text-white/40 uppercase tracking-wider mb-1 font-medium">{label}</p>
      <p
        class="text-2xl font-bold tracking-tight"
        style="background: linear-gradient(135deg, #e4891b, #f5d78e); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;"
      >
        {typeof value === 'number' ? value.toLocaleString('id-ID') : value}
      </p>

      {#if trend}
        <p
          class="text-xs mt-1 font-medium"
          style="color: {trendPositive ? '#5d8f8a' : trendNegative ? '#ef4444' : '#6b7280'};"
        >
          {trend}
        </p>
      {/if}
    </div>
  </div>
</div>
