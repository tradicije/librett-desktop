<script lang="ts">
  import InfoRows from './InfoRows.svelte';
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import { onMount, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import Select from './Select.svelte';
  import { groupName, roundTitle } from './draw-view';
  import { getMatchPage, saveMatchResult, desktopAvailable, type Tournament, type Category, type MatchPage, type ScheduledMatch, type MatchOutcome, type SaveMatchRequest } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false), onsettings }: { active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean; onsettings: () => void } = $props();
  let text = $derived(messages[language]);
  let readOnly=$derived(category.completed || tournament.completed);
  let sr = $derived(language === 'sr');
  let data = $state<MatchPage | null>(null);
  let loading = $state(false); let saving = $state(false);
  let error = $state<MessageKey | null>(null); let editorError = $state<MessageKey | null>(null);
  let stage = $state('groups');
  let group = $state('0'); let round = $state('0'); let page = $state(0);
  let selected = $state<ScheduledMatch | null>(null);
  let outcome = $state<MatchOutcome>('played'); let winner = $state('');
  let sets = $state<{ first: number | undefined; second: number | undefined }[]>([]);
  let pending = $state<SaveMatchRequest | null>(null);
  let impact = $state<SaveMatchRequest | null>(null);
  let confirmation = $state<'discard' | 'reload' | null>(null);
  let original = '';
  let dialog: HTMLDialogElement;
  const uid = $props.id();
  const snapshot = () => JSON.stringify([outcome, winner, sets]);
  let names = $derived(new Map(data?.draw?.participants.map(entry => [entry.id, entry.members.map(playerLabel).join(' / ')]) ?? []));
  const name = (id: string | null) => id ? names.get(id) ?? id : sr ? 'Čeka protivnika' : 'Awaiting opponent';
  let groupOptions = $derived(data?.draw?.sections.map((_, index) => ({ value: String(index), label: `${sr ? 'Grupa' : 'Group'} ${groupName(index)}` })) ?? []);
  let roundOptions = $derived(Array.from({ length: data?.round_count ?? 0 }, (_, index) => ({ value: String(index), label: category.format === 'knockout' || stage === 'knockout' ? roundTitle(2 ** ((data?.round_count ?? 0) - index), language) : `${sr ? 'Kolo' : 'Round'} ${index + 1}` })));
  let winnerOptions = $derived(selected ? [selected.first, selected.second].filter((id): id is string => id !== null).map(id => ({ value: id, label: name(id) })) : []);
  function setWinner(set: { first: number | undefined; second: number | undefined }, rules = data?.rules): 0 | 1 | null {
    if (!rules || set.first === undefined || set.second === undefined || !Number.isInteger(set.first) || !Number.isInteger(set.second)) return null;
    const high = Math.max(set.first, set.second), low = Math.min(set.first, set.second);
    if (high < rules.points_to_win || high - low < rules.win_by) return null;
    if (!(high === rules.points_to_win && low <= rules.points_to_win - rules.win_by || high > rules.points_to_win && high - low === rules.win_by)) return null;
    return set.first > set.second ? 0 : 1;
  }
  function countSets(input: { first: number | undefined; second: number | undefined }[]) {
    const wins = [0, 0];
    for (const set of input) { const side = setWinner(set); if (side !== null) wins[side]++; }
    return wins;
  }
  let playedWins = $derived(countSets(sets));
  function changeScore(index: number, side: 'first' | 'second', value: number | undefined) {
    sets = sets.map((set, position) => position === index ? { ...set, [side]: value } : set);
    editorError = null;
  }
  $effect(() => { busy = loading || saving || selected !== null || pending !== null; dirty = selected !== null && snapshot() !== original; });
  async function load() {
    if (loading) return;
    loading = true; error = null;
    try { data = await getMatchPage(tournament.id, category.id, Number(group), Number(round), page, stage === 'knockout'); }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => {
    if (desktopAvailable) void load();
    const updated = (event: Event) => { if ((event as CustomEvent).detail === category.id && !selected && !pending && !loading) void load(); };
    window.addEventListener('librett-results-updated', updated);
    return () => window.removeEventListener('librett-results-updated', updated);
  });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && desktopAvailable && !selected && !pending) void load(); wasActive = active; });
  async function filterChanged(kind: 'group' | 'round' | 'stage') { if (kind !== 'round') round = '0'; page = 0; await load(); }
  function edit(item: ScheduledMatch) {
    if (readOnly || loading || selected || !data || data.stale || !item.first || !item.second || item.bye) return;
    selected = item; outcome = item.result?.outcome ?? 'played'; winner = item.result?.winner ?? item.first;
    sets = item.result?.sets.map(set => ({ ...set })) ?? Array.from({ length: Math.floor(data.rules.best_of / 2) + 1 }, () => ({ first: undefined, second: undefined }));
    original = snapshot(); editorError = null; impact = null; confirmation = null;
    dialog.showModal();
  }
  function closeEditor() { selected = null; pending = null; impact = null; confirmation = null; dialog.close(); }
  function cancel() {
    if (saving || pending) return;
    if (dirty) confirmation = 'discard'; else closeEditor();
  }
  async function submit(event: SubmitEvent) {
    event.preventDefault(); if (saving || !selected || !data?.draw) return;
    if (!pending) {
      const input = outcome === 'walkover' ? [] : sets;
      if (input.some(set => set.first === undefined || set.second === undefined || !Number.isInteger(set.first) || !Number.isInteger(set.second))) { editorError = 'invalid_result'; return; }
      const scores = input.map(set => ({ first: set.first!, second: set.second! }));
      const wins = countSets(scores);
      const victor = outcome === 'played' ? (wins[0] > wins[1] ? selected.first : selected.second) : winner;
      if (!selected.first || !selected.second || !victor) { editorError = 'invalid_result'; return; }
      pending = { request_id: crypto.randomUUID(), tournament_id: tournament.id, category_id: category.id, draw_id: data.draw.id, rules_revision: data.rules_revision, key: selected.key, expected_revision: selected.revision, result: { first: selected.first, second: selected.second, winner: victor, outcome, sets: scores, rules: JSON.parse(JSON.stringify(data.rules)) }, invalidate_downstream: false };
    }
    await write();
  }
  async function write() {
    if (!pending || saving) return;
    saving = true; editorError = null;
    try { await saveMatchResult(pending); closeEditor(); window.dispatchEvent(new CustomEvent('librett-results-updated', { detail: category.id })); await load(); }
    catch (cause) {
      const code = typeof cause === 'string' ? cause : '';
      if (code === 'result_impact') { impact = pending; pending = null; }
      else { editorError = errorKey(cause); if (['competition_closed', 'invalid_result', 'match_conflict', 'not_found', 'invalid_draw', 'invalid_rules'].includes(code)) pending = null; }
    } finally { saving = false; }
  }
  async function acceptImpact() { if (!impact) return; pending = { ...impact, request_id: crypto.randomUUID(), invalidate_downstream: true }; impact = null; await write(); }
  async function reloadEditor() { closeEditor(); await load(); }
  function changeOutcome(value: MatchOutcome) { outcome = value; if (outcome !== 'walkover' && sets.length === 0) sets = [{ first: undefined, second: undefined }]; }
  const score = (item: ScheduledMatch) => item.result?.sets.reduce((wins, set) => {
    const rules = item.result!.rules;
    if (Math.max(set.first, set.second) >= rules.points_to_win && Math.abs(set.first - set.second) >= rules.win_by) wins[set.first > set.second ? 0 : 1]++;
    return wins;
  }, [0, 0]).join(' : ');
