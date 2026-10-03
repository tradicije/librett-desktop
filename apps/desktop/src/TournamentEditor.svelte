<script lang="ts">
  import { tick, untrack } from 'svelte';
  import Icon from './Icon.svelte';
  import { createTournamentWithCover, updateTournamentDetails, desktopAvailable, type Tournament } from './api';
  import { imageData } from './image-limits';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { tournament, language, busy = $bindable(false), dirty = $bindable(false), onsaved, oncancel }: { tournament?: Tournament; language: Language; busy?: boolean; dirty?: boolean; onsaved: (tournament: Tournament) => void; oncancel: () => void } = $props();
  let text = $derived(messages[language]);
  const original = untrack(() => tournament);
  const id = original?.id ?? crypto.randomUUID();
  let name = $state(original?.name ?? ''); let cover = $state<string | null>(original?.cover ?? null);
  let action = $state(false); let reading = $state(false); let error = $state<MessageKey | null>(null);
  let fileInput: HTMLInputElement;
  let pending = $state<{ name: string; cover: string | null } | null>(null);
  let saved = $state(false);
  $effect(() => { dirty = !saved && (original ? name !== original.name || cover !== original.cover : !!name.trim() || cover !== null); });
  async function readCover(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0]; if (!file || busy) return;
    reading = true; busy = true; error = null;
    try { cover = await imageData(file, true); }
    catch { error = 'invalid_profile'; fileInput.value = ''; }
    finally { reading = false; busy = pending !== null; }
  }
  async function save(event: SubmitEvent) {
    event.preventDefault(); if (action || reading) return;
    if (!pending) pending = { name, cover };
    action = true; busy = true; error = null;
    try {
      const result = original ? await updateTournamentDetails(id, pending.name, pending.cover, original.name, original.cover) : await createTournamentWithCover(id, pending.name, pending.cover);
      pending = null; saved = true; dirty = false; busy = false; await tick(); onsaved(result);
    } catch (cause) {
      error = errorKey(cause);
      if (['name_required','name_too_long','invalid_profile','draw_conflict'].includes(String(cause))) pending = null;
    } finally { action = false; busy = pending !== null; }
  }
</script>
<div class="heading"><div><h1>{original ? (language === 'sr' ? 'Podešavanja turnira' : 'Tournament settings') : text.addTournament}</h1><p class="muted">{language === 'sr' ? 'Unesi naziv i po želji dodaj naslovnu sliku turnira.' : 'Enter a name and optionally add a tournament cover image.'}</p></div></div>
{#if error}<p class="error" role="alert">{text[error]}</p>{/if}
<form class="panel form-panel tournament-editor" onsubmit={save}>
  <fieldset class="form-group" disabled={busy}><legend>{language === 'sr' ? 'Osnovni podaci' : 'Tournament details'}</legend><label>{text.tournamentName}<input bind:value={name} required maxlength="120" /></label></fieldset>
  <fieldset class="form-group" disabled={busy}><legend>{language === 'sr' ? 'Naslovna slika' : 'Cover image'}</legend>
    <div class="cover-preview">{#if cover}<img src={cover} alt={name || text.tournaments} />{:else}<Icon name="trophy" size={40} />{/if}</div>
    <label>{language === 'sr' ? 'Izaberi sliku' : 'Choose an image'}<input type="file" accept="image/jpeg,image/png,image/webp" bind:this={fileInput} onchange={readCover} /></label>
    <p class="muted">{language === 'sr' ? 'Slika se centrira i kropuje u odnos 16:9. JPEG, PNG ili statični WebP, do 10 MB.' : 'The image is centered and cropped to 16:9. JPEG, PNG or static WebP, up to 10 MB.'}</p>
    {#if cover}<button type="button" class="secondary icon-label" onclick={() => { cover = null; fileInput.value = ''; }}><Icon name="trash" size={16} />{language === 'sr' ? 'Ukloni sliku' : 'Remove image'}</button>{/if}
  </fieldset>
  <div class="form-actions"><button class="primary" disabled={action || reading || !desktopAvailable}><Icon name="check-circle" />{action ? text.saving : pending ? text.retry : original ? (language === 'sr' ? 'Sačuvaj izmene' : 'Save changes') : text.addTournament}</button><button type="button" class="secondary icon-label" disabled={busy} onclick={oncancel}><Icon name="arrow-left" size={18} />{text.cancelEdit}</button></div>
</form>
<style>
  .tournament-editor { display: grid; gap: 24px; max-width: 850px; }
  label { display: grid; gap: 6px; }
  .cover-preview { width: min(320px, 100%); aspect-ratio: 16 / 9; border-radius: 6px; overflow: hidden; display: grid; place-items: center; background: var(--background); color: var(--text-muted); border: 1px solid var(--border-subtle); }
  .cover-preview img { width: 100%; height: 100%; object-fit: cover; }
</style>
