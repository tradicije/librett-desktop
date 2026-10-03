<script lang="ts">
  import Icon from './Icon.svelte';
  import { roundRobinRound } from './draw-view';
  import type { Language } from './i18n';
  let { ids, names, language }: { ids: string[]; names: Map<string, string>; language: Language } = $props();
  let round = $state(0); let page = $state(0);
  let count = $derived(Math.max(0, ids.length + ids.length % 2 - 1));
  let pairs = $derived(roundRobinRound(ids, round));
  let pages = $derived(Math.max(1, Math.ceil(pairs.length / 32)));
</script>
<div class="schedule-navigation"><button class="icon-button" disabled={round === 0} aria-label={language === 'sr' ? 'Prethodno kolo' : 'Previous round'} onclick={() => { round--; page = 0; }}><Icon name="arrow-left" size={16} /></button><span>{language === 'sr' ? 'Kolo' : 'Round'} {round + 1} / {count}</span><button class="icon-button" disabled={round + 1 >= count} aria-label={language === 'sr' ? 'Sledeće kolo' : 'Next round'} onclick={() => { round++; page = 0; }}><Icon name="arrow-right" size={16} /></button></div>
{#each pairs.slice(page * 32, (page + 1) * 32) as [a, b]}<p class="pair"><span>{names.get(a)}</span><span class="versus">vs</span><span>{names.get(b)}</span></p>{/each}
{#if pages > 1}<div class="schedule-navigation"><button class="icon-button" disabled={page === 0} aria-label={language === 'sr' ? 'Prethodni parovi' : 'Previous pairs'} onclick={() => page--}><Icon name="arrow-left" size={16} /></button><span>{page + 1} / {pages}</span><button class="icon-button" disabled={page + 1 >= pages} aria-label={language === 'sr' ? 'Sledeći parovi' : 'Next pairs'} onclick={() => page++}><Icon name="arrow-right" size={16} /></button></div>{/if}
<style>
  .schedule-navigation { display: flex; align-items: center; justify-content: space-between; gap: 8px; margin: 12px 0; color: var(--text-muted); font-size: 10px; }
  .pair { display: grid; grid-template-columns: minmax(0, 1fr) 16px minmax(0, 1fr); gap: 4px; font-size: 10px; }
  .pair span { overflow-wrap: anywhere; }
  .versus { color: var(--text-muted); text-align: center; }
</style>
