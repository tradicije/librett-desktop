<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import Select from './Select.svelte';
  import PlayerName from './PlayerName.svelte';
  import Icon from './Icon.svelte';
  import { confirmDiscard } from './confirmation';
  import { playerLabel } from './player-label';
  import { groupName } from './draw-view';
  import { getCompetitionState, getCategoryDraw, saveKnockoutFillers, desktopAvailable, type CompetitionState, type FillerChoice, type SaveFillersRequest, type Tournament, type Category } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  let { manual=false, active=true, tournament, category, language, rulesRevision, externalLocked=false, busy=$bindable(false), dirty=$bindable(false) }: { manual?:boolean; active?:boolean; tournament:Tournament; category:Category; language:Language; rulesRevision:number; externalLocked?:boolean; busy?:boolean; dirty?:boolean }=$props();
  let sr=$derived(language==='sr');let text=$derived(messages[language]);
  let source=$state<CompetitionState|null>(null);let names=$state(new Map<string,string>());
  let choices=$state<Record<string,FillerChoice>>({});let baseline=$state('');let loading=$state(false);let saving=$state(false);
  let pending=$state<SaveFillersRequest|null>(null);let impact=$state<SaveFillersRequest|null>(null);let error=$state<MessageKey|null>(null);
  let impactDialog:HTMLDialogElement;const uid=$props.id();let refreshRequested=$state(false);
  let ready=$derived(!!source?.draw_id && !source.stale && source.groups.length>0 && source.groups.every(g=>g.complete && g.resolved));
  let vacancies=$derived(source?.slots.map((slot,index)=>({slot,index})).filter(({slot})=>slot.lucky_loser)??[]);
  let locked=$derived(externalLocked || loading || saving || !!pending || !!impact);
  $effect(()=>{busy=loading || saving || !!pending || !!impact;dirty=baseline!=='' && JSON.stringify(choices)!==baseline;});
  async function load(discard=false){
    if(!desktopAvailable || loading || saving || pending || impact || (dirty && !discard))return;
    loading=true;error=null;
    try {
      const [state,draw]=await Promise.all([getCompetitionState(tournament.id,category.id),getCategoryDraw(tournament.id,category.id)]);
      if(state.draw_id!==draw?.id && state.draw_id!==null){error='match_conflict';return;}
      source=state;choices=structuredClone(state.fillers);baseline=JSON.stringify(choices);
      names=new Map(draw?.participants.map(entry=>[entry.id,entry.members.map(playerLabel).join(' / ')])??[]);
    }catch(cause){error=errorKey(cause);}finally{loading=false;}
  }
  onMount(()=>{void load();const refresh=(event:Event)=>{if((event as CustomEvent<string>).detail===category.id)refreshRequested=true;};window.addEventListener('librett-results-updated',refresh);return()=>window.removeEventListener('librett-results-updated',refresh);});
  let wasActive=untrack(()=>active);let seenRevision=untrack(()=>rulesRevision);
  $effect(()=>{if((active && !wasActive) || rulesRevision!==seenRevision)refreshRequested=true;wasActive=active;seenRevision=rulesRevision;});
  $effect(()=>{if(refreshRequested && !locked && !dirty){refreshRequested=false;void untrack(load);}});
  function selected(index:number){const choice=choices[index];return choice?.kind==='entry'?choice.entry_id:choice?.kind==='bye'?'bye':'auto';}
  function choose(index:number,value:string){const next={...choices};if(value==='auto')delete next[index];else next[index]=value==='bye'?{kind:'bye'}:{kind:'entry',entry_id:value};choices=next;error=null;}
  function options(index:number){
    const used=new Set(Object.entries(choices).filter(([key])=>Number(key)!==index).flatMap(([,choice])=>choice.kind==='entry'?[choice.entry_id]:[]));
    const result=[{value:'auto',label:manual ? sr?'Izaberi učesnika':'Choose participant' : sr?'Automatski':'Automatic'},{value:'bye',label:'BYE'},...(source?.candidates??[]).filter(c=>!used.has(c.standing.entry_id)).map(c=>({value:c.standing.entry_id,label:`LL ${groupName(c.group)}${c.place} ${names.get(c.standing.entry_id)??c.standing.entry_id}`}))];
    const value=selected(index);if(!result.some(option=>option.value===value))result.push({value,label:`${sr?'Proveri izbor':'Review choice'}: ${names.get(value)??value}`});
    return result;
  }
  async function save(){
    if(!ready || !source?.draw_id || saving)return;
    if(!pending)pending={request_id:crypto.randomUUID(),tournament_id:tournament.id,category_id:category.id,draw_id:source.draw_id,expected_revision:source.filler_revision,expected_match_version:source.match_version,rules_revision:source.rules_revision,order_revisions:[...source.order_revisions],result_versions:[...source.result_versions],fillers:structuredClone(choices),invalidate_downstream:false};
    await write();
  }
  async function write(){
    if(!pending || saving)return;saving=true;error=null;
    try{const result=await saveKnockoutFillers(pending);source=result;choices=structuredClone(result.fillers);baseline=JSON.stringify(choices);pending=null;window.dispatchEvent(new CustomEvent('librett-results-updated',{detail:category.id}));}
    catch(cause){if(cause==='result_impact'){impact=pending;pending=null;impactDialog.showModal();}else{error=errorKey(cause);if(['competition_closed','match_conflict','invalid_result','not_found'].includes(String(cause)))pending=null;}}
    finally{saving=false;}
  }
  function cancelImpact(){impact=null;impactDialog.close();}
  async function confirmImpact(){if(!impact)return;pending={...impact,request_id:crypto.randomUUID(),invalidate_downstream:true};impact=null;impactDialog.close();await write();}
  async function reload(){if(locked)return;if(!dirty || await confirmDiscard())await load(true);}
