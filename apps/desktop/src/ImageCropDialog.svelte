<script lang="ts">
  import { onDestroy } from 'svelte';
  import Icon from './Icon.svelte';
  import { readBoundedImage } from './image-limits';
  import type { Language } from './i18n';
  let { language }: { language: Language } = $props();
  const uid = $props.id();
  let dialog: HTMLDialogElement;
  let canvas: HTMLCanvasElement;
  let image = $state<HTMLImageElement | null>(null);
  let cover = $state(false);
  let zoom = $state(1);
  let horizontal = $state(0);
  let vertical = $state(0);
  let error = $state(false);
  let resolve: ((value: string | null) => void) | undefined;
  let result: string | null = null;
  let drag: { id: number; x: number; y: number; horizontal: number; vertical: number } | null = null;
  let loading = false;
  let disposed = false;
  function geometry() {
    if (!image) return null;
    const ratio = cover ? 16 / 9 : 1;
    const width = Math.min(image.naturalWidth, image.naturalHeight * ratio) / zoom;
    const height = width / ratio;
    const left = (image.naturalWidth - width) * (horizontal + 1) / 2;
    const top = (image.naturalHeight - height) * (vertical + 1) / 2;
    return { width, height, left, top };
  }
  function paint() {
    const crop = geometry();
    if (!canvas || !image || !crop) return;
    canvas.width = cover ? 1024 : 512;
    canvas.height = cover ? 576 : 512;
    const context = canvas.getContext('2d');
    if (!context) throw new Error('Canvas unavailable');
    context.fillStyle = '#fff'; context.fillRect(0, 0, canvas.width, canvas.height);
    context.drawImage(image, crop.left, crop.top, crop.width, crop.height, 0, 0, canvas.width, canvas.height);
  }
  $effect(() => { image; cover; zoom; horizontal; vertical; paint(); });
  export async function crop(file: File, isCover = false): Promise<string | null> {
    if (loading || resolve || disposed || document.querySelector('dialog[open]')) return null;
    loading = true;
    try {
      const decoded = await readBoundedImage(file);
      if (disposed) return null;
      image = decoded; cover = isCover; zoom = 1; horizontal = 0; vertical = 0; error = false; result = null;
      paint();
      return await new Promise<string | null>(done => { resolve = done; dialog.showModal(); });
    } finally { loading = false; }
  }
  function finish() {
    const done = resolve; resolve = undefined; image = null; drag = null; done?.(result);
  }
  function apply() {
    try {
      paint();
      const encoded = canvas.toDataURL('image/jpeg', 0.8);
      if (encoded.length > (cover ? 1_000_000 : 350_000)) throw new Error('Encoded image limit');
      result = encoded; dialog.close();
    } catch { error = true; }
  }
  function move(event: PointerEvent) {
    const crop = geometry();
    if (!drag || event.pointerId !== drag.id || !image || !crop) return;
    const bounds = canvas.getBoundingClientRect();
    const clamp = (value: number) => Math.min(1, Math.max(-1, value));
    const extraX = image.naturalWidth - crop.width, extraY = image.naturalHeight - crop.height;
    horizontal = extraX ? clamp(drag.horizontal - (event.clientX - drag.x) * crop.width / bounds.width * 2 / extraX) : 0;
    vertical = extraY ? clamp(drag.vertical - (event.clientY - drag.y) * crop.height / bounds.height * 2 / extraY) : 0;
  }
  onDestroy(() => { disposed = true; result = null; finish(); });
</script>
<dialog class="confirm-dialog crop-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} onclose={finish}>
  <div class="crop-heading"><div><h2 id={`${uid}-title`}>{language === 'sr' ? 'Kropovanje slike' : 'Crop image'}</h2><p class="muted">{cover ? '16:9' : '1:1'} · {language === 'sr' ? 'Pomeri sliku i podesi zum.' : 'Drag the image and adjust the zoom.'}</p></div><button type="button" class="icon-button" aria-label={language === 'sr' ? 'Zatvori' : 'Close'} onclick={() => dialog.close()}><Icon name="close" /></button></div>
  <div class="crop-preview" class:cover>
    <canvas bind:this={canvas} aria-label={language === 'sr' ? 'Pregled isečene slike' : 'Cropped image preview'} onpointerdown={(event) => { canvas.setPointerCapture(event.pointerId); drag = { id: event.pointerId, x: event.clientX, y: event.clientY, horizontal, vertical }; }} onpointermove={move} onpointerup={() => drag = null} onpointercancel={() => drag = null} onlostpointercapture={() => drag = null}></canvas>
    <div class="crop-grid" aria-hidden="true"><span></span><span></span><span></span><span></span></div>
  </div>
  <div class="crop-controls">
    <label>{language === 'sr' ? 'Zum' : 'Zoom'}<input type="range" min="1" max="4" step="0.01" bind:value={zoom} /><output>{zoom.toFixed(2)}×</output></label>
    <label>{language === 'sr' ? 'Levo / desno' : 'Left / right'}<input type="range" min="-1" max="1" step="0.01" bind:value={horizontal} /></label>
    <label>{language === 'sr' ? 'Gore / dole' : 'Up / down'}<input type="range" min="-1" max="1" step="0.01" bind:value={vertical} /></label>
  </div>
  {#if error}<p class="error" role="alert">{language === 'sr' ? 'Slika nije obrađena. Pokušaj ponovo.' : 'Could not process the image. Try again.'}</p>{/if}
  <div class="dialog-actions"><button type="button" class="secondary crop-reset" onclick={() => { zoom = 1; horizontal = 0; vertical = 0; }}>{language === 'sr' ? 'Resetuj' : 'Reset'}</button><button type="button" class="secondary" onclick={() => dialog.close()}>{language === 'sr' ? 'Otkaži' : 'Cancel'}</button><button type="button" class="primary" onclick={apply}><Icon name="check-circle" size={18} />{language === 'sr' ? 'Primeni' : 'Apply'}</button></div>
</dialog>
<style>
  .crop-dialog { width: min(620px, calc(100vw - 32px)); max-height: calc(100dvh - 32px); overflow: auto; }
  .crop-heading { display: flex; justify-content: space-between; align-items: flex-start; gap: 16px; margin-bottom: 20px; }
  h2 { margin: 0; } .crop-heading p { margin: 8px 0 0; }
  .crop-preview { position: relative; width: min(360px, 100%); margin: auto; border-radius: 8px; overflow: hidden; border: 1px solid var(--border); }
  .crop-preview.cover { width: 100%; }
  canvas { display: block; width: 100%; height: auto; touch-action: none; cursor: grab; }
  canvas:active { cursor: grabbing; }
  .crop-grid { position: absolute; inset: 0; pointer-events: none; }
  .crop-grid span { position: absolute; background: #ffffff60; }
  .crop-grid span:nth-child(1), .crop-grid span:nth-child(2) { width: 1px; height: 100%; left: 33.333%; }
  .crop-grid span:nth-child(2) { left: 66.666%; }
  .crop-grid span:nth-child(3), .crop-grid span:nth-child(4) { height: 1px; width: 100%; top: 33.333%; }
  .crop-grid span:nth-child(4) { top: 66.666%; }
  .crop-controls { display: grid; gap: 14px; margin-top: 20px; }
  label { display: flex; align-items: center; gap: 12px; font-size: 12px; }
  input { flex: 1; min-width: 0; padding: 0; accent-color: var(--primary); }
  output { min-width: 42px; font-variant-numeric: tabular-nums; }
  .crop-reset { margin-right: auto; }
</style>