</script>
<div class="matches-view">
  <div class="matches-toolbar"><div><h2>{text.matches}</h2>{#if data}<InfoRows compact items={[{label:language==='sr'?'Setova':'Sets',value:data.rules.best_of},{label:language==='sr'?'Poena':'Points',value:data.rules.points_to_win},{label:language==='sr'?'Razlika':'Win by',value:`+${data.rules.win_by}`}]} />{/if}</div><button class="secondary icon-label" disabled={loading || !!selected} onclick={load}><Icon name="restore" size={16} />{sr ? 'Osveži' : 'Refresh'}</button></div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if !desktopAvailable}<p class="banner">{text.preview}</p>{:else if loading && !data}<p class="muted">{text.loading}</p>{:else if !data?.draw}<section class="panel"><p class="muted">{sr ? 'Prvo sačuvaj žreb u podešavanjima kategorije.' : 'Save the draw in category settings first.'}</p><button data-open-tab class="secondary" onclick={onsettings}>{text.settings}</button></section>{:else}
    {#if data.stale}<p class="banner" role="alert">{sr ? 'Žreb je nepotpun ili su prijave i pravila grupa promenjeni. Unos je zaključan dok ne sačuvaš važeći žreb.' : 'The draw is incomplete or registrations/group rules changed. Save a current draw before entering results.'}</p>{/if}
    {#if category.format === 'groups_knockout'}<p class="muted">{sr ? 'Tabele se ažuriraju posle svakog rezultata. Prolaznici se određuju po završetku grupa.' : 'Standings update after each result. Qualifiers are resolved when groups finish.'}</p>{/if}
    <div class="match-filters">
      {#if category.format === 'groups_knockout'}<label>{sr ? 'Faza' : 'Stage'}<Select label={sr ? 'Faza' : 'Stage'} bind:value={() => stage, (value) => { stage = value; void filterChanged('stage'); }} disabled={loading} options={[{value:'groups', label:sr ? 'Grupe' : 'Groups'}, {value:'knockout', label:sr ? 'Nokaut' : 'Knockout'}]} /></label>{/if}
      {#if category.format === 'groups_knockout' && stage === 'groups'}<label>{sr ? 'Grupa' : 'Group'}<Select label={sr ? 'Grupa' : 'Group'} bind:value={() => group, (value) => { group = value; void filterChanged('group'); }} options={groupOptions} disabled={loading} /></label>{/if}
      <label>{sr ? 'Kolo' : 'Round'}<Select label={sr ? 'Kolo' : 'Round'} bind:value={() => round, (value) => { round = value; void filterChanged('round'); }} options={roundOptions} disabled={loading} /></label>
      <span class="muted" role="status">{loading ? text.loading : `${data.total} ${sr ? 'mečeva' : 'matches'}`}</span>
    </div>
    <div class="match-list" aria-busy={loading}>
      {#each data.matches as item (item.key)}
        <article class="panel match-card">
          <div class="match-label"><span class="eyebrow">{sr ? 'Meč' : 'Match'} {item.position + 1}</span><span class="pill">{item.bye ? 'BYE' : item.result ? (item.result.outcome === 'retired' ? sr ? 'Predaja' : 'Retired' : item.result.outcome === 'walkover' ? sr ? 'Nedolazak' : 'Walkover' : sr ? 'Završen' : 'Completed') : item.first && item.second ? sr ? 'Spreman' : 'Ready' : sr ? 'Čeka' : 'Pending'}</span></div>
          <div class="match-players"><div class:winner={item.result?.winner === item.first}><span><PlayerName label={item.first ? name(item.first) : item.bye ? 'BYE' : name(null)} /></span><b>{item.result ? score(item)?.split(' : ')[0] : '—'}</b></div><div class:winner={item.result?.winner === item.second}><span><PlayerName label={item.second ? name(item.second) : item.bye ? 'BYE' : name(null)} /></span><b>{item.result ? score(item)?.split(' : ')[1] : '—'}</b></div></div>
          {#if item.result}<p class="match-set-summary muted"><span class="set-scores">{#each item.result.sets as set,index}<span class="set-score"><small>{sr?'Set':'Set'} {index+1}</small><strong>{set.first}:{set.second}</strong></span>{/each}</span>{#if item.result.outcome !== 'played'}<span class="metadata-line">{sr ? 'Pobednik' : 'Winner'}: <PlayerName label={name(item.result.winner)} /></span>{/if}</p>{/if}
          {#if !item.bye}<button class="secondary icon-label" disabled={readOnly || loading || data.stale || !item.first || !item.second} onclick={() => edit(item)}><Icon name="edit" size={16} />{item.result ? sr ? 'Ispravi rezultat' : 'Edit result' : sr ? 'Unesi rezultat' : 'Enter result'}</button>{/if}
        </article>
      {/each}
    </div>
    {#if data.total > 32}<div class="match-pagination"><button class="secondary" disabled={loading || page === 0} onclick={() => { page--; void load(); }}><Icon name="arrow-left" />{sr ? 'Prethodna' : 'Previous'}</button><span>{page + 1} / {Math.ceil(data.total / 32)}</span><button class="secondary" disabled={loading || (page + 1) * 32 >= data.total} onclick={() => { page++; void load(); }}>{sr ? 'Sledeća' : 'Next'}<Icon name="arrow-right" /></button></div>{/if}
  {/if}
</div>
<dialog class="confirm-dialog result-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} oncancel={(event) => { event.preventDefault(); cancel(); }}>
  <h2 id={`${uid}-title`}>{sr ? 'Rezultat meča' : 'Match result'}</h2>
  {#if selected}
    <p class="result-opponents"><span><PlayerName label={name(selected.first)} /></span><span class="muted">vs</span><span><PlayerName label={name(selected.second)} /></span></p>
    {#if confirmation}<p>{confirmation === 'reload' ? sr ? 'Odbaci unos i učitaj najnovije podatke?' : 'Discard your input and reload the latest data?' : sr ? 'Odbaci nesačuvane izmene?' : 'Discard unsaved changes?'}</p><div class="dialog-actions"><button class="secondary" onclick={() => confirmation = null}>{sr ? 'Nastavi unos' : 'Keep editing'}</button><button class="primary" onclick={() => { if (confirmation === 'reload') void reloadEditor(); else closeEditor(); }}>{sr ? 'Odbaci' : 'Discard'}</button></div>
    {:else if impact}<p class="banner" role="alert">{sr ? 'Ispravka menja prolaznike ili pobednika i poništiće sačuvane rezultate nokaut mečeva koji zavise od tih učesnika. Ti mečevi će čekati novi unos. Istorija rezultata ostaje sačuvana.' : 'This correction changes qualifiers or a winner and will clear dependent knockout results. Those matches will need new results. Result history is retained.'}</p><div class="dialog-actions"><button class="secondary" onclick={() => impact = null}>{sr ? 'Vrati se na unos' : 'Back to editing'}</button><button class="primary" onclick={acceptImpact}>{sr ? 'Potvrdi ispravku' : 'Confirm correction'}</button></div>
    {:else}
      {#if editorError}<p class="error" role="alert">{text[editorError]}</p>{/if}
      <form onsubmit={submit}>
        <fieldset disabled={readOnly || saving || pending !== null}>
          <label>{sr ? 'Ishod' : 'Outcome'}<Select label={sr ? 'Ishod' : 'Outcome'} bind:value={() => outcome, (value) => changeOutcome(value as MatchOutcome)} disabled={readOnly || saving || pending !== null} options={[{ value: 'played', label: sr ? 'Odigran meč' : 'Played match' }, { value: 'retired', label: sr ? 'Predaja' : 'Retired' }, { value: 'walkover', label: sr ? 'Nedolazak' : 'Walkover' }]} /></label>
          {#if outcome !== 'played'}<label>{sr ? 'Pobednik' : 'Winner'}<Select playerLabels label={sr ? 'Pobednik' : 'Winner'} bind:value={winner} options={winnerOptions} disabled={readOnly || saving || pending !== null} /></label>{/if}
          {#if outcome !== 'walkover'}
            <p class="muted">{sr ? 'Unesi samo odigrane setove, redom. Kod predaje poslednji set može biti nezavršen.' : 'Enter only played sets, in order. For a retirement the last set may be incomplete.'}</p>
            <div class="set-head"><span>{sr ? 'Set' : 'Set'}</span><span><PlayerName label={name(selected.first)} /></span><span><PlayerName label={name(selected.second)} /></span><span></span></div>
            {#each sets as row, index}<div class="set-row"><b class:completed={setWinner(row) !== null} title={setWinner(row) !== null ? (sr ? 'Set završen' : 'Set complete') : (sr ? 'Set u toku' : 'Set in progress')}>{index + 1}{#if setWinner(row) !== null}<Icon name="check-circle" size={12} />{/if}</b><input aria-label={`${sr ? 'Set' : 'Set'} ${index + 1}: ${name(selected.first)}`} type="number" min="0" max="999" step="1" required bind:value={() => sets[index]?.first, (value) => changeScore(index, 'first', value)} /><input aria-label={`${sr ? 'Set' : 'Set'} ${index + 1}: ${name(selected.second)}`} type="number" min="0" max="999" step="1" required bind:value={() => sets[index]?.second, (value) => changeScore(index, 'second', value)} /><button type="button" class="icon-button" aria-label={`${sr ? 'Ukloni set' : 'Remove set'} ${index + 1}`} onclick={() => sets = sets.filter((_, i) => i !== index)}><Icon name="trash" size={16} /></button></div>{/each}
            <button type="button" class="secondary icon-label add-set" disabled={sets.length >= (data?.rules.best_of ?? 5)} onclick={() => sets = [...sets, { first: undefined, second: undefined }]}><Icon name="plus" size={16} />{sr ? 'Dodaj set' : 'Add set'}</button>
            <p class="result-total">{sr ? 'Setovi' : 'Sets'}: {playedWins[0]} : {playedWins[1]}</p>
          {/if}
        </fieldset>
        <div class="dialog-actions">
          {#if editorError === 'match_conflict'}<button type="button" class="secondary" onclick={() => confirmation = 'reload'}>{sr ? 'Učitaj ponovo' : 'Reload'}</button>{/if}
          <button type="button" class="secondary" disabled={saving || pending !== null} onclick={cancel}>{sr ? 'Otkaži' : 'Cancel'}</button><button class="primary" disabled={(readOnly && !pending) || saving}>{saving ? text.saving : pending ? text.retry : sr ? 'Sačuvaj rezultat' : 'Save result'}</button>
        </div>
      </form>
    {/if}
  {/if}
</dialog>
<style>
  .matches-view { display: grid; gap: 20px; }
  .matches-toolbar, .match-filters, .match-label, .match-pagination { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .matches-toolbar h2 { margin: 0; }
  .match-filters { justify-content: flex-start; flex-wrap: wrap; }
  .match-filters label { display: flex; align-items: center; gap: 10px; }
  .match-list { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
  .match-card { display: flex; flex-direction: column; gap: 16px; min-width: 0; }
  .match-label .eyebrow { margin: 0; }
  .match-players { display: grid; gap: 10px; }
  .match-players > div { display: flex; justify-content: space-between; gap: 12px; }
  .match-players span { overflow-wrap: anywhere; }
  .winner { color: var(--primary); font-weight: 600; }
  .match-set-summary { margin: 0; font-variant-numeric: tabular-nums; }
  .match-card > button { margin-top: auto; align-self: flex-start; }
  .match-pagination { justify-content: center; }
  .result-dialog { width: min(640px, calc(100vw - 32px)); max-height: calc(100dvh - 32px); overflow: auto; }
  .result-dialog h2 { margin: 0; }
  .result-opponents { display: flex; gap: 12px; align-items: center; overflow-wrap: anywhere; }
  fieldset { border: 0; padding: 0; margin: 0; display: grid; gap: 16px; min-width: 0; }
  fieldset > label { display: grid; gap: 8px; }
  .set-head, .set-row { display: grid; grid-template-columns: 30px minmax(0, 1fr) minmax(0, 1fr) 30px; gap: 12px; align-items: center; }
  .set-head { font-size: 11px; color: var(--text-secondary); overflow-wrap: anywhere; }
  .set-row > b { display: flex; align-items: center; gap: 3px; }
  .set-row > b.completed { color: var(--primary); }
  .set-row input { width: 100%; min-width: 0; text-align: center; }
  .add-set { justify-self: start; }
  .result-total { margin: 0; font-weight: 600; }
  @media (max-width: 900px) { .match-list { grid-template-columns: 1fr; } }
</style>
