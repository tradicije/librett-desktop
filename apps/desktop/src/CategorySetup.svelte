<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Players from './Players.svelte';
  import DrawSetup from './DrawSetup.svelte';
  import CategoryRulesFields from './CategoryRulesFields.svelte';
  import { getCategoryRules, saveCategoryRules, defaultCategoryRules, desktopAvailable, type CategoryRules, type Tournament, type Category } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onview }: { active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onview: () => void } = $props();
  let text = $derived(messages[language]);
  let rules = $state(defaultCategoryRules());
  let savedRules = $state(defaultCategoryRules());
  let baseline = $state('');
  let revision = $state(0);
  let loading = $state(true);
  let saving = $state(false);
  let pending = $state<{ rules: CategoryRules; revision: number } | null>(null);
  let error = $state<MessageKey | null>(null);
  let playersBusy = $state(false); let drawBusy = $state(false);
  let playersDirty = $state(false); let drawDirty = $state(false);
  let refreshToken = $state(0);
  let rulesDirty = $derived(baseline !== '' && JSON.stringify(rules) !== baseline);
  $effect(() => { busy = saving || pending !== null || playersBusy || drawBusy; dirty = rulesDirty || playersDirty || drawDirty; });
  async function load() {
    loading = true; error = null;
    try {
      const stored = await getCategoryRules(tournament.id, category.id);
      rules = structuredClone(stored.rules); savedRules = structuredClone(stored.rules);
      revision = stored.revision; baseline = JSON.stringify(stored.rules);
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); else loading = false; });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && !busy && !dirty && desktopAvailable) void load(); wasActive = active; });
  async function save(event: SubmitEvent) {
    event.preventDefault(); if (saving) return;
    if (!pending) pending = { rules: JSON.parse(JSON.stringify(rules)), revision };
    saving = true; error = null;
    try {
      const stored = await saveCategoryRules(tournament.id, category.id, pending.rules, pending.revision);
      rules = structuredClone(stored.rules); savedRules = structuredClone(stored.rules); revision = stored.revision;
      baseline = JSON.stringify(stored.rules); pending = null;
    } catch (cause) {
      error = errorKey(cause);
      if (cause === 'invalid_rules' || cause === 'draw_conflict' || cause === 'not_found') pending = null;
    } finally { saving = false; }
  }
</script>
<div class="category-setup">
  <section class="panel">
    <div class="section-heading"><h2>{text.categoryRules}</h2><button class="secondary" disabled={busy || rulesDirty} onclick={load}>{language === 'sr' ? 'Učitaj pravila' : 'Reload rules'}</button></div>
    {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
    {#if loading}<p>{text.loading}</p>
    {:else if baseline}
      <form onsubmit={save}>
        <CategoryRulesFields {language} format={category.format} bind:rules disabled={busy} />
        <div class="rule-save"><button disabled={saving || playersBusy || drawBusy || (!pending && !rulesDirty)}>{saving ? text.saving : pending ? text.retry : language === 'sr' ? 'Sačuvaj pravila' : 'Save rules'}</button>
          {#if rulesDirty}<span class="muted">{language === 'sr' ? 'Sačuvaj pravila pre pravljenja rasporeda.' : 'Save rules before creating an arrangement.'}</span>{/if}</div>
      </form>
    {/if}
  </section>
  {#if baseline}
    <fieldset class="participants-setup" disabled={saving || pending !== null || drawBusy}><h2>{language === 'sr' ? 'Učesnici kategorije' : 'Category participants'}</h2><Players {active} {tournament} {category} {language} bind:busy={playersBusy} bind:dirty={playersDirty} onentrieschange={() => refreshToken += 1} /></fieldset>
    <DrawSetup {active} {tournament} {category} {language} rules={savedRules} {refreshToken} externalLocked={rulesDirty || saving || playersBusy || pending !== null} bind:busy={drawBusy} bind:dirty={drawDirty} {onview} />
  {/if}
</div>
<style>
  .category-setup { display: grid; gap: 28px; }
  .participants-setup { margin: 0; padding: 0; border: 0; min-width: 0; }
  .rule-save { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; margin-top: 16px; }
</style>
