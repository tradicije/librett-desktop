<script lang="ts">
  import { onMount, tick } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, listPlayers, deletePlayer, type Player } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { language, busy = $bindable(false), onadd, onedit, ondeletebegin }: {
    language: Language; busy?: boolean; onadd: () => void; onedit: (id: string) => void; ondeletebegin?: () => void;
  } = $props();
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
  <button class="primary" disabled={busy || !desktopAvailable} onclick={onadd}><Icon name="user-plus" />{text.addPlayer}</button>
</div>
{#if error}<p class="error" role="alert">{text[error]}{#if error !== 'player_in_use'}<button onclick={load} disabled={loading || busy}>{text.retry}</button>{/if}</p>{/if}
<p role="status" class="notice">{notice ? text.playerDeleted : ''}</p>
<section class="panel player-directory">
  <div class="section-heading"><h2>{text.directory}</h2><span class="pill">{players.length}</span></div>
  <label class="field-label">{text.searchPlayers}<input type="search" bind:value={search} /></label>
  {#if loading}<p role="status">{text.loading}</p>{:else if !filtered.length}<p class="muted">{text.noPlayers}</p>{/if}
  {#each filtered as player (player.id)}
    <article class="player-profile">
      {#if player.photo}<img class="player-photo" src={player.photo} alt={player.name} />{:else}<span class="player-avatar"><Icon name="user" size={28} /></span>{/if}
      <div class="player-details"><h3>{player.name}</h3><p>{[player.club, player.birth_year, player.city, player.country].filter(Boolean).join(' · ')}</p>
        {#if player.email}<p>{player.email}</p>{/if}{#if player.phone}<p>{player.phone}</p>{/if}{#if player.notes}<p class="player-notes">{player.notes}</p>{/if}
      </div>
      <div class="player-actions">
        <button class="secondary icon-label" disabled={busy} onclick={() => onedit(player.id)} aria-label={`${text.editPlayer}: ${player.name}`}><Icon name="edit" size={18} />{text.editPlayer}</button>
        <button class="secondary icon-label" disabled={busy} onclick={() => confirmDelete(player)} aria-label={`${text.deletePlayer}: ${player.name}`}><Icon name="trash" size={18} />{text.deletePlayer}</button>
      </div>
    </article>
  {/each}
</section>
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby="delete-player-title" aria-describedby="delete-player-description"
  oncancel={(event) => { if (busy) event.preventDefault(); }} onclose={() => { pendingDelete = null; }}>
  <h2 id="delete-player-title">{text.deletePlayer}</h2>
  <p id="delete-player-description">{text.deletePlayerPrompt} <strong>{pendingDelete?.name}</strong>?</p>
  <div class="dialog-actions">
    <button class="secondary" disabled={busy} onclick={cancelDelete}>{text.cancelDelete}</button>
    <button class="primary" disabled={busy} onclick={remove}><Icon name="trash" size={18} />{busy ? text.saving : text.deletePlayer}</button>
  </div>
</dialog>
