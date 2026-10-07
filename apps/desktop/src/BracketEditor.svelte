<script lang="ts">
  import KnockoutBracket from './KnockoutBracket.svelte';
  import { untrack } from 'svelte';
  import Select from './Select.svelte';
  import Icon from './Icon.svelte';
  import { bracketSlots, type BracketSlot } from './draw-view';
  import { playerLabel } from './player-label';
  import { confirmDiscard } from './confirmation';
  import { saveCategoryDraw, saveKnockoutFillers, type CategoryDraw, type CategoryRules, type CompetitionState, type Category, type Tournament, type FillerChoice, type SaveFillersRequest } from './api';
  import { messages, errorKey, type MessageKey, type Language } from './i18n';
  let {draw, rules, competition, tournament, category, language, busy=$bindable(false), dirty=$bindable(false), ondone}: {draw:CategoryDraw;rules:CategoryRules;competition:CompetitionState|null;tournament:Tournament;category:Category;language:Language;busy?:boolean;dirty?:boolean;ondone:()=>void}=$props();
  const uid=$props.id();
  const initialDraw=untrack(()=>structuredClone($state.snapshot(draw)));
  const initialState=untrack(()=>competition ? structuredClone($state.snapshot(competition)) : null);
  let draft=$state(initialDraw);
  let choices=$state<Record<string,FillerChoice>>(structuredClone(initialState?.fillers??{}));
  let resetAutomatic=$state(false);
  let selected=$state<number|null>(null);let selection=$state('');let saving=$state(false);let confirming=$state(false);
  let error=$state<MessageKey|null>(null);
  let pendingDraw=$state<CategoryDraw|null>(null);let pendingFillers=$state<SaveFillersRequest|null>(null);
  let slotDialog:HTMLDialogElement;let impactDialog:HTMLDialogElement;
  let sr=$derived(language==='sr');let text=$derived(messages[language]);
  let grouped=$derived(category.format==='groups_knockout');
  let ready=$derived(!grouped || !!initialState && !initialState.stale && initialState.groups.length>0 && initialState.groups.every(group=>group.complete && group.resolved));
  let locked=$derived(saving || confirming || !!pendingDraw || !!pendingFillers || category.completed || tournament.completed);
  let originalSlots=$derived(bracketSlots(initialDraw,rules,category.format,language,initialState?.slots));
  let slots=$derived.by(()=>{
    if(!grouped)return bracketSlots(draft,rules,category.format,language);
    return originalSlots.map((slot,index):BracketSlot=>{
      const choice=choices[index];
      if(choice?.kind==='bye')return {kind:'bye',label:'BYE'};
      if(choice?.kind==='entry'){
        const entry=draft.participants.find(entry=>entry.id===choice.entry_id);
        const group=initialState?.groups.find(group=>group.rows.some(row=>row.entry_id===choice.entry_id));
        const place=group ? group.rows.findIndex(row=>row.entry_id===choice.entry_id)+1 : undefined;
        return {id:choice.entry_id,kind:'entry',label:entry?.members.map(playerLabel).join(' / ')??choice.entry_id,group:group?.group,place,lucky_loser:!!place && place>draft.settings.qualifiers_per_group};
      }
      if(resetAutomatic || initialState?.fillers[index])return {kind:'pending',label:rules.knockout_filling==='lucky_loser_manual' ? sr?'Čeka izbor':'Awaiting selection' : sr?'Automatski po čuvanju':'Automatic after saving'};
      return slot;
    });
  });
  $effect(()=>{dirty=grouped ? JSON.stringify(choices)!==JSON.stringify(initialState?.fillers??{}) : JSON.stringify(draft.sections)!==JSON.stringify(initialDraw.sections);busy=saving || confirming || selected!==null || !!pendingDraw || !!pendingFillers;});
  let unassigned=$derived(draft.participants.filter(entry=>!draft.sections[0]?.includes(entry.id)));
  let options=$derived([{value:'',label:grouped ? rules.knockout_filling==='lucky_loser_manual' ? (sr?'Bez izbora':'Unassigned') : (sr?'Automatski':'Automatic') : 'BYE'},...(grouped?[{value:'bye',label:'BYE'}]:[]),...draft.participants.map(entry=>({value:entry.id,label:entry.members.map(playerLabel).join(' / ')}))]);
  function open(index:number){if(locked || !ready)return;selected=index;selection=grouped ? choices[index]?.kind==='bye'?'bye':choices[index]?.kind==='entry'?choices[index].entry_id:slots[index].id??'' : draft.sections[0][index]??'';slotDialog.showModal();}
  function apply(){
    if(selected===null || locked)return;
    const index=selected;
    if(grouped){
      const next={...choices};
      if(!selection)delete next[index];
      else if(selection==='bye')next[index]={kind:'bye'};
      else {
        const other=slots.findIndex((slot,i)=>i!==index && slot.id===selection);
        if(other>=0)next[other]=slots[index].id?{kind:'entry',entry_id:slots[index].id!}:{kind:'bye'};
        next[index]={kind:'entry',entry_id:selection};
      }
      choices=next;
    }else{
      const section=[...draft.sections[0]];const previous=section[index];const other=selection ? section.indexOf(selection) : -1;
      if(other>=0 && other!==index)section[other]=previous;
      if(!selection && previous){const empty=section.findIndex((id,i)=>i!==index && !id);if(empty>=0)section[empty]=previous;}
      section[index]=selection||null;draft={...draft,mode:'manual',sections:[section]};
    }
    error=null;slotDialog.close();selected=null;
  }
  async function close(){if(locked)return;if(!dirty || await confirmDiscard())ondone();}
  async function save(){
    if(saving || !ready || category.completed || tournament.completed)return;
    error=null;
    if(!grouped && !pendingDraw){
      pendingDraw={...structuredClone($state.snapshot(draft)),id:crypto.randomUUID(),revision:0};
      confirming=true;impactDialog.showModal();return;
    }
    if(grouped && !pendingFillers && initialState){pendingFillers={request_id:crypto.randomUUID(),tournament_id:tournament.id,category_id:category.id,draw_id:initialDraw.id,expected_revision:initialState.filler_revision,expected_match_version:initialState.match_version,rules_revision:initialState.rules_revision,order_revisions:[...initialState.order_revisions],result_versions:[...initialState.result_versions],fillers:structuredClone($state.snapshot(choices)),invalidate_downstream:false};}
    await write();
  }
  async function write(){
    if(saving)return;saving=true;
    try{
      if(grouped && pendingFillers)await saveKnockoutFillers(pendingFillers);
      else if(pendingDraw)await saveCategoryDraw(tournament.id,pendingDraw,initialDraw.revision,true);
      else return;
      pendingDraw=null;pendingFillers=null;dirty=false;window.dispatchEvent(new CustomEvent('librett-results-updated',{detail:category.id}));ondone();
    }catch(cause){
      if(cause==='result_impact'){confirming=true;impactDialog.showModal();}
      else {error=errorKey(cause);if(['competition_closed','match_conflict','draw_conflict','invalid_draw','invalid_result','invalid_rules','not_found'].includes(String(cause))){pendingDraw=null;pendingFillers=null;}}
    }finally{saving=false;}
  }
  function cancelImpact(){confirming=false;pendingDraw=null;pendingFillers=null;impactDialog.close();}
  async function confirmImpact(){confirming=false;if(pendingFillers)pendingFillers={...pendingFillers,request_id:crypto.randomUUID(),invalidate_downstream:true};impactDialog.close();await write();}
