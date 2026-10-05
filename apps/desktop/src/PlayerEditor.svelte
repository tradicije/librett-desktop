<script lang="ts">
  import { onMount, untrack, tick } from 'svelte';
  import Icon from './Icon.svelte';
  import { confirmDiscard } from './confirmation';
  import ImageCropDialog from './ImageCropDialog.svelte';
  let cropDialog: ImageCropDialog;
  import { desktopAvailable, getPlayer, savePlayerChecked, type Player, type PlayerProfile } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { language, playerId, busy = $bindable(false), dirty = $bindable(false), onsaved, oncancel }: {
    language: Language; playerId?: string; busy?: boolean; dirty?: boolean;
    onsaved: () => void; oncancel: () => void;
  } = $props();
  let text = $derived(messages[language]);
  let id = $derived(playerId ?? null);
  const stableId = untrack(() => playerId ?? crypto.randomUUID());
  let expected = $state<Player | null>(null);
  let action = $state(false);
  let photoLoading = $state(false);
  let pending = $state<{ requestId: string; name: string; club: string; profile: PlayerProfile; expected: Player | null } | null>(null);
  let loaded = $state(false);
  let loading = $state(false);
  let error = $state<MessageKey | null>(null);
  let name = $state(''); let club = $state('');
  let birthYear = $state<number | undefined>();
  let city = $state(''); let country = $state(''); let email = $state(''); let phone = $state(''); let notes = $state('');
  let photo = $state<string | null>(null);
  let fileInput = $state<HTMLInputElement>();
  const snapshot = () => JSON.stringify([name, club, birthYear, city, country, email, phone, notes, photo]);
  let baseline = $state(snapshot());
  $effect(() => { dirty = loaded && snapshot() !== baseline; });
  const currentYear = new Date().getFullYear();
  function fill(player: Player) {
    name = player.name; club = player.club; birthYear = player.birth_year ?? undefined;
    city = player.city; country = player.country; email = player.email; phone = player.phone;
    notes = player.notes; photo = player.photo;
  }
  async function load() {
    loading = true; error = null;
    try { if (playerId) { const current = await getPlayer(playerId); fill(current); expected = current; } baseline = snapshot(); loaded = true; }
    catch (cause) { error = errorKey(cause) === 'not_found' ? 'player_not_found' : errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  async function reload() { if (!busy && (!dirty || await confirmDiscard())) await load(); }
  async function readPhoto(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0]; if (!file || busy) return;
    photoLoading = true; busy = true; error = null;
    try { const result = await cropDialog.crop(file); if (result !== null) photo = result; }
    catch { error = 'invalid_profile'; if (fileInput) fileInput.value = ''; }
    finally { if (fileInput) fileInput.value = ''; photoLoading = false; busy = pending !== null; }
  }
  async function save(event: SubmitEvent) {
    event.preventDefault(); if (!loaded || action || photoLoading) return;
    if (!pending) pending = { requestId: crypto.randomUUID(), name, club, profile: { birth_year: birthYear ?? null, city, country, email, phone, notes, photo }, expected: expected ? JSON.parse(JSON.stringify(expected)) : null };
    action = true; busy = true; error = null;
    try {
      const request = pending;
      await savePlayerChecked(request.requestId, stableId, request.name, request.club, request.profile, request.expected);
      pending = null; baseline = snapshot(); dirty = false; busy = false; await tick(); onsaved();
    } catch (cause) {
      error = errorKey(cause);
      if (['birth_year_required', 'player_conflict', 'not_found', 'name_required', 'name_too_long', 'invalid_profile'].includes(String(cause))) pending = null;
    } finally { action = false; busy = pending !== null; }
  }
</script>
<ImageCropDialog {language} bind:this={cropDialog} />

<div class="heading"><div><h1>{id ? text.editPlayer : text.addPlayer}</h1><p class="muted">{text.playerEditorIntro}</p></div></div>
{#if error}<p class="error" role="alert">{text[error]}{#if (!loaded || error === 'player_conflict') && error !== 'player_not_found'}<button onclick={reload} disabled={loading}>{text.retry}</button>{/if}</p>{/if}
{#if error === 'player_not_found'}<button class="secondary icon-label" onclick={oncancel}><Icon name="arrow-left" size={18} />{text.playerTab}</button>{/if}
{#if loading}<p role="status">{text.loading}</p>{/if}
{#if loaded || !desktopAvailable}
  <section class="panel form-panel player-editor">
    <form onsubmit={save}>
      <fieldset class="form-group"><legend>{text.profileDetails}</legend><div class="form-fields">
      <label>{text.playerName}<input bind:value={name} required maxlength="120" disabled={busy} /></label>
      <label>{text.club}<input bind:value={club} maxlength="120" disabled={busy} /></label>
      <label>{text.birthYear}<input type="number" bind:value={birthYear} min="1900" max={currentYear} step="1" required disabled={busy} /></label>
      <label>{text.city}<input bind:value={city} maxlength="2000" disabled={busy} /></label>
      <label>{text.country}<input bind:value={country} maxlength="2000" disabled={busy} /></label>
      </div></fieldset>
      <fieldset class="form-group"><legend>{text.contactDetails}</legend><div class="form-fields">
      <label>{text.email}<input type="email" bind:value={email} maxlength="2000" disabled={busy} /></label>
      <label>{text.phone}<input type="tel" bind:value={phone} maxlength="2000" disabled={busy} /></label>
      </div></fieldset>
      <fieldset class="form-group"><legend>{text.additionalDetails}</legend>
      <label>{text.notes}<textarea bind:value={notes} maxlength="2000" rows="3" disabled={busy}></textarea></label>
      <label>{text.photo}<input type="file" accept="image/jpeg,image/png,image/webp" onchange={readPhoto} bind:this={fileInput} disabled={busy} /></label>
      <p class="muted">{text.photoHint}</p>
      {#if photo}<div class="photo-preview"><img class="player-photo" src={photo} alt={text.photo} /><button type="button" class="secondary" disabled={busy} onclick={() => { photo = null; if (fileInput) fileInput.value = ''; }}>{text.removePhoto}</button></div>{/if}
      </fieldset>
      <div class="form-actions"><button class="primary" disabled={action || photoLoading || !desktopAvailable || !loaded}><Icon name="check-circle" />{action ? text.saving : pending ? text.retry : text.savePlayer}</button>
      <button type="button" class="secondary icon-label" disabled={busy} onclick={oncancel}><Icon name="arrow-left" size={18} />{text.cancelEdit}</button></div>
    </form>
  </section>
{/if}
