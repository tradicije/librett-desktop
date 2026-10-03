<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import { getCategoryDraw, listEntries, previewCategoryDraw, saveCategoryDraw, desktopAvailable,
    type CategoryDraw, type Category, type Tournament, type Entry, type DrawMode } from './api';
  import { messages, type Language } from './i18n';
  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false) }: {
    active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean;
  } = $props();
  const labels = {
    sr: {
      title: 'Nacrt žreba', intro: 'Rasporedi učesnike automatski ili ručno. Nacrt još ne pokreće mečeve.',
      automatic: 'Automatski', manual: 'Ručno', groupCount: 'Broj grupa', qualifiers: 'Prolaznika iz svake grupe',
      seeds: 'Nosioci, od najjačeg', addSeed: 'Dodaj nosioca', choose: 'Izaberi prijavu', up: 'Pomeri gore', down: 'Pomeri dole', remove: 'Ukloni',
      generate: 'Napravi raspored', replace: 'Zameni trenutni nacrt', replaceHint: 'Novi raspored će zameniti trenutni nacrt. Sačuvana prethodna verzija ostaje u istoriji.',
      cancel: 'Odustani', save: 'Sačuvaj nacrt', saved: 'Nacrt sačuvan.', unsaved: 'Nesačuvane izmene', revision: 'Verzija',
      group: 'Grupa', pair: 'Par', position: 'Pozicija', empty: 'Prazno', bye: 'Slobodan prolaz (bye)', missing: 'Neraspoređeno',
      min: 'Za žreb su potrebne 2–4096 aktivne prijave.', invalid: 'Proveri grupe, prolaznike i raspored. Svaka prijava može biti raspoređena samo jednom.',
      stale: 'Prijave su promenjene. Učitaj trenutno stanje i napravi novi nacrt.', conflict: 'Druga verzija je već sačuvana. Učitaj trenutno stanje.',
      retry: 'Ponovi čuvanje', uncertain: 'Čuvanje nije potvrđeno. Ponovi isti upis pre nastavka.', load: 'Učitaj trenutno stanje',
      hint: 'Izbor već raspoređene prijave menja njeno mesto sa ovom pozicijom. Prazna pozicija je bye tek kada je ceo kostur popunjen.',
    },
    en: {
      title: 'Draw draft', intro: 'Arrange entries automatically or manually. Drafts do not start matches yet.',
      automatic: 'Automatic', manual: 'Manual', groupCount: 'Number of groups', qualifiers: 'Qualifiers per group',
      seeds: 'Seeds, strongest first', addSeed: 'Add seed', choose: 'Choose an entry', up: 'Move up', down: 'Move down', remove: 'Remove',
      generate: 'Create arrangement', replace: 'Replace current draft', replaceHint: 'The new arrangement replaces this draft. The previous saved revision stays in history.',
      cancel: 'Cancel', save: 'Save draft', saved: 'Draft saved.', unsaved: 'Unsaved changes', revision: 'Revision',
      group: 'Group', pair: 'Pair', position: 'Position', empty: 'Empty', bye: 'Bye', missing: 'Unassigned',
      min: 'A draw requires 2–4096 active entries.', invalid: 'Check groups, qualifiers and positions. Each entry can be placed only once.',
      stale: 'Registrations changed. Reload the current state and create a new draft.', conflict: 'Another revision has already been saved. Reload the current state.',
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
  let groupCount = $state(2);
  let qualifiers = $state(2);
  let mode = $state<DrawMode>('automatic');
  let replacing = $state(false);
  let saved = $state(false);
  let loading = $state(true);
  let action = $state(false);
  let error = $state<'invalid' | 'conflict' | 'stale' | 'uncertain' | 'error' | null>(null);
  let pending = $state<{ draw: CategoryDraw; revision: number } | null>(null);
  let stale = $derived(!!draft && (draft.participants.length !== entries.length || draft.participants.some(e => !entries.some(active => active.id === e.id))));
  let placed = $derived(new Set(draft?.sections.flat().filter((id): id is string => id !== null) ?? []));
  let missing = $derived((draft?.participants.length ?? 0) - placed.size);
  let locked = $derived(action || loading || pending !== null);
  function label(entry: Entry) { return entry.members.map(member => member.name).join(' / '); }
  function entryLabel(id: string) { const entry = draft?.participants.find(e => e.id === id) ?? entries.find(e => e.id === id); return entry ? label(entry) : id; }
  function changed() { dirty = true; saved = false; replacing = false; }
  async function load() {
    loading = true; error = null;
    try {
      const [all, stored] = await Promise.all([listEntries(category.id), getCategoryDraw(tournament.id, category.id)]);
      entries = all.filter(e => e.status === 'registered'); draft = stored; revision = stored?.revision ?? 0;
      seeds = stored?.seeds.filter(id => entries.some(e => e.id === id)) ?? [];
      groupCount = stored?.settings.group_count ?? Math.max(1, Math.min(2, Math.floor(entries.length / 2)));
      qualifiers = stored?.settings.qualifiers_per_group ?? Math.min(2, Math.floor(entries.length / groupCount));
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
      if (!dirty && stored && stored.revision !== revision) {
        draft = stored; revision = stored.revision; seeds = [...stored.seeds];
        groupCount = stored.settings.group_count; qualifiers = stored.settings.qualifiers_per_group; mode = stored.mode;
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
    action = true; error = null;
    try {
      draft = await previewCategoryDraw(tournament.id, category.id, mode,
        { group_count: groupCount, qualifiers_per_group: qualifiers }, seeds);
      changed();
    } catch (cause) { error = cause === 'invalid_draw' ? 'invalid' : 'error'; }
    finally { action = false; }
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
      if (cause === 'invalid_draw' || cause === 'draw_conflict' || cause === 'not_found') {
        error = cause === 'draw_conflict' ? 'conflict' : 'stale'; pending = null;
      } else error = 'uncertain';
    } finally { action = false; busy = pending !== null; }
  }
</script>

<section class="panel draw-workspace">
  <h2>{t.title}</h2><p class="muted">{t.intro}</p>
  {#if !desktopAvailable}<p>{text.preview}</p>
  {:else if loading}<p>{text.loading}</p>
  {:else}
    {#if error}<p role="alert">{error === 'error' ? text.error : t[error]}</p>{/if}
    {#if saved}<p role="status">{t.saved}</p>{/if}
    {#if stale}<p role="alert">{t.stale}</p>{/if}
    <button class="secondary" disabled={locked} onclick={load}>{t.load}</button>
    {#if entries.length < 2 || entries.length > 4096}<p>{t.min}</p>
    {:else}
      <fieldset disabled={locked}>
        <legend>{text.draw}</legend>
        <div class="draw-options">
          <label>{text.draw}<select bind:value={mode}><option value="automatic">{t.automatic}</option><option value="manual">{t.manual}</option></select></label>
          {#if category.format === 'groups_knockout'}
            <label>{t.groupCount}<input type="number" min="1" max={Math.floor(entries.length / 2)} step="1" bind:value={groupCount} /></label>
            <label>{t.qualifiers}<input type="number" min="1" max={Math.floor(entries.length / Math.max(1, groupCount))} step="1" bind:value={qualifiers} /></label>
          {/if}
        </div>
        <h3>{t.seeds}</h3>
        <ol class="seed-list">
          {#each seeds as id, index (id)}
            <li><span>{entryLabel(id)}</span><button class="secondary" disabled={index === 0 || stale} aria-label={`${t.up}: ${entryLabel(id)}`} onclick={() => moveSeed(index, -1)}>↑</button><button class="secondary" disabled={index === seeds.length - 1 || stale} aria-label={`${t.down}: ${entryLabel(id)}`} onclick={() => moveSeed(index, 1)}>↓</button><button class="secondary" disabled={stale} onclick={() => editSeeds(seeds.filter(seed => seed !== id))}>{t.remove}</button></li>
          {/each}
        </ol>
        <div class="draw-options"><label>{t.addSeed}<select bind:value={newSeed} disabled={stale}><option value="">{t.choose}</option>{#each entries.filter(e => !seeds.includes(e.id)) as entry}<option value={entry.id}>{label(entry)}</option>{/each}</select></label><button class="secondary" disabled={!newSeed || stale} onclick={() => { editSeeds([...seeds, newSeed]); newSeed = ''; }}>{t.addSeed}</button></div>
        {#if replacing}<p>{t.replaceHint}</p>{/if}
        <div class="draw-options"><button onclick={generate}>{replacing ? t.replace : t.generate}</button>{#if replacing}<button class="secondary" onclick={() => replacing = false}>{t.cancel}</button>{/if}</div>
      </fieldset>
    {/if}
    {#if draft}
      <p class="muted">{t.revision}: {revision} · {dirty ? t.unsaved : t.saved} · {t.missing}: {missing}</p>
      {#if draft.format === 'groups_knockout'}<p class="muted">{t.groupCount}: {draft.settings.group_count} · {t.qualifiers}: {draft.settings.qualifiers_per_group}</p>{/if}
      <p class="muted">{t.hint}</p>
      <div class="draw-sections">
        {#each draft.sections as section, group}
          <section class="draw-section">
            <h3>{category.format === 'groups_knockout' ? `${t.group} ${group + 1}` : text.bracket}</h3>
            {#each section as id, slot}
              <label class:pair-start={category.format === 'knockout' && slot % 2 === 0}>
                {category.format === 'knockout' ? `${t.pair} ${Math.floor(slot / 2) + 1} · ${t.position} ${slot % 2 + 1}` : `${t.position} ${slot + 1}`}
                <select value={id ?? ''} disabled={locked || stale} onchange={event => assign(group, slot, event.currentTarget.value)}>
                  <option value="">{category.format === 'knockout' && missing === 0 && section[slot ^ 1] ? t.bye : t.empty}</option>
                  {#each draft.participants as entry}<option value={entry.id}>{draft.seeds.includes(entry.id) ? `#${draft.seeds.indexOf(entry.id) + 1} ` : ''}{label(entry)}</option>{/each}
                </select>
              </label>
            {/each}
          </section>
        {/each}
      </div>
      <button disabled={action || loading || (!pending && (stale || !dirty))} onclick={save}>{action ? text.saving : pending ? t.retry : t.save}</button>
    {/if}
  {/if}
</section>

<style>
  .draw-workspace { display: grid; gap: 1rem; }
  fieldset { margin: 0; padding: 1rem; border: 1px solid var(--border); border-radius: 8px; min-width: 0; }
  legend { padding: 0 .4rem; }
  .draw-options { display: flex; gap: .75rem; align-items: end; flex-wrap: wrap; margin: .75rem 0; }
  label { display: grid; gap: .4rem; min-width: 0; }
  select { padding: .6rem; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); color: inherit; max-width: 100%; }
  .seed-list { padding-left: 1.5rem; }
  .seed-list li { padding: .25rem 0; }
  .seed-list span { display: inline-block; min-width: 12rem; }
  .seed-list button { margin-left: .35rem; }
  .draw-sections { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 280px), 1fr)); gap: 1rem; }
  .draw-section { display: grid; align-content: start; gap: .75rem; }
  .pair-start { padding-top: .75rem; border-top: 1px solid var(--border); }
</style>
