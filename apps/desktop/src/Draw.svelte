<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import KnockoutBracket from './KnockoutBracket.svelte';
  import { bracketSlots } from './draw-view';
  import GroupsGrid from './GroupsGrid.svelte';
  import { getCategoryDraw, getCompetitionState, getCategoryRules, listEntries, desktopAvailable, defaultCategoryRules, type CompetitionState, type ScheduledMatch, type CategoryDraw, type Category, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { view = 'bracket', active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onsettings }: { view?: 'bracket' | 'groups'; active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onsettings: () => void } = $props();
  let text = $derived(messages[language]);
  let draw = $state<CategoryDraw | null>(null);
  let competition = $state<CompetitionState | null>(null);
  let progress = $state<ScheduledMatch[]>([]);
  let rules = $state(defaultCategoryRules());
  let loading = $state(true);
  let error = $state<MessageKey | null>(null);
  let stale = $state(false);
  let layoutRules = $derived(draw && category.format === 'groups_knockout' ? { ...rules, ...draw.settings } : rules);
  let slots = $derived(bracketSlots(draw, layoutRules, category.format, language, competition?.draw_id === draw?.id ? competition?.slots : undefined));
  let missing = $derived(draw ? draw.participants.length - new Set(draw.sections.flat().filter(Boolean)).size : 0);
  $effect(() => { if (view !== 'groups') { busy = false; dirty = false; } });
  async function load() {
    loading = true; error = null;
    try {
      const [stored, configuration, entries, state] = await Promise.all([getCategoryDraw(tournament.id, category.id), getCategoryRules(tournament.id, category.id), listEntries(category.id), getCompetitionState(tournament.id, category.id)]);
      draw = stored; rules = configuration.rules;
      competition = state.draw_id === stored?.id ? state : null;
      progress = competition?.matches ?? [];
      const active = entries.filter(entry => entry.status === 'registered');
      stale = !!stored && (stored.participants.length !== active.length || stored.participants.some(entry => !active.some(current => current.id === entry.id)) || category.format === 'groups_knockout' && (stored.settings.group_count !== rules.group_count || stored.settings.qualifiers_per_group !== rules.qualifiers_per_group));
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => {
    if (desktopAvailable) void load(); else loading = false;
    const updated = (event: Event) => { if ((event as CustomEvent).detail === category.id && !busy && !dirty) void load(); };
    window.addEventListener('librett-results-updated', updated);
    return () => window.removeEventListener('librett-results-updated', updated);
  });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && desktopAvailable) void load(); wasActive = active; });
</script>
<div class="draw-view">
  <div class="draw-toolbar">
    <div class="draw-heading"><h2>{view === 'groups' ? text.groups : text.draw}</h2><p class="muted">{rules.best_of} {language === 'sr' ? 'setova' : 'sets'} · {rules.points_to_win} {language === 'sr' ? 'poena' : 'points'} · +{rules.win_by}</p></div>
    <div class="draw-actions"><button class="icon-button" disabled={loading || busy} onclick={load} aria-label={language === 'sr' ? 'Osveži' : 'Refresh'} title={language === 'sr' ? 'Osveži' : 'Refresh'}><Icon name="restore" size={18} /></button><button data-open-tab class="secondary icon-label" disabled={busy} onclick={onsettings} aria-label={text.settings} title={text.settings}><Icon name="edit" size={16} />{language === 'sr' ? 'Uredi' : 'Edit'}</button></div>
  </div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if !desktopAvailable}<p class="banner">{text.preview}</p>{:else if loading}<p>{text.loading}</p>{:else}
    {#if stale}<p class="banner" role="alert">{language === 'sr' ? 'Žreb je zastareo. Napravi novi u Podešavanjima.' : 'The draw is outdated. Create a new one in Settings.'}</p>{/if}
    {#if missing > 0}<p class="banner">{language === 'sr' ? 'Nepotpun ručni raspored — neraspoređeno' : 'Incomplete manual layout — unassigned'}: {missing}</p>{/if}
    {#if view === 'groups'}
      <GroupsGrid {draw} {category} {tournament} {language} {competition} bind:busy bind:dirty onreload={load} />
    {:else}
      <section class="knockout-column" aria-label={text.bracket}>
        {#if draw}<div class="bracket-tools">
          {#if category.format === 'groups_knockout' && slots.some(slot => slot.kind === 'qualifier')}<span>{language === 'sr' ? slots.some(slot=>slot.lucky_loser && slot.kind==='qualifier') ? 'Čeka prolaznike / Lucky loser izbor.' : 'Čeka prolaznike iz grupa.' : slots.some(slot=>slot.lucky_loser && slot.kind==='qualifier') ? 'Awaiting qualifiers / lucky loser selection.' : 'Awaiting group qualifiers.'}</span>{/if}
          <span class="scroll-hint" title={language === 'sr' ? 'Horizontalni skrol' : 'Scroll horizontally'} aria-label={language === 'sr' ? 'Horizontalni skrol' : 'Scroll horizontally'}><Icon name="arrow-left" size={14} /><Icon name="arrow-right" size={14} /></span>
        </div>{/if}
        <KnockoutBracket {slots} {language} {progress} />
      </section>
    {/if}
  {/if}
</div>
<style>
  .draw-view { display: grid; gap: 20px; min-width: 0; }
  .draw-toolbar, .draw-actions { display: flex; align-items: center; gap: 12px; }
  .draw-toolbar { justify-content: space-between; flex-wrap: wrap; }
  .draw-toolbar h2 { margin: 0; }
  .draw-heading { display: flex; align-items: baseline; gap: 12px; flex-wrap: wrap; }
  .draw-toolbar p { margin: 0; font-size: 11px; }
  .knockout-column { min-width: 0; }
  .bracket-tools { display: flex; align-items: center; gap: 12px; color: var(--text-muted); font-size: 11px; }
  .scroll-hint { margin-left: auto; display: flex; align-items: center; gap: 3px; color: var(--text-muted); font-size: 10px; }
</style>
