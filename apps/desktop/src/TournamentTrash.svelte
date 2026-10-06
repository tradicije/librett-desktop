<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, listTrashedTournaments, restoreTournament, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';

  let { language, active, busy = $bindable(false), onrestored }: {
    language: Language; active: boolean; busy?: boolean; onrestored: (tournament: Tournament) => void;
  } = $props();
  let text = $derived(messages[language]);
  let tournaments = $state<Tournament[]>([]);
  let search = $state('');
  let loading = $state(false);
  let loaded = $state(false);
  let error = $state<MessageKey | null>(null);
  let version = 0;
  let visible = $derived(tournaments.filter(t => t.name.toLocaleLowerCase(language).includes(search.trim().toLocaleLowerCase(language))));
  async function load() {
    if (!desktopAvailable) return;
    const current = ++version;
    loading = true; error = null;
    try {
      const rows = await listTrashedTournaments();
      if (current === version) { tournaments = rows; loaded = true; }
    } catch (cause) { if (current === version) error = errorKey(cause); }
    finally { if (current === version) loading = false; }
  }
  async function restore(tournament: Tournament) {
    if (busy || loading) return;
    busy = true; error = null;
    try {
      const restored = await restoreTournament(tournament.id);
      ++version;
      tournaments = tournaments.filter(t => t.id !== restored.id);
      onrestored(restored);
      window.dispatchEvent(new Event('librett-trash-updated'));
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  onMount(() => {
    const refresh = () => { if (!busy) void load(); };
    window.addEventListener('librett-trash-updated', refresh);
    return () => { ++version; window.removeEventListener('librett-trash-updated', refresh); };
  });
  $effect(() => { if (active && desktopAvailable) void load(); });
</script>

<div class="heading"><div><h1>{text.trash}</h1><p class="muted">{text.trashIntro}</p></div></div>
{#if error}<div class="error" role="alert">{text[error]} <button class="secondary" disabled={busy || loading} onclick={load}>{text.retry}</button></div>{/if}
<section class="panel">
  <label class="trash-search"><Icon name="search" size={18}/><input type="search" bind:value={search} aria-label={text.trash} placeholder={language === 'sr' ? 'Pretraži turnire u korpi…' : 'Search tournaments in trash…'}/></label>
  {#if loading}<p class="muted" role="status">{text.loading}</p>
  {:else if loaded && tournaments.length === 0}<p class="muted">{text.trashEmpty}</p>
  {:else}
    {#each visible as tournament (tournament.id)}
      <article class="player-profile trash-row">
        <span class="player-avatar" aria-hidden="true"><Icon name="trophy" size={20}/></span>
        <div class="trash-details"><strong>{tournament.name}</strong><p class="muted">{text.categories}: {tournament.categories.filter(c => !c.archived).length} · {text.tournamentPlayers}: {tournament.registered_count}</p></div>
        <button class="secondary icon-label" disabled={busy || loading} onclick={()=>restore(tournament)} aria-label={`${text.restoreTournament}: ${tournament.name}`}><Icon name="restore" size={18}/>{text.restoreTournament}</button>
      </article>
    {/each}
    {#if loaded && tournaments.length > 0 && visible.length === 0}<p class="muted">{language === 'sr' ? 'Nema turnira za ovu pretragu.' : 'No tournaments match this search.'}</p>{/if}
  {/if}
</section>

<style>
  .trash-search { display: flex; align-items: center; gap: 10px; margin-bottom: 18px; }
  .trash-search input { flex: 1; min-width: 0; }
  .trash-row { gap: 14px; }
  .trash-details { flex: 1; min-width: 0; overflow-wrap: anywhere; }
  .trash-details p { margin: 5px 0 0; }
  @media (max-width: 650px) { .trash-row { flex-wrap: wrap; } }
</style>
