<script lang="ts">
  import { onMount, untrack, tick } from 'svelte';
  import Select from './Select.svelte';
  import Icon from './Icon.svelte';
  import { confirmDiscard } from './confirmation';
  import CategoryRulesFields from './CategoryRulesFields.svelte';
  import { createCategoryWithRules, updateCategoryWithRules, getCategoryEditorState, desktopAvailable, defaultCategoryRules, type Category, type Tournament, type Discipline, type CompetitionFormat, type CategoryRules } from './api';
  import { parseMoney } from './money';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onsaved, oncancel }: { tournament: Tournament; category?: Category; language: Language; busy?: boolean; dirty?: boolean; onsaved: (tournament: Tournament, categoryId: string) => void; oncancel: () => void } = $props();
  let text = $derived(messages[language]);
  const categoryId = untrack(() => category?.id ?? crypto.randomUUID());
  let name = $state(untrack(() => category?.name ?? ''));
  let fee = $state(untrack(() => category ? (category.fee_minor / 100).toFixed(2).replace('.', language === 'sr' ? ',' : '.') : '0'));
  let discipline = $state<Discipline>(untrack(() => category?.discipline ?? 'singles'));
  let format = $state<CompetitionFormat>(untrack(() => category?.format ?? 'groups_knockout'));
  let rules = $state(defaultCategoryRules());
  let revision = $state(0);
  let used = $state(false);
  let loading = $state(!!untrack(() => category));
  let loaded = $state(!untrack(() => category));
  let action = $state(false);
  let error = $state<MessageKey | null>(null);
  const snapshot = () => JSON.stringify({ name, fee, discipline, format, rules });
  let baseline = $state(snapshot());
  type Request = { name: string; fee: number; discipline: Discipline; format: CompetitionFormat; rules: CategoryRules; revision: number };
  let pending = $state<Request | null>(null);
  let impact=$state(false);let impactDialog:HTMLDialogElement;const uid=$props.id();
  $effect(() => { dirty = loaded && snapshot() !== baseline; });
  async function load() {
    if (!category) return;
    loading = true; error = null;
    try {
      const state = await getCategoryEditorState(tournament.id, category.id);
      name = state.category.name; fee = (state.category.fee_minor / 100).toFixed(2).replace('.', language === 'sr' ? ',' : '.');
      discipline = state.category.discipline; format = state.category.format;
      rules = structuredClone(state.configuration.rules); revision = state.configuration.revision; used = state.used;
      baseline = snapshot(); loaded = true;
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  async function reload() { if (!busy && (!dirty || await confirmDiscard())) await load(); }
  onMount(() => { if (desktopAvailable && category) void load(); });
  async function save(event?: SubmitEvent, invalidateDownstream=false) {
    event?.preventDefault(); if (action || !loaded) return;
    if (!pending) {
      const amount = parseMoney(fee, true);
      if (amount === null) { error = 'invalid_category_fee'; return; }
      pending = { name, fee: amount, discipline, format, rules: JSON.parse(JSON.stringify(rules)), revision };
    }
    action = true; busy = true; error = null;
    try {
      const request = pending;
      const result = category
        ? await updateCategoryWithRules(tournament.id, categoryId, request.name, request.discipline, request.format, request.fee, request.rules, request.revision, invalidateDownstream)
        : await createCategoryWithRules(tournament.id, categoryId, request.name, request.discipline, request.format, request.fee, request.rules);
      pending = null; baseline = snapshot(); dirty = false; busy = false; await tick(); onsaved(result, categoryId);
    } catch (cause) {
      if(cause==='result_impact'){impact=true;impactDialog.showModal();}else error = errorKey(cause);
      if (typeof cause === 'string' && ['invalid_rules', 'invalid_cash', 'draw_conflict', 'not_found', 'name_required', 'name_too_long', 'duplicate_category'].includes(cause)) pending = null;
    } finally { action = false; busy = pending !== null; }
  }
</script>
<div class="heading"><div><p class="eyebrow">{tournament.name}</p><h1>{category ? text.editCategory : text.addCategory}</h1></div></div>
{#if error}<p class="error" role="alert">{text[error]}{#if !loaded || error === 'draw_conflict'}<button disabled={busy || loading} onclick={reload}>{text.retry}</button>{/if}</p>{/if}
{#if loading}<p>{text.loading}</p>{:else}
  <form class="panel form-panel category-editor" onsubmit={event=>save(event)}>
    <fieldset class="form-group"><legend>{language === 'sr' ? 'Osnovni podaci' : 'Category details'}</legend><div class="form-fields">
      <label>{text.categoryName}<input bind:value={name} required maxlength="120" disabled={busy || !loaded} /></label>
      <label>{text.categoryFee}<input bind:value={fee} inputmode="decimal" required disabled={busy || !loaded} /></label>
      <label>{text.discipline}<Select label={text.discipline} bind:value={discipline} options={[{ value: 'singles', label: text.singles }, { value: 'doubles', label: text.doubles }]} disabled={busy || used || !loaded} /></label>
      <label>{text.format}<Select label={text.format} bind:value={format} options={[{ value: 'groups_knockout', label: text.groups_knockout }, { value: 'knockout', label: text.knockout }]} disabled={busy || used || !loaded} /></label>
    </div>
    {#if used}<p class="muted">{text.categoryFormatLocked}</p>{/if}
    </fieldset>
    <CategoryRulesFields {language} {format} bind:rules disabled={busy || !loaded} />
    <div class="form-actions"><button class="primary" disabled={action || impact || !loaded || !desktopAvailable}><Icon name="check-circle" />{action ? text.saving : pending ? text.retry : category ? text.editCategory : text.addCategory}</button><button type="button" class="secondary" disabled={busy} onclick={oncancel}><Icon name="arrow-left" size={18} />{text.cancelEdit}</button></div>
  </form>
{/if}
<dialog class="confirm-dialog" bind:this={impactDialog} aria-labelledby={`${uid}-impact`} oncancel={event=>{event.preventDefault();impact=false;pending=null;busy=false;impactDialog.close();}}><h2 id={`${uid}-impact`}>{language==='sr'?'Promena prolaznika':'Change qualifiers'}</h2><p>{language==='sr'?'Promena pravila poništiće nokaut rezultate koji zavise od promenjenih učesnika. Istorija ostaje sačuvana.':'Changing rules clears knockout results that depend on changed participants. History is retained.'}</p><div class="dialog-actions"><button class="secondary" onclick={()=>{impact=false;pending=null;busy=false;impactDialog.close();}}>{language==='sr'?'Vrati se':'Back'}</button><button class="primary" onclick={()=>{impact=false;impactDialog.close();void save(undefined,true);}}>{language==='sr'?'Potvrdi promenu':'Confirm change'}</button></div></dialog>
<style>
  .category-editor { display: grid; gap: 24px; max-width: 850px; }
  label { display: grid; gap: 6px; min-width: 0; }
</style>
