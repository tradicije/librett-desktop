<script lang="ts">
  import Icon from './Icon.svelte';
  import darkLogo from '../../../assets/img/logo-dark.png';
  import lightLogo from '../../../assets/img/logo-light.png';
  import license from '../../../LICENSE?raw';
  import metadata from '../package.json';
  import type { Language } from './i18n';
  import type { ResolvedTheme } from './theme';
  let { language, resolvedTheme }: { language: Language; resolvedTheme: ResolvedTheme } = $props();
  let dialog: HTMLDialogElement;
  const uid = $props.id();
  export function open() { dialog.showModal(); }
</script>
<dialog class="about-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} aria-describedby={`${uid}-description`}>
  <div class="about-heading"><span>{language === 'sr' ? 'O aplikaciji' : 'About the application'}</span><button class="icon-button" aria-label={language === 'sr' ? 'Zatvori' : 'Close'} onclick={() => dialog.close()}><Icon name="close" size={18} /></button></div>
  <div class="about-content">
    <div class="about-hero">
      <img src={resolvedTheme === 'dark' ? darkLogo : lightLogo} alt="LibreTT" width="2048" height="552" />
      <h2 id={`${uid}-title`}>LibreTT</h2>
      <p id={`${uid}-description`}>{language === 'sr' ? 'Besplatna aplikacija otvorenog koda za organizaciju stonoteniskih turnira. Tvoji podaci ostaju lokalno, a rad je moguć i bez interneta.' : 'Free and open-source software for organizing table-tennis tournaments. Your data stays local, and you can work offline.'}</p>
      <div class="about-badges"><span><Icon name="desktop" size={14} />{language === 'sr' ? 'Rad bez interneta' : 'Works offline'}</span><span>AGPL-3.0-or-later</span></div>
    </div>
    <dl class="about-details">
      <div><dt>{language === 'sr' ? 'Verzija' : 'Version'}</dt><dd>{metadata.version}<small>{language === 'sr' ? 'Rani razvoj' : 'Early development'}</small></dd></div>
      <div><dt>{language === 'sr' ? 'Autor' : 'Author'}</dt><dd>Aleksa Dimitrijević</dd></div>
      <div><dt>{language === 'sr' ? 'Licenca' : 'License'}</dt><dd>GNU AGPL v3<small>AGPL-3.0-or-later</small></dd></div>
    </dl>
    <section class="about-license"><h3>{language === 'sr' ? 'Besplatno i otvoreno' : 'Free and open'}</h3><p>{language === 'sr' ? 'LibreTT je namenjen klubovima, organizatorima, igračima i zajednici. Kod možeš koristiti, proučavati, menjati i deliti u skladu sa GNU Affero General Public License, verzijom 3 ili kasnijom.' : 'LibreTT is built for clubs, organizers, players and the community. You can use, study, modify and share its code under the GNU Affero General Public License, version 3 or later.'}</p><details><summary>{language === 'sr' ? 'Pročitaj pun tekst licence' : 'Read the full license'}</summary><pre>{license}</pre></details></section>
    <footer>© 2026 Aleksa Dimitrijević</footer>
  </div>
</dialog>
<style>
  .about-dialog { width: min(560px, calc(100vw - 32px)); max-height: calc(100dvh - 64px); padding: 0; border: 1px solid var(--border); border-radius: 12px; background: var(--surface); color: var(--text-primary); box-shadow: 0 24px 80px #0004; overflow: hidden; }
  .about-dialog::backdrop { background: #0006; }
  .about-heading { display: flex; justify-content: space-between; align-items: center; padding: 12px 18px; border-bottom: 1px solid var(--border-subtle); font-size: 12px; color: var(--text-secondary); }
  .about-content { max-height: calc(100dvh - 122px); overflow: auto; overscroll-behavior: contain; padding: 28px; }
  .about-hero { text-align: center; }
  .about-hero img { display: block; width: min(256px, 100%); height: auto; margin: 8px auto 24px; }
  h2 { margin: 0 0 12px; font-size: 24px; letter-spacing: -.7px; }
  .about-hero p, .about-license p { color: var(--text-secondary); font-size: 12px; line-height: 1.8; }
  .about-badges { display: flex; justify-content: center; gap: 8px; flex-wrap: wrap; margin: 18px 0 24px; }
  .about-badges span { display: inline-flex; align-items: center; gap: 6px; padding: 5px 9px; border: 1px solid var(--border); border-radius: 20px; color: var(--text-secondary); font-size: 10px; }
  .about-details { display: grid; grid-template-columns: 1fr 1.4fr 1.2fr; gap: 16px; margin: 0; padding: 20px 0; border-top: 1px solid var(--border-subtle); border-bottom: 1px solid var(--border-subtle); }
  dt { color: var(--text-muted); font-size: 10px; margin-bottom: 8px; }
  dd { margin: 0; font-size: 12px; font-weight: 550; }
  dd small { display: block; margin-top: 5px; font-size: 10px; font-weight: 400; color: var(--text-muted); }
  .about-license { padding-top: 22px; }
  summary { cursor: pointer; color: var(--primary); font-size: 11px; padding: 8px 0; }
  pre { white-space: pre-wrap; overflow-wrap: anywhere; font-size: 10px; line-height: 1.7; background: var(--background); padding: 16px; border-radius: 6px; }
  footer { margin-top: 24px; font-size: 10px; color: var(--text-muted); text-align: center; }
  @media (max-width: 500px) { .about-details { grid-template-columns: 1fr; } .about-content { padding: 20px; } }
</style>
