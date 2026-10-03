<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, getPlayer, savePlayerProfile, type Player } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { language, playerId, busy = $bindable(false), dirty = $bindable(false), onsaved, oncancel }: {
    language: Language; playerId?: string; busy?: boolean; dirty?: boolean;
    onsaved: () => void; oncancel: () => void;
  } = $props();
  let text = $derived(messages[language]);
  let id = $derived(playerId ?? null);
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
    try { if (playerId) fill(await getPlayer(playerId)); baseline = snapshot(); loaded = true; }
    catch (cause) { error = errorKey(cause) === 'not_found' ? 'player_not_found' : errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  async function readPhoto(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    busy = true; error = null;
    try {
      if (!['image/jpeg', 'image/png', 'image/webp'].includes(file.type) || file.size > 10 * 1024 * 1024) throw new Error();
      const data = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result)); reader.onerror = reject;
        reader.readAsDataURL(file);
      });
      const image = new Image();
      await new Promise<void>((resolve, reject) => { image.onload = () => resolve(); image.onerror = reject; image.src = data; });
      if (!image.width || !image.height || image.width > 6000 || image.height > 6000) throw new Error();
      const scale = Math.min(1, 512 / Math.max(image.width, image.height));
      const canvas = document.createElement('canvas');
      canvas.width = Math.max(1, Math.round(image.width * scale)); canvas.height = Math.max(1, Math.round(image.height * scale));
      const context = canvas.getContext('2d'); if (!context) throw new Error();
      context.fillStyle = '#fff'; context.fillRect(0, 0, canvas.width, canvas.height); context.drawImage(image, 0, 0, canvas.width, canvas.height);
      const result = canvas.toDataURL('image/jpeg', 0.8);
      if (result.length > 350000) throw new Error();
      photo = result;
    } catch { error = 'invalid_profile'; if (fileInput) fileInput.value = ''; }
    finally { busy = false; }
  }
  async function save(event: SubmitEvent) {
    event.preventDefault(); if (!loaded) return;
    busy = true; error = null;
    try {
      await savePlayerProfile(id, name, club, { birth_year: birthYear ?? null, city, country, email, phone, notes, photo });
      baseline = snapshot(); dirty = false; busy = false; onsaved();
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
</script>

<div class="heading"><div><h1>{id ? text.editPlayer : text.addPlayer}</h1><p class="muted">{text.playerEditorIntro}</p></div></div>
{#if error}<p class="error" role="alert">{text[error]}{#if !loaded && error !== 'player_not_found'}<button onclick={load} disabled={loading}>{text.retry}</button>{/if}</p>{/if}
{#if error === 'player_not_found'}<button class="secondary icon-label" onclick={oncancel}><Icon name="arrow-left" size={18} />{text.playerTab}</button>{/if}
{#if loading}<p role="status">{text.loading}</p>{/if}
{#if loaded || !desktopAvailable}
  <section class="panel form-panel player-editor">
    <form onsubmit={save}>
      <fieldset class="form-group"><legend>{text.profileDetails}</legend><div class="form-fields">
      <label>{text.playerName}<input bind:value={name} required maxlength="120" disabled={busy} /></label>
      <label>{text.club}<input bind:value={club} maxlength="120" disabled={busy} /></label>
      <label>{text.birthYear}<input type="number" bind:value={birthYear} min="1900" max={currentYear} step="1" disabled={busy} /></label>
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
      <div class="form-actions"><button class="primary" disabled={busy || !desktopAvailable || !loaded}><Icon name="check-circle" />{busy ? text.saving : text.savePlayer}</button>
      <button type="button" class="secondary icon-label" disabled={busy} onclick={oncancel}><Icon name="arrow-left" size={18} />{text.cancelEdit}</button></div>
    </form>
  </section>
{/if}
