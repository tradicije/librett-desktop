<script lang="ts">
  import { playerLabel } from './player-label';
  let { player, label = '' }: { player?: { name: string; club: string } | null; label?: string } = $props();
  let parts = $derived((player ? playerLabel(player) : label).split(/(\([\p{L}\p{N}]{1,3}\))/gu));
</script>
{#each parts as part}{#if /^\([\p{L}\p{N}]{1,3}\)$/u.test(part)}<span class="club-code">{part}</span>{:else}{part}{/if}{/each}
<style>
  .club-code { color: var(--text-muted); font-size: .8em; font-weight: 400; white-space: nowrap; }
</style>
