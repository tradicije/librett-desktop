<script lang="ts">
  import Icon from './Icon.svelte';
  import Players from './Players.svelte';
  import { formatMoney } from './money';
  import type { Tournament, Category } from './api';
  import { messages, type Language } from './i18n';
  export type CategoryTab = 'registrations' | 'draw' | 'matches' | 'results';
  let { tournament, category, language, tab, ontab, busy = $bindable(false) }: {
    tournament: Tournament; category: Category; language: Language; tab: CategoryTab;
    ontab: (tab: CategoryTab) => void; busy?: boolean;
  } = $props();
  let text = $derived(messages[language]);
  const tabs: CategoryTab[] = ['registrations', 'draw', 'matches', 'results'];
</script>
<div class="heading"><div><p class="eyebrow">{tournament.name}</p><h1>{category.name}</h1><p class="muted">{text[category.discipline]} · {text[category.format]} · {formatMoney(category.fee_minor, language)} {text.feePerEntry}</p></div></div>
<nav class="category-tabs" aria-label={text.categorySections}>
  {#each tabs as item}<button class:active={tab === item} aria-current={tab === item ? 'page' : undefined} disabled={busy} onclick={() => ontab(item)}>{text[item]}</button>{/each}
</nav>
{#if tab === 'registrations'}
  {#key category.id}<Players {tournament} {category} {language} bind:busy />{/key}
{:else}
  <section class="panel stage-placeholder"><Icon name={tab === 'matches' ? 'list' : tab === 'results' ? 'trophy' : 'layer-group'} size={26} /><h2>{text[tab]}</h2><p class="muted">{text.stageNotReady}</p><button class="secondary" onclick={() => ontab('registrations')}>{text.registrations}</button></section>
{/if}
