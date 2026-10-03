<script lang="ts">
  import type { Language } from './i18n';
  let { language }: { language: Language } = $props();
  let dialog: HTMLDialogElement;
  let resolve: ((value: boolean) => void) | undefined;
  let accepted = false;
  const uid = $props.id();
  export function confirm(): Promise<boolean> {
    if (resolve || document.querySelector('dialog[open]')) return Promise.resolve(false);
    accepted = false;
    return new Promise(done => { resolve = done; dialog.showModal(); });
  }
  function finish() { const done = resolve; resolve = undefined; done?.(accepted); }
</script>
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} onclose={finish}>
  <h2 id={`${uid}-title`}>{language === 'sr' ? 'Odbaci nesačuvane izmene?' : 'Discard unsaved changes?'}</h2>
  <p>{language === 'sr' ? 'Nesačuvani unos ili nacrt biće izgubljeni ako nastaviš.' : 'Unsaved input or drafts will be lost if you continue.'}</p>
  <div class="dialog-actions"><button class="secondary" onclick={() => dialog.close()}>{language === 'sr' ? 'Ostani' : 'Stay'}</button><button class="primary" onclick={() => { accepted = true; dialog.close(); }}>{language === 'sr' ? 'Odbaci i nastavi' : 'Discard and continue'}</button></div>
</dialog>