</script>
<section class="fill-setup" aria-labelledby={`${uid}-heading`}>
  <div class="section-heading"><div><h2 id={`${uid}-heading`}>{sr?'Lucky loser mesta':'Lucky loser places'}</h2><p class="muted">{manual ? sr?'Izaberi igrača ili BYE za svako mesto.':'Choose a player or BYE for each place.' : sr?'Automatski popuni sva mesta ili izaberi igrača / BYE za svako mesto.':'Fill every place automatically, or choose a player / BYE for each place.'}</p></div><button class="secondary icon-label" disabled={locked} onclick={reload}><Icon name="restore" size={16} />{sr?'Osveži':'Refresh'}</button></div>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if loading}<p role="status">{text.loading}</p>
  {:else if !source?.draw_id}<p class="muted">{sr?'Prvo sačuvaj raspored grupa.':'Save a group arrangement first.'}</p>
  {:else if source.stale}<p class="banner">{sr?'Raspored je zastareo. Prvo sačuvaj novi raspored.':'The arrangement is outdated. Save a new arrangement first.'}</p>
  {:else if !vacancies.length}<p class="muted">{sr?'Kostur je popunjen direktnim prolaznicima — nema slobodnih mesta.':'Direct qualifiers fill the bracket — there are no vacant places.'}</p>
  {:else if !ready}<p class="muted">{sr?'Izbor čeka završetak i razrešen plasman svih grupa. Do tada ova mesta nisu BYE.':'Selection awaits completed, resolved standings in every group. Until then these places are not BYEs.'}</p>
  {:else}
    {#if source.slots.some(slot=>slot.lucky_loser && !slot.bye && !slot.entry_id)}<p class="banner" role="status">{manual ? sr?'Izaberi učesnika ili BYE za sva prazna mesta.':'Choose a participant or BYE for every vacant place.' : sr?'Mesto čeka ručni izbor ili razrešenje izjednačenja.':'A place awaits manual selection or a resolved tie.'}</p>{/if}
    <div class="fill-places">{#each vacancies as {slot,index} (index)}<label><span><span>{sr?'Meč':'Match'} {Math.floor(index/2)+1}</span> · <small class="muted">{index%2===0 ? (sr?'Prvo mesto':'First place') : (sr?'Drugo mesto':'Second place')}</small></span><Select playerLabels label={`${sr?'Lucky loser mesto':'Lucky loser place'} ${index+1}`} bind:value={() => selected(index), value => choose(index,value)} options={options(index)} disabled={locked} /><small>{sr?'Sačuvano':'Saved'}: {#if slot.entry_id}LL {groupName(slot.group!)}{slot.place} <PlayerName label={names.get(slot.entry_id)??slot.entry_id} />{:else if slot.bye}BYE{:else}{sr?'Čeka izbor':'Awaiting selection'}{/if}</small></label>{/each}</div>
    <div class="form-actions"><button class="secondary" disabled={locked || !Object.keys(choices).length} onclick={()=>choices={}}>{manual ? sr?'Očisti izbor':'Clear selections' : sr?'Sve automatski':'All automatic'}</button><button class="primary" disabled={saving || externalLocked || loading || !!impact || (!pending && !dirty)} onclick={save}>{saving?text.saving:pending?text.retry:sr?'Sačuvaj mesta':'Save places'}</button></div>
    <details><summary>{sr?'Kandidati ispod crte':'Non-qualifying candidates'} ({source.candidates.length})</summary><p class="muted">{sr?'Plasman → procenat pobeda → odnos setova → odnos poena. Izjednačenje na granici rešava organizator.':'Group place → win percentage → set ratio → point ratio. The organizer resolves cutoff ties.'}</p><div class="candidates-scroll"><table><thead><tr><th>{sr?'Igrač':'Player'}</th><th>{sr?'Plasman':'Place'}</th><th>{sr?'Pobede':'Wins'}</th><th>{sr?'Setovi':'Sets'}</th><th>{sr?'Poeni':'Points'}</th></tr></thead><tbody>{#each source.candidates as candidate (candidate.standing.entry_id)}<tr><td><PlayerName label={names.get(candidate.standing.entry_id)??candidate.standing.entry_id} /></td><td>{groupName(candidate.group)}{candidate.place}</td><td>{candidate.standing.played ? (candidate.standing.wins/candidate.standing.played*100).toFixed(1) : '0'}% <small>({candidate.standing.wins}/{candidate.standing.played})</small></td><td>{candidate.standing.sets_for}:{candidate.standing.sets_against}</td><td>{candidate.standing.points_for}:{candidate.standing.points_against}</td></tr>{/each}</tbody></table></div></details>
  {/if}
</section>
<dialog class="confirm-dialog" bind:this={impactDialog} aria-labelledby={`${uid}-impact`} oncancel={event=>{event.preventDefault();cancelImpact();}}><h2 id={`${uid}-impact`}>{sr?'Promena nokaut učesnika':'Change knockout participants'}</h2><p>{sr?'Promena poništava nokaut rezultate koji zavise od ovih učesnika. Istorija ostaje sačuvana.':'This change clears knockout results that depend on these participants. History is retained.'}</p><div class="dialog-actions"><button class="secondary" onclick={cancelImpact}>{sr?'Vrati se':'Back'}</button><button class="primary" onclick={confirmImpact}>{sr?'Potvrdi promenu':'Confirm change'}</button></div></dialog>
<style>
 .fill-setup{border-top:1px solid var(--border-subtle);padding-top:24px;min-width:0;}h2{margin:0;font-size:16px;}.section-heading{align-items:flex-start;flex-wrap:wrap;} .fill-places{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:20px 24px;margin:22px 0;}label{display:grid;gap:8px;min-width:0;}label>span{font-size:11px;color:var(--text-secondary);}small{font-size:10px;color:var(--text-muted);}details{margin-top:24px;}summary{cursor:pointer;font-size:12px;font-weight:550;}details p{margin:16px 0;}.candidates-scroll{overflow:auto;max-height:400px;}table{width:100%;border-collapse:collapse;font-size:12px;}th,td{text-align:left;padding:12px 10px;border-bottom:1px solid var(--border-subtle);}th{font-size:11px;color:var(--text-secondary);}td:first-child{min-width:180px;} .confirm-dialog{max-width:520px;}@media(max-width:750px){.fill-places{grid-template-columns:1fr;}}
</style>
