<script lang="ts">
  import Select from './Select.svelte';
  import { onMount } from 'svelte';
  import Players from './Players.svelte';
  import PlayerDirectory from './PlayerDirectory.svelte';
  import PlayerEditor from './PlayerEditor.svelte';
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import { applyTheme, savedTheme, saveTheme, watchSystemTheme, type ThemePreference, type ResolvedTheme } from './theme';
  import { addCategory, createTournament, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Tournament } from './api';
  import { messages, errorKey, savedLanguage, type Language, type MessageKey } from './i18n';

  let language = $state<Language>(savedLanguage());
  let theme = $state<ThemePreference>(savedTheme());
  let resolvedTheme = $state<ResolvedTheme>(document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light');
  let text = $derived(messages[language]);
  type Route = { view: 'dashboard' | 'tournaments' | 'players' | 'player-create' | 'player-edit' | 'tournament'; id?: string };
  let history = $state<Route[]>([{ view: 'dashboard' }]);
  let historyIndex = $state(0);
  let route = $derived(history[historyIndex]);
  let inPlayers = $derived(route.view === 'players' || route.view === 'player-create' || route.view === 'player-edit');
  let mode = $derived(route.view === 'dashboard' ? 'dashboard' : 'tournaments');
  let childBusy = $state(false);
  let tournaments = $state<Tournament[]>([]);
  let selectedId = $derived(route.view === 'tournament' ? route.id : null);
  let selected = $derived(tournaments.find(t => t.id === selectedId));
  let tournamentName = $state('');
  let categoryName = $state('');
  let discipline = $state<Discipline>('singles');
  let format = $state<CompetitionFormat>('groups_knockout');
  let busy = $state(false);
  let navigationLocked = $derived(busy || childBusy);
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
  $effect(() => { resolvedTheme = saveTheme(theme); });
  onMount(() => watchSystemTheme(() => {
    if (theme === 'system') resolvedTheme = applyTheme(theme);
  }));


  async function create(event: SubmitEvent) {
    event.preventDefault();
    busy = true; error = null; notice = null;
    try {
      const tournament = await createTournament(tournamentName);
      tournaments = [tournament, ...tournaments];
      navigate({ view: 'tournament', id: tournament.id }); tournamentName = ''; notice = 'created';
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
  function resetView() {
    categoryName = ''; error = null; notice = null;
    discipline = 'singles'; format = 'groups_knockout';
  }
  function navigate(next: Route) {
    const current = history[historyIndex];
    if (current.view === next.view && current.id === next.id) return;
    history = [...history.slice(0, historyIndex + 1), next];
    historyIndex = history.length - 1;
    resetView();
    if (next.view !== 'dashboard' && desktopAvailable && !loaded && !loading) void load();
  }
  function travel(delta: number) {
    if (navigationLocked || historyIndex + delta < 0 || historyIndex + delta >= history.length) return;
    historyIndex += delta;
    resetView();
  }
  function home() {
    navigate({ view: route.view === 'tournaments' ? 'dashboard' : 'tournaments' });
  }
  function openTournaments() { navigate({ view: 'tournaments' }); }
  function openDashboard() { navigate({ view: 'dashboard' }); }
  function select(id: string | null) { navigate(id ? { view: 'tournament', id } : { view: 'tournaments' }); }

</script>

<div class="shell" class:dashboard-shell={mode === 'dashboard'}>
  {#if mode !== 'dashboard'}<aside>
    <a class="brand" href="/" onclick={(event) => { event.preventDefault(); if (!navigationLocked) home(); }}><img src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" /></a>
    <nav aria-label={text.navigation}>
      <button class="nav-item" class:active={!inPlayers} aria-current={!inPlayers ? 'page' : undefined} disabled={navigationLocked} onclick={openTournaments}><Icon name="trophy" />{text.tournaments}</button>
      <button class="nav-item" class:active={inPlayers} aria-current={inPlayers ? 'page' : undefined} disabled={navigationLocked} onclick={() => navigate({ view: 'players' })}><Icon name="users" />{text.playerTab}</button>
    </nav>
    <div class="sidebar-bottom"><span class="icon-label"><Icon name="desktop" size={16} />{text.local}</span><small>© 2026 Aleksa Dimitrijević</small></div>
  </aside>{/if}
  <main>
    <header>
      <div class="header-navigation">
        {#if mode === 'dashboard'}<img class="dashboard-logo" src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" />{/if}
        <div class="navigation-controls" aria-label={text.navigation}>
          <button class="icon-button" aria-label={text.goBack} title={text.goBack} disabled={navigationLocked || historyIndex === 0} onclick={() => travel(-1)}><Icon name="arrow-left" /></button>
          <button class="icon-button" aria-label={text.goForward} title={text.goForward} disabled={navigationLocked || historyIndex === history.length - 1} onclick={() => travel(1)}><Icon name="arrow-right" /></button>
          <button class="icon-button" aria-label={text.goHome} title={text.goHome} disabled={navigationLocked || mode === 'dashboard'} onclick={home}><Icon name="home" /></button>
        </div>
      </div>
      <div class="preferences">
        <label><Icon name={theme === 'system' ? 'desktop' : theme === 'dark' ? 'moon' : 'sun'} size={18} />{text.theme}<Select label={text.theme} bind:value={theme} options={[{ value: 'system', label: text.themeSystem }, { value: 'light', label: text.themeLight }, { value: 'dark', label: text.themeDark }]} /></label>
        <label><Icon name="globe" size={18} />{text.language}<Select label={text.language} bind:value={language} options={[{ value: 'sr', label: 'Srpski' }, { value: 'en', label: 'English' }]} /></label>
      </div>
    </header>
    {#if !desktopAvailable}<p class="banner">{text.preview}</p>{/if}
    {#if mode === 'tournaments' && error}<div class="error" role="alert">{text[error]} {#if !loaded && desktopAvailable}<button disabled={loading} onclick={load}>{text.retry}</button>{/if}</div>{/if}
    <div class="notice" role="status" aria-live="polite">{notice ? text[notice] : ''}</div>
    {#if mode === 'dashboard'}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.chooseMode}</h1><p class="muted">{text.dashboardIntro}</p></div></div>
      <div class="mode-grid">
        <button class="mode-card" onclick={openTournaments}>
          <span class="mode-icon"><Icon name="trophy" size={32} /></span>
          <span class="mode-title">{text.tournaments}</span>
          <span class="mode-description">{text.tournamentModeDescription}</span>
          <span class="mode-action">{text.openTournaments}<Icon name="arrow-right" size={18} /></span>
        </button>
        <button class="mode-card mode-unavailable" disabled aria-describedby="league-status">
          <span class="mode-icon"><Icon name="list" size={32} /></span>
          <span class="mode-title">{text.leagues}</span>
          <span class="mode-description">{text.leagueModeDescription}</span>
          <span class="pill" id="league-status">{text.later}</span>
        </button>
      </div>
    {:else if route.view === 'players'}
      <PlayerDirectory {language} bind:busy={childBusy} onadd={() => navigate({ view: 'player-create' })} onedit={(id) => navigate({ view: 'player-edit', id })} ondeletebegin={() => { notice = null; }} />
    {:else if route.view === 'player-create' || route.view === 'player-edit'}
      {#key `${route.view}:${route.id ?? ''}`}
        <PlayerEditor {language} playerId={route.id} bind:busy={childBusy}
          onsaved={() => { navigate({ view: 'players' }); notice = 'playerSaved'; }} oncancel={() => navigate({ view: 'players' })} />
      {/key}
    {:else if selected}
      <div class="heading"><div><p class="eyebrow">{text.selected}</p><h1>{selected.name}</h1></div><span class="pill">{selected.categories.length} · {text.categories}</span></div>
      <div class="columns">
        <section class="panel">
          <h2>{text.categories}</h2>
          {#if selected.categories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each selected.categories as category (category.id)}
            <article class="category"><span class="category-icon"><Icon name={category.discipline === 'singles' ? 'user' : 'users'} /></span><div><h3>{category.name}</h3><p>{text[category.discipline]} · {text[category.format]}</p></div></article>
          {/each}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="layer-group" />{text.addCategory}</h2>
          <form onsubmit={saveCategory}>
            <label>{text.categoryName}<input bind:value={categoryName} required disabled={busy} /></label>
            <label>{text.discipline}<Select label={text.discipline} bind:value={discipline} options={[{ value: 'singles', label: text.singles }, { value: 'doubles', label: text.doubles }]} disabled={busy} /></label>
            <label>{text.format}<Select label={text.format} bind:value={format} options={[{ value: 'groups_knockout', label: text.groups_knockout }, { value: 'knockout', label: text.knockout }]} disabled={busy} /></label>
            <button class="primary" disabled={busy || !desktopAvailable}><Icon name="check-circle" size={18} />{busy ? text.saving : text.save}</button>
          </form>
        </section>
      </div>
      {#key selected.id}<Players tournament={selected} {language} bind:busy={childBusy} />{/key}
    {:else}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.subtitle}</h1><p class="muted">{text.intro}</p></div></div>
      <div class="columns">
        <section class="panel">
          <div class="section-heading"><h2>{text.tournaments}</h2><span class="pill">{tournaments.length}</span></div>
          {#if loading}<p class="muted" role="status">{text.loading}</p>
          {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon"><Icon name="trophy" size={28} /></span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
          {:else}
            {#each tournaments as tournament (tournament.id)}<button class="tournament" disabled={busy} onclick={() => select(tournament.id)}><div><strong>{tournament.name}</strong><small>{tournament.categories.length} · {text.categories}</small></div><Icon name="arrow-right" /></button>{/each}
          {/if}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="trophy" />{text.newTournament}</h2><form onsubmit={create}><label>{text.tournamentName}<input bind:value={tournamentName} required disabled={busy || loading} /></label><button class="primary" disabled={busy || loading || !desktopAvailable || !loaded}><Icon name="plus" size={18} />{busy ? text.saving : text.create}</button></form></section>
      </div>
    {/if}
  </main>
</div>
