<script lang="ts">
  import { onMount } from 'svelte';
  import { addCategory, createTournament, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Tournament } from './api';
  import { messages, errorKey, savedLanguage, type Language, type MessageKey } from './i18n';

  let language = $state<Language>(savedLanguage());
  let text = $derived(messages[language]);
  let tournaments = $state<Tournament[]>([]);
  let selectedId = $state<string | null>(null);
  let selected = $derived(tournaments.find(t => t.id === selectedId));
  let tournamentName = $state('');
  let categoryName = $state('');
  let discipline = $state<Discipline>('singles');
  let format = $state<CompetitionFormat>('groups_knockout');
  let busy = $state(false);
  let loading = $state(false);
  let loaded = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);

  $effect(() => {
    document.documentElement.lang = language;
    try { localStorage.setItem('librett.language', language); } catch { /* Preference is optional. */ }
  });

  async function load() {
    loading = true;
    error = null;
    try { tournaments = await listTournaments(); loaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });

  async function create(event: SubmitEvent) {
    event.preventDefault();
    busy = true; error = null; notice = null;
    try {
      const tournament = await createTournament(tournamentName);
      tournaments = [tournament, ...tournaments];
      selectedId = tournament.id; tournamentName = ''; notice = 'created';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function saveCategory(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    busy = true; error = null; notice = null;
    try {
      const updated = await addCategory(selected.id, categoryName, discipline, format);
      tournaments = tournaments.map(t => t.id === updated.id ? updated : t);
      categoryName = ''; notice = 'categorySaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  function select(id: string | null) {
    selectedId = id; categoryName = ''; error = null; notice = null;
    discipline = 'singles'; format = 'groups_knockout';
  }
</script>

<div class="shell">
  <aside>
    <a class="brand" href="/" onclick={(event) => { event.preventDefault(); if (!busy) select(null); }}>Libre<span>TT</span><small>TABLE TENNIS</small></a>
    <nav aria-label={text.tournaments}>
      <button class="active" disabled={busy} onclick={() => select(null)}><span aria-hidden="true">◉</span> {text.tournaments}</button>
      <div class="future"><span>{text.leagues}</span><small>{text.later}</small></div>
    </nav>
    <div class="sidebar-bottom"><span class="dot"></span>{text.local}<small>© 2026 Aleksa Dimitrijević</small></div>
  </aside>
  <main>
    <header>
      <span class="eyebrow">{text.tournaments} / LibreTT</span>
      <label class="language">{text.language}<select bind:value={language}><option value="sr">Srpski</option><option value="en">English</option></select></label>
    </header>
    {#if !desktopAvailable}<p class="banner">{text.preview}</p>{/if}
    {#if error}<div class="error" role="alert">{text[error]} {#if !loaded && desktopAvailable}<button disabled={loading} onclick={load}>{text.retry}</button>{/if}</div>{/if}
    <div class="notice" role="status" aria-live="polite">{notice ? text[notice] : ''}</div>
    {#if selected}
      <button class="back" disabled={busy} onclick={() => select(null)}>← {text.back}</button>
      <div class="heading"><div><p class="eyebrow">{text.selected}</p><h1>{selected.name}</h1></div><span class="pill">{selected.categories.length} · {text.categories}</span></div>
      <div class="columns">
        <section class="panel">
          <h2>{text.categories}</h2>
          {#if selected.categories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each selected.categories as category (category.id)}
            <article class="category"><span class="category-icon" aria-hidden="true">{category.discipline === 'singles' ? '1' : '2'}</span><div><h3>{category.name}</h3><p>{text[category.discipline]} · {text[category.format]}</p></div></article>
          {/each}
        </section>
        <section class="panel form-panel"><h2>{text.addCategory}</h2>
          <form onsubmit={saveCategory}>
            <label>{text.categoryName}<input bind:value={categoryName} required disabled={busy} /></label>
            <label>{text.discipline}<select bind:value={discipline} disabled={busy}><option value="singles">{text.singles}</option><option value="doubles">{text.doubles}</option></select></label>
            <label>{text.format}<select bind:value={format} disabled={busy}><option value="groups_knockout">{text.groups_knockout}</option><option value="knockout">{text.knockout}</option></select></label>
            <button class="primary" disabled={busy || !desktopAvailable}>{busy ? text.saving : text.save}</button>
          </form>
        </section>
      </div>
    {:else}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.subtitle}</h1><p class="muted">{text.intro}</p></div></div>
      <div class="columns">
        <section class="panel">
          <div class="section-heading"><h2>{text.tournaments}</h2><span class="pill">{tournaments.length}</span></div>
          {#if loading}<p class="muted" role="status">{text.loading}</p>
          {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon" aria-hidden="true">↗</span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
          {:else}
            {#each tournaments as tournament (tournament.id)}<button class="tournament" disabled={busy} onclick={() => select(tournament.id)}><div><strong>{tournament.name}</strong><small>{tournament.categories.length} · {text.categories}</small></div><span aria-hidden="true">→</span></button>{/each}
          {/if}
        </section>
        <section class="panel form-panel"><h2>{text.newTournament}</h2><form onsubmit={create}><label>{text.tournamentName}<input bind:value={tournamentName} required disabled={busy || loading} /></label><button class="primary" disabled={busy || loading || !desktopAvailable || !loaded}>{busy ? text.saving : text.create}</button></form></section>
      </div>
    {/if}
  </main>
</div>
