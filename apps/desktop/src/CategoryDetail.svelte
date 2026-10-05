<script lang="ts">
  import InfoRows from './InfoRows.svelte';
  import CategoryResults from './CategoryResults.svelte';
  import Matches from './Matches.svelte';
  import Players from './Players.svelte';
  import Draw from './Draw.svelte';
  import CategorySetup from './CategorySetup.svelte';
  import { formatMoney } from './money';
  import type { Tournament, Category } from './api';
  import { messages, type Language } from './i18n';
  export type CategoryTab = 'settings' | 'registrations' | 'groups' | 'draw' | 'matches' | 'results';
  let { active = true, tournament, category, language, tab, ontab, busy = $bindable(false), dirty = $bindable(false) }: {
    active?: boolean; tournament: Tournament; category: Category; language: Language; tab: CategoryTab;
    ontab: (tab: CategoryTab) => void; busy?: boolean; dirty?: boolean;
  } = $props();
  let text = $derived(messages[language]);
  let tabs: CategoryTab[] = $derived(category.format === 'groups_knockout'
    ? ['settings', 'registrations', 'groups', 'draw', 'matches', 'results']
    : ['settings', 'registrations', 'draw', 'matches', 'results']);
</script>
<div class="heading"><div><p class="eyebrow">{tournament.name}</p><h1>{category.name}</h1><InfoRows compact items={[{label:text.discipline,value:text[category.discipline]},{label:text.format,value:text[category.format]},{label:language==='sr'?'Kotizacija':'Entry fee',value:`${formatMoney(category.fee_minor, language)} ${text.feePerEntry}`}]} /></div></div>
{#if category.completed || tournament.completed}<p class="banner">{language==='sr'?'Takmičenje je završeno. Za izmene ga ponovo otvori u Rezultatima / Pregledu turnira.':'Competition completed. Reopen it in Results / Tournament Overview to make changes.'}</p>{/if}
<nav class="category-tabs" aria-label={text.categorySections}>
  {#each tabs as item}<button data-open-tab class:active={tab === item} aria-current={tab === item ? 'page' : undefined} disabled={busy} onclick={() => ontab(item)}>{text[item]}</button>{/each}
</nav>
{#if tab === 'settings'}
  {#key category.id}<CategorySetup {active} {tournament} {category} {language} bind:busy bind:dirty onview={() => ontab('draw')} />{/key}
{:else if tab === 'registrations'}
  {#key category.id}<Players {active} {tournament} {category} {language} bind:busy bind:dirty />{/key}
{:else if tab === 'groups' && category.format === 'groups_knockout'}
  {#key category.id}<Draw view="groups" {active} {tournament} {category} {language} bind:busy bind:dirty onsettings={() => ontab('settings')} />{/key}
{:else if tab === 'draw' || tab === 'groups'}
  {#key category.id}<Draw {active} {tournament} {category} {language} bind:busy bind:dirty onsettings={() => ontab('settings')} />{/key}
{:else if tab === 'matches'}
  {#key category.id}<Matches {active} {tournament} {category} {language} bind:busy bind:dirty onsettings={() => ontab('settings')} />{/key}
{:else}
  {#key category.id}<CategoryResults {active} {tournament} {category} {language} bind:busy ontab={ontab} />{/key}
{/if}
