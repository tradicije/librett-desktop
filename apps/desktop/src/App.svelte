<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Workspace from './Workspace.svelte';
  import Icon from './Icon.svelte';
  import { fadeOverflow } from './tab-label';
  import { savedSidebarCollapsed, saveSidebarCollapsed } from './sidebar';
  import { desktopAvailable, type Tournament } from './api';
  import { messages, savedLanguage, type Language } from './i18n';
  import { applyTheme, savedTheme, saveTheme, watchSystemTheme, type ThemePreference, type ResolvedTheme } from './theme';
  import type { Route, WorkspaceStatus } from './workspace';

  type Tab = { id: string; initialRoute: Route; status: WorkspaceStatus; instance?: { travel: (delta: number) => void; getRoute: () => Route; goHome: () => void }; scroll: number; focus?: HTMLElement };
  function makeTab(initialRoute: Route): Tab {
    return { id: crypto.randomUUID(), initialRoute, status: { title: '', busy: false, dirty: false, view: initialRoute.view }, scroll: 0 };
  }
  const homeId = 'home';
  let homeWorkspace = $state<Tab>({ ...makeTab({ view: 'dashboard' }), id: homeId });
  let tabs = $state<Tab[]>([]);
  let activeId = $state(homeId);
  let activeTab = $derived(activeId === homeId ? homeWorkspace : tabs.find(tab => tab.id === activeId)!);
  let tournaments = $state<Tournament[]>([]);
  let language = $state<Language>(savedLanguage());
  let sidebarCollapsed = $state(savedSidebarCollapsed());
  $effect(() => { saveSidebarCollapsed(sidebarCollapsed); });
  let theme = $state<ThemePreference>(savedTheme());
  let resolvedTheme = $state<ResolvedTheme>(document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light');
  let text = $derived(messages[language]);
  let nativePending = $state(false);
  let locked = $derived(nativePending || homeWorkspace.status.busy || tabs.some(tab => tab.status.busy));
  let closeId = $state<string | null>(null);
  let closeDialog: HTMLDialogElement;
  let contextMenu = $state<{ x: number; y: number; target?: HTMLElement; route?: Route } | null>(null);
  let menuElement = $state<HTMLDivElement>();
  async function showContext(event: MouseEvent) {
    if (locked || modalOpen()) return;
    const target = event.target instanceof Element ? event.target.closest<HTMLElement>('[data-open-tab], [data-duplicate-tab]') : null;
    if (!target || target instanceof HTMLButtonElement && target.disabled) return;
    event.preventDefault();
    const tab = tabs.find(tab => tab.id === target.dataset.duplicateTab);
    contextMenu = { x: Math.min(event.clientX, window.innerWidth - 230), y: Math.min(event.clientY, window.innerHeight - 56),
      ...(tab ? { route: tab.instance?.getRoute() } : { target }) };
    await tick(); menuElement?.querySelector<HTMLButtonElement>('button')?.focus();
  }
  function openContextTarget() {
    const menu = contextMenu; contextMenu = null;
    if (menu?.route) void openTab(menu.route);
    else menu?.target?.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, metaKey: true }));
  }
  let openIntent = false;
  let macOS = /Mac/.test(navigator.platform);
  const historySession = crypto.randomUUID();
  let restoring = false;
  const nativeState = (index: number) => ({ librettSession: historySession, librettIndex: index });
  const wantsNewTab = () => openIntent;
  const modalOpen = () => !!document.querySelector('dialog[open]');

  $effect(() => {
    document.documentElement.lang = language;
    try { localStorage.setItem('librett.language', language); } catch { /* Optional preference. */ }
  });
  $effect(() => { resolvedTheme = saveTheme(theme); });
  function workspaceScroller() {
    return document.getElementById(activeId === homeId ? 'home-workspace' : `work-panel-${activeId}`)?.querySelector<HTMLElement>('.workspace-scroll');
  }
  function restoreScroll(position: number) {
    workspaceScroller()?.scrollTo({ top: position, left: 0, behavior: 'instant' });
  }
  function remember() {
    activeTab.scroll = workspaceScroller()?.scrollTop ?? 0;
    const focused = document.activeElement;
    if (focused instanceof HTMLElement && focused.closest('.workspace-frame')) activeTab.focus = focused;
  }
  async function activate(id: string) {
    if (locked || modalOpen() || id === activeId) return;
    contextMenu = null; remember(); activeId = id;
    await tick();
    restoreScroll(activeTab.scroll);
    if (activeTab.focus?.isConnected) activeTab.focus.focus({ preventScroll: true });
    else document.getElementById(`work-tab-${id}`)?.focus({ preventScroll: true });
    document.getElementById(`work-tab-${id}`)?.scrollIntoView({ block: 'nearest', inline: 'nearest' });
  }
  async function openTab(route: Route = { view: 'dashboard' }) {
    if (locked || modalOpen()) return;
    if (route.view === 'dashboard') { await activate(homeId); homeWorkspace.instance?.goHome(); return; }
    contextMenu = null; remember(); const tab = makeTab(route); tabs.push(tab); activeId = tab.id;
    await tick(); restoreScroll(0);
    document.getElementById(`work-tab-${tab.id}`)?.focus({ preventScroll: true });
    document.getElementById(`work-tab-${tab.id}`)?.scrollIntoView({ block: 'nearest', inline: 'nearest' });
  }
  function home() { void openTab({ view: 'dashboard' }); }
  async function requestClose(id: string) {
    if (locked || modalOpen() || id === homeId) return;
    const tab = tabs.find(tab => tab.id === id);
    if (!tab) return;
    if (tab.status.dirty) { closeId = id; await tick(); closeDialog.showModal(); }
    else void closeTab(id);
  }
  async function closeTab(id: string) {
    if (locked) return;
    const index = tabs.findIndex(tab => tab.id === id);
    if (index < 0) return;
    if (id !== activeId) { tabs = tabs.filter(tab => tab.id !== id); return; }
    if (tabs.length === 1) { tabs = []; activeId = homeId; }
    else {
      if (id === activeId) activeId = tabs[index + 1]?.id ?? tabs[index - 1].id;
      tabs = tabs.filter(tab => tab.id !== id);
    }
    await tick(); restoreScroll(activeTab.scroll);
    document.getElementById(`work-tab-${activeId}`)?.focus({ preventScroll: true });
  }
  function travel(delta: number) {
    if (!locked && !modalOpen()) activeTab.instance?.travel(delta);
  }
  function onTabKey(event: KeyboardEvent, id: string) {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    if (tabs.length === 0) return;
    const index = tabs.findIndex(tab => tab.id === id);
    const next = event.key === 'Home' ? 0 : event.key === 'End' ? tabs.length - 1
      : (index + (event.key === 'ArrowRight' ? 1 : -1) + tabs.length) % tabs.length;
    void activate(tabs[next].id);
  }
  function drag(event: MouseEvent) {
    if (!desktopAvailable || event.button !== 0 || (event.target as HTMLElement).closest('button')) return;
    void getCurrentWindow().startDragging().catch(() => { /* Native title bar stays usable. */ });
  }
  onMount(() => {
    const stopTheme = watchSystemTheme(() => { if (theme === 'system') resolvedTheme = applyTheme(theme); });
    // Keep native history centered between two same-document entries. Native
    // gestures dispatch back/forward to the active workspace's own history;
    // returning to center prevents histories of different tabs from mixing.
    window.history.replaceState(nativeState(0), '');
    window.history.pushState(nativeState(1), '');
    window.history.pushState(nativeState(2), '');
    nativePending = true; restoring = true; window.history.go(-1);
    function onHistory(event: PopStateEvent) {
      const state = event.state;
      if (state?.librettSession !== historySession) return;
      const index = state.librettIndex;
      if (index === 1) { nativePending = false; restoring = false; return; }
      if (index !== 0 && index !== 2) return;
      if (!restoring && !homeWorkspace.status.busy && !tabs.some(tab => tab.status.busy) && !modalOpen()) activeTab.instance?.travel(index === 0 ? -1 : 1);
      nativePending = true; restoring = true; window.history.go(1 - index);
    }
    function onMouse(event: MouseEvent) {
      if (event.button !== 3 && event.button !== 4) return;
      event.preventDefault(); if (event.type === 'mouseup') travel(event.button === 3 ? -1 : 1);
    }
    function onClick(event: MouseEvent) {
      if (contextMenu && !(event.target instanceof Node && menuElement?.contains(event.target))) contextMenu = null;
      openIntent = event.metaKey || event.ctrlKey;
      queueMicrotask(() => { openIntent = false; });
    }
    function onMiddle(event: MouseEvent) {
      if (event.button !== 1 || locked || modalOpen()) return;
      const target = event.target instanceof Element ? event.target.closest<HTMLButtonElement>('[data-open-tab]') : null;
      if (!target || target.disabled) return;
      event.preventDefault();
      target.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true, metaKey: true }));
    }
    function onKey(event: KeyboardEvent) {
      if (event.key === 'Escape' && contextMenu) { event.preventDefault(); contextMenu = null; return; }
      if ((event.metaKey || event.ctrlKey) && !event.altKey && ['t', 'w'].includes(event.key.toLowerCase())) {
        event.preventDefault(); if (event.repeat) return;
        if (event.key.toLowerCase() === 't') void openTab(); else void requestClose(activeId);
      } else if (event.ctrlKey && event.key === 'Tab') {
        event.preventDefault(); const ids = [homeId, ...tabs.map(tab => tab.id)]; const index = ids.indexOf(activeId);
        void activate(ids[(index + (event.shiftKey ? -1 : 1) + ids.length) % ids.length]);
      }
    }
    window.addEventListener('contextmenu', showContext);
    window.addEventListener('popstate', onHistory);
    window.addEventListener('mousedown', onMouse);
    window.addEventListener('mouseup', onMouse);
    window.addEventListener('auxclick', onMouse);
    window.addEventListener('auxclick', onMiddle, true);
    window.addEventListener('click', onClick, true);
    window.addEventListener('keydown', onKey);
    return () => {
      stopTheme(); window.removeEventListener('contextmenu', showContext); window.removeEventListener('popstate', onHistory);
      window.removeEventListener('mousedown', onMouse); window.removeEventListener('mouseup', onMouse);
      window.removeEventListener('auxclick', onMouse); window.removeEventListener('auxclick', onMiddle, true);
      window.removeEventListener('click', onClick, true); window.removeEventListener('keydown', onKey);
    };
  });
