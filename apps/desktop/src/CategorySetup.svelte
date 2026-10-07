<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import DrawSetup from './DrawSetup.svelte';
  import KnockoutFillSetup from './KnockoutFillSetup.svelte';
  import CategoryRulesFields from './CategoryRulesFields.svelte';
  import { getCategoryRules, saveCategoryRules, defaultCategoryRules, desktopAvailable, type CategoryRules, type Tournament, type Category } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onview }: { active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onview: () => void } = $props();
  let text = $derived(messages[language]);
  let readOnly=$derived(category.completed || tournament.completed);
  let rules = $state(defaultCategoryRules());
  let savedRules = $state(defaultCategoryRules());
  let baseline = $state('');
  let revision = $state(0);
  let loading = $state(true);
  let saving = $state(false);
  let pending = $state<{ rules: CategoryRules; revision: number } | null>(null);
  let error = $state<MessageKey | null>(null);
  let drawBusy = $state(false);
  let drawDirty = $state(false);
  let fillBusy = $state(false);let fillDirty = $state(false);
  let scoringChanged=$derived(savedRules.best_of!==rules.best_of || savedRules.points_to_win!==rules.points_to_win || savedRules.win_by!==rules.win_by);
  let impact = $state(false);let impactDialog:HTMLDialogElement;const uid=$props.id();
  let rulesDirty = $derived(baseline !== '' && JSON.stringify(rules) !== baseline);
  $effect(() => { busy = saving || pending !== null || drawBusy || fillBusy || impact; dirty = rulesDirty || drawDirty || fillDirty; });
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
  async function save(event?: SubmitEvent, invalidateDownstream=false) {
    event?.preventDefault(); if (saving) return;
    if (!pending) pending = { rules: JSON.parse(JSON.stringify(rules)), revision };
    saving = true; error = null;
    try {
      const stored = await saveCategoryRules(tournament.id, category.id, pending.rules, pending.revision, invalidateDownstream);
      rules = structuredClone(stored.rules); savedRules = structuredClone(stored.rules); revision = stored.revision;
      baseline = JSON.stringify(stored.rules); pending = null;
      window.dispatchEvent(new CustomEvent('librett-results-updated',{detail:category.id}));
    } catch (cause) {
      if(cause==='result_impact'){impact=true;impactDialog.showModal();}else error = errorKey(cause);
      if (cause === 'competition_closed' || cause === 'invalid_rules' || cause === 'draw_conflict' || cause === 'not_found') pending = null;
    } finally { saving = false; }
  }
</script>
<div class="panel form-panel category-setup">
  <section class="rules-setup">
    <div class="rules-toolbar"><button class="secondary" disabled={busy || rulesDirty || drawDirty || fillDirty} onclick={load}>{language === 'sr' ? 'Učitaj pravila' : 'Reload rules'}</button></div>
    {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
    {#if loading}<p>{text.loading}</p>
    {:else if baseline}
      <form onsubmit={event=>save(event)}>
        <CategoryRulesFields {language} format={category.format} bind:rules disabled={readOnly || busy || fillDirty || drawDirty} />
        <div class="rule-save"><button class="primary" disabled={(readOnly && !pending) || saving || drawBusy || fillBusy || impact || (!pending && !rulesDirty)}>{saving ? text.saving : pending ? text.retry : language === 'sr' ? 'Sačuvaj pravila' : 'Save rules'}</button>
          {#if rulesDirty}<span class="muted">{language === 'sr' ? 'Sačuvaj pravila pre pravljenja rasporeda.' : 'Save rules before creating an arrangement.'}</span>{/if}</div>
      </form>
    {/if}
  </section>
  {#if baseline}
    <DrawSetup {active} {tournament} {category} {language} rules={savedRules} externalLocked={readOnly || rulesDirty || saving || pending !== null || fillBusy || fillDirty || impact} bind:busy={drawBusy} bind:dirty={drawDirty} {onview} />
    {#if category.format==='groups_knockout' && savedRules.knockout_filling!=='bye'}<KnockoutFillSetup manual={savedRules.knockout_filling==='lucky_loser_manual'} {active} {tournament} {category} {language} rulesRevision={revision} externalLocked={readOnly || rulesDirty || saving || pending!==null || drawBusy || drawDirty || impact} bind:busy={fillBusy} bind:dirty={fillDirty} />{/if}
  {/if}
</div>
<dialog class="confirm-dialog" bind:this={impactDialog} aria-labelledby={`${uid}-impact`} oncancel={event=>{event.preventDefault();impact=false;pending=null;impactDialog.close();}}><h2 id={`${uid}-impact`}>{language==='sr'?'Potvrdi promenu pravila':'Confirm rule changes'}</h2>{#if scoringChanged}<p>{language==='sr'?'Kategorija je već počela. Novi broj setova, poena i potrebna razlika važe za naredne unose ili ispravke rezultata. Već sačuvani rezultati zadržavaju pravila po kojima su odigrani.':'This category has started. The new set count, point target and winning margin apply to subsequent result entries or corrections. Previously saved results retain their original scoring rules.'}</p>{/if}<p>{language==='sr'?'Ako se promene učesnici nokauta, zavisni rezultati biće poništeni. Istorija ostaje sačuvana.':'If knockout participants change, dependent results will be cleared. History is retained.'}</p><div class="dialog-actions"><button class="secondary" onclick={()=>{impact=false;pending=null;impactDialog.close();}}>{language==='sr'?'Vrati se':'Back'}</button><button class="primary" onclick={()=>{impact=false;impactDialog.close();void save(undefined,true);}}>{language==='sr'?'Potvrdi promenu':'Confirm change'}</button></div></dialog>
<style>
  .category-setup { display: grid; gap: 28px; }
  .rules-toolbar { display: flex; justify-content: flex-end; margin-bottom: 12px; }
  .rule-save { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; margin-top: 20px; }
</style>
