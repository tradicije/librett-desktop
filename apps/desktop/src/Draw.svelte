<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import KnockoutBracket from './KnockoutBracket.svelte';
  import { bracketSlots, groupName, roundRobin } from './draw-view';
  import { getCategoryDraw, getCategoryRules, listEntries, desktopAvailable, defaultCategoryRules, type CategoryDraw, type Category, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onsettings }: { active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onsettings: () => void } = $props();
  let text = $derived(messages[language]);
  let draw = $state<CategoryDraw | null>(null);
  let rules = $state(defaultCategoryRules());
  let loading = $state(true);
  let error = $state<MessageKey | null>(null);
  let stale = $state(false);
  let layoutRules = $derived(draw && category.format === 'groups_knockout' ? { ...rules, ...draw.settings } : rules);
  let slots = $derived(bracketSlots(draw, layoutRules, category.format, language));
  let groups = $derived(draw?.sections ?? []);
  let missing = $derived(draw ? draw.participants.length - new Set(draw.sections.flat().filter(Boolean)).size : 0);
  $effect(() => { busy = false; dirty = false; });
  function entryName(id: string | null) { return draw?.participants.find(entry => entry.id === id)?.members.map(member => member.name).join(' / ') ?? (language === 'sr' ? 'Neraspoređeno' : 'Unassigned'); }
  function clubName(id: string | null) { return [...new Set(draw?.participants.find(entry => entry.id === id)?.members.map(member => member.club).filter(Boolean))].join(' / '); }
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
    <div><h2>{text.draw}</h2><p class="muted">{category.format === 'groups_knockout' ? (language === 'sr' ? 'Round robin grupe → nokaut' : 'Round-robin groups → knockout') : text.knockout} · {rules.best_of} {language === 'sr' ? 'setova' : 'sets'} · {rules.points_to_win} {language === 'sr' ? 'poena' : 'points'} · +{rules.win_by}</p></div>
    <div class="draw-actions"><button class="secondary" disabled={loading} onclick={load}><Icon name="restore" size={16} />{language === 'sr' ? 'Osveži' : 'Refresh'}</button><button data-open-tab class="secondary" onclick={onsettings}><Icon name="edit" size={16} />{text.settings}</button></div>
  </div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if !desktopAvailable}<p class="banner">{text.preview}</p>{:else if loading}<p>{text.loading}</p>{:else}
    {#if stale}<p class="banner" role="alert">{language === 'sr' ? 'Prijave ili pravila su promenjeni. Prikazan je poslednji sačuvan raspored; napravi novi u podešavanjima.' : 'Registrations or rules changed. This is the last saved layout; create a new one in settings.'}</p>{/if}
    {#if !draw}<p class="banner">{language === 'sr' ? 'Raspored još nije sačuvan. Dodaj učesnike, poređaj nosioce i pripremi raspored u podešavanjima kategorije.' : 'No arrangement has been saved. Add participants, order seeds and prepare the layout in category settings.'}</p>{/if}
    {#if missing > 0}<p class="banner">{language === 'sr' ? 'Nepotpun ručni raspored — neraspoređeno' : 'Incomplete manual layout — unassigned'}: {missing}</p>{/if}
    <div class="draw-layout" class:knockout-only={category.format === 'knockout'}>
      {#if category.format === 'groups_knockout'}
        <section class="group-column" aria-label={text.groups}>
          <div class="column-heading"><h3>{text.groups}</h3><span class="pill">{draw ? groups.length : rules.group_count}</span></div>
          {#if draw}
            {#each groups as group, index}
              <section class="group-card">
                <div class="group-heading"><h3>{language === 'sr' ? 'Grupa' : 'Group'} {groupName(index)}</h3><span>{group.filter(Boolean).length} / {group.length}</span></div>
                <ol class="group-entries">{#each group as id}<li><span class="entry-position">{#if id && draw.seeds.includes(id)}<span class="group-seed">#{draw.seeds.indexOf(id) + 1}</span>{:else}<Icon name={category.discipline === 'doubles' ? 'users' : 'user'} size={15} />{/if}</span><span class="group-player" title={entryName(id)}>{entryName(id)}{#if clubName(id)}<small>{clubName(id)}</small>{/if}</span></li>{/each}</ol>
                <div class="group-qualifiers"><Icon name="arrow-right" size={14} />{draw.settings.qualifiers_per_group} {language === 'sr' ? 'prolaze u nokaut' : 'advance to knockout'}</div>
                {#if group.every(id => id !== null)}
                  <details><summary>{language === 'sr' ? 'Ko sa kim igra' : 'Round-robin pairings'}</summary><div class="round-robin">{#each roundRobin(group.filter((id): id is string => id !== null)) as round, roundIndex}<h4>{language === 'sr' ? 'Kolo' : 'Round'} {roundIndex + 1}</h4>{#each round as [a, b]}<p><span>{entryName(a)}</span><span class="versus">vs</span><span>{entryName(b)}</span></p>{/each}{/each}</div></details>
                {/if}
              </section>
            {/each}
          {:else}<div class="group-empty"><Icon name="users" size={28} /><p>{language === 'sr' ? 'Grupe će se prikazati posle pripreme rasporeda.' : 'Groups will appear after the layout is prepared.'}</p></div>{/if}
        </section>
      {/if}
      <section class="knockout-column" aria-label={text.bracket}>
        <div class="column-heading"><h3>{text.bracket}</h3><span class="scroll-hint"><Icon name="arrow-left" size={14} /><Icon name="arrow-right" size={14} />{language === 'sr' ? 'Pomeri horizontalno' : 'Scroll horizontally'}</span></div>
        {#if category.format === 'groups_knockout'}<p class="projection-note">{language === 'sr' ? 'Pregled kostura po mestima u grupama. Igrači se određuju posle rezultata grupa.' : 'Bracket preview by group positions. Players are determined after group results.'}</p>{/if}
        <KnockoutBracket {slots} {language} />
      </section>
    </div>
  {/if}
</div>
<style>
  .draw-view { display: grid; gap: 20px; min-width: 0; }
  .draw-toolbar, .draw-actions { display: flex; align-items: center; gap: 12px; }
  .draw-toolbar { justify-content: space-between; flex-wrap: wrap; }
  .draw-toolbar h2 { margin: 0; }
  .draw-toolbar p { margin: 6px 0 0; }
  .draw-layout { display: grid; grid-template-columns: 260px minmax(0, 1fr); gap: 24px; align-items: start; }
  .draw-layout.knockout-only { grid-template-columns: minmax(0, 1fr); }
  .group-column, .knockout-column { min-width: 0; }
  .column-heading { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 14px; }
  .column-heading h3 { margin: 0; font-size: 13px; }
  .group-card { border: 1px solid var(--border); border-radius: 8px; margin-bottom: 16px; background: var(--surface); overflow: hidden; }
  .group-heading { display: flex; justify-content: space-between; align-items: center; padding: 12px 14px; border-bottom: 1px solid var(--border-subtle); }
  .group-heading h3 { margin: 0; font-size: 12px; }
  .group-heading > span { color: var(--text-muted); font-size: 11px; }
  .group-entries { padding: 0; margin: 0; list-style: none; }
  .group-entries li { display: flex; gap: 10px; align-items: center; padding: 10px 14px; border-bottom: 1px solid var(--border-subtle); }
  .entry-position { width: 22px; flex-shrink: 0; color: var(--text-muted); }
  .group-seed { color: var(--primary); font-weight: 650; font-size: 10px; }
  .group-player { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 550; }
  small { display: block; font-size: 10px; font-weight: 400; color: var(--text-muted); }
  .group-qualifiers { display: flex; align-items: center; gap: 6px; padding: 10px 14px; color: var(--text-secondary); font-size: 10px; }
  details { border-top: 1px solid var(--border-subtle); padding: 10px 14px; font-size: 11px; }
  summary { cursor: pointer; color: var(--text-secondary); }
  .round-robin h4 { font-size: 10px; color: var(--text-muted); margin: 14px 0 6px; }
  .round-robin p { display: grid; grid-template-columns: minmax(0, 1fr) 16px minmax(0, 1fr); gap: 4px; font-size: 10px; }
  .round-robin p span { overflow-wrap: anywhere; }
  .versus { color: var(--text-muted); text-align: center; }
  .projection-note { font-size: 11px; color: var(--text-muted); margin: 0 0 12px; }
  .scroll-hint { display: flex; align-items: center; gap: 3px; color: var(--text-muted); font-size: 10px; }
  .group-empty { display: grid; place-items: center; color: var(--text-muted); padding: 32px 16px; border: 1px dashed var(--border); border-radius: 8px; text-align: center; }
  @media (max-width: 650px) { .draw-layout { grid-template-columns: 1fr; } }
</style>
