<script lang="ts">
  import Icon from './Icon.svelte';
  import PlayerName from './PlayerName.svelte';
  import { saveGroupOrder, type CompetitionState, type GroupOrderRequest, type Tournament, type Category } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  import { groupName } from './draw-view';
  let { language, tournament, category, names, busy = $bindable(false), dirty = $bindable(false), onreload }: { language:Language; tournament:Tournament; category:Category; names:Map<string,string>; busy?:boolean; dirty?:boolean; onreload:()=>Promise<void> }=$props();
  let text=$derived(messages[language]);let sr=$derived(language==='sr');
  let dialog:HTMLDialogElement;const uid=$props.id();
  let source=$state<CompetitionState|null>(null);let group=$state(0);let order=$state<string[]>([]);
  let baseline='';let saving=$state(false);let pending=$state<GroupOrderRequest|null>(null);let impact=$state<GroupOrderRequest|null>(null);let error=$state<MessageKey|null>(null);let discard=$state<'cancel'|'reload'|null>(null);
  $effect(()=>{busy=source!==null || saving || pending!==null;dirty=source!==null && JSON.stringify(order)!==baseline;});
  export function open(state:CompetitionState,index:number){source=state;group=index;order=state.groups[index].rows.map(r=>r.entry_id);baseline=JSON.stringify(order);error=null;discard=null;impact=null;dialog.showModal();}
  function close(){source=null;pending=null;impact=null;discard=null;dialog.close();}
  function cancel(){if(saving || pending)return;if(dirty)discard='cancel';else close();}
  function move(index:number,delta:number){const target=index+delta;if(target<0 || target>=order.length)return;const next=[...order];[next[index],next[target]]=[next[target],next[index]];order=next;}
  async function save(automatic=false){
    if(saving || !source?.draw_id)return;
    if(!pending)pending={request_id:crypto.randomUUID(),tournament_id:tournament.id,category_id:category.id,draw_id:source.draw_id,group,expected_revision:source.order_revisions[group],expected_result_version:source.result_versions[group],order:automatic ? null : [...order],invalidate_downstream:false};
    await write();
  }
  async function write(){if(!pending || saving)return;saving=true;error=null;try{await saveGroupOrder(pending);close();window.dispatchEvent(new CustomEvent('librett-results-updated',{detail:category.id}));await onreload();}catch(cause){if(cause==='result_impact'){impact=pending;pending=null;}else{error=errorKey(cause);if(['match_conflict','invalid_result','not_found'].includes(String(cause)))pending=null;}}finally{saving=false;}}
  async function confirmImpact(){if(!impact)return;pending={...impact,request_id:crypto.randomUUID(),invalidate_downstream:true};impact=null;await write();}
</script>
<dialog class="confirm-dialog ranking-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} oncancel={event=>{event.preventDefault();cancel();}}>
  <h2 id={`${uid}-title`}>{sr?'Plasman grupe':'Group ranking'} {groupName(group)}</h2>
  {#if discard}<p>{sr?'Odbaci nesačuvan redosled?':'Discard the unsaved order?'}</p><div class="dialog-actions"><button class="secondary" onclick={()=>discard=null}>{sr?'Nastavi':'Continue'}</button><button class="primary" onclick={async()=>{const reload=discard==='reload';close();if(reload)await onreload();}}>{sr?'Odbaci':'Discard'}</button></div>
  {:else if impact}<p class="banner">{sr?'Promena prolaznika poništiće nokaut rezultate koji zavise od njih. Istorija ostaje sačuvana.':'Changing qualifiers clears their dependent knockout results. History is retained.'}</p><div class="dialog-actions"><button class="secondary" onclick={()=>impact=null}>{sr?'Vrati se':'Back'}</button><button class="primary" onclick={confirmImpact}>{sr?'Potvrdi':'Confirm'}</button></div>
  {:else}
    <p class="muted">{sr?'Automatski plasman koristi pobede, mini-tabelu izjednačenih i zadate odnose. Organizator može da potvrdi ručni redosled. Novi grupni rezultat vraća automatski obračun.':'Automatic ranking uses wins, the tied-player mini-table and configured ratios. The organizer can confirm a manual order. A new group result restores automatic ranking.'}</p>
    {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
    <ol>{#each order as id,index}<li><b>{index+1}</b><span><PlayerName label={names.get(id)??id} /></span><button class="icon-button" aria-label={`${sr?'Pomeri gore':'Move up'}: ${names.get(id)}`} disabled={saving || !!pending || index===0} onclick={()=>move(index,-1)}><Icon name="arrow-up" size={16} /></button><button class="icon-button" aria-label={`${sr?'Pomeri dole':'Move down'}: ${names.get(id)}`} disabled={saving || !!pending || index===order.length-1} onclick={()=>move(index,1)}><Icon name="arrow-down" size={16} /></button></li>{/each}</ol>
    <div class="dialog-actions">{#if error==='match_conflict'}<button class="secondary" onclick={()=>discard='reload'}>{sr?'Učitaj ponovo':'Reload'}</button>{:else}<button class="secondary" disabled={saving || !!pending} onclick={()=>save(true)}>{sr?'Automatski':'Automatic'}</button>{/if}<button class="secondary" disabled={saving || !!pending} onclick={cancel}>{sr?'Otkaži':'Cancel'}</button><button class="primary" disabled={saving} onclick={()=>save()}>{saving?text.saving:pending?text.retry:sr?'Sačuvaj ručno':'Save manual order'}</button></div>
  {/if}
</dialog>
<style>
 .ranking-dialog{width:min(620px,calc(100vw - 32px));max-height:calc(100dvh - 32px);overflow:auto;}h2{margin:0;}ol{list-style:none;padding:0;margin:20px 0;}li{display:flex;align-items:center;gap:12px;padding:10px 0;border-bottom:1px solid var(--border-subtle);}li>span{flex:1;overflow-wrap:anywhere;}li>b{width:22px;color:var(--text-muted);} .dialog-actions{flex-wrap:wrap;}
</style>
