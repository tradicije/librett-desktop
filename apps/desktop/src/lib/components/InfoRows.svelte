<script lang="ts">
  let { items, compact = false }: { compact?: boolean; items: { label: string; value: string | number | null | undefined }[] } = $props();
</script>
<span class="info-rows" class:compact>{#each items as item}{#if item.value !== null && item.value !== undefined && item.value !== ''}<span class="info-row"><span class="info-label">{item.label}</span><span class="info-value">{item.value}</span></span>{/if}{/each}</span>
<style>
.info-rows{display:grid;gap:6px;margin-top:8px;font-size:12px;line-height:1.5;}.info-row{display:grid;grid-template-columns:minmax(80px,auto) minmax(0,1fr);gap:16px;align-items:baseline;}.info-label{color:var(--text-muted);font-weight:400;}.info-value{color:var(--text-secondary);font-weight:400;overflow-wrap:anywhere;}
.compact{display:flex;flex-wrap:wrap;gap:8px 22px;}.compact .info-row{display:inline-flex;gap:6px;}.compact .info-label::after{content:":";}
</style>
