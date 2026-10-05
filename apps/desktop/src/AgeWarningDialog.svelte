<script lang="ts">
  import { onDestroy } from 'svelte';
  import PlayerName from './PlayerName.svelte';
  import type { Player, CategoryRules } from './api';
  import type { Language } from './i18n';
  let { language }: {language:Language}=$props();
  let dialog:HTMLDialogElement;const uid=$props.id();
  let players=$state<Player[]>([]);let minimum=$state(0);let maximum=$state(130);let year=$state(new Date().getFullYear());
  let resolve:((accepted:boolean)=>void)|undefined;let accepted=false;
  export function confirm(selected:Player[],rules:CategoryRules):Promise<boolean>{
    if(!rules.age_enabled)return Promise.resolve(true);
    year=new Date().getFullYear();minimum=rules.age_min??0;maximum=rules.age_max??130;
    players=selected.filter(player=>player.birth_year===null || year-player.birth_year<minimum || year-player.birth_year>maximum);
    if(!players.length)return Promise.resolve(true);
    if(resolve || document.querySelector('dialog[open]'))return Promise.resolve(false);
    accepted=false;return new Promise(done=>{resolve=done;dialog.showModal();});
  }
  function finish(){const done=resolve;resolve=undefined;done?.(accepted);}
  onDestroy(()=>{accepted=false;finish();});
</script>
<dialog class="confirm-dialog age-dialog" bind:this={dialog} aria-labelledby={`${uid}-title`} onclose={finish}>
  <h2 id={`${uid}-title`}>{language==='sr'?'Igrači van starosne grupe':'Players outside the age group'}</h2>
  <p>{language==='sr'?`Ova kategorija je za uzrast ${minimum}–${maximum} godina. Potvrdi prijavu kao izuzetak.`:`This category is for ages ${minimum}–${maximum}. Confirm registration as an exception.`}</p>
  <ul>{#each players as player (player.id)}<li><span><PlayerName {player} /></span><small>{player.birth_year===null ? language==='sr'?'Godište nije uneto':'Birth year unknown' : `${year-player.birth_year} ${language==='sr'?'godina':'years'} · ${player.birth_year}`}</small></li>{/each}</ul>
  <p class="muted">{language==='sr'?`Starost se računa prema godini ${year}.`:`Age is calculated using year ${year}.`}</p>
  <div class="dialog-actions"><button type="button" class="secondary" onclick={()=>dialog.close()}>{language==='sr'?'Otkaži':'Cancel'}</button><button type="button" class="primary" onclick={()=>{accepted=true;dialog.close();}}>{language==='sr'?'Prijavi kao izuzetak':'Register as exception'}</button></div>
</dialog>
<style>
 .age-dialog{width:min(520px,calc(100vw - 32px));max-height:calc(100dvh - 32px);overflow:auto;}h2{margin:0;}ul{list-style:none;padding:0;margin:20px 0;}li{padding:10px 0;border-bottom:1px solid var(--border-subtle);}small{display:block;margin-top:5px;color:var(--text-muted);font-size:11px;} .dialog-actions{flex-wrap:wrap;}
</style>
