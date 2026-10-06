<script lang="ts">
  import AgeWarningDialog from './AgeWarningDialog.svelte';
  let ageDialog:AgeWarningDialog;
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import Select from './Select.svelte';
  import { onMount, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import { registrationStarted, desktopAvailable, getCategoryRules, listEntries, listPlayers, registerEntries, setEntryStatus, setPlayerAttendance, type EntryStatus, type Entry, type Player, type Tournament, type Category } from './api';
  import { errorKey, messages, type Language, type MessageKey } from './i18n';

  let { active = true, tournament, category, language, busy = $bindable(false), dirty = $bindable(false) }: { active?: boolean; tournament: Tournament; category: Category; language: Language; busy?: boolean; dirty?: boolean } = $props();
  let text = $derived(messages[language]);
  let readOnly=$derived(category.completed || tournament.completed);
  let players = $state<Player[]>([]);
  let entries = $state<Entry[]>([]);
  let search = $state('');
  let statusFilter = $state<'all' | EntryStatus>('registered');
  let started = $state(false);
  let registrationLocked = $derived(readOnly || started);
  let visibleEntries = $derived(entries.filter(entry => statusFilter === 'all' || entry.status === statusFilter));
  let activeEntries = $derived(entries.filter(entry => entry.status !== 'withdrawn'));
  let checkedInCount = $derived(activeEntries.reduce((sum, entry) => sum + entry.members.filter(member => member.checked_in).length, 0));
  let categoryId = $derived(category.id);
  let checked = $state<string[]>([]);
  let pairs = $state<string[][]>([]);
  let queued = $derived(pairs.flat());
  $effect(() => { dirty = checked.length > 0 || pairs.length > 0; });
  let groups = $derived(category.discipline === 'singles' ? checked.map(id => [id]) : pairs);
  let loading = $state(false);
  let playersLoaded = $state(false);
  let entriesLoaded = $state(false);
  let reload = $state(0);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);
  let filtered = $derived(players.filter(p => `${p.name} ${p.club}`.toLowerCase().includes(search.trim().toLowerCase())));

  async function loadPlayers() {
    loading = true; error = null;
    try { players = await listPlayers(); playersLoaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void loadPlayers(); });
  let requestVersion = 0;
  let wasActive = untrack(() => active);
  $effect(() => {
    if (active && !wasActive && desktopAvailable && !busy) void refresh();
    wasActive = active;
  });
  async function refresh() {
    const version = ++requestVersion;
    loading = true;
    try {
      const [current, directory, hasStarted] = await Promise.all([listEntries(categoryId), listPlayers(), registrationStarted(tournament.id,categoryId)]);
      if (version !== requestVersion) return;
      started = hasStarted;
      entries = current; players = directory;
      const claimed = new Set(current.filter(entry => entry.status === 'registered').flatMap(entry => entry.members.map(member => member.id)));
      checked = checked.filter(id => !claimed.has(id));
      pairs = pairs.filter(pair => pair.every(id => !claimed.has(id)));
      entriesLoaded = true; playersLoaded = true;
    } catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  $effect(() => {
    void reload;
    const id = categoryId;
    const version = ++requestVersion;
    entries = []; entriesLoaded = false; checked = []; pairs = [];
    if (!desktopAvailable || !id) return;
    void Promise.all([listEntries(id),registrationStarted(tournament.id,id)]).then(([result,hasStarted]) => {
      if (version === requestVersion) { entries = result; started = hasStarted; entriesLoaded = true; }
    }).catch(cause => { if (version === requestVersion) error = errorKey(cause); });
  });

  async function register(event: SubmitEvent) {
    event.preventDefault(); if (!category || registrationLocked || busy) return;
    busy = true; error = null; notice = null;
    try {
      const selectedGroups=groups.map(group=>[...group]);
      const [configuration,directory]=await Promise.all([getCategoryRules(tournament.id,category.id),listPlayers()]);
      players=directory;
      const selectedIds=new Set(selectedGroups.flat());
      const selectedPlayers=directory.filter(player=>selectedIds.has(player.id));
      if(!await ageDialog.confirm(selectedPlayers,configuration.rules))return;
      const saved = await registerEntries(tournament.id, category.id, selectedGroups);
      entries = [...entries, ...saved]; checked = []; pairs = []; notice = 'entrySaved'; window.dispatchEvent(new CustomEvent('librett-registration-updated',{detail:tournament.id}));
    } catch (cause) {
      error = errorKey(cause);
      try {
        entries = await listEntries(category.id);
        const claimed = new Set(entries.flatMap(entry => entry.members.map(member => member.id)));
        checked = checked.filter(id => !claimed.has(id));
        pairs = pairs.filter(pair => !pair.some(id => claimed.has(id)));
      } catch { /* Keep the original error and selection for retry. */ }
    }
    finally { busy = false; }
  }
  function toggle(id: string) {
    if (registrationLocked || busy) return;
    checked = checked.includes(id) ? checked.filter(item => item !== id) : [...checked, id];
  }
  function addPair() { if (checked.length === 2) { pairs = [...pairs, [...checked]]; checked = []; } }
  async function changeStatus(entry: Entry) {
    if (registrationLocked || busy) return;
    const next: EntryStatus = entry.status === 'withdrawn' ? 'registered' : 'withdrawn';
    busy = true; error = null; notice = null;
    try {
      if(next==='registered'){
        const [configuration,directory]=await Promise.all([getCategoryRules(tournament.id,category.id),listPlayers()]);
        players=directory;
        const ids=new Set(entry.members.map(member=>member.id));
        if(!await ageDialog.confirm(directory.filter(player=>ids.has(player.id)),configuration.rules))return;
      }
      await setEntryStatus(tournament.id, entry.id, next);
      entries = entries.map(item => item.id === entry.id ? { ...item, status: next } : item);
      notice = 'registrationUpdated'; window.dispatchEvent(new CustomEvent('librett-registration-updated',{detail:tournament.id}));
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function changeAttendance(member: Entry['members'][number]) {
    if (readOnly || busy) return;
    const checkedIn = !member.checked_in;
    busy = true; error = null; notice = null;
    try {
      await setPlayerAttendance(tournament.id, member.id, checkedIn);
      entries = entries.map(entry => ({ ...entry, members: entry.members.map(item => item.id === member.id ? { ...item, checked_in: checkedIn } : item) }));
      notice = 'attendanceUpdated'; window.dispatchEvent(new CustomEvent('librett-registration-updated',{detail:tournament.id}));
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }

</script>
<AgeWarningDialog {language} bind:this={ageDialog} />

<section class="players-section">
  {#if error}<p class="error" role="alert">{text[error]}<button disabled={loading || busy} onclick={() => { reload += 1; void loadPlayers(); }}>{text.retry}</button></p>{/if}
  <p class="notice" role="status">{notice ? text[notice] : ''}</p>
  <div class="columns">
    <section class="panel"><h3 class="icon-label"><Icon name="list" />{text.activeRegistrations}</h3><p class="muted">{text.registrationStartHint}</p>
      {#if entriesLoaded}
        <div class="registration-summary"><span><strong>{activeEntries.length}</strong>{text.activeRegistrations}</span><span><strong>{checkedInCount}</strong>{text.arrivedPlayers}</span><span><strong>{entries.length - activeEntries.length}</strong>{text.withdrawnRegistrations}</span></div>
      {/if}
      <label>{text.entryFilter}<Select label={text.entryFilter} bind:value={statusFilter} options={[{ value: 'all', label: text.allRegistrations }, { value: 'registered', label: text.activeRegistrations }, { value: 'withdrawn', label: text.withdrawnRegistrations }]} disabled={busy} /></label>
      {#if categoryId && !entriesLoaded && desktopAvailable}<p class="muted">{text.loading}</p>
      {:else if !visibleEntries.length}<p class="muted">{entries.length ? text.noFilteredEntries : text.noEntries}</p>{/if}
      {#each visibleEntries as entry (entry.id)}
        <article class="entry-card" class:entry-withdrawn={entry.status === 'withdrawn'}>
          <div class="entry-heading"><h3><PlayerName label={entry.members.map(playerLabel).join(' / ')} /></h3><span class="pill" class:status-active={entry.status !== 'withdrawn'}>{entry.status === 'withdrawn' ? text.withdrawnRegistrations : text.activeRegistrations}</span></div>
          <p class="muted">{entry.members.map(p => p.club).filter(Boolean).join(' / ')}</p>
          {#each entry.members as member (member.id)}
            <div class="attendance-row"><span><PlayerName player={member} /><small class="metadata-line muted">{member.checked_in ? text.arrived : text.notArrived}</small></span>
              <button class="secondary icon-label" disabled={readOnly || busy || !desktopAvailable || entry.status === 'withdrawn'} aria-label={`${member.checked_in ? text.markAbsent : text.markArrived}: ${playerLabel(member)}`} onclick={() => changeAttendance(member)}><Icon name={member.checked_in ? 'restore' : 'check-circle'} size={18} />{member.checked_in ? text.markAbsent : text.markArrived}</button>
            </div>
          {/each}
          <button class="secondary icon-label" disabled={readOnly || busy || !desktopAvailable || started} title={started ? text.competition_started : undefined} onclick={() => changeStatus(entry)}><Icon name={entry.status === 'withdrawn' ? 'restore' : 'withdraw'} size={18} />{entry.status === 'withdrawn' ? text.restoreEntry : text.withdrawEntry}</button>
        </article>
      {/each}
    </section>
    <section class="panel form-panel"><h3>{text.register}</h3><form onsubmit={register}>
      <label>{text.searchPlayers}<input type="search" bind:value={search} disabled={busy} /></label>
      <p class="muted">{category.discipline === 'singles' ? text.bulkSinglesHint : text.bulkDoublesHint}</p>
      <div class="registration-picker">
        {#each filtered as player (player.id)}
          {@const registered = entries.some(e => e.members.some(m => m.id === player.id))}
          {@const inPair = queued.includes(player.id)}
          <label class="picker-row"><input type="checkbox" checked={registered || inPair || checked.includes(player.id)} onchange={() => toggle(player.id)} disabled={registrationLocked || busy || !entriesLoaded || registered || inPair || (category.discipline === 'doubles' && checked.length >= 2 && !checked.includes(player.id))} /><span><strong><PlayerName {player} /></strong><small>{player.club}</small></span>{#if registered}<span class="pill">{text.alreadyInCategory}</span>{/if}</label>
        {/each}
        {#if !filtered.length}<p class="muted">{text.noPlayers}</p>{/if}
      </div>
      {#if category.discipline === 'doubles'}
        <button type="button" class="secondary icon-label" onclick={addPair} disabled={registrationLocked || busy || checked.length !== 2}><Icon name="users" size={16} />{text.addPair}</button>
        {#each pairs as pair, index}
          <div class="pair-row"><span><PlayerName label={pair.map(id => playerLabel(players.find(p => p.id === id))).join(' / ')} /></span><button type="button" class="icon-button" aria-label={`${text.removePair}: ${index + 1}`} disabled={registrationLocked || busy} onclick={() => { pairs = pairs.filter((_, i) => i !== index); }}><Icon name="trash" size={16} /></button></div>
        {/each}
      {/if}
      <button class="primary" disabled={registrationLocked || busy || !desktopAvailable || !playersLoaded || !entriesLoaded || !groups.length}><Icon name="check-circle" size={18} />{busy ? text.saving : text.registerSelected} ({groups.length})</button>
    </form></section>
  </div>
</section>
