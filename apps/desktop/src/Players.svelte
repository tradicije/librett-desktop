<script lang="ts">
  import Select from './Select.svelte';
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, listEntries, listPlayers, registerEntry, setEntryStatus, setPlayerAttendance, type EntryStatus, type Entry, type Player, type Tournament } from './api';
  import { errorKey, messages, type Language, type MessageKey } from './i18n';

  let { tournament, language, busy = $bindable(false) }: { tournament: Tournament; language: Language; busy?: boolean } = $props();
  let text = $derived(messages[language]);
  let players = $state<Player[]>([]);
  let entries = $state<Entry[]>([]);
  let search = $state('');
  let statusFilter = $state<'all' | EntryStatus>('all');
  let visibleEntries = $derived(entries.filter(entry => statusFilter === 'all' || entry.status === statusFilter));
  let activeEntries = $derived(entries.filter(entry => entry.status !== 'withdrawn'));
  let checkedInCount = $derived(activeEntries.reduce((sum, entry) => sum + entry.members.filter(member => member.checked_in).length, 0));
  let categoryId = $state('');
  let category = $derived(tournament.categories.find(c => c.id === categoryId));
  let first = $state('');
  let second = $state('');
  let loading = $state(false);
  let playersLoaded = $state(false);
  let entriesLoaded = $state(false);
  let reload = $state(0);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);
  let filtered = $derived(players.filter(p => `${p.name} ${p.club}`.toLowerCase().includes(search.trim().toLowerCase())));
  let available = $derived(players.filter(p => !entries.some(e => e.members.some(m => m.id === p.id))));

  async function loadPlayers() {
    loading = true; error = null;
    try { players = await listPlayers(); playersLoaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void loadPlayers(); });
  let requestVersion = 0;
  $effect(() => { if (!categoryId && tournament.categories.length) categoryId = tournament.categories[0].id; });
  $effect(() => {
    void reload;
    const id = categoryId;
    const version = ++requestVersion;
    entries = []; entriesLoaded = false; first = ''; second = '';
    if (!desktopAvailable || !id) return;
    void listEntries(id).then(result => {
      if (version === requestVersion) { entries = result; entriesLoaded = true; }
    }).catch(cause => { if (version === requestVersion) error = errorKey(cause); });
  });

  async function register(event: SubmitEvent) {
    event.preventDefault(); if (!category) return;
    busy = true; error = null; notice = null;
    try {
      const entry = await registerEntry(tournament.id, category.id, category.discipline === 'doubles' ? [first, second] : [first]);
      entries = [...entries, entry]; first = ''; second = ''; notice = 'entrySaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function changeStatus(entry: Entry) {
    if (busy) return;
    const next: EntryStatus = entry.status === 'withdrawn' ? 'registered' : 'withdrawn';
    busy = true; error = null; notice = null;
    try {
      await setEntryStatus(tournament.id, entry.id, next);
      entries = entries.map(item => item.id === entry.id ? { ...item, status: next } : item);
      notice = 'registrationUpdated';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function changeAttendance(member: Entry['members'][number]) {
    if (busy) return;
    const checkedIn = !member.checked_in;
    busy = true; error = null; notice = null;
    try {
      await setPlayerAttendance(tournament.id, member.id, checkedIn);
      entries = entries.map(entry => ({ ...entry, members: entry.members.map(item => item.id === member.id ? { ...item, checked_in: checkedIn } : item) }));
      notice = 'attendanceUpdated';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }

</script>

<section class="players-section">
  <h2 class="icon-label"><Icon name="users" />{text.players}</h2>
  <p class="muted">{text.registrationDirectoryHint}</p>
  {#if error}<p class="error" role="alert">{text[error]}<button disabled={loading || busy} onclick={() => { reload += 1; void loadPlayers(); }}>{text.retry}</button></p>{/if}
  <p class="notice" role="status">{notice ? text[notice] : ''}</p>
  <div class="columns">
    <section class="panel"><h3 class="icon-label"><Icon name="list" />{text.entries}</h3>
      {#if entriesLoaded}
        <div class="registration-summary"><span><strong>{activeEntries.length}</strong>{text.activeRegistrations}</span><span><strong>{checkedInCount}</strong>{text.arrivedPlayers}</span><span><strong>{entries.length - activeEntries.length}</strong>{text.withdrawnRegistrations}</span></div>
      {/if}
      <label>{text.entryFilter}<Select label={text.entryFilter} bind:value={statusFilter} options={[{ value: 'all', label: text.allRegistrations }, { value: 'registered', label: text.activeRegistrations }, { value: 'withdrawn', label: text.withdrawnRegistrations }]} disabled={busy} /></label>
      {#if categoryId && !entriesLoaded && desktopAvailable}<p class="muted">{text.loading}</p>
      {:else if !visibleEntries.length}<p class="muted">{entries.length ? text.noFilteredEntries : text.noEntries}</p>{/if}
      {#each visibleEntries as entry (entry.id)}
        <article class="entry-card" class:entry-withdrawn={entry.status === 'withdrawn'}>
          <div class="entry-heading"><h3>{entry.members.map(p => p.name).join(' / ')}</h3><span class="pill" class:status-active={entry.status !== 'withdrawn'}>{entry.status === 'withdrawn' ? text.withdrawnRegistrations : text.activeRegistrations}</span></div>
          <p class="muted">{entry.members.map(p => p.club).filter(Boolean).join(' / ')}</p>
          {#each entry.members as member (member.id)}
            <div class="attendance-row"><span>{member.name} · {member.checked_in ? text.arrived : text.notArrived}</span>
              <button class="secondary icon-label" disabled={busy || !desktopAvailable} aria-label={`${member.checked_in ? text.markAbsent : text.markArrived}: ${member.name}`} onclick={() => changeAttendance(member)}><Icon name={member.checked_in ? 'restore' : 'check-circle'} size={18} />{member.checked_in ? text.markAbsent : text.markArrived}</button>
            </div>
          {/each}
          <button class="secondary icon-label" disabled={busy || !desktopAvailable} onclick={() => changeStatus(entry)}><Icon name={entry.status === 'withdrawn' ? 'restore' : 'withdraw'} size={18} />{entry.status === 'withdrawn' ? text.restoreEntry : text.withdrawEntry}</button>
        </article>
      {/each}
    </section>
    <section class="panel form-panel"><h3>{text.register}</h3><form onsubmit={register}>
      <label>{text.chooseCategory}<Select label={text.chooseCategory} bind:value={categoryId} options={tournament.categories.map(item => ({ value: item.id, label: `${item.name} · ${text[item.discipline]}` }))} disabled={busy} /></label>
      <label>{text.searchPlayers}<input type="search" bind:value={search} disabled={busy} /></label>
      <label>{text.firstPlayer}<Select label={text.firstPlayer} bind:value={first} options={available.filter(player => player.id === first || filtered.some(p => p.id === player.id)).map(player => ({ value: player.id, label: player.name + (player.club ? ` · ${player.club}` : '') }))} disabled={busy || !entriesLoaded} placeholder={text.choosePlayer} /></label>
      {#if category?.discipline === 'doubles'}<label>{text.secondPlayer}<Select label={text.secondPlayer} bind:value={second} options={available.filter(p => p.id !== first && (p.id === second || filtered.some(item => item.id === p.id))).map(player => ({ value: player.id, label: player.name + (player.club ? ` · ${player.club}` : '') }))} disabled={busy || !entriesLoaded} placeholder={text.choosePlayer} /></label>{/if}
      <button class="primary" disabled={busy || !desktopAvailable || !category || !playersLoaded || !entriesLoaded || !first || (category.discipline === 'doubles' && (!second || first === second))}><Icon name="check-circle" size={18} />{busy ? text.saving : text.register}</button>
    </form></section>
  </div>
</section>
