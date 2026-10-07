<script lang="ts">
  import TournamentGuide from './TournamentGuide.svelte';
  import ActionHistory from './ActionHistory.svelte';
  import TournamentTrash from './TournamentTrash.svelte';
  import InfoRows from './InfoRows.svelte';
  import Backups, {type RestoreSource} from './Backups.svelte';
  import Select from './Select.svelte';
  import { confirmDiscard } from './confirmation';
  import { formatMoney } from './money';
  import { onMount, tick, untrack } from 'svelte';
  import CategoryDetail, { type CategoryTab } from './CategoryDetail.svelte';
  import CashDesk from './CashDesk.svelte';
  import CategoryEditor from './CategoryEditor.svelte';
  import TournamentEditor from './TournamentEditor.svelte';
  import TournamentOverview from './TournamentOverview.svelte';
  import PlayerDirectory from './PlayerDirectory.svelte';
  import PlayerEditor from './PlayerEditor.svelte';
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import darkIcon from '../../../assets/img/icon-dark.png';
  import lightIcon from '../../../assets/img/icon-light.png';
  import { type ThemePreference, type ResolvedTheme } from './theme';
  import { trashTournament, deleteCategory, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Category, type Tournament } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';

  import type { Route, TournamentTab, WorkspaceStatus } from './workspace';
  let { initialRoute, sidebarCollapsed = $bindable(false), language = $bindable(), theme = $bindable(), resolvedTheme, tournaments = $bindable(),
    status = $bindable(), externalLocked, active, pinned = false, onopen, wantsNewTab, onrestore }: {
    initialRoute: Route; sidebarCollapsed?: boolean; language: Language; theme: ThemePreference; resolvedTheme: ResolvedTheme;
    tournaments: Tournament[]; status: WorkspaceStatus; externalLocked: boolean; pinned?: boolean; active: boolean;
    onrestore:(source:RestoreSource)=>Promise<boolean>;
    onopen: (route: Route) => void; wantsNewTab: () => boolean;
  } = $props();
  const uid = $props.id();
  const platformName = /Win/.test(navigator.platform) ? 'Windows'
    : /Mac/.test(navigator.platform) ? 'MacOS'
    : /Linux/.test(navigator.platform) ? 'Linux' : null;
  let text = $derived(messages[language]);
  const tournamentTabs: TournamentTab[] = ['settings', 'overview', 'categories', 'cash'];
  let history = $state<Route[]>([untrack(() => initialRoute)]);
  let historyIndex = $state(0);
  let route = $derived(history[historyIndex]);
  let childDirty = $state(false);
  let inPlayers = $derived(route.view === 'players' || route.view === 'player-create' || route.view === 'player-edit');
  let pageLabel = $derived(route.view === 'guide' ? (language==='sr'?'Vodič':'Guide') : route.view === 'history' ? (language==='sr'?'Istorija':'History') : route.view === 'trash' ? text.trash : route.view === 'backups' ? (language==='sr'?'Rezervne kopije':'Backups') : route.view === 'tournament-create' ? text.addTournament : route.view === 'dashboard' ? text.dashboard : inPlayers ? text.playerTab : text.tournaments);
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
      case 'settings': return language === 'sr' ? 'Podešavanja' : 'Settings';
    }
  }
  let selectedCategory = $derived(activeCategories.find(c => c.id === route.categoryId));
  let breadcrumbs = $derived.by(() => {
    const items: { label: string; route: Route }[] = [];
    if(route.view==='guide')return [{label:language==='sr'?'Vodič':'Guide',route:{view:'guide' as const}}];
    if(route.view==='history')return [{label:language==='sr'?'Istorija':'History',route:{view:'history' as const}}];
    if(route.view==='trash')return [{label:text.trash,route:{view:'trash' as const}}];
    if(route.view==='backups')return [{label:language==='sr'?'Rezervne kopije':'Backups',route:{view:'backups' as const}}];
    if (inPlayers) {
      items.push({ label: text.playerTab, route: { view: 'players' } });
      if (route.view !== 'players') items.push({ label: route.view === 'player-create' ? text.addPlayer : text.editPlayer, route: { ...route } });
      return items;
    }
    items.push({ label: text.tournaments, route: { view: 'tournaments' } });
    if (selected) {
      items.push({ label: selected.name, route: { view: 'tournament', id: selected.id, tournamentTab: 'overview' } });
      if (route.view === 'tournament') {
        if (tournamentTab !== 'overview') items.push({ label: tournamentTabLabel(tournamentTab), route: { ...route } });
      } else {
        items.push({ label: text.categories, route: { view: 'tournament', id: selected.id, tournamentTab: 'categories' } });
        if (selectedCategory) items.push({ label: selectedCategory.name, route: { view: 'category', id: selected.id, categoryId: selectedCategory.id, categoryTab: 'settings' } });
        items.push({ label: route.view === 'category-create' ? text.addCategory : route.view === 'category-edit' ? text.editCategory : text[route.categoryTab ?? 'registrations'], route: { ...route } });
      }
    }
    if (route.view === 'tournament-create') items.push({ label: text.addTournament, route: { ...route } });
    return items;
  });
  let pendingTournament = $state<Tournament | null>(null);
  let tournamentDialog: HTMLDialogElement;
  let pendingCategory = $state<Category | null>(null);
  let categoryDialog: HTMLDialogElement;
  let busy = $state(false);
  let navigationLocked = $derived(busy || childBusy || externalLocked);
  let loading = $state(false);
  let loaded = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);

  let loadVersion = 0;
  async function load() {
    const version = ++loadVersion;
    loading = true;
    error = null;
    try { const current = await listTournaments(); if (version === loadVersion) { tournaments = current; loaded = true; } }
    catch (cause) { error = errorKey(cause); }
    finally { if (version === loadVersion) loading = false; }
  }
  let dirty = $derived(childDirty);
  let workspaceContext = $derived(route.view === 'category-create' ? `${selected?.name} · ${text.addCategory}` : route.view === 'category-edit' ? `${selected?.name} · ${selectedCategory?.name} · ${text.editCategory}` : route.view === 'category' && selectedCategory
    ? `${selected?.name} · ${selectedCategory.name} · ${text[route.categoryTab ?? 'registrations']}`
    : selected ? `${selected.name} · ${tournamentTabLabel(tournamentTab)}`
    : route.view === 'player-create' ? text.addPlayer : route.view === 'player-edit' ? text.editPlayer : pageLabel);
  let workspaceTitle = $derived(route.view === 'category-create' ? `${selected?.name ?? text.tournaments} | ${text.addCategory}` : route.view === 'category-edit' ? `${selectedCategory?.name ?? text.categories} | ${text.editCategory}` : route.view === 'category' ? `${selectedCategory?.name ?? text.categories} | ${text[route.categoryTab ?? 'registrations']}`
    : route.view === 'tournament' ? `${selected?.name ?? text.tournaments} | ${tournamentTabLabel(tournamentTab)}`
    : route.view === 'player-create' ? text.addPlayer : route.view === 'player-edit' ? text.editPlayer : pageLabel);
  $effect(() => { status = { title: workspaceTitle, context: workspaceContext, busy: busy || childBusy, dirty, view: route.view }; });
  onMount(() => {
    if (route.view !== 'dashboard' && desktopAvailable) void load();
    let refreshVersion=0;
    const refresh=async()=>{const version=++refreshVersion;try{const current=await listTournaments();if(version===refreshVersion)tournaments=current;}catch(cause){error=errorKey(cause);}};
    window.addEventListener('librett-completion-updated',refresh);
    window.addEventListener('librett-trash-updated',refresh);window.addEventListener('librett-registration-updated',refresh);
    return()=>{refreshVersion++;window.removeEventListener('librett-completion-updated',refresh);window.removeEventListener('librett-trash-updated',refresh);window.removeEventListener('librett-registration-updated',refresh);};
  });
  let wasActive = untrack(() => active);
  $effect(() => { if (active && !wasActive && route.view === 'tournaments' && desktopAvailable && !navigationLocked && !dirty) void load(); wasActive = active; });

  function resetView() { childDirty = false; error = null; notice = null; }
  export function getRoute(): Route { return { ...route }; }
  export function goHome() { navigate({ view: 'dashboard' }, true); }
  function navigate(next: Route, forceCurrent = false) {
    if (next.view === 'dashboard' && !pinned || pinned && !['dashboard', 'tournaments'].includes(next.view)) { onopen(next); return; }
    if (!forceCurrent && wantsNewTab()) { onopen(next); return; }
    if (navigationLocked) return;
    const current = history[historyIndex];
    if (current.view === next.view && current.id === next.id && current.categoryId === next.categoryId &&
      current.tournamentTab === next.tournamentTab && current.categoryTab === next.categoryTab) return;
    const apply = () => {
      if (navigationLocked || history[historyIndex] !== current) return;
      history = [...history.slice(0, historyIndex + 1), next]; historyIndex = history.length - 1;
      resetView();
      if (next.view !== 'dashboard' && desktopAvailable && (!loaded || next.view === 'tournaments') && !loading) void load();
    };
    if (dirty) void confirmDiscard().then(accepted => { if (accepted) apply(); }); else apply();
  }
  export function travel(delta: number) {
    if (navigationLocked || historyIndex + delta < 0 || historyIndex + delta >= history.length) return;
    const current = history[historyIndex];
    const apply = () => { if (navigationLocked || history[historyIndex] !== current) return; historyIndex += delta; resetView(); };
    if (dirty) void confirmDiscard().then(accepted => { if (accepted) apply(); }); else apply();
  }
  function openTournaments() { navigate({ view: 'tournaments' }); }
  function select(id: string | null) { navigate(id ? { view: 'tournament', id } : { view: 'tournaments' }); }
  function openTournamentTab(tab: TournamentTab) {
    if (selected) navigate({ view: 'tournament', id: selected.id, tournamentTab: tab });
  }

  async function confirmTournament(tournament: Tournament) {
    if (navigationLocked) return;
    pendingTournament = tournament; await tick(); tournamentDialog.showModal();
  }
  async function removeTournament() {
    if (!pendingTournament || navigationLocked) return;
    busy = true; error = null;
    try {
      const id = pendingTournament.id;
      await trashTournament(id);
      ++loadVersion; loading = false;
      tournaments = tournaments.filter(t => t.id !== id);
      tournamentDialog.close(); notice = 'tournamentTrashed';
      window.dispatchEvent(new Event('librett-trash-updated'));
    } catch (cause) { error = errorKey(cause); tournamentDialog.close(); }
    finally { busy = false; }
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
    <nav id={`${uid}-sidebar-nav`} aria-label={text.navigation}>
      <button data-open-tab class="nav-item" class:active={!inPlayers && !['backups','trash','history','guide'].includes(route.view)} aria-current={!inPlayers && !['backups','trash','history','guide'].includes(route.view) ? 'page' : undefined} disabled={navigationLocked} aria-label={text.tournaments} title={text.tournaments} onclick={openTournaments}><Icon name="trophy" />{#if !sidebarCollapsed}<span>{text.tournaments}</span>{/if}</button>
      <button data-open-tab class="nav-item" class:active={inPlayers} aria-current={inPlayers ? 'page' : undefined} disabled={navigationLocked} aria-label={text.playerTab} title={text.playerTab} onclick={() => navigate({ view: 'players' })}><Icon name="users" />{#if !sidebarCollapsed}<span>{text.playerTab}</span>{/if}</button>
      <button data-open-tab class="nav-item" class:active={route.view==='backups'} aria-current={route.view==='backups'?'page':undefined} disabled={navigationLocked} aria-label={language==='sr'?'Rezervne kopije':'Backups'} title={language==='sr'?'Rezervne kopije':'Backups'} onclick={()=>navigate({view:'backups'})}><Icon name="backup"/>{#if !sidebarCollapsed}<span>{language==='sr'?'Rezervne kopije':'Backups'}</span>{/if}</button>
      <button data-open-tab class="nav-item" class:active={route.view==='history'} aria-current={route.view==='history'?'page':undefined} disabled={navigationLocked} aria-label={language==='sr'?'Istorija':'History'} title={language==='sr'?'Istorija':'History'} onclick={()=>navigate({view:'history'})}><Icon name="history"/>{#if !sidebarCollapsed}<span>{language==='sr'?'Istorija':'History'}</span>{/if}</button>
      <button data-open-tab class="nav-item" class:active={route.view==='trash'} aria-current={route.view==='trash'?'page':undefined} disabled={navigationLocked} aria-label={text.trash} title={text.trash} onclick={()=>navigate({view:'trash'})}><Icon name="trash"/>{#if !sidebarCollapsed}<span>{text.trash}</span>{/if}</button>
    </nav>
    <nav class="guide-navigation" aria-label={language==='sr'?'Pomoć':'Help'}>
      <button data-open-tab class="nav-item" class:active={route.view==='guide'} aria-current={route.view==='guide'?'page':undefined} disabled={navigationLocked} aria-label={language==='sr'?'Vodič':'Guide'} title={language==='sr'?'Vodič':'Guide'} onclick={()=>navigate({view:'guide'})}><Icon name="guide"/>{#if !sidebarCollapsed}<span>{language==='sr'?'Vodič':'Guide'}</span>{/if}</button>
    </nav>
    <div class="sidebar-bottom">
      <span class="icon-label" title={text.local}><Icon name="desktop" size={16} />{#if !sidebarCollapsed}{text.local}{/if}</span>
      {#if !sidebarCollapsed}
        {#if platformName}<small>LibreTT for {platformName}</small>{/if}
        <small>© 2026 Aleksa Dimitrijević</small>
      {/if}
    </div>
  </aside>{/if}
  <main>
    <header class="app-toolbar">
      <div class="header-navigation">
        {#if mode === 'dashboard'}<img class="dashboard-logo" src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" />{/if}
        <div class="navigation-controls" aria-label={text.navigation}>
          <button class="icon-button" aria-label={text.goBack} title={text.goBack} disabled={navigationLocked || historyIndex === 0} onclick={() => travel(-1)}><Icon name="arrow-left" /></button>
          <button class="icon-button" aria-label={text.goForward} title={text.goForward} disabled={navigationLocked || historyIndex === history.length - 1} onclick={() => travel(1)}><Icon name="arrow-right" /></button>
        </div>
        {#if mode !== 'dashboard'}
          <nav class="breadcrumb" aria-label="Breadcrumb">
            {#each breadcrumbs as crumb, index}
              {#if index > 0}<span class="breadcrumb-divider" aria-hidden="true">/</span>{/if}
              <button data-open-tab disabled={navigationLocked} aria-current={index === breadcrumbs.length - 1 ? 'page' : undefined} title={crumb.label} onclick={() => navigate(crumb.route)}>{crumb.label}</button>
            {/each}
          </nav>
        {/if}
      </div>
      <div class="preferences">
        <label><Icon name={theme === 'system' ? 'desktop' : theme === 'dark' ? 'moon' : 'sun'} size={18} /><span class="preference-label">{text.theme}</span><Select label={text.theme} bind:value={theme} options={[{ value: 'system', label: text.themeSystem }, { value: 'light', label: text.themeLight }, { value: 'dark', label: text.themeDark }]} /></label>
        <label><Icon name="globe" size={18} /><span class="preference-label">{text.language}</span><Select label={text.language} bind:value={language} options={[{ value: 'sr', label: 'Srpski' }, { value: 'en', label: 'English' }]} /></label>
      </div>
    </header>
    <div class="workspace-scroll">
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
    {:else if route.view === 'guide'}
      <TournamentGuide {language}/>
    {:else if route.view === 'history'}
      <ActionHistory {language} {active} bind:busy={childBusy}/>
    {:else if route.view === 'trash'}
      <TournamentTrash {language} {active} bind:busy={childBusy} onrestored={(tournament) => { tournaments = [tournament, ...tournaments.filter(t => t.id !== tournament.id)]; notice = 'tournamentRestored'; }} />
    {:else if route.view === 'backups'}
      <Backups {language} bind:busy={childBusy} {onrestore}/>
    {:else if route.view === 'players'}
      <PlayerDirectory {language} {active} bind:busy={childBusy} onadd={() => navigate({ view: 'player-create' })} onedit={(id) => navigate({ view: 'player-edit', id })} ondeletebegin={() => { notice = null; }} />
    {:else if route.view === 'player-create' || route.view === 'player-edit'}
      {#key `${route.view}:${route.id ?? ''}`}
        <PlayerEditor {language} playerId={route.id} bind:busy={childBusy} bind:dirty={childDirty}
          onsaved={() => { navigate({ view: 'players' }); notice = 'playerSaved'; }} oncancel={() => navigate({ view: 'players' })} />
      {/key}
    {:else if route.view === 'tournament-create'}
      <TournamentEditor {language} bind:busy={childBusy} bind:dirty={childDirty} onsaved={(tournament) => { tournaments = [tournament, ...tournaments.filter(current => current.id !== tournament.id)]; navigate({ view: 'tournament', id: tournament.id }); notice = 'created'; }} oncancel={openTournaments} />
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
        {#key selected.id}<TournamentOverview {active} tournament={selected} {language} bind:busy={childBusy} oncategory={categoryId=>navigate({view:'category',id:selected.id,categoryId,categoryTab:'results'})} />{/key}
      {:else if tournamentTab === 'categories'}
        <section class="panel">
          <div class="section-heading"><div class="icon-label"><h2>{text.categories}</h2><span class="pill">{activeCategories.length}</span></div><button data-open-tab class="primary" disabled={navigationLocked || selected.completed} onclick={() => navigate({ view: 'category-create', id: selected.id })}><Icon name="plus" />{text.addCategory}</button></div>
          {#if activeCategories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each activeCategories as category (category.id)}
            <article class="player-profile category-list-row">
              <button data-open-tab class="category-open" disabled={navigationLocked} onclick={() => navigate({ view: 'category', id: selected.id, categoryId: category.id, categoryTab: 'registrations' })}><span class="player-avatar" aria-hidden="true"><Icon name={category.discipline === 'singles' ? 'user' : 'users'} size={18} /></span><span class="category-details"><strong>{category.name}{#if category.completed}<span class="pill">{language==='sr'?'Završeno':'Completed'}</span>{/if}</strong><InfoRows compact items={[{label:text.discipline,value:text[category.discipline]},{label:text.format,value:text[category.format]},{label:language==='sr'?'Kotizacija':'Entry fee',value:`${formatMoney(category.fee_minor, language)} ${text.feePerEntry}`}]} /></span></button>
              <div class="player-actions">
              <button data-open-tab class="secondary icon-label" disabled={navigationLocked || selected.completed || category.completed} aria-label={`${text.editCategory}: ${category.name}`} onclick={() => navigate({ view: 'category-edit', id: selected.id, categoryId: category.id })}><Icon name="edit" size={18} />{text.editCategory}</button>
              <button class="secondary icon-label" disabled={navigationLocked || selected.completed || category.completed} aria-label={`${text.deleteCategory}: ${category.name} · ${text[category.discipline]}`} onclick={() => confirmCategory(category)}><Icon name="trash" size={18} />{text.deleteCategory}</button>
              </div>
            </article>
          {/each}
        </section>
      {:else if tournamentTab === 'settings'}
        {#key selected.id}
          <TournamentEditor tournament={selected} {language} bind:busy={childBusy} bind:dirty={childDirty} onsaved={(updated) => { tournaments = tournaments.map(t => t.id === updated.id ? updated : t); navigate({ view: 'tournament', id: updated.id }); }} oncancel={() => openTournamentTab('overview')} />
        {/key}
      {:else if tournamentTab === 'cash'}
        {#key selected.id}<CashDesk {active} tournament={selected} {language} bind:busy={childBusy} />{/key}
      {/if}
    {:else if selectedId}
      <p class="banner">{text.tournamentUnavailable}</p><button data-open-tab class="secondary icon-label" disabled={navigationLocked} onclick={()=>navigate({view:'trash'})}><Icon name="trash" size={18}/>{text.trash}</button>
    {:else}
      <div class="heading"><div><h1>{text.tournaments}</h1><p class="muted">{text.intro}</p></div><button data-open-tab class="primary" disabled={navigationLocked || loading || !desktopAvailable} onclick={() => navigate({ view: 'tournament-create' })}><Icon name="plus" />{text.addTournament}</button></div>
      {#if loading}<p class="muted" role="status">{text.loading}</p>
      {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon"><Icon name="trophy" size={28} /></span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
      {:else}
        <div class="tournament-grid">
          {#each tournaments as tournament (tournament.id)}
            <div data-open-tab class="panel tournament-card" role="link" tabindex={navigationLocked ? -1 : 0} aria-disabled={navigationLocked} aria-label={tournament.name} onclick={() => { if (!navigationLocked) select(tournament.id); }} onkeydown={(event) => { if (event.target === event.currentTarget && (event.key === 'Enter' || event.key === ' ')) { event.preventDefault(); if (!navigationLocked) select(tournament.id); } }}>
              <span class="tournament-cover">{#if tournament.cover}<img src={tournament.cover} alt="" loading="lazy" />{:else}<Icon name="trophy" size={40} />{/if}</span>
              <div class="tournament-card-body">
                <h2>{tournament.name}</h2>{#if tournament.completed}<span class="pill">{language==='sr'?'Završeno':'Completed'}</span>{/if}
                <div class="tournament-card-meta">
                  <span><Icon name="users" size={16} />{tournament.registered_count} {text.tournamentPlayers.toLocaleLowerCase()}</span>
                  <span><Icon name="layer-group" size={16} />{text.categories}: {tournament.categories.filter(c => !c.archived).length}</span>
                </div>
                <div class="tournament-card-actions">
                <button data-open-tab class="secondary tournament-open" disabled={navigationLocked} onclick={(event) => { event.stopPropagation(); select(tournament.id); }} aria-label={`${language === 'sr' ? 'Otvori turnir' : 'Open tournament'}: ${tournament.name}`}>
                  {language === 'sr' ? 'Otvori turnir' : 'Open tournament'}<Icon name="arrow-right" size={18} />
                </button>
                <button class="icon-button" disabled={navigationLocked || loading || !desktopAvailable} title={text.trashTournament} aria-label={`${text.trashTournament}: ${tournament.name}`} onclick={(event) => { event.stopPropagation(); void confirmTournament(tournament); }}><Icon name="trash" size={18}/></button>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    {/if}
    </div>
    </div>
  </main>
</div>

<dialog class="confirm-dialog" bind:this={tournamentDialog} aria-labelledby={`${uid}-tournament-trash-title`} oncancel={(event)=>{if(busy)event.preventDefault();}} onclose={()=>{pendingTournament=null;}}>
  <h2 id={`${uid}-tournament-trash-title`}>{text.trashTournament}</h2><p><strong>{pendingTournament?.name}</strong></p><p>{text.trashHint}</p>
  <div class="dialog-actions"><button class="secondary" disabled={busy} onclick={()=>tournamentDialog.close()}>{text.cancelDelete}</button><button class="primary" disabled={busy} onclick={removeTournament}><Icon name="trash" size={16}/>{text.trashTournament}</button></div>
</dialog>

<dialog class="confirm-dialog" bind:this={categoryDialog} aria-labelledby={`${uid}-category-delete-title`} oncancel={(event) => { if (busy) event.preventDefault(); }} onclose={() => { pendingCategory = null; }}>
  <h2 id={`${uid}-category-delete-title`}>{text.deleteCategory}</h2><p><strong>{pendingCategory?.name}</strong></p><p class="muted">{pendingCategory ? text[pendingCategory.discipline] : ''}</p><p>{text.deleteCategoryHint}</p>
  <div class="dialog-actions"><button class="secondary" disabled={busy} onclick={() => categoryDialog.close()}>{text.cancelDelete}</button><button class="primary" disabled={busy} onclick={removeCategory}><Icon name="trash" size={16} />{text.deleteCategory}</button></div>
</dialog>

<style>
  .guide-navigation { margin-top:auto; padding-top:24px; }
  .sidebar-bottom { margin-top:18px; padding-top:18px; border-top:1px solid var(--border-subtle); }
  .tournament-card-actions { display: flex; align-items: center; gap: 8px; margin-top: auto; }
  .tournament-card-actions .tournament-open { flex: 1; margin-top: 0; width: auto; }
  .sidebar { width: 208px; padding-top: 0; }
  .sidebar-brand-row { display: flex; align-items: center; gap: 16px; height: var(--workspace-toolbar-height); flex-shrink: 0; margin-bottom: 30px; }
  .sidebar:not(.collapsed) .sidebar-brand-row { padding-inline: 12px; }
  .sidebar-brand-row .brand { margin: 0; padding: 0; width: auto; flex: 1; min-width: 0; }
  .sidebar-toggle { flex-shrink: 0; }
  .sidebar.collapsed { width: 68px; padding-left: 10px; padding-right: 10px; }
  .collapsed .sidebar-brand-row { justify-content: center; gap: 6px; margin-bottom: 22px; }
  .collapsed .sidebar-brand-row .brand { flex: none; width: 36px; }
  .compact-brand { border: 0; background: transparent; height: 36px; min-height: 36px; }
  .brand-icon { width: 36px; height: 36px; object-fit: contain; }
  .collapsed .nav-item { justify-content: center; padding: 9px; }
  .collapsed .sidebar-bottom { padding-left: 0; padding-right: 0; text-align: center; }
  .collapsed .sidebar-bottom .icon-label { justify-content: center; }
  @media (max-width: 1000px) { .sidebar:not(.collapsed) { width: 184px; } }
  @media (min-width: 651px) {
    .shell:not(.dashboard-shell) .app-toolbar { height: var(--workspace-toolbar-height); }
  }
  @media (max-width: 650px) {
    .sidebar, .sidebar.collapsed { width: 100%; padding-top: 14px; }
    .sidebar-brand-row, .collapsed .sidebar-brand-row { flex-direction: row; height: auto; margin: 0; }
    .sidebar-brand-row .brand { width: 115px; flex: none; }
  }
</style>
