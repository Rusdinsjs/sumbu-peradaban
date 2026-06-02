<script lang="ts">
  let {
    tier = 'draft',
    size = 'md',
  }: {
    tier?: 'draft' | 'verified' | 'reviewed' | 'canonical';
    size?: 'sm' | 'md' | 'lg';
  } = $props();

  const tierConfig = {
    draft: { icon: '📝', label: 'Draft', color: '#6b7280', bg: 'rgba(107, 114, 128, 0.15)', border: 'rgba(107, 114, 128, 0.3)' },
    verified: { icon: '✓', label: 'Verified', color: '#3b82f6', bg: 'rgba(59, 130, 246, 0.12)', border: 'rgba(59, 130, 246, 0.3)' },
    reviewed: { icon: '⭐', label: 'Reviewed', color: '#e4891b', bg: 'rgba(212, 168, 83, 0.12)', border: 'rgba(212, 168, 83, 0.3)' },
    canonical: { icon: '✅', label: 'Canonical', color: '#5d8f8a', bg: 'rgba(16, 185, 129, 0.12)', border: 'rgba(16, 185, 129, 0.3)' },
  };

  const sizeConfig = {
    sm: { padding: '2px 8px', fontSize: '10px', iconSize: '10px', gap: '3px' },
    md: { padding: '4px 12px', fontSize: '12px', iconSize: '12px', gap: '4px' },
    lg: { padding: '6px 16px', fontSize: '14px', iconSize: '14px', gap: '6px' },
  };

  let config = $derived(tierConfig[tier] || tierConfig.draft);
  let sizes = $derived(sizeConfig[size] || sizeConfig.md);
</script>

<span
  class="inline-flex items-center rounded-full font-medium whitespace-nowrap transition-all duration-200 hover:scale-105"
  style="
    padding: {sizes.padding};
    font-size: {sizes.fontSize};
    color: {config.color};
    background: {config.bg};
    border: 1px solid {config.border};
    gap: {sizes.gap};
  "
>
  <span style="font-size: {sizes.iconSize}; line-height: 1;">{config.icon}</span>
  <span>{config.label}</span>
</span>
