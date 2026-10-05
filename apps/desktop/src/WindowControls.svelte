<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Icon from './Icon.svelte';
  import type { Language } from './i18n';

  let { language, platform }: { language: Language; platform: 'linux' | 'windows' } = $props();
  const window = getCurrentWindow();
  let maximized = $state(false);
  let fullscreen = $state(false);
  let working = $state(false);
  let error = $state(false);
  let sr = $derived(language === 'sr');
  let maximizeLabel = $derived(maximized ? (sr ? 'Vrati veličinu prozora' : 'Restore window') : (sr ? 'Maksimizuj prozor' : 'Maximize window'));
  const directions = ['North', 'South', 'East', 'West', 'NorthEast', 'NorthWest', 'SouthEast', 'SouthWest'] as const;

  async function refresh() {
    [maximized, fullscreen] = await Promise.all([window.isMaximized(), window.isFullscreen()]);
  }
  $effect(() => {
    document.documentElement.classList.toggle('window-flush', maximized || fullscreen);
  });
  onMount(() => {
    document.documentElement.classList.add(`${platform}-window`);
    let disposed = false;
    let stop: (() => void) | undefined;
    void refresh().catch(() => { error = true; });
    void window.onResized(() => { void refresh().catch(() => { error = true; }); })
      .then(unlisten => { if (disposed) unlisten(); else stop = unlisten; })
      .catch(() => { error = true; });
    return () => { disposed = true; stop?.(); document.documentElement.classList.remove(`${platform}-window`, 'window-flush'); };
  });
  async function act(action: 'minimize' | 'maximize' | 'close') {
    if (working) return;
    working = true; error = false;
    try {
      if (action === 'minimize') await window.minimize();
      else if (action === 'maximize') { await window.toggleMaximize(); await refresh(); }
      // close() sends CloseRequested, preserving the app's dirty-form guard.
      else await window.close();
    } catch { error = true; }
    finally { working = false; }
  }
  function resize(event: MouseEvent, direction: typeof directions[number]) {
    if (event.button !== 0) return;
    event.preventDefault();
    void window.startResizeDragging(direction).catch(() => { error = true; });
  }
</script>

<div class="window-controls" class:windows-controls={platform === 'windows'} role="group" aria-label={sr ? 'Kontrole prozora' : 'Window controls'}>
  <button disabled={working} aria-label={sr ? 'Minimizuj prozor' : 'Minimize window'} title={sr ? 'Minimizuj prozor' : 'Minimize window'} onclick={() => act('minimize')}><Icon name="minimize" size={14} /></button>
  <button disabled={working || fullscreen} aria-label={maximizeLabel} title={maximizeLabel} onclick={() => act('maximize')}><Icon name={maximized ? 'restore-window' : 'maximize'} size={13} /></button>
  <button class="window-close" disabled={working} aria-label={sr ? 'Zatvori prozor' : 'Close window'} title={sr ? 'Zatvori prozor' : 'Close window'} onclick={() => act('close')}><Icon name="close" size={15} /></button>
</div>
<div class="window-outline" aria-hidden="true"></div>
{#if error}<p class="window-control-error" role="alert">{sr ? 'Kontrola prozora nije uspela. Pokušaj ponovo.' : 'Window action failed. Try again.'}</p>{/if}
{#if !maximized && !fullscreen}
  {#each directions as direction}
    <div class="resize-edge" class:north={direction === 'North'} class:south={direction === 'South'} class:east={direction === 'East'} class:west={direction === 'West'} class:north-east={direction === 'NorthEast'} class:north-west={direction === 'NorthWest'} class:south-east={direction === 'SouthEast'} class:south-west={direction === 'SouthWest'} role="presentation" onmousedown={event => resize(event, direction)}></div>
  {/each}
{/if}

<style>
  .window-controls { display: flex; align-items: center; align-self: stretch; flex-shrink: 0; gap: 4px; margin-left: 0; padding: 0 10px 0 12px; border-left: 1px solid var(--border-subtle); }
  .window-controls button { width: 28px; height: 28px; min-height: 28px; border: 1px solid transparent; border-radius: 7px; padding: 0; background: transparent; color: var(--text-secondary); }
  .window-controls button:hover:enabled { background: var(--surface-hover); border-color: var(--border); color: var(--text-primary); }
  .window-controls button:active:enabled { background: var(--primary-subtle); border-color: var(--primary); color: var(--primary); }
  .window-controls .window-close:hover:enabled { background: #C73E4D; border-color: #C73E4D; color: #FFFFFF; }
  .window-controls .window-close:active:enabled { background: #AE303E; border-color: #AE303E; color: #FFFFFF; }
  .window-controls button:focus-visible { outline-offset: 1px; }
  .windows-controls { gap: 0; padding: 0; border-left: 0; }
  .windows-controls button { width: 46px; height: 100%; min-height: 28px; border: 0; border-radius: 0; }
  .windows-controls button:focus-visible { outline-offset: -3px; }
  .window-outline { position: fixed; inset: 0; border: 1px solid var(--border); border-radius: var(--window-radius); z-index: 115; pointer-events: none; }
  :global(.window-flush) .window-outline { display: none; }
  .window-control-error { position: fixed; top: 52px; right: 12px; z-index: 110; margin: 0; padding: 10px 14px; background: var(--surface); color: var(--text-primary); border: 1px solid var(--border); border-radius: 6px; font-size: 12px; }
  .resize-edge { position: fixed; z-index: 120; }
  .north, .south { left: 8px; right: 8px; height: 4px; cursor: ns-resize; }
  .north { top: 0; } .south { bottom: 0; }
  .east, .west { top: 8px; bottom: 8px; width: 4px; cursor: ew-resize; }
  .east { right: 0; } .west { left: 0; }
  .north-east, .north-west, .south-east, .south-west { width: 8px; height: 8px; }
  .north-east { top: 0; right: 0; cursor: nesw-resize; }
  .north-west { top: 0; left: 0; cursor: nwse-resize; }
  .south-east { bottom: 0; right: 0; cursor: nwse-resize; }
  .south-west { bottom: 0; left: 0; cursor: nesw-resize; }
</style>
