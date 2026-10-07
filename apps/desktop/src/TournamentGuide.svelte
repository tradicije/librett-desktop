<script lang="ts">
  import type {Language} from './i18n';
  import {tournamentGuides} from './tournament-guide';
  let {language}:{language:Language}=$props();
  let guide=$derived(tournamentGuides[language]);
  const uid=$props.id();
  function jump(event:MouseEvent,target:string){event.preventDefault();document.getElementById(target)?.scrollIntoView({block:'start'});}
</script>
<div class="heading"><div><h1>{language==='sr'?'Vodič':'Guide'}</h1><p class="muted">{guide.intro}</p></div></div>
<article class="guide">
  <h2>{guide.title}</h2>
  <nav class="guide-index" aria-label={language==='sr'?'Koraci vođenja turnira':'Tournament steps'}>{#each guide.steps as step,index}<a href={`#${uid}-step-${index}`} onclick={event=>jump(event,`${uid}-step-${index}`)}>{index+1}. {step.title}</a>{/each}<a href={`#${uid}-help`} onclick={event=>jump(event,`${uid}-help`)}>{language==='sr'?'Ako nešto zapne':'If you get stuck'}</a></nav>
  {#each guide.steps as step,index}<section class="guide-step" id={`${uid}-step-${index}`}><h3><span class="step-number">{index+1}</span>{step.title}</h3><ul>{#each step.steps as instruction}<li>{instruction}</li>{/each}</ul>{#if step.note}<p class="guide-note">{step.note}</p>{/if}</section>{/each}
  <section class="guide-help" id={`${uid}-help`}><h2>{language==='sr'?'Ako nešto zapne':'If you get stuck'}</h2>{#each guide.help as item}<details><summary>{item.title}</summary><p>{item.answer}</p></details>{/each}</section>
  <section class="guide-terms"><h2>{language==='sr'?'Šta znače ove reči?':'What do these words mean?'}</h2><dl>{#each guide.terms as term}<div><dt>{term.word}</dt><dd>{term.meaning}</dd></div>{/each}</dl></section>
</article>
<style>
  .guide { max-width:880px; font-size:14px; line-height:1.8; }
  h2 { font-size:20px; margin:0 0 18px; } h3 { display:flex; align-items:center; gap:12px; font-size:17px; margin:0 0 12px; line-height:1.5; }
  .guide-index { display:flex; flex-wrap:wrap; gap:8px 18px; margin-bottom:28px; padding-bottom:24px; border-bottom:1px solid var(--border-subtle); }
  a { color:var(--primary); font-size:12px; text-underline-offset:4px; } a:focus-visible,summary:focus-visible { outline:2px solid var(--primary); outline-offset:4px; }
  .guide-step { padding:22px 0; border-bottom:1px solid var(--border-subtle); scroll-margin-top:20px; }
  .step-number { display:flex; align-items:center; justify-content:center; width:30px; height:30px; flex-shrink:0; border-radius:6px; background:var(--primary-subtle); color:var(--primary); font-size:13px; }
  ul { margin:0; padding-left:26px; } li { padding-left:4px; margin:8px 0; }
  .guide-note { padding-left:14px; border-left:3px solid var(--primary); color:var(--text-secondary); margin:16px 0 0; font-size:13px; }
  .guide-help,.guide-terms { margin-top:32px; scroll-margin-top:20px; } details { padding:14px 0; border-bottom:1px solid var(--border-subtle); } summary { cursor:pointer; font-weight:550; } details p { color:var(--text-secondary); margin:12px 0 0; }
  dl { margin:0; } dl>div { margin-bottom:16px; } dt { font-weight:550; } dd { margin:2px 0 0; color:var(--text-secondary); }
</style>
