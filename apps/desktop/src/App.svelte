<script lang="ts">
  import Select from './Select.svelte';
  import { parseMoney, formatMoney } from './money';
  import { onMount, tick } from 'svelte';
  import CategoryDetail, { type CategoryTab } from './CategoryDetail.svelte';
  import CashDesk from './CashDesk.svelte';
  import PlayerDirectory from './PlayerDirectory.svelte';
  import PlayerEditor from './PlayerEditor.svelte';
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import { applyTheme, savedTheme, saveTheme, watchSystemTheme, type ThemePreference, type ResolvedTheme } from './theme';
  import { addCategory, deleteCategory, createTournament, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Category, type Tournament } from './api';
  import { messages, errorKey, savedLanguage, type Language, type MessageKey } from './i18n';

  let language = $state<Language>(savedLanguage());
  let theme = $state<ThemePreference>(savedTheme());
  let resolvedTheme = $state<ResolvedTheme>(document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light');
  let text = $derived(messages[language]);
  type Route = { view: 'dashboard' | 'tournaments' | 'players' | 'player-create' | 'player-edit' | 'tournament' | 'cash' | 'category'; id?: string; categoryId?: string; tab?: CategoryTab };
  let history = $state<Route[]>([{ view: 'dashboard' }]);
  let historyIndex = $state(0);
  let route = $derived(history[historyIndex]);
  let inPlayers = $derived(route.view === 'players' || route.view === 'player-create' || route.view === 'player-edit');
  let pageLabel = $derived(route.view === 'dashboard' ? text.dashboard : route.view === 'cash' ? text.cashDesk : inPlayers ? text.playerTab : text.tournaments);
  let mode = $derived(route.view === 'dashboard' ? 'dashboard' : 'tournaments');
  let childBusy = $state(false);
  let tournaments = $state<Tournament[]>([]);
  let selectedId = $derived((route.view === 'tournament' || route.view === 'cash' || route.view === 'category') ? route.id : null);
  let selected = $derived(tournaments.find(t => t.id === selectedId));
  let activeCategories = $derived(selected?.categories.filter(c => !c.archived) ?? []);
  let selectedCategory = $derived(activeCategories.find(c => c.id === route.categoryId));
  let pendingCategory = $state<Category | null>(null);
  let categoryDialog: HTMLDialogElement;
  let tournamentName = $state('');
  let categoryName = $state('');
  let categoryFee = $state('0');
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
    const fee = parseMoney(categoryFee, true);
    if (fee === null) { error = 'invalid_category_fee'; return; }
    busy = true; error = null; notice = null;
    try {
      const updated = await addCategory(selected.id, categoryName, discipline, format, fee);
      tournaments = tournaments.map(t => t.id === updated.id ? updated : t);
      categoryName = ''; categoryFee = '0'; notice = 'categorySaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  function resetView() {
    categoryName = ''; categoryFee = '0'; error = null; notice = null;
    discipline = 'singles'; format = 'groups_knockout';
  }
  function navigate(next: Route) {
    const current = history[historyIndex];
    if (current.view === next.view && current.id === next.id && current.categoryId === next.categoryId && current.tab === next.tab) return;
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

  async function confirmCategory(category: Category) {
    pendingCategory = category; await tick(); categoryDialog.showModal();
  }
  async function removeCategory() {
    if (!selected || !pendingCategory || navigationLocked) return;
    busy = true; error = null;
    try {
      const updated = await deleteCategory(selected.id, pendingCategory.id);
      tournaments = tournaments.map(t => t.id === updated.id ? updated : t);
      categoryDialog.close(); notice = 'categoryRemoved';
    } catch (cause) { error = errorKey(cause); categoryDialog.close(); }
    finally { busy = false; }
  }
</script>

<div class="shell" class:dashboard-shell={mode === 'dashboard'}>
  {#if mode !== 'dashboard'}<aside>
    <a class="brand" href="/" onclick={(event) => { event.preventDefault(); if (!navigationLocked) home(); }}><img src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" /></a>
    <p class="sidebar-label">{text.workspace}</p>
    <nav aria-label={text.navigation}>
      <button class="nav-item" class:active={!inPlayers} aria-current={!inPlayers ? 'page' : undefined} disabled={navigationLocked} onclick={openTournaments}><Icon name="trophy" />{text.tournaments}</button>
      <button class="nav-item" class:active={inPlayers} aria-current={inPlayers ? 'page' : undefined} disabled={navigationLocked} onclick={() => navigate({ view: 'players' })}><Icon name="users" />{text.playerTab}</button>
    </nav>
    <div class="sidebar-bottom"><span class="icon-label"><Icon name="desktop" size={16} />{text.local}</span><small>© 2026 Aleksa Dimitrijević</small></div>
  </aside>{/if}
  <main>
    <header class="app-toolbar">
      <div class="header-navigation">
        {#if mode === 'dashboard'}<img class="dashboard-logo" src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" />{/if}
        <div class="navigation-controls" aria-label={text.navigation}>
          <button class="icon-button" aria-label={text.goBack} title={text.goBack} disabled={navigationLocked || historyIndex === 0} onclick={() => travel(-1)}><Icon name="arrow-left" /></button>
          <button class="icon-button" aria-label={text.goForward} title={text.goForward} disabled={navigationLocked || historyIndex === history.length - 1} onclick={() => travel(1)}><Icon name="arrow-right" /></button>
          <button class="icon-button" aria-label={text.goHome} title={text.goHome} disabled={navigationLocked || mode === 'dashboard'} onclick={home}><Icon name="home" /></button>
        </div>
        {#if mode !== 'dashboard'}<div class="breadcrumb"><span>{text.tournaments}</span><span class="breadcrumb-divider">/</span><strong>{selected?.name ?? pageLabel}</strong>{#if route.view === 'category' && selectedCategory}<span class="breadcrumb-divider">/</span><span>{selectedCategory.name}</span>{/if}{#if route.view === 'cash'}<span class="breadcrumb-divider">/</span><span>{text.cashDesk}</span>{/if}</div>{/if}
      </div>
      <div class="preferences">
        <label><Icon name={theme === 'system' ? 'desktop' : theme === 'dark' ? 'moon' : 'sun'} size={18} /><span class="preference-label">{text.theme}</span><Select label={text.theme} bind:value={theme} options={[{ value: 'system', label: text.themeSystem }, { value: 'light', label: text.themeLight }, { value: 'dark', label: text.themeDark }]} /></label>
        <label><Icon name="globe" size={18} /><span class="preference-label">{text.language}</span><Select label={text.language} bind:value={language} options={[{ value: 'sr', label: 'Srpski' }, { value: 'en', label: 'English' }]} /></label>
      </div>
    </header>
    <div class="workspace-content" class:mode-content={mode === 'dashboard'}>
    {#if !desktopAvailable}<p class="banner">{text.preview}</p>{/if}
    {#if mode === 'tournaments' && error}<div class="error" role="alert">{text[error]} {#if !loaded && desktopAvailable}<button disabled={loading} onclick={load}>{text.retry}</button>{/if}</div>{/if}
    <div class="notice" role="status" aria-live="polite">{notice ? text[notice] : ''}</div>
    {#if mode === 'dashboard'}
      <div class="heading"><div><p class="eyebrow">{text.workspace}</p><h1>{text.chooseMode}</h1><p class="muted">{text.dashboardIntro}</p></div></div>
      <div class="mode-grid">
        <button class="mode-card" onclick={openTournaments}>
          <span class="mode-card-top"><span class="mode-icon"><Icon name="trophy" size={24} /></span></span>
          <span class="mode-title">{text.tournaments}</span>
          <span class="mode-description">{text.tournamentModeDescription}</span>
          <span class="mode-action">{text.openTournaments}<Icon name="arrow-right" size={18} /></span>
        </button>
        <button class="mode-card mode-unavailable" disabled aria-describedby="league-status">
          <span class="mode-card-top"><span class="mode-icon"><Icon name="list" size={24} /></span></span>
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
    {:else if selected && route.view === 'cash'}
      {#key selected.id}<CashDesk tournament={selected} {language} bind:busy={childBusy} />{/key}
    {:else if selected && route.view === 'category'}
      {#if selectedCategory}<CategoryDetail tournament={selected} category={selectedCategory} {language} tab={route.tab ?? 'registrations'} ontab={(tab) => navigate({ view: 'category', id: selected.id, categoryId: selectedCategory.id, tab })} bind:busy={childBusy} />
      {:else}<p class="banner">{text.categoryUnavailable}</p><button class="secondary" onclick={() => select(selected.id)}>{text.backToTournament}</button>{/if}
    {:else if selected}
      <div class="heading"><div><p class="eyebrow">{text.selected}</p><h1>{selected.name}</h1></div><button class="secondary icon-label" disabled={navigationLocked} onclick={() => navigate({ view: 'cash', id: selected.id })}><Icon name="cash" size={18} />{text.cashDesk}<Icon name="arrow-right" size={16} /></button></div>
      <div class="columns">
        <section class="panel">
          <div class="section-heading"><h2>{text.categories}</h2><span class="pill">{activeCategories.length}</span></div>
          {#if activeCategories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each activeCategories as category (category.id)}
            <div class="category-list-row"><button class="tournament category-open" disabled={navigationLocked} onclick={() => navigate({ view: 'category', id: selected.id, categoryId: category.id, tab: 'registrations' })}><span class="category-icon"><Icon name={category.discipline === 'singles' ? 'user' : 'users'} /></span><div class="row-content"><strong>{category.name}</strong><small>{text[category.discipline]} · {text[category.format]} · {formatMoney(category.fee_minor, language)} {text.feePerEntry}</small></div><Icon name="arrow-right" size={16} /></button><button class="icon-button" disabled={navigationLocked} aria-label={`${text.deleteCategory}: ${category.name} · ${text[category.discipline]}`} onclick={() => confirmCategory(category)}><Icon name="trash" size={16} /></button></div>
          {/each}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="layer-group" />{text.addCategory}</h2>
          <form onsubmit={saveCategory}>
            <label>{text.categoryName}<input bind:value={categoryName} required disabled={navigationLocked} /></label>
            <label>{text.categoryFee}<input bind:value={categoryFee} inputmode="decimal" required disabled={navigationLocked} placeholder="1000,00" aria-describedby="category-fee-hint" /></label><small class="muted" id="category-fee-hint">{text.categoryFeeHint}</small>
            <label>{text.discipline}<Select label={text.discipline} bind:value={discipline} options={[{ value: 'singles', label: text.singles }, { value: 'doubles', label: text.doubles }]} disabled={navigationLocked} /></label>
            <label>{text.format}<Select label={text.format} bind:value={format} options={[{ value: 'groups_knockout', label: text.groups_knockout }, { value: 'knockout', label: text.knockout }]} disabled={navigationLocked} /></label>
            <button class="primary" disabled={navigationLocked || !desktopAvailable}><Icon name="check-circle" size={18} />{busy ? text.saving : text.save}</button>
          </form>
        </section>
      </div>
    {:else}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.tournaments}</h1><p class="muted">{text.intro}</p></div></div>
      <div class="columns">
        <section class="panel tournament-directory">
          <div class="section-heading"><h2>{text.tournamentList}</h2><span class="pill">{tournaments.length}</span></div>
          {#if loading}<p class="muted" role="status">{text.loading}</p>
          {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon"><Icon name="trophy" size={28} /></span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
          {:else}
            {#each tournaments as tournament (tournament.id)}<button class="tournament" disabled={busy} onclick={() => select(tournament.id)}><span class="row-icon"><Icon name="trophy" size={18} /></span><div class="row-content"><strong>{tournament.name}</strong><small>{tournament.categories.filter(c => !c.archived).length} · {text.categories}</small></div><Icon name="arrow-right" /></button>{/each}
          {/if}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="trophy" />{text.newTournament}</h2><form onsubmit={create}><label>{text.tournamentName}<input bind:value={tournamentName} required disabled={busy || loading} /></label><button class="primary" disabled={busy || loading || !desktopAvailable || !loaded}><Icon name="plus" size={18} />{busy ? text.saving : text.create}</button></form></section>
      </div>
    {/if}
    </div>
  </main>
</div>

<dialog class="confirm-dialog" bind:this={categoryDialog} aria-labelledby="category-delete-title" oncancel={(event) => { if (busy) event.preventDefault(); }} onclose={() => { pendingCategory = null; }}>
  <h2 id="category-delete-title">{text.deleteCategory}</h2><p><strong>{pendingCategory?.name} · {pendingCategory ? text[pendingCategory.discipline] : ''}</strong></p><p>{text.deleteCategoryHint}</p>
  <div class="dialog-actions"><button class="secondary" disabled={busy} onclick={() => categoryDialog.close()}>{text.cancelDelete}</button><button class="primary" disabled={busy} onclick={removeCategory}><Icon name="trash" size={16} />{text.deleteCategory}</button></div>
</dialog>
