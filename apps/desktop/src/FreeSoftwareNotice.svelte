<script lang="ts">
  import Icon from './Icon.svelte';
  import { desktopAvailable, openOfficialLink } from './api';
  import { messages, type Language } from './i18n';
  let { language }: { language: Language } = $props();
  let text = $derived(messages[language]);
  let failed = $state(false);
  async function open(event: MouseEvent, destination: 'website' | 'source') {
    if (!desktopAvailable) return;
    event.preventDefault();
    failed = false;
    try { await openOfficialLink(destination); }
    catch { failed = true; }
  }
</script>

<section class="free-notice" aria-label={text.freeSoftwareTitle}>
  <h2>{text.freeSoftwareTitle}</h2>
  <p class="notice-intro">{text.freeSoftwareNotice}</p>
  <p class="refund-note">{text.freeSoftwareRefund}</p>
  <div class="official-links">
    <span>{text.officialLinks}</span>
    <div class="link-list">
    <a href="https://librett.org" target="_blank" rel="noopener noreferrer" onclick={event => open(event, 'website')}>{text.officialWebsite}<Icon name="arrow-right" size={15}/></a>
    <a href="https://github.com/tradicije/librett-desktop" target="_blank" rel="noopener noreferrer" onclick={event => open(event, 'source')}>{text.officialSourceCode}<Icon name="arrow-right" size={15}/></a>
    </div>
  </div>
  {#if failed}<p class="link-error" role="alert">{text.officialLinkError}</p>{/if}
</section>

<style>
  .free-notice { width: 100%; box-sizing: border-box; margin-top: 26px; padding-top: 22px; border-top: 1px solid var(--border); }
  h2 { margin: 0 0 12px; font-size: 22px; font-weight: 600; line-height: 1.25; letter-spacing: -.4px; }
  p { margin: 0; line-height: 1.6; }
  .notice-intro { color: var(--text-primary); font-size: 14px; }
  .refund-note { margin-top: 6px; color: var(--text-secondary); font-size: 12px; }
  .official-links { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 10px 24px; margin-top: 20px; }
  .official-links > span { color: var(--text-muted); font-size: 11px; }
  .link-list { display: flex; flex-wrap: wrap; align-items: center; gap: 10px 24px; }
  a { display: inline-flex; align-items: center; gap: 7px; color: var(--primary); font-size: 12px; font-weight: 500; text-decoration: none; }
  a:hover { color: var(--primary-hover); text-decoration: underline; text-underline-offset: 3px; }
  .link-error { margin-top: 12px; color: var(--text-secondary); font-size: 12px; }
  @media (max-width: 540px) {
    .official-links { flex-direction: column; align-items: flex-start; gap: 10px; }
  }
</style>
