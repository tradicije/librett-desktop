<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { desktopAvailable, listPlayers, savePlayerProfile, type Player } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { language, busy = $bindable(false) }: { language: Language; busy?: boolean } = $props();
  let text = $derived(messages[language]);
  let players = $state<Player[]>([]);
  let loaded = $state(false);
  let loading = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state(false);
  let search = $state('');
  let id = $state<string | null>(null);
  let name = $state(''); let club = $state('');
  let birthYear = $state<number | undefined>();
  let city = $state(''); let country = $state(''); let email = $state(''); let phone = $state(''); let notes = $state('');
  let photo = $state<string | null>(null);
  let fileInput: HTMLInputElement;
  const currentYear = new Date().getFullYear();
  let filtered = $derived(players.filter(p => `${p.name} ${p.club} ${p.city} ${p.country} ${p.birth_year ?? ''}`.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase())));
  async function load() {
    loading = true; error = null;
    try { players = await listPlayers(); loaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  onMount(() => { if (desktopAvailable) void load(); });
  function edit(player?: Player) {
    id = player?.id ?? null; name = player?.name ?? ''; club = player?.club ?? '';
    birthYear = player?.birth_year ?? undefined; city = player?.city ?? ''; country = player?.country ?? '';
    email = player?.email ?? ''; phone = player?.phone ?? ''; notes = player?.notes ?? ''; photo = player?.photo ?? null;
    if (fileInput) fileInput.value = '';
    notice = false;
  }
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
    } catch { error = 'invalid_profile'; fileInput.value = ''; }
    finally { busy = false; }
  }
  async function save(event: SubmitEvent) {
    event.preventDefault(); busy = true; error = null;
    try {
      const player = await savePlayerProfile(id, name, club, { birth_year: birthYear ?? null, city, country, email, phone, notes, photo });
      players = [...players.filter(p => p.id !== player.id), player].sort((a, b) => a.name.localeCompare(b.name, language));
      edit(); notice = true;
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
</script>

<div class="heading"><div><h1>{text.playerTab}</h1><p class="muted">{text.playerDirectoryIntro}</p></div><span class="pill">{players.length}</span></div>
{#if error}<p class="error" role="alert">{text[error]}{#if !loaded}<button onclick={load} disabled={loading}>{text.retry}</button>{/if}</p>{/if}
<p role="status" class="notice">{notice ? text.playerSaved : ''}</p>
<div class="columns player-directory">
  <section class="panel">
    <h2>{text.directory}</h2>
    <label class="field-label">{text.searchPlayers}<input type="search" bind:value={search} /></label>
    {#if loading}<p role="status">{text.loading}</p>{:else if !filtered.length}<p class="muted">{text.noPlayers}</p>{/if}
    {#each filtered as player (player.id)}
      <article class="player-profile">
        {#if player.photo}<img class="player-photo" src={player.photo} alt={player.name} />{:else}<span class="player-avatar"><Icon name="user" size={28} /></span>{/if}
        <div class="player-details"><h3>{player.name}</h3><p>{[player.club, player.birth_year, player.city, player.country].filter(Boolean).join(' · ')}</p>
          {#if player.email}<p>{player.email}</p>{/if}{#if player.phone}<p>{player.phone}</p>{/if}{#if player.notes}<p class="player-notes">{player.notes}</p>{/if}
        </div>
        <button class="secondary" disabled={busy} onclick={() => edit(player)}>{text.editPlayer}</button>
      </article>
    {/each}
  </section>
  <section class="panel form-panel"><h2>{id ? text.editPlayer : text.addPlayer}</h2>
    <form onsubmit={save}>
      <label>{text.playerName}<input bind:value={name} required maxlength="120" disabled={busy} /></label>
      <label>{text.club}<input bind:value={club} maxlength="120" disabled={busy} /></label>
      <label>{text.birthYear}<input type="number" bind:value={birthYear} min="1900" max={currentYear} step="1" disabled={busy} /></label>
      <label>{text.city}<input bind:value={city} maxlength="2000" disabled={busy} /></label>
      <label>{text.country}<input bind:value={country} maxlength="2000" disabled={busy} /></label>
      <label>{text.email}<input type="email" bind:value={email} maxlength="2000" disabled={busy} /></label>
      <label>{text.phone}<input type="tel" bind:value={phone} maxlength="2000" disabled={busy} /></label>
      <label>{text.notes}<textarea bind:value={notes} maxlength="2000" rows="3" disabled={busy}></textarea></label>
      <label>{text.photo}<input type="file" accept="image/jpeg,image/png,image/webp" onchange={readPhoto} bind:this={fileInput} disabled={busy} /></label>
      <p class="muted">{text.photoHint}</p>
      {#if photo}<div class="photo-preview"><img class="player-photo" src={photo} alt={text.photo} /><button type="button" class="secondary" disabled={busy} onclick={() => { photo = null; fileInput.value = ''; }}>{text.removePhoto}</button></div>{/if}
      <button class="primary" disabled={busy || !desktopAvailable || !loaded}><Icon name="check-circle" />{busy ? text.saving : text.savePlayer}</button>
      {#if id}<button type="button" class="secondary" disabled={busy} onclick={() => edit()}>{text.cancelEdit}</button>{/if}
    </form>
  </section>
</div>
