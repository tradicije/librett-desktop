<script lang="ts">
  import Select from './Select.svelte';
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { createPlayer, desktopAvailable, listEntries, listPlayers, registerEntry, type Entry, type Player, type Tournament } from './api';
  import { errorKey, messages, type Language, type MessageKey } from './i18n';

  let { tournament, language }: { tournament: Tournament; language: Language } = $props();
  let text = $derived(messages[language]);
  let players = $state<Player[]>([]);
  let entries = $state<Entry[]>([]);
  let name = $state('');
  let club = $state('');
  let search = $state('');
  let categoryId = $state('');
  let category = $derived(tournament.categories.find(c => c.id === categoryId));
  let first = $state('');
  let second = $state('');
  let busy = $state(false);
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

  async function savePlayer(event: SubmitEvent) {
    event.preventDefault(); busy = true; error = null; notice = null;
    try {
      const player = await createPlayer(name, club);
      players = [player, ...players]; name = ''; club = ''; notice = 'playerSaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function register(event: SubmitEvent) {
    event.preventDefault(); if (!category) return;
    busy = true; error = null; notice = null;
    try {
      const entry = await registerEntry(tournament.id, category.id, category.discipline === 'doubles' ? [first, second] : [first]);
      entries = [...entries, entry]; first = ''; second = ''; notice = 'entrySaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
</script>

<section class="players-section">
  <h2 class="icon-label"><Icon name="users" />{text.players}</h2>
  {#if error}<p class="error" role="alert">{text[error]}<button disabled={loading || busy} onclick={() => { reload += 1; void loadPlayers(); }}>{text.retry}</button></p>{/if}
  <p class="notice" role="status">{notice ? text[notice] : ''}</p>
  <div class="columns">
    <section class="panel"><h3>{text.directory}</h3>
      <label><span class="icon-label"><Icon name="search" size={16} />{text.searchPlayers}</span><input bind:value={search} type="search" /></label>
      {#if loading}<p role="status">{text.loading}</p>{:else if playersLoaded && !filtered.length}<p class="muted">{text.noPlayers}</p>{/if}
      <div class="player-list">{#each filtered as player (player.id)}<article class="category"><div><h3>{player.name}</h3><p>{player.club}</p></div></article>{/each}</div>
    </section>
    <section class="panel form-panel"><h3 class="icon-label"><Icon name="user-plus" />{text.addPlayer}</h3><form onsubmit={savePlayer}>
      <label>{text.playerName}<input bind:value={name} required disabled={busy} /></label>
      <label>{text.club}<input bind:value={club} disabled={busy} /></label>
      <button class="primary" disabled={busy || !desktopAvailable || !playersLoaded}><Icon name="plus" size={18} />{busy ? text.saving : text.addPlayer}</button>
    </form></section>
    <section class="panel"><h3 class="icon-label"><Icon name="list" />{text.entries}</h3>
      {#if categoryId && !entriesLoaded && desktopAvailable}<p class="muted">{text.loading}</p>
      {:else if !entries.length}<p class="muted">{text.noEntries}</p>{/if}
      {#each entries as entry (entry.id)}<article class="category"><div><h3>{entry.members.map(p => p.name).join(' / ')}</h3><p>{entry.members.map(p => p.club).filter(Boolean).join(' / ')}</p></div></article>{/each}
    </section>
    <section class="panel form-panel"><h3>{text.register}</h3><form onsubmit={register}>
      <label>{text.chooseCategory}<Select label={text.chooseCategory} bind:value={categoryId} options={tournament.categories.map(item => ({ value: item.id, label: `${item.name} · ${text[item.discipline]}` }))} disabled={busy} /></label>
      <label>{text.firstPlayer}<Select label={text.firstPlayer} bind:value={first} options={available.map(player => ({ value: player.id, label: player.name + (player.club ? ` · ${player.club}` : '') }))} disabled={busy || !entriesLoaded} placeholder={text.choosePlayer} /></label>
      {#if category?.discipline === 'doubles'}<label>{text.secondPlayer}<Select label={text.secondPlayer} bind:value={second} options={available.filter(p => p.id !== first).map(player => ({ value: player.id, label: player.name + (player.club ? ` · ${player.club}` : '') }))} disabled={busy || !entriesLoaded} placeholder={text.choosePlayer} /></label>{/if}
      <button class="primary" disabled={busy || !desktopAvailable || !category || !playersLoaded || !entriesLoaded || !first || (category.discipline === 'doubles' && (!second || first === second))}><Icon name="check-circle" size={18} />{busy ? text.saving : text.register}</button>
    </form></section>
  </div>
</section>