</script>
<div class="editor-toolbar"><div><strong>{sr?'Uređivanje žreba':'Edit bracket'}</strong><p class="muted">{sr?'Klikni na igrača ili BYE u prvoj rundi. Izbor postojećeg učesnika menja njihova mesta.':'Click a player or BYE in the opening round. Selecting an existing participant swaps their places.'}</p></div><div class="editor-actions">{#if grouped}<button class="secondary" disabled={locked || !ready || !Object.keys(choices).length} onclick={()=>{choices={};resetAutomatic=true;}}>{rules.knockout_filling==='lucky_loser_manual' ? sr?'Očisti ručni raspored':'Clear manual layout' : sr?'Vrati automatski raspored':'Restore automatic layout'}</button>{/if}<button class="secondary" disabled={locked} onclick={close}>{sr?'Odustani':'Cancel'}</button><button class="primary" disabled={saving || confirming || !ready || (!grouped && unassigned.length>0 && !pendingDraw) || (!dirty && !pendingDraw && !pendingFillers)} onclick={save}><Icon name="check-circle" size={16}/>{saving?text.saving:pendingDraw||pendingFillers?text.retry:sr?'Sačuvaj žreb':'Save bracket'}</button></div></div>
{#if error}<p class="error" role="alert">{text[error]}</p>{/if}
{#if !ready}<p class="banner">{sr?'Učesnike nokauta možeš menjati kada sve grupe završe mečeve i plasman bude razrešen.':'Knockout participants can be edited once every group finishes and its standings are resolved.'}</p>{/if}
{#if !grouped && unassigned.length}<p class="banner">{sr?'Rasporedi sve prijavljene učesnike pre čuvanja. Neraspoređeno':'Place every registered participant before saving. Unassigned'}: {unassigned.map(entry=>entry.members.map(playerLabel).join(' / ')).join(', ')}</p>{/if}
<KnockoutBracket {slots} {language} progress={dirty?[]:initialState?.matches??[]} onslot={open} editingDisabled={locked || !ready}/>
<dialog class="confirm-dialog slot-dialog" bind:this={slotDialog} aria-labelledby={`${uid}-slot`} onclose={()=>selected=null}>
  <h2 id={`${uid}-slot`}>{sr?'Učesnik na mestu':'Slot participant'} {selected===null?'':selected+1}</h2>
  <label>{sr?'Igrač / par ili slobodan prolaz':'Player / pair or bye'}<Select inline playerLabels label={sr?'Učesnik':'Participant'} bind:value={selection} {options}/></label>
  <p class="muted">{sr?'Učesnici dolaze iz prijava ove kategorije. Dodaj novog igrača u Prijavama.':'Participants come from this category’s registrations. Add new players in Registrations.'}</p>
  <div class="dialog-actions"><button class="secondary" onclick={()=>slotDialog.close()}>{sr?'Odustani':'Cancel'}</button><button class="primary" onclick={apply}>{sr?'Primeni':'Apply'}</button></div>
</dialog>
<dialog class="confirm-dialog" bind:this={impactDialog} aria-labelledby={`${uid}-impact`} oncancel={event=>{event.preventDefault();cancelImpact();}}>
  <h2 id={`${uid}-impact`}>{sr?'Promena učesnika':'Change participants'}</h2><p>{grouped ? sr?'Izmena poništava rezultate koji zavise od promenjenih učesnika. Prethodni upisi ostaju u istoriji.':'This clears results that depend on changed participants. Previous writes remain in history.' : sr?'Novi raspored pokreće novu verziju nokauta bez prethodnih rezultata. Stari žreb i rezultati ostaju u istoriji.':'The new layout starts a new knockout revision without previous results. The old draw and results remain in history.'}</p>
  <div class="dialog-actions"><button class="secondary" onclick={cancelImpact}>{sr?'Vrati se':'Back'}</button><button class="primary" onclick={confirmImpact}>{sr?'Potvrdi promenu':'Confirm change'}</button></div>
</dialog>
<style>
.editor-toolbar{display:flex;justify-content:space-between;align-items:flex-start;gap:20px;flex-wrap:wrap;}.editor-toolbar strong{font-size:14px;}.editor-toolbar p{margin:8px 0;max-width:560px;line-height:1.6;}.editor-actions{display:flex;gap:8px;flex-wrap:wrap;}label{display:grid;gap:8px;margin:20px 0;}
</style>
