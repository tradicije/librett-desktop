<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Icon from './Icon.svelte';
  import type { Language } from './i18n';

  let { language }: { language: Language } = $props();
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
  onMount(() => {
    let disposed = false;
    let stop: (() => void) | undefined;
    void refresh().catch(() => { error = true; });
    void window.onResized(() => { void refresh().catch(() => { error = true; }); })
      .then(unlisten => { if (disposed) unlisten(); else stop = unlisten; })
      .catch(() => { error = true; });
    return () => { disposed = true; stop?.(); };
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

<div class="window-controls" role="group" aria-label={sr ? 'Kontrole prozora' : 'Window controls'}>
  <button disabled={working} aria-label={sr ? 'Minimizuj prozor' : 'Minimize window'} title={sr ? 'Minimizuj prozor' : 'Minimize window'} onclick={() => act('minimize')}><Icon name="minimize" size={17} /></button>
  <button disabled={working || fullscreen} aria-label={maximizeLabel} title={maximizeLabel} onclick={() => act('maximize')}><Icon name={maximized ? 'restore-window' : 'maximize'} size={16} /></button>
  <button class="window-close" disabled={working} aria-label={sr ? 'Zatvori prozor' : 'Close window'} title={sr ? 'Zatvori prozor' : 'Close window'} onclick={() => act('close')}><Icon name="close" size={18} /></button>
</div>
{#if error}<p class="window-control-error" role="alert">{sr ? 'Kontrola prozora nije uspela. Pokušaj ponovo.' : 'Window action failed. Try again.'}</p>{/if}
{#if !maximized && !fullscreen}
  {#each directions as direction}
    <div class="resize-edge" class:north={direction === 'North'} class:south={direction === 'South'} class:east={direction === 'East'} class:west={direction === 'West'} class:north-east={direction === 'NorthEast'} class:north-west={direction === 'NorthWest'} class:south-east={direction === 'SouthEast'} class:south-west={direction === 'SouthWest'} role="presentation" onmousedown={event => resize(event, direction)}></div>
  {/each}
{/if}

<style>
  .window-controls { display: flex; align-items: stretch; align-self: stretch; flex-shrink: 0; margin-left: 2px; padding-right: 4px; border-left: 1px solid var(--border-subtle); }
  .window-controls button { width: 40px; border: 0; border-radius: 0; padding: 0; background: transparent; color: var(--text-secondary); }
  .window-controls button:hover:enabled { background: var(--surface-hover); color: var(--text-primary); }
  .window-controls .window-close:hover:enabled { background: #C73E4D; color: #FFFFFF; }
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