</script>

<header class="work-titlebar" class:mac-titlebar={desktopAvailable && macOS}>
  <div class="window-drag-space" role="presentation" onmousedown={drag}></div>
  <button id="work-tab-home" aria-controls="home-workspace" class="titlebar-home" class:active={activeId === homeId} aria-pressed={activeId === homeId} disabled={locked} aria-label={text.goHome} title={text.goHome} onclick={home}><Icon name="home" size={18} /></button>
  <div class="work-tabs" role="tablist" aria-label={text.workTabs}>
    {#each tabs as tab (tab.id)}
      <div class="work-tab" class:active={tab.id === activeId}>
        <button id={`work-tab-${tab.id}`} data-duplicate-tab={tab.id} class="work-tab-select" role="tab" aria-selected={tab.id === activeId}
          aria-controls={`work-panel-${tab.id}`} tabindex={tab.id === activeId || activeId === homeId && tab === tabs[0] ? 0 : -1}
          disabled={locked} title={tab.status.context || tab.status.title || text.dashboard} onclick={() => activate(tab.id)}
          onkeydown={event => onTabKey(event, tab.id)} onauxclick={event => { if (event.button === 1) { event.preventDefault(); void requestClose(tab.id); } }}>
          {#if tab.status.dirty}<span class="work-tab-dot" aria-label={text.unsavedChanges}>●</span>{/if}<span class="work-tab-label" use:fadeOverflow>{tab.status.title || text.dashboard}</span>
        </button>
        <button class="work-tab-close" disabled={locked} aria-label={`${text.closeTab}: ${tab.status.title || text.dashboard}`} title={text.closeTab} onclick={() => requestClose(tab.id)}>×</button>
      </div>
    {/each}
  </div>
  <button class="new-work-tab" disabled={locked} aria-label={text.newTab} title={text.newTab} onclick={() => openTab()}>+</button>
  <div class="window-drag-space trailing-drag" role="presentation" onmousedown={drag}></div>
</header>
<div class="workspace-frame" id="home-workspace" hidden={activeId !== homeId}>
  <Workspace initialRoute={homeWorkspace.initialRoute} pinned bind:language bind:theme bind:sidebarCollapsed {resolvedTheme} bind:tournaments
    bind:status={homeWorkspace.status} externalLocked={nativePending} active={activeId === homeId}
    onopen={openTab} {wantsNewTab} bind:this={homeWorkspace.instance} />
</div>
{#each tabs as tab (tab.id)}
  <div class="workspace-frame" id={`work-panel-${tab.id}`} role="tabpanel" aria-labelledby={`work-tab-${tab.id}`} hidden={tab.id !== activeId}>
    <Workspace initialRoute={tab.initialRoute} bind:language bind:theme bind:sidebarCollapsed {resolvedTheme} bind:tournaments
      bind:status={tab.status} externalLocked={nativePending} active={tab.id === activeId} onopen={openTab} {wantsNewTab} bind:this={tab.instance} />
  </div>
{/each}
<dialog class="confirm-dialog" bind:this={closeDialog} aria-labelledby="close-work-tab-title" onclose={() => closeId = null}>
  <h2 id="close-work-tab-title">{text.closeTab}</h2><p>{text.closeUnsavedTab}</p>
  <div class="dialog-actions"><button class="secondary" onclick={() => closeDialog.close()}>{text.cancelDelete}</button><button onclick={() => { const id = closeId; closeDialog.close(); if (id) void closeTab(id); }}>{text.discardAndClose}</button></div>
</dialog>
{#if contextMenu}
  <div class="workspace-context-menu" bind:this={menuElement} role="menu" aria-label={text.workTabs} style:left={`${Math.max(0, contextMenu.x)}px`} style:top={`${Math.max(0, contextMenu.y)}px`}>
    <button role="menuitem" onclick={openContextTarget}>{text.openInNewTab}</button>
  </div>
{/if}

<style>
  .work-titlebar { height: 48px; position: fixed; top: 0; left: 0; right: 0; z-index: 30; display: flex; align-items: center; gap: 6px; background: var(--background); border-bottom: 1px solid var(--border); user-select: none; }
  .window-drag-space { align-self: stretch; width: 12px; flex-shrink: 0; }
  .mac-titlebar > .window-drag-space:first-child { width: 80px; }
  .trailing-drag { flex: 1; min-width: 24px; }
  .work-tabs { display: flex; overflow-x: auto; min-width: 0; max-width: calc(100% - 80px); height: 100%; align-items: center; gap: 4px; overflow-y: hidden; scrollbar-width: none; }
  .work-tabs::-webkit-scrollbar { display: none; }
  .work-tab { display: flex; flex: 0 0 180px; width: 180px; min-width: 180px; max-width: 180px; border: 1px solid transparent; border-radius: 6px; }
  .work-tab.active { background: var(--surface); border-color: var(--border); }
  .work-tab-select { background: transparent; border: 0; flex: 1; min-width: 0; display: flex; overflow: hidden; text-align: left; font-size: 12px; }
  .work-tab-dot { flex-shrink: 0; font-size: 9px; }
  .work-tab-label { display: block; min-width: 0; flex: 1; white-space: nowrap; overflow: hidden; }
  .work-tab-label:global(.overflowing) { -webkit-mask-image: linear-gradient(to right, #000 calc(100% - 20px), transparent); mask-image: linear-gradient(to right, #000 calc(100% - 20px), transparent); }
  .work-tab-close { border: 0; background: transparent; min-width: 28px; padding: 2px 7px; }
  .workspace-context-menu { position: fixed; z-index: 100; padding: 5px; background: var(--surface); border: 1px solid var(--border); border-radius: 6px; box-shadow: 0 8px 24px #0002; }
  .workspace-context-menu button { border: 0; width: 100%; }
  .titlebar-home.active { background: var(--surface); }
  .titlebar-home { border: 0; background: transparent; width: 32px; flex-shrink: 0; padding: 6px; }
  .new-work-tab { border: 0; background: transparent; font-size: 20px; width: 32px; flex-shrink: 0; }
  .workspace-frame { position: fixed; inset: 48px 0 0; overflow: hidden; }
  .workspace-frame[hidden] { display: none; }
  .workspace-frame :global(.shell) { height: 100%; min-height: 0; }
  .workspace-frame :global(aside) { top: 0; height: calc(100dvh - 48px); }
  .workspace-frame :global(.app-toolbar) { top: 0; }
  @media (max-width: 650px) { .workspace-frame :global(aside) { height: auto; } }
</style>
