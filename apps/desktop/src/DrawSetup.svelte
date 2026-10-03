<script lang="ts">
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import { onMount, untrack } from 'svelte';
  import Select from './Select.svelte';
  import { confirmDiscard } from './confirmation';
  import Icon from './Icon.svelte';
  import { groupName } from './draw-view';
  import { getCategoryDraw, listEntries, previewCategoryDraw, saveCategoryDraw, desktopAvailable,
    type CategoryRules, type CategoryDraw, type Category, type Tournament, type Entry, type DrawMode } from './api';
  import { messages, type Language } from './i18n';
  let { rules, externalLocked = false, onview, active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false) }: {
    rules: CategoryRules; externalLocked?: boolean; onview: () => void; active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean;
  } = $props();
  const labels = {
    sr: {
      title: 'Nosioci i raspored', intro: 'Poređaj nosioce, napravi raspored i po potrebi ručno odredi ko sa kim igra.',
      automatic: 'Automatski', manual: 'Ručno', groupCount: 'Broj grupa', qualifiers: 'Prolaznika iz svake grupe',
      seeds: 'Nosioci, od najjačeg', addSeed: 'Dodaj nosioca', choose: 'Izaberi prijavu', up: 'Pomeri gore', down: 'Pomeri dole', remove: 'Ukloni',
      generate: 'Napravi raspored', replace: 'Zameni trenutni nacrt', replaceHint: 'Novi raspored će zameniti trenutni nacrt. Sačuvana prethodna verzija ostaje u istoriji.',
      cancel: 'Odustani', save: 'Sačuvaj nacrt', saved: 'Nacrt sačuvan.', unsaved: 'Nesačuvane izmene', revision: 'Verzija',
      group: 'Grupa', pair: 'Par', position: 'Pozicija', empty: 'Prazno', bye: 'Slobodan prolaz (bye)', missing: 'Neraspoređeno',
      min: 'Za žreb su potrebne 2–4096 aktivne prijave.', invalid: 'Proveri grupe, prolaznike i raspored. Svaka prijava može biti raspoređena samo jednom.',
      stale: 'Prijave ili pravila su promenjeni. Napravi novi raspored pre čuvanja.', conflict: 'Druga verzija je već sačuvana. Učitaj trenutno stanje.',
      retry: 'Ponovi čuvanje', uncertain: 'Čuvanje nije potvrđeno. Ponovi isti upis pre nastavka.', load: 'Učitaj trenutno stanje',
      hint: 'Izbor već raspoređene prijave menja njeno mesto sa ovom pozicijom. Prazna pozicija je bye tek kada je ceo kostur popunjen.',
    },
    en: {
      title: 'Seeds and arrangement', intro: 'Order seeds, create a layout and adjust group membership or first-round opponents manually.',
      automatic: 'Automatic', manual: 'Manual', groupCount: 'Number of groups', qualifiers: 'Qualifiers per group',
      seeds: 'Seeds, strongest first', addSeed: 'Add seed', choose: 'Choose an entry', up: 'Move up', down: 'Move down', remove: 'Remove',
      generate: 'Create arrangement', replace: 'Replace current draft', replaceHint: 'The new arrangement replaces this draft. The previous saved revision stays in history.',
      cancel: 'Cancel', save: 'Save draft', saved: 'Draft saved.', unsaved: 'Unsaved changes', revision: 'Revision',
      group: 'Group', pair: 'Pair', position: 'Position', empty: 'Empty', bye: 'Bye', missing: 'Unassigned',
      min: 'A draw requires 2–4096 active entries.', invalid: 'Check groups, qualifiers and positions. Each entry can be placed only once.',
      stale: 'Registrations or rules changed. Create a new arrangement before saving.', conflict: 'Another revision has already been saved. Reload the current state.',
      retry: 'Retry saving', uncertain: 'Saving was not confirmed. Retry the same write before continuing.', load: 'Reload current state',
      hint: 'Selecting an entry already placed swaps it with this position. An empty position is a bye only when the whole bracket is filled.',
    },
  };
  let t = $derived(labels[language]);
  let text = $derived(messages[language]);
  let entries = $state<Entry[]>([]);
  let draft = $state<CategoryDraw | null>(null);
  let revision = $state(0);
  let seeds = $state<string[]>([]);
  let newSeed = $state('');
  let groupCount = $derived(rules.group_count);
  let qualifiers = $derived(rules.qualifiers_per_group);
  let mode = $state<DrawMode>('automatic');
  let replacing = $state(false);
  let saved = $state(false);
  let loading = $state(true);
  let action = $state(false);
  let error = $state<'invalid' | 'conflict' | 'stale' | 'uncertain' | 'error' | null>(null);
  let pending = $state<{ draw: CategoryDraw; revision: number } | null>(null);
  let stale = $derived(!!draft && (category.format === 'groups_knockout' && (draft.settings.group_count !== groupCount || draft.settings.qualifiers_per_group !== qualifiers) || draft.participants.length !== entries.length || draft.participants.some(e => !entries.some(active => active.id === e.id))));
  let placed = $derived(new Set(draft?.sections.flat().filter((id): id is string => id !== null) ?? []));
  let missing = $derived((draft?.participants.length ?? 0) - placed.size);
  let entryOptions = $derived.by(() => { const current = draft; return current?.participants.map(entry => ({ value: entry.id, label: `${current.seeds.includes(entry.id) ? `#${current.seeds.indexOf(entry.id) + 1} ` : ''}${label(entry)}` })) ?? []; });
  let emptyOptions = $derived([{ value: '', label: t.empty }, ...entryOptions]);
  let byeOptions = $derived([{ value: '', label: t.bye }, ...entryOptions]);
  let locked = $derived(action || loading || pending !== null || externalLocked);
  function label(entry: Entry) { return entry.members.map(playerLabel).join(' / '); }
  function entryLabel(id: string) { const entry = draft?.participants.find(e => e.id === id) ?? entries.find(e => e.id === id); return entry ? label(entry) : id; }
  function changed() { dirty = true; saved = false; replacing = false; }
  async function load() {
    loading = true; error = null;
    try {
      const [all, stored] = await Promise.all([listEntries(category.id), getCategoryDraw(tournament.id, category.id)]);
      entries = all.filter(e => e.status === 'registered'); draft = stored; revision = stored?.revision ?? 0;
      seeds = stored?.seeds.filter(id => entries.some(e => e.id === id)) ?? [];
      mode = stored?.mode ?? 'automatic'; dirty = false; saved = false; replacing = false;
    } catch { error = 'error'; }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); else loading = false; });
  let wasActive = untrack(() => active);
  $effect(() => {
    if (active && !wasActive && desktopAvailable && !busy) void refresh();
    wasActive = active;
  });
  async function refresh() {
    loading = true;
    try {
      const [all, stored] = await Promise.all([listEntries(category.id), getCategoryDraw(tournament.id, category.id)]);
      entries = all.filter(entry => entry.status === 'registered');
      seeds = seeds.filter(id => entries.some(entry => entry.id === id));
      if (!dirty && stored && stored.revision !== revision) {
        draft = stored; revision = stored.revision; seeds = [...stored.seeds];
        mode = stored.mode;
      }
    } catch { error = 'error'; }
    finally { loading = false; }
  }
  function editSeeds(next: string[]) {
    seeds = next;
    if (draft) { draft.seeds = [...next]; draft.mode = 'manual'; }
    changed();
  }
  function moveSeed(index: number, delta: number) {
    const next = [...seeds]; [next[index], next[index + delta]] = [next[index + delta], next[index]]; editSeeds(next);
  }
  async function generate() {
    if (draft && !replacing) { replacing = true; return; }
    action = true; busy = true; error = null;
    try {
      draft = await previewCategoryDraw(tournament.id, category.id, mode,
        { group_count: groupCount, qualifiers_per_group: qualifiers }, seeds);
      changed();
    } catch (cause) { error = cause === 'invalid_draw' || cause === 'invalid_rules' ? 'invalid' : 'error'; }
    finally { action = false; busy = pending !== null; }
  }
  function assign(section: number, slot: number, value: string) {
    if (!draft || locked || stale) return;
    const id = value || null;
    const previous = draft.sections[section][slot];
    const next = draft.sections.map(slots => [...slots]);
    if (id !== null) {
      for (let group = 0; group < next.length; group++) {
        const position = next[group].indexOf(id);
        if (position !== -1) next[group][position] = previous;
      }
    }
    next[section][slot] = id; draft.sections = next; draft.mode = 'manual'; changed();
  }
  async function save() {
    if (!draft || action) return;
    if (!pending) pending = { draw: JSON.parse(JSON.stringify({ ...draft, id: crypto.randomUUID(), revision: 0 })) as CategoryDraw, revision };
    action = true; busy = true; error = null;
    try {
      const result = await saveCategoryDraw(tournament.id, pending.draw, pending.revision);
      draft = result; revision = result.revision; pending = null; dirty = false; saved = true;
    } catch (cause) {
      if (cause === 'invalid_draw' || cause === 'invalid_rules' || cause === 'draw_conflict' || cause === 'not_found') {
        error = cause === 'draw_conflict' ? 'conflict' : cause === 'invalid_draw' ? 'invalid' : 'stale'; pending = null;
      } else error = 'uncertain';
    } finally { action = false; busy = pending !== null; }
  }
</script>

<section class="draw-workspace">
  <div class="section-heading arrangement-heading"><div><h2>{t.title}</h2><p class="muted">{t.intro}</p></div><button class="secondary icon-label" disabled={locked} onclick={async () => { if (!dirty || await confirmDiscard()) await load(); }}><Icon name="restore" size={16} />{t.load}</button></div>
  {#if !desktopAvailable}<p class="muted">{text.preview}</p>
  {:else if loading}<p role="status">{text.loading}</p>
  {:else}
    {#if error}<p class="error" role="alert">{error === 'error' ? text.error : t[error]}</p>{/if}
    {#if saved}<p class="notice" role="status">{t.saved}</p>{/if}
    {#if stale}<p class="banner" role="alert">{t.stale}</p>{/if}
    {#if entries.length < 2 || entries.length > 4096}<p class="muted">{t.min}</p>
    {:else}
      <fieldset class="arrangement-options" disabled={locked}>
        <legend>{language === 'sr' ? 'Način raspoređivanja' : 'Arrangement method'}</legend>
        <div class="method-field"><Select label={language === 'sr' ? 'Način raspoređivanja' : 'Arrangement method'} bind:value={mode} options={[{ value: 'automatic', label: t.automatic }, { value: 'manual', label: t.manual }]} disabled={locked} /></div>
        <div class="seed-heading"><h3>{t.seeds}</h3><span class="pill">{seeds.length}</span></div>
        {#if !seeds.length}<p class="field-hint">{language === 'sr' ? 'Dodaj nosioce redom, od najjačeg. Ostali učesnici biće raspoređeni bez statusa nosioca.' : 'Add seeds in order, strongest first. Other entries will be placed without seed status.'}</p>{/if}
        <ol class="seed-list">
          {#each seeds as id, index (id)}
            <li><span class="seed-rank">{index + 1}</span><span class="seed-name"><PlayerName label={entryLabel(id)} /></span><div class="seed-actions">
              <button class="icon-button" disabled={locked || index === 0} aria-label={`${t.up}: ${entryLabel(id)}`} title={t.up} onclick={() => moveSeed(index, -1)}><Icon name="arrow-up" size={16} /></button>
              <button class="icon-button" disabled={locked || index === seeds.length - 1} aria-label={`${t.down}: ${entryLabel(id)}`} title={t.down} onclick={() => moveSeed(index, 1)}><Icon name="arrow-down" size={16} /></button>
              <button class="secondary icon-label" disabled={locked} aria-label={`${t.remove}: ${entryLabel(id)}`} onclick={() => editSeeds(seeds.filter(seed => seed !== id))}><Icon name="trash" size={16} />{t.remove}</button>
            </div></li>
          {/each}
        </ol>
        <div class="seed-add"><label>{t.addSeed}<Select playerLabels label={t.addSeed} bind:value={newSeed} options={[{ value: '', label: t.choose }, ...entries.filter(e => !seeds.includes(e.id)).map(entry => ({ value: entry.id, label: label(entry) }))]} disabled={locked} /></label><button class="secondary icon-label" disabled={locked || !newSeed} onclick={() => { editSeeds([...seeds, newSeed]); newSeed = ''; }}><Icon name="plus" size={18} />{t.addSeed}</button></div>
        {#if replacing}<p class="banner">{t.replaceHint}</p>{/if}
        <div class="form-actions"><button class="primary" disabled={locked} onclick={generate}><Icon name="layer-group" size={18} />{replacing ? t.replace : t.generate}</button>{#if replacing}<button class="secondary" disabled={locked} onclick={() => replacing = false}>{t.cancel}</button>{/if}</div>
      </fieldset>
    {/if}
    {#if draft}
      <section class="layout-editor">
        <div class="section-heading layout-heading"><h3>{language === 'sr' ? 'Raspored učesnika' : 'Entry arrangement'}</h3><div class="layout-status"><span class="pill">{t.revision}: {revision}</span><span class="pill">{dirty ? t.unsaved : t.saved}</span><span class="pill">{t.missing}: {missing}</span></div></div>
        {#if draft.format === 'groups_knockout'}<p class="field-hint">{t.groupCount}: {draft.settings.group_count} · {t.qualifiers}: {draft.settings.qualifiers_per_group}</p>{/if}
        <p class="field-hint">{t.hint}</p>
        <div class="draw-sections" class:knockout-layout={category.format === 'knockout'}>
          {#each draft.sections as section, group}
            <section class="draw-section">
              <h3>{category.format === 'groups_knockout' ? `${t.group} ${groupName(group)}` : text.bracket}</h3>
              {#each section as id, slot}
                <label class:pair-start={category.format === 'knockout' && slot % 2 === 0}>
                  <span>{category.format === 'knockout' ? `${t.pair} ${Math.floor(slot / 2) + 1} · ${t.position} ${slot % 2 + 1}` : `${t.position} ${slot + 1}`}</span>
                  <Select playerLabels label={`${category.format === 'groups_knockout' ? `${t.group} ${groupName(group)}` : text.bracket} · ${t.position} ${slot + 1}`} bind:value={() => id ?? '', value => assign(group, slot, value)} disabled={locked || stale} options={category.format === 'knockout' && missing === 0 && section[slot ^ 1] ? byeOptions : emptyOptions} />
                </label>
              {/each}
            </section>
          {/each}
        </div>
        <div class="form-actions layout-actions"><button class="primary" disabled={externalLocked || action || loading || (!pending && (stale || !dirty))} onclick={save}><Icon name="check-circle" size={18} />{action ? text.saving : pending ? t.retry : t.save}</button><button class="secondary icon-label" disabled={busy || dirty} onclick={onview}><Icon name="arrow-right" size={18} />{text.draw}</button></div>
      </section>
    {/if}
  {/if}
</section>

<style>
  .draw-workspace { display: grid; gap: 24px; border-top: 1px solid var(--border-subtle); padding-top: 28px; }
  .arrangement-heading { margin: 0; align-items: flex-start; }
  .arrangement-heading p { margin: 8px 0 0; }
  .arrangement-heading button { flex-shrink: 0; }
  fieldset { margin: 0; padding: 0; border: 0; min-width: 0; }
  legend { padding: 0 0 12px; font-size: 12px; font-weight: 550; }
  .method-field { max-width: 320px; margin-bottom: 24px; }
  .seed-heading { display: flex; align-items: center; gap: 10px; margin-bottom: 12px; }
  h3 { margin: 0; font-size: 12px; }
  label { display: grid; gap: 6px; min-width: 0; }
  .field-hint { font-size: 11px; line-height: 1.7; color: var(--text-muted); margin: 0 0 16px; }
  .seed-list { list-style: none; margin: 0; padding: 0; }
  .seed-list li { display: flex; align-items: center; gap: 12px; padding: 12px 0; border-bottom: 1px solid var(--border-subtle); }
  .seed-rank { width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; flex-shrink: 0; border-radius: 5px; background: var(--background); color: var(--primary); font-size: 11px; font-weight: 600; }
  .seed-name { flex: 1; min-width: 0; font-size: 12px; overflow-wrap: anywhere; }
  .seed-actions { display: flex; align-items: center; gap: 4px; }
  .seed-actions .secondary { border-color: transparent; min-height: 30px; padding: 5px 8px; color: var(--text-secondary); }
  .seed-add { display: flex; gap: 12px; align-items: end; margin: 20px 0 24px; }
  .seed-add label { flex: 1; max-width: 480px; }
  .layout-editor { min-width: 0; border-top: 1px solid var(--border-subtle); padding-top: 24px; }
  .layout-heading { align-items: flex-start; flex-wrap: wrap; }
  .layout-status { display: flex; gap: 6px; flex-wrap: wrap; }
  .draw-sections { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 24px; }
  .draw-section { display: grid; align-content: start; gap: 14px; min-width: 0; }
  .knockout-layout { grid-template-columns: 1fr; }
  .knockout-layout .draw-section { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .knockout-layout .draw-section h3 { grid-column: 1 / -1; }
  .draw-section label > span { color: var(--text-secondary); font-size: 11px; }
  .pair-start { padding-top: 14px; border-top: 1px solid var(--border-subtle); }
  .layout-actions { margin-top: 24px; }
  @media (max-width: 1100px) { .draw-sections { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
  @media (max-width: 650px) { .arrangement-heading, .seed-add { flex-direction: column; align-items: stretch; } .draw-sections, .knockout-layout .draw-section { grid-template-columns: 1fr; } .seed-add label { max-width: none; } .seed-list li { flex-wrap: wrap; } .seed-actions { margin-left: 40px; } }
</style>
