<script lang="ts">
  import InfoRows from './InfoRows.svelte';
  import { onMount, untrack } from 'svelte';
  import CompletionControl from './CompletionControl.svelte';
  import PlayerName from './PlayerName.svelte';
  import Icon from './Icon.svelte';
  import { playerLabel } from './player-label';
  import { getTournamentProgress, desktopAvailable, type TournamentProgress, type Tournament, type CategoryResults } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let {active=true,tournament,language,busy=$bindable(false),oncategory}: {active?:boolean;tournament:Tournament;language:Language;busy?:boolean;oncategory:(id:string)=>void}=$props();
  let sr=$derived(language==='sr');let text=$derived(messages[language]);let data=$state<TournamentProgress|null>(null);let loading=$state(false);let writing=$state(false);let error=$state<MessageKey|null>(null);
  let finished=$derived(data?.categories.filter(c=>c.completion.completed_at).length??0);
  $effect(()=>{busy=loading || writing;});
  async function load(){if(!desktopAvailable || loading || writing)return;loading=true;error=null;try{data=await getTournamentProgress(tournament.id);}catch(cause){error=errorKey(cause);}finally{loading=false;}}
  onMount(()=>{void load();const refresh=(event:Event)=>{if((event as CustomEvent).detail===tournament.id && !writing)void load();};window.addEventListener('librett-completion-updated',refresh);return()=>window.removeEventListener('librett-completion-updated',refresh);});
  let wasActive=untrack(()=>active);$effect(()=>{if(active && !wasActive && !busy)void load();wasActive=active;});
  function winner(category:CategoryResults){const id=category.placements.find(p=>p.stage==='winner')?.entry_id;const entry=category.draw?.participants.find(e=>e.id===id);return entry?.members.map(playerLabel).join(' / ');}
</script>
<section class="overview">
  <div class="section-heading"><div><h2>{text.tournamentOverview}</h2><p class="muted">{sr?'Napredak kategorija i završetak turnira.':'Category progress and tournament completion.'}</p></div><button class="secondary icon-label" disabled={busy} onclick={load}><Icon name="restore" size={16} />{sr?'Osveži':'Refresh'}</button></div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if loading}<p role="status">{text.loading}</p>{:else if !desktopAvailable}<p class="banner">{text.preview}</p>{:else if data}
    <div class="overview-status"><span class="pill">{data.completion.completed_at?(sr?'Turnir završen':'Tournament completed'):(sr?'Turnir u toku':'Tournament in progress')}</span><span>{finished}/{data.categories.length} {sr?'završenih kategorija':'completed categories'}</span>{#if data.completion.completed_at}<time datetime={data.completion.completed_at}>{new Date(data.completion.completed_at).toLocaleString(sr?'sr-Latn-RS':'en-GB')}</time>{/if}</div>
    {#if !data.categories.length}<p class="muted">{text.noCategories}</p>{/if}
    <div class="category-progress">{#each data.categories as category (category.category_id)}<button data-open-tab class="progress-card" disabled={writing} onclick={()=>oncategory(category.category_id)}><span class="progress-title"><strong>{category.category_name}</strong><span class="pill">{category.completion.completed_at ? (sr?'Završeno':'Completed') : category.ready ? (sr?'Spremno za potvrdu':'Ready to confirm') : (sr?'U toku':'In progress')}</span></span><InfoRows items={[{label:sr?'Grupe':'Groups',value:category.group_total?`${category.group_completed}/${category.group_total}`:null},{label:sr?'Nokaut':'Knockout',value:`${category.knockout_completed}/${category.knockout_total}`}]} />{#if winner(category)}<span class="progress-winner"><Icon name="trophy" size={16} /><PlayerName label={winner(category)!} /></span>{/if}<span class="progress-open">{text.results}<Icon name="arrow-right" size={17} /></span></button>{/each}</div>
    {#if !data.ready && !data.completion.completed_at}<p class="muted">{sr?'Za završetak turnira potvrdi završetak svake aktivne kategorije. Nekorišćene kategorije možeš prethodno ukloniti.':'To complete the tournament, confirm every active category. Remove unused categories first.'}</p>{/if}
    <div class="completion-actions"><CompletionControl tournamentId={tournament.id} completion={data.completion} version={data.version} ready={data.ready} {language} bind:busy={writing} onchanged={progress=>{data=progress;}} onreload={load} /><span class="muted">{sr?'Blagajna ostaje dostupna i posle završetka.':'The cash desk remains available after completion.'}</span></div>
  {/if}
</section>
<style>
.overview{min-width:0;}h2{font-size:18px;margin:0;}.section-heading{align-items:flex-start;flex-wrap:wrap;}.overview-status{display:flex;align-items:center;gap:18px;flex-wrap:wrap;font-size:12px;color:var(--text-secondary);margin:24px 0;}time{color:var(--text-muted);}.category-progress{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:20px;margin:24px 0;}.progress-card{display:flex;align-items:stretch;flex-direction:column;gap:16px;text-align:left;padding:22px;background:var(--surface);border:1px solid var(--border-subtle);border-radius:8px;color:var(--text);cursor:pointer;}.progress-card:hover{border-color:var(--primary);}.progress-card:focus-visible{outline:2px solid var(--primary);outline-offset:3px;}.progress-title{display:flex;flex-wrap:wrap;gap:10px;align-items:center;}.progress-title strong{font-size:14px;overflow-wrap:anywhere;}.progress-winner{display:flex;align-items:center;gap:10px;font-size:12px;overflow-wrap:anywhere;}.progress-winner :global(svg){flex-shrink:0;color:var(--primary);}.progress-open{display:flex;align-items:center;justify-content:space-between;margin-top:auto;padding-top:12px;border-top:1px solid var(--border-subtle);font-size:11px;color:var(--text-secondary);}.completion-actions{display:flex;align-items:center;gap:16px;flex-wrap:wrap;margin-top:26px;}@media(max-width:1050px){.category-progress{grid-template-columns:repeat(2,minmax(0,1fr));}}@media(max-width:700px){.category-progress{grid-template-columns:1fr;}}
</style>
