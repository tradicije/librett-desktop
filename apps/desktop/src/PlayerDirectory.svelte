<script lang="ts">
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import { onMount, tick, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, listPlayers, deletePlayer, type Player } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { active = true, language, busy = $bindable(false), onadd, onedit, ondeletebegin }: {
    active?: boolean; language: Language; busy?: boolean; onadd: () => void; onedit: (id: string) => void; ondeletebegin?: () => void;
  } = $props();
  const uid = $props.id();
  let text = $derived(messages[language]);
  let players = $state<Player[]>([]);
  let loading = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state(false);
  let search = $state('');
  let pendingDelete = $state<Player | null>(null);
  let dialog: HTMLDialogElement;
  let filtered = $derived(players.filter(p => `${p.name} ${p.club} ${p.city} ${p.country} ${p.birth_year ?? ''}`.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase())));
  async function load() {
    loading = true; error = null;
    try { players = await listPlayers(); }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && desktopAvailable && !busy) void load(); wasActive = active; });
  async function confirmDelete(player: Player) {
    ondeletebegin?.();
    pendingDelete = player; error = null; notice = false;
    await tick(); dialog.showModal();
  }
  function cancelDelete() { if (!busy) dialog.close(); }
  async function remove() {
    if (!pendingDelete || busy) return;
    busy = true; error = null;
    try {
      await deletePlayer(pendingDelete.id);
      players = players.filter(p => p.id !== pendingDelete!.id);
      notice = true; dialog.close();
    } catch (cause) { error = errorKey(cause); dialog.close(); }
    finally { busy = false; }
  }
</script>

<div class="heading"><div><h1>{text.playerTab}</h1><p class="muted">{text.playerDirectoryIntro}</p></div>
  <button class="primary" disabled={busy || !desktopAvailable} data-open-tab onclick={onadd}><Icon name="user-plus" />{text.addPlayer}</button>
</div>
{#if error}<p class="error" role="alert">{text[error]}{#if error !== 'player_in_use'}<button onclick={load} disabled={loading || busy}>{text.retry}</button>{/if}</p>{/if}
<p role="status" class="notice">{notice ? text.playerDeleted : ''}</p>
<section class="panel player-directory">
  <div class="section-heading"><h2>{text.directory}</h2><span class="pill">{players.length}</span></div>
  <div class="directory-toolbar"><label class="search-field"><Icon name="search" size={17} /><input aria-label={text.searchPlayers} placeholder={text.searchPlayers} type="search" bind:value={search} /></label><span class="muted">{filtered.length} / {players.length}</span></div>
  {#if loading}<p role="status">{text.loading}</p>{:else if !filtered.length}<p class="muted">{text.noPlayers}</p>{/if}
  {#each filtered as player (player.id)}
    <article class="player-profile">
      {#if player.photo}<img class="player-photo" src={player.photo} alt={playerLabel(player)} />{:else}<span class="player-avatar" aria-hidden="true">{player.name.split(/\s+/).slice(0, 2).map(part => part[0]).join('').toLocaleUpperCase()}</span>{/if}
      <div class="player-details"><h3><PlayerName {player} /></h3>{#if player.birth_year !== null}<p class="birth-year"><span>{text.birthYear}</span><strong>{player.birth_year}</strong></p>{/if}
      </div>
      <div class="player-actions">
        <button class="secondary icon-label" disabled={busy} data-open-tab onclick={() => onedit(player.id)} aria-label={`${text.editPlayer}: ${playerLabel(player)}`}><Icon name="edit" size={18} />{text.editPlayer}</button>
        <button class="secondary icon-label" disabled={busy} onclick={() => confirmDelete(player)} aria-label={`${text.deletePlayer}: ${playerLabel(player)}`}><Icon name="trash" size={18} />{text.deletePlayer}</button>
      </div>
    </article>
  {/each}
</section>
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby={`${uid}-delete-player-title`} aria-describedby={`${uid}-delete-player-description`}
  oncancel={(event) => { if (busy) event.preventDefault(); }} onclose={() => { pendingDelete = null; }}>
  <h2 id={`${uid}-delete-player-title`}>{text.deletePlayer}</h2>
  <p id={`${uid}-delete-player-description`}>{text.deletePlayerPrompt} <strong><PlayerName player={pendingDelete} /></strong>?</p>
  <div class="dialog-actions">
    <button class="secondary" disabled={busy} onclick={cancelDelete}>{text.cancelDelete}</button>
    <button class="primary" disabled={busy} onclick={remove}><Icon name="trash" size={18} />{busy ? text.saving : text.deletePlayer}</button>
  </div>
</dialog>

<style>
  .birth-year { display:flex; gap:8px; align-items:baseline; }
  .birth-year > span { color:var(--text-muted); }
  .birth-year > strong { color:var(--text-primary); font-weight:500; font-variant-numeric:tabular-nums; }
</style>
