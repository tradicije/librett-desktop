<script lang="ts">
  import Icon from './Icon.svelte';
  import { tick } from 'svelte';
  import { changeCompletion, type CompletionState, type CompletionRequest, type TournamentProgress } from './api';
  import { messages, errorKey, type MessageKey, type Language } from './i18n';
  let { tournamentId, categoryId=null, completion, version, ready, parentClosed=false, language, busy=$bindable(false), onchanged, onreload }: {
    tournamentId:string;categoryId?:string|null;completion:CompletionState;version:string;ready:boolean;parentClosed?:boolean;language:Language;busy?:boolean;onchanged:(progress:TournamentProgress)=>Promise<void>|void;onreload:()=>Promise<void>;
  }=$props();
  let sr=$derived(language==='sr');let text=$derived(messages[language]);
  let closed=$derived(!!completion.completed_at);let saving=$state(false);let opened=$state(false);
  let draft=$state<CompletionRequest|null>(null);let pending=$state<CompletionRequest|null>(null);let error=$state<MessageKey|null>(null);
  let dialog:HTMLDialogElement;const uid=$props.id();
  $effect(()=>{busy=opened || saving || !!pending;});
  function open(){
    if(saving || parentClosed || (!closed && !ready))return;
    draft={request_id:crypto.randomUUID(),tournament_id:tournamentId,category_id:categoryId,expected_revision:completion.revision,expected_version:version,complete:!closed};
    error=null;opened=true;dialog.showModal();
  }
  function close(){if(saving || pending)return;opened=false;draft=null;dialog.close();}
  async function save(){
    if(saving || !draft)return;if(!pending)pending=structuredClone(draft);saving=true;error=null;
    try{
      const result=await changeCompletion(pending);
      pending=null;opened=false;draft=null;dialog.close();
      await onchanged(result);
      window.dispatchEvent(new CustomEvent('librett-completion-updated',{detail:tournamentId}));
      if(categoryId)window.dispatchEvent(new CustomEvent('librett-results-updated',{detail:categoryId}));
    }catch(cause){error=errorKey(cause);if(['completion_conflict','competition_closed','competition_incomplete','not_found'].includes(String(cause)))pending=null;}
    finally{saving=false;}
  }
</script>
<button class={closed?'secondary icon-label':'primary icon-label'} disabled={parentClosed || saving || (!closed && !ready)} onclick={open}><Icon name={closed?'restore':'check-circle'} size={18} />{closed ? (sr?'Ponovo otvori':'Reopen') : categoryId ? (sr?'Završi kategoriju':'Complete category') : (sr?'Završi turnir':'Complete tournament')}</button>
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} oncancel={event=>{event.preventDefault();close();}}>
  <h2 id={`${uid}-title`}>{draft?.complete ? (categoryId ? sr?'Završi kategoriju':'Complete category' : sr?'Završi turnir':'Complete tournament') : sr?'Ponovno otvaranje':'Reopen competition'}</h2>
  <p>{draft?.complete ? (sr?'Potvrđuješ završetak i čuvaš konačni plasman. Prijave, pravila, žreb i rezultati se zaključavaju. Blagajna ostaje dostupna.':'Confirm completion and preserve final standings. Registrations, rules, draws and results become read-only. The cash desk remains available.') : categoryId ? (sr?'Kategorija se otvara za ispravke. Sačuvana istorija rezultata i prethodnog završetka ostaje.':'Reopen this category for corrections. Result and completion history is retained.') : (sr?'Turnir se ponovo otvara. Završene kategorije ostaju zaključane dok ih zasebno ne otvoriš u Rezultatima.':'Reopen the tournament. Completed categories remain locked until individually reopened in Results.')}</p>
  {#if error}<p class="error" role="alert">{text[error]}</p>{/if}
  {#if pending && error}<p class="muted">{sr?'Upis nije potvrđen. Ponovi isti zahtev pre nastavka.':'The write was not confirmed. Retry the same request before continuing.'}</p>{/if}
  <div class="dialog-actions"><button class="secondary" disabled={saving || !!pending} onclick={close}>{sr?'Otkaži':'Cancel'}</button>{#if error==='completion_conflict' || error==='competition_incomplete' || error==='competition_closed'}<button class="primary" disabled={saving} onclick={async()=>{close();await tick();await onreload();}}>{sr?'Učitaj ponovo':'Reload'}</button>{:else}<button class="primary" disabled={saving} onclick={save}>{saving?text.saving:pending?text.retry:sr?'Potvrdi':'Confirm'}</button>{/if}</div>
</dialog>
<style>.confirm-dialog{max-width:540px;}.dialog-actions{flex-wrap:wrap;}</style>
