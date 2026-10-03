<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import KnockoutBracket from './KnockoutBracket.svelte';
  import { bracketSlots } from './draw-view';
  import GroupsGrid from './GroupsGrid.svelte';
  import { getCategoryDraw, getCategoryRules, listEntries, desktopAvailable, defaultCategoryRules, type CategoryDraw, type Category, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { view = 'bracket', active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onsettings }: { view?: 'bracket' | 'groups'; active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onsettings: () => void } = $props();
  let text = $derived(messages[language]);
  let draw = $state<CategoryDraw | null>(null);
  let rules = $state(defaultCategoryRules());
  let loading = $state(true);
  let error = $state<MessageKey | null>(null);
  let stale = $state(false);
  let layoutRules = $derived(draw && category.format === 'groups_knockout' ? { ...rules, ...draw.settings } : rules);
  let slots = $derived(bracketSlots(draw, layoutRules, category.format, language));
  let missing = $derived(draw ? draw.participants.length - new Set(draw.sections.flat().filter(Boolean)).size : 0);
  $effect(() => { busy = false; dirty = false; });
  async function load() {
    loading = true; error = null;
    try {
      const [stored, configuration, entries] = await Promise.all([getCategoryDraw(tournament.id, category.id), getCategoryRules(tournament.id, category.id), listEntries(category.id)]);
      draw = stored; rules = configuration.rules;
      const active = entries.filter(entry => entry.status === 'registered');
      stale = !!stored && (stored.participants.length !== active.length || stored.participants.some(entry => !active.some(current => current.id === entry.id)) || category.format === 'groups_knockout' && (stored.settings.group_count !== rules.group_count || stored.settings.qualifiers_per_group !== rules.qualifiers_per_group));
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); else loading = false; });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && desktopAvailable) void load(); wasActive = active; });
</script>
<div class="draw-view">
  <div class="draw-toolbar">
    <div><h2>{view === 'groups' ? text.groups : text.draw}</h2><p class="muted">{category.format === 'groups_knockout' ? (language === 'sr' ? 'Round robin grupe → nokaut' : 'Round-robin groups → knockout') : text.knockout} · {rules.best_of} {language === 'sr' ? 'setova' : 'sets'} · {rules.points_to_win} {language === 'sr' ? 'poena' : 'points'} · +{rules.win_by}</p></div>
    <div class="draw-actions"><button class="secondary" disabled={loading} onclick={load}><Icon name="restore" size={16} />{language === 'sr' ? 'Osveži' : 'Refresh'}</button><button data-open-tab class="secondary" onclick={onsettings}><Icon name="edit" size={16} />{text.settings}</button></div>
  </div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if !desktopAvailable}<p class="banner">{text.preview}</p>{:else if loading}<p>{text.loading}</p>{:else}
    {#if stale}<p class="banner" role="alert">{language === 'sr' ? 'Prijave ili pravila su promenjeni. Prikazan je poslednji sačuvan raspored; napravi novi u podešavanjima.' : 'Registrations or rules changed. This is the last saved layout; create a new one in settings.'}</p>{/if}
    {#if !draw}<p class="banner">{language === 'sr' ? 'Raspored još nije sačuvan. Dodaj učesnike, poređaj nosioce i pripremi raspored u podešavanjima kategorije.' : 'No arrangement has been saved. Add participants, order seeds and prepare the layout in category settings.'}</p>{/if}
    {#if missing > 0}<p class="banner">{language === 'sr' ? 'Nepotpun ručni raspored — neraspoređeno' : 'Incomplete manual layout — unassigned'}: {missing}</p>{/if}
    {#if view === 'groups'}
      <GroupsGrid {draw} {category} {language} />
    {:else}
      <section class="knockout-column" aria-label={text.bracket}>
        <div class="column-heading"><h3>{text.bracket}</h3><span class="scroll-hint"><Icon name="arrow-left" size={14} /><Icon name="arrow-right" size={14} />{language === 'sr' ? 'Pomeri horizontalno' : 'Scroll horizontally'}</span></div>
        {#if category.format === 'groups_knockout'}<p class="projection-note">{language === 'sr' ? 'Pregled kostura po mestima u grupama. Igrači se određuju posle rezultata grupa.' : 'Bracket preview by group positions. Players are determined after group results.'}</p>{/if}
        <KnockoutBracket {slots} {language} />
      </section>
    {/if}
  {/if}
</div>
<style>
  .draw-view { display: grid; gap: 20px; min-width: 0; }
  .draw-toolbar, .draw-actions { display: flex; align-items: center; gap: 12px; }
  .draw-toolbar { justify-content: space-between; flex-wrap: wrap; }
  .draw-toolbar h2 { margin: 0; }
  .draw-toolbar p { margin: 6px 0 0; }
  .knockout-column { min-width: 0; }
  .column-heading { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 14px; }
  .column-heading h3 { margin: 0; font-size: 13px; }
  .projection-note { font-size: 11px; color: var(--text-muted); margin: 0 0 12px; }
  .scroll-hint { display: flex; align-items: center; gap: 3px; color: var(--text-muted); font-size: 10px; }
</style>
