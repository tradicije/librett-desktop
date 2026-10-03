<script lang="ts">
  import Select from './Select.svelte';
  import { formatMoney } from './money';
  import { onMount, tick, untrack } from 'svelte';
  import CategoryDetail, { type CategoryTab } from './CategoryDetail.svelte';
  import CashDesk from './CashDesk.svelte';
  import CategoryEditor from './CategoryEditor.svelte';
  import PlayerDirectory from './PlayerDirectory.svelte';
  import PlayerEditor from './PlayerEditor.svelte';
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import darkIcon from '../../../assets/img/icon-dark.png';
  import lightIcon from '../../../assets/img/icon-light.png';
  import { type ThemePreference, type ResolvedTheme } from './theme';
  import { deleteCategory, createTournament, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Category, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';

  import type { Route, TournamentTab, WorkspaceStatus } from './workspace';
  let { initialRoute, sidebarCollapsed = $bindable(false), language = $bindable(), theme = $bindable(), resolvedTheme, tournaments = $bindable(),
    status = $bindable(), externalLocked, active, pinned = false, onopen, wantsNewTab }: {
    initialRoute: Route; sidebarCollapsed?: boolean; language: Language; theme: ThemePreference; resolvedTheme: ResolvedTheme;
    tournaments: Tournament[]; status: WorkspaceStatus; externalLocked: boolean; pinned?: boolean; active: boolean;
    onopen: (route: Route) => void; wantsNewTab: () => boolean;
  } = $props();
  const uid = $props.id();
  let text = $derived(messages[language]);
  const tournamentTabs: TournamentTab[] = ['overview', 'categories', 'cash'];
  let history = $state<Route[]>([untrack(() => initialRoute)]);
  let historyIndex = $state(0);
  let route = $derived(history[historyIndex]);
  let childDirty = $state(false);
  let inPlayers = $derived(route.view === 'players' || route.view === 'player-create' || route.view === 'player-edit');
  let pageLabel = $derived(route.view === 'dashboard' ? text.dashboard : inPlayers ? text.playerTab : text.tournaments);
  let mode = $derived(route.view === 'dashboard' ? 'dashboard' : 'tournaments');
  let childBusy = $state(false);
  let selectedId = $derived((['tournament', 'category', 'category-create', 'category-edit'].includes(route.view)) ? route.id : null);
  let selected = $derived(tournaments.find(t => t.id === selectedId));
  let activeCategories = $derived(selected?.categories.filter(c => !c.archived) ?? []);
  let tournamentTab = $derived(route.view === 'tournament' ? route.tournamentTab ?? 'overview' : 'overview');
  function tournamentTabLabel(tab: TournamentTab): string {
    switch (tab) {
      case 'overview': return text.tournamentTab_overview;
      case 'categories': return text.tournamentTab_categories;
      case 'cash': return text.tournamentTab_cash;
    }
  }
  let selectedCategory = $derived(activeCategories.find(c => c.id === route.categoryId));
  let pendingCategory = $state<Category | null>(null);
  let categoryDialog: HTMLDialogElement;
  let tournamentName = $state('');
  let busy = $state(false);
  let navigationLocked = $derived(busy || childBusy || externalLocked);
  let loading = $state(false);
  let loaded = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);

  async function load() {
    loading = true;
    error = null;
    try { tournaments = await listTournaments(); loaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  let dirty = $derived(childDirty || !!tournamentName.trim());
  let workspaceContext = $derived(route.view === 'category-create' ? `${selected?.name} · ${text.addCategory}` : route.view === 'category-edit' ? `${selected?.name} · ${selectedCategory?.name} · ${text.editCategory}` : route.view === 'category' && selectedCategory
    ? `${selected?.name} · ${selectedCategory.name} · ${text[route.categoryTab ?? 'registrations']}`
    : selected ? `${selected.name} · ${tournamentTabLabel(tournamentTab)}`
    : route.view === 'player-create' ? text.addPlayer : route.view === 'player-edit' ? text.editPlayer : pageLabel);
  let workspaceTitle = $derived(route.view === 'category-create' ? `${selected?.name ?? text.tournaments} | ${text.addCategory}` : route.view === 'category-edit' ? `${selectedCategory?.name ?? text.categories} | ${text.editCategory}` : route.view === 'category' ? `${selectedCategory?.name ?? text.categories} | ${text[route.categoryTab ?? 'registrations']}`
    : route.view === 'tournament' ? `${selected?.name ?? text.tournaments} | ${tournamentTabLabel(tournamentTab)}`
    : route.view === 'player-create' ? text.addPlayer : route.view === 'player-edit' ? text.editPlayer : pageLabel);
  $effect(() => { status = { title: workspaceTitle, context: workspaceContext, busy: busy || childBusy, dirty, view: route.view }; });
  onMount(() => { if (route.view !== 'dashboard' && desktopAvailable) void load(); });

  async function create(event: SubmitEvent) {
    event.preventDefault();
    busy = true; error = null; notice = null;
    try {
      const tournament = await createTournament(tournamentName);
      tournaments = [tournament, ...tournaments];
      tournamentName = ''; busy = false; await tick();
      navigate({ view: 'tournament', id: tournament.id }); notice = 'created';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  function resetView() { childDirty = false; error = null; notice = null; }
  export function getRoute(): Route { return { ...route }; }
  export function goHome() { navigate({ view: 'dashboard' }, true); }
  function navigate(next: Route, forceCurrent = false) {
    if (next.view === 'dashboard' && !pinned || pinned && !['dashboard', 'tournaments'].includes(next.view)) { onopen(next); return; }
    if (!forceCurrent && wantsNewTab()) { onopen(next); return; }
    if (externalLocked) return;
    const current = history[historyIndex];
    if (current.view === next.view && current.id === next.id && current.categoryId === next.categoryId &&
      current.tournamentTab === next.tournamentTab && current.categoryTab === next.categoryTab) return;
    history = [...history.slice(0, historyIndex + 1), next];
    historyIndex = history.length - 1;
    resetView();
    if (next.view !== 'dashboard' && desktopAvailable && !loaded && !loading) void load();
  }
  export function travel(delta: number) {
    if (navigationLocked || historyIndex + delta < 0 || historyIndex + delta >= history.length) return;
    historyIndex += delta;
    resetView();
  }
  function openTournaments() { navigate({ view: 'tournaments' }); }
  function select(id: string | null) { navigate(id ? { view: 'tournament', id } : { view: 'tournaments' }); }
  function openTournamentTab(tab: TournamentTab) {
    if (selected) navigate({ view: 'tournament', id: selected.id, tournamentTab: tab });
  }

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
  {#if mode !== 'dashboard'}<aside class="sidebar" class:collapsed={sidebarCollapsed}>
    <div class="sidebar-brand-row">
      {#if sidebarCollapsed}
        <button class="brand compact-brand" aria-label={text.expandSidebar} title={text.expandSidebar} aria-expanded="false" aria-controls={`${uid}-sidebar-nav`} onclick={() => sidebarCollapsed = false}>
          <img class="brand-icon" src={resolvedTheme === 'dark' ? darkIcon : lightIcon} alt="LibreTT" width="1024" height="1024" />
        </button>
      {:else}
        <div class="brand"><img src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" /></div>
        <button class="icon-button sidebar-toggle" aria-label={text.collapseSidebar} title={text.collapseSidebar}
          aria-expanded="true" aria-controls={`${uid}-sidebar-nav`} onclick={() => sidebarCollapsed = true}>
          <Icon name="sidebar-collapse" size={18} />
        </button>
      {/if}
    </div>
    {#if !sidebarCollapsed}<p class="sidebar-label">{text.workspace}</p>{/if}
    <nav id={`${uid}-sidebar-nav`} aria-label={text.navigation}>
      <button data-open-tab class="nav-item" class:active={!inPlayers} aria-current={!inPlayers ? 'page' : undefined} disabled={navigationLocked} aria-label={text.tournaments} title={text.tournaments} onclick={openTournaments}><Icon name="trophy" />{#if !sidebarCollapsed}<span>{text.tournaments}</span>{/if}</button>
      <button data-open-tab class="nav-item" class:active={inPlayers} aria-current={inPlayers ? 'page' : undefined} disabled={navigationLocked} aria-label={text.playerTab} title={text.playerTab} onclick={() => navigate({ view: 'players' })}><Icon name="users" />{#if !sidebarCollapsed}<span>{text.playerTab}</span>{/if}</button>
    </nav>
    <div class="sidebar-bottom"><span class="icon-label" title={text.local}><Icon name="desktop" size={16} />{#if !sidebarCollapsed}{text.local}{/if}</span>{#if !sidebarCollapsed}<small>© 2026 Aleksa Dimitrijević</small>{/if}</div>
  </aside>{/if}
  <main>
    <header class="app-toolbar">
      <div class="header-navigation">
        {#if mode === 'dashboard'}<img class="dashboard-logo" src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" />{/if}
        <div class="navigation-controls" aria-label={text.navigation}>
          <button class="icon-button" aria-label={text.goBack} title={text.goBack} disabled={navigationLocked || historyIndex === 0} onclick={() => travel(-1)}><Icon name="arrow-left" /></button>
          <button class="icon-button" aria-label={text.goForward} title={text.goForward} disabled={navigationLocked || historyIndex === history.length - 1} onclick={() => travel(1)}><Icon name="arrow-right" /></button>
        </div>
        {#if mode !== 'dashboard'}<div class="breadcrumb"><span>{text.tournaments}</span><span class="breadcrumb-divider">/</span><strong>{selected?.name ?? pageLabel}</strong>{#if route.view === 'category' && selectedCategory}<span class="breadcrumb-divider">/</span><span>{selectedCategory.name}</span>{/if}</div>{/if}
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
        <button data-open-tab class="mode-card" onclick={openTournaments}>
          <span class="mode-card-top"><span class="mode-icon"><Icon name="trophy" size={24} /></span></span>
          <span class="mode-title">{text.tournaments}</span>
          <span class="mode-description">{text.tournamentModeDescription}</span>
          <span class="mode-action">{text.openTournaments}<Icon name="arrow-right" size={18} /></span>
        </button>
        <button class="mode-card mode-unavailable" disabled aria-describedby={`${uid}-league-status`}>
          <span class="mode-card-top"><span class="mode-icon"><Icon name="list" size={24} /></span></span>
          <span class="mode-title">{text.leagues}</span>
          <span class="mode-description">{text.leagueModeDescription}</span>
          <span class="pill" id={`${uid}-league-status`}>{text.later}</span>
        </button>
      </div>
    {:else if route.view === 'players'}
      <PlayerDirectory {language} {active} bind:busy={childBusy} onadd={() => navigate({ view: 'player-create' })} onedit={(id) => navigate({ view: 'player-edit', id })} ondeletebegin={() => { notice = null; }} />
    {:else if route.view === 'player-create' || route.view === 'player-edit'}
      {#key `${route.view}:${route.id ?? ''}`}
        <PlayerEditor {language} playerId={route.id} bind:busy={childBusy} bind:dirty={childDirty}
          onsaved={() => { navigate({ view: 'players' }); notice = 'playerSaved'; }} oncancel={() => navigate({ view: 'players' })} />
      {/key}
    {:else if selected && (route.view === 'category-create' || route.view === 'category-edit')}
      {#if route.view === 'category-create' || selectedCategory}
        {#key `${route.view}:${route.categoryId ?? ''}`}
          <CategoryEditor tournament={selected} category={route.view === 'category-edit' ? selectedCategory : undefined} {language} bind:busy={childBusy} bind:dirty={childDirty}
            onsaved={(updated, categoryId) => { tournaments = tournaments.map(t => t.id === updated.id ? updated : t); navigate({ view: 'category', id: updated.id, categoryId, categoryTab: 'settings' }); notice = 'categorySaved'; }}
            oncancel={() => navigate({ view: 'tournament', id: selected.id, tournamentTab: 'categories' })} />
        {/key}
      {:else}<p class="banner">{text.categoryUnavailable}</p>{/if}
    {:else if selected && route.view === 'category'}
      {#if selectedCategory}<CategoryDetail {active} tournament={selected} category={selectedCategory} {language} tab={route.categoryTab ?? 'registrations'} bind:dirty={childDirty} ontab={(tab) => navigate({ view: 'category', id: selected.id, categoryId: selectedCategory.id, categoryTab: tab })} bind:busy={childBusy} />
      {:else}<p class="banner">{text.categoryUnavailable}</p><button data-open-tab class="secondary" onclick={() => select(selected.id)}>{text.backToTournament}</button>{/if}
    {:else if selected}
      <div class="heading"><div><p class="eyebrow">{text.selected}</p><h1>{selected.name}</h1></div></div>
      <nav class="category-tabs" aria-label={text.tournamentSections}>
        {#each tournamentTabs as tab}
          <button data-open-tab class:active={tournamentTab === tab} aria-current={tournamentTab === tab ? 'page' : undefined} disabled={navigationLocked} onclick={() => openTournamentTab(tab)}>{tournamentTabLabel(tab)}</button>
        {/each}
      </nav>
      {#if tournamentTab === 'overview'}
        <div class="columns">
          <section class="panel"><div class="section-heading"><h2>{text.tournamentOverview}</h2></div><p class="muted">{text.tournamentOverviewIntro}</p></section>
          <section class="panel"><div class="section-heading"><h2>{text.categories}</h2><span class="pill">{activeCategories.length}</span></div><p class="muted">{activeCategories.length ? activeCategories.map(category => `${category.name} · ${text[category.discipline]}`).join(' / ') : text.noCategories}</p><button data-open-tab class="secondary" disabled={navigationLocked} onclick={() => openTournamentTab('categories')}>{text.openCategories}<Icon name="arrow-right" size={16} /></button></section>
        </div>
      {:else if tournamentTab === 'categories'}
        <section class="panel">
          <div class="section-heading"><div class="icon-label"><h2>{text.categories}</h2><span class="pill">{activeCategories.length}</span></div><button data-open-tab class="primary" disabled={navigationLocked} onclick={() => navigate({ view: 'category-create', id: selected.id })}><Icon name="plus" />{text.addCategory}</button></div>
          {#if activeCategories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each activeCategories as category (category.id)}
            <article class="player-profile category-list-row">
              <button data-open-tab class="category-open" disabled={navigationLocked} onclick={() => navigate({ view: 'category', id: selected.id, categoryId: category.id, categoryTab: 'registrations' })}><span class="player-avatar" aria-hidden="true"><Icon name={category.discipline === 'singles' ? 'user' : 'users'} size={18} /></span><span class="category-details"><strong>{category.name}</strong><span>{text[category.discipline]} · {text[category.format]} · {formatMoney(category.fee_minor, language)} {text.feePerEntry}</span></span></button>
              <div class="player-actions">
              <button data-open-tab class="secondary icon-label" disabled={navigationLocked} aria-label={`${text.editCategory}: ${category.name}`} onclick={() => navigate({ view: 'category-edit', id: selected.id, categoryId: category.id })}><Icon name="edit" size={18} />{text.editCategory}</button>
              <button class="secondary icon-label" disabled={navigationLocked} aria-label={`${text.deleteCategory}: ${category.name} · ${text[category.discipline]}`} onclick={() => confirmCategory(category)}><Icon name="trash" size={18} />{text.deleteCategory}</button>
              </div>
            </article>
          {/each}
        </section>
      {:else if tournamentTab === 'cash'}
        {#key selected.id}<CashDesk {active} tournament={selected} {language} bind:busy={childBusy} />{/key}
      {/if}
    {:else}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.tournaments}</h1><p class="muted">{text.intro}</p></div></div>
      <div class="columns">
        <section class="panel tournament-directory">
          <div class="section-heading"><h2>{text.tournamentList}</h2><span class="pill">{tournaments.length}</span></div>
          {#if loading}<p class="muted" role="status">{text.loading}</p>
          {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon"><Icon name="trophy" size={28} /></span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
          {:else}
            {#each tournaments as tournament (tournament.id)}<button data-open-tab class="tournament" disabled={busy} onclick={() => select(tournament.id)}><span class="row-icon"><Icon name="trophy" size={18} /></span><div class="row-content"><strong>{tournament.name}</strong><small>{tournament.categories.filter(c => !c.archived).length} · {text.categories}</small></div><Icon name="arrow-right" /></button>{/each}
          {/if}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="trophy" />{text.newTournament}</h2><form onsubmit={create}><label>{text.tournamentName}<input bind:value={tournamentName} required disabled={busy || loading} /></label><button class="primary" disabled={busy || loading || !desktopAvailable || !loaded}><Icon name="plus" size={18} />{busy ? text.saving : text.create}</button></form></section>
      </div>
    {/if}
    </div>
  </main>
</div>

<dialog class="confirm-dialog" bind:this={categoryDialog} aria-labelledby={`${uid}-category-delete-title`} oncancel={(event) => { if (busy) event.preventDefault(); }} onclose={() => { pendingCategory = null; }}>
  <h2 id={`${uid}-category-delete-title`}>{text.deleteCategory}</h2><p><strong>{pendingCategory?.name} · {pendingCategory ? text[pendingCategory.discipline] : ''}</strong></p><p>{text.deleteCategoryHint}</p>
  <div class="dialog-actions"><button class="secondary" disabled={busy} onclick={() => categoryDialog.close()}>{text.cancelDelete}</button><button class="primary" disabled={busy} onclick={removeCategory}><Icon name="trash" size={16} />{text.deleteCategory}</button></div>
</dialog>

<style>
  .sidebar { width: 208px; }
  .sidebar-brand-row { display: flex; align-items: center; gap: 16px; margin-bottom: 38px; }
  .sidebar-brand-row .brand { margin: 0; padding: 0; width: auto; flex: 1; min-width: 0; }
  .sidebar-toggle { flex-shrink: 0; }
  .sidebar.collapsed { width: 68px; padding-left: 10px; padding-right: 10px; }
  .collapsed .sidebar-brand-row { flex-direction: column; gap: 6px; margin-bottom: 24px; }
  .collapsed .sidebar-brand-row .brand { flex: none; width: 36px; }
  .compact-brand { border: 0; background: transparent; height: 36px; min-height: 36px; }
  .brand-icon { width: 36px; height: 36px; object-fit: contain; }
  .collapsed .nav-item { justify-content: center; padding: 9px; }
  .collapsed .sidebar-bottom { padding-left: 0; padding-right: 0; text-align: center; }
  .collapsed .sidebar-bottom .icon-label { justify-content: center; }
  @media (max-width: 1000px) { .sidebar:not(.collapsed) { width: 184px; } }
  @media (max-width: 650px) {
    .sidebar, .sidebar.collapsed { width: 100%; }
    .sidebar-brand-row, .collapsed .sidebar-brand-row { flex-direction: row; margin: 0; }
    .sidebar-brand-row .brand { width: 115px; flex: none; }
  }
</style>
