<script lang="ts">
  import Select from './Select.svelte';
  import { onMount } from 'svelte';
  import Players from './Players.svelte';
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import { applyTheme, savedTheme, saveTheme, watchSystemTheme, type ThemePreference, type ResolvedTheme } from './theme';
  import { addCategory, createTournament, desktopAvailable, listTournaments, type CompetitionFormat, type Discipline, type Tournament } from './api';
  import { messages, errorKey, savedLanguage, type Language, type MessageKey } from './i18n';

  let language = $state<Language>(savedLanguage());
  let theme = $state<ThemePreference>(savedTheme());
  let resolvedTheme = $state<ResolvedTheme>(document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light');
  let text = $derived(messages[language]);
  let tournaments = $state<Tournament[]>([]);
  let selectedId = $state<string | null>(null);
  let selected = $derived(tournaments.find(t => t.id === selectedId));
  let tournamentName = $state('');
  let categoryName = $state('');
  let discipline = $state<Discipline>('singles');
  let format = $state<CompetitionFormat>('groups_knockout');
  let busy = $state(false);
  let loading = $state(false);
  let loaded = $state(false);
  let error = $state<MessageKey | null>(null);
  let notice = $state<MessageKey | null>(null);

  $effect(() => {
    document.documentElement.lang = language;
    try { localStorage.setItem('librett.language', language); } catch { /* Preference is optional. */ }
  });

  async function load() {
    loading = true;
    error = null;
    try { tournaments = await listTournaments(); loaded = true; }
    catch (cause) { error = errorKey(cause); }
    finally { loading = false; }
  }
  $effect(() => { resolvedTheme = saveTheme(theme); });
  onMount(() => watchSystemTheme(() => {
    if (theme === 'system') resolvedTheme = applyTheme(theme);
  }));
  onMount(() => { if (desktopAvailable) void load(); });

  async function create(event: SubmitEvent) {
    event.preventDefault();
    busy = true; error = null; notice = null;
    try {
      const tournament = await createTournament(tournamentName);
      tournaments = [tournament, ...tournaments];
      selectedId = tournament.id; tournamentName = ''; notice = 'created';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  async function saveCategory(event: SubmitEvent) {
    event.preventDefault();
    if (!selected) return;
    busy = true; error = null; notice = null;
    try {
      const updated = await addCategory(selected.id, categoryName, discipline, format);
      tournaments = tournaments.map(t => t.id === updated.id ? updated : t);
      categoryName = ''; notice = 'categorySaved';
    } catch (cause) { error = errorKey(cause); }
    finally { busy = false; }
  }
  function select(id: string | null) {
    selectedId = id; categoryName = ''; error = null; notice = null;
    discipline = 'singles'; format = 'groups_knockout';
  }
</script>

<div class="shell">
  <aside>
    <a class="brand" href="/" onclick={(event) => { event.preventDefault(); if (!busy) select(null); }}><img src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" /></a>
    <nav aria-label={text.tournaments}>
      <button class="active" disabled={busy} onclick={() => select(null)}><Icon name="trophy" /> {text.tournaments}</button>
    </nav>
    <div class="sidebar-bottom"><span class="icon-label"><Icon name="desktop" size={16} />{text.local}</span><small>© 2026 Aleksa Dimitrijević</small></div>
  </aside>
  <main>
    <header>
      <span class="eyebrow">{text.tournaments} / LibreTT</span>
      <div class="preferences">
        <label><Icon name={theme === 'system' ? 'desktop' : theme === 'dark' ? 'moon' : 'sun'} size={18} />{text.theme}<Select label={text.theme} bind:value={theme} options={[{ value: 'system', label: text.themeSystem }, { value: 'light', label: text.themeLight }, { value: 'dark', label: text.themeDark }]} /></label>
        <label><Icon name="globe" size={18} />{text.language}<Select label={text.language} bind:value={language} options={[{ value: 'sr', label: 'Srpski' }, { value: 'en', label: 'English' }]} /></label>
      </div>
    </header>
    {#if !desktopAvailable}<p class="banner">{text.preview}</p>{/if}
    {#if error}<div class="error" role="alert">{text[error]} {#if !loaded && desktopAvailable}<button disabled={loading} onclick={load}>{text.retry}</button>{/if}</div>{/if}
    <div class="notice" role="status" aria-live="polite">{notice ? text[notice] : ''}</div>
    {#if selected}
      <button class="back" disabled={busy} onclick={() => select(null)}><Icon name="arrow-left" size={18} /> {text.back}</button>
      <div class="heading"><div><p class="eyebrow">{text.selected}</p><h1>{selected.name}</h1></div><span class="pill">{selected.categories.length} · {text.categories}</span></div>
      <div class="columns">
        <section class="panel">
          <h2>{text.categories}</h2>
          {#if selected.categories.length === 0}<p class="muted">{text.noCategories}</p>{/if}
          {#each selected.categories as category (category.id)}
            <article class="category"><span class="category-icon"><Icon name={category.discipline === 'singles' ? 'user' : 'users'} /></span><div><h3>{category.name}</h3><p>{text[category.discipline]} · {text[category.format]}</p></div></article>
          {/each}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="layer-group" />{text.addCategory}</h2>
          <form onsubmit={saveCategory}>
            <label>{text.categoryName}<input bind:value={categoryName} required disabled={busy} /></label>
            <label>{text.discipline}<Select label={text.discipline} bind:value={discipline} options={[{ value: 'singles', label: text.singles }, { value: 'doubles', label: text.doubles }]} disabled={busy} /></label>
            <label>{text.format}<Select label={text.format} bind:value={format} options={[{ value: 'groups_knockout', label: text.groups_knockout }, { value: 'knockout', label: text.knockout }]} disabled={busy} /></label>
            <button class="primary" disabled={busy || !desktopAvailable}><Icon name="check-circle" size={18} />{busy ? text.saving : text.save}</button>
          </form>
        </section>
      </div>
      {#key selected.id}<Players tournament={selected} {language} />{/key}
    {:else}
      <div class="heading"><div><p class="eyebrow">LibreTT</p><h1>{text.subtitle}</h1><p class="muted">{text.intro}</p></div></div>
      <div class="columns">
        <section class="panel">
          <div class="section-heading"><h2>{text.tournaments}</h2><span class="pill">{tournaments.length}</span></div>
          {#if loading}<p class="muted" role="status">{text.loading}</p>
          {:else if tournaments.length === 0 && (!desktopAvailable || loaded)}<div class="empty"><span class="empty-icon"><Icon name="trophy" size={28} /></span><h3>{text.empty}</h3><p>{text.emptyText}</p></div>
          {:else}
            {#each tournaments as tournament (tournament.id)}<button class="tournament" disabled={busy} onclick={() => select(tournament.id)}><div><strong>{tournament.name}</strong><small>{tournament.categories.length} · {text.categories}</small></div><Icon name="arrow-right" /></button>{/each}
          {/if}
        </section>
        <section class="panel form-panel"><h2 class="icon-label"><Icon name="trophy" />{text.newTournament}</h2><form onsubmit={create}><label>{text.tournamentName}<input bind:value={tournamentName} required disabled={busy || loading} /></label><button class="primary" disabled={busy || loading || !desktopAvailable || !loaded}><Icon name="plus" size={18} />{busy ? text.saving : text.create}</button></form></section>
      </div>
    {/if}
  </main>
</div>
