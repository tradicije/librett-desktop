<script lang="ts">
  import Select from './Select.svelte';
  import Icon from './Icon.svelte';
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import { desktopAvailable, getActionHistory, type HistoryItem, type HistoryOption, type CategoryDraw, type FinalPlacement } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  import { formatMoney } from './money';
  let { language, active, busy=$bindable(false) }:{language:Language;active:boolean;busy?:boolean}=$props();
  let sr=$derived(language==='sr');let text=$derived(messages[language]);
  let items=$state<HistoryItem[]>([]);let tournaments=$state<HistoryOption[]>([]);let categories=$state<HistoryOption[]>([]);
  let selected=$state<string[]>([]);let tournament=$state('');let category=$state('');let action=$state('');
  let loading=$state(false);let hasMore=$state(false);let error=$state<MessageKey|null>(null);let request=0;
  const kinds=['results','categories','rules','draws','registrations','attendance','cash','tables','completion','tournaments','players','trash','backups','exports'];
  const actions=['created','updated','deleted','recorded','corrected','invalidated','archived','restored','completed','reopened','trashed','image_changed','charge','discount','payment','refund','allocated','exported'];
  function kindLabel(key:string){return ({results:sr?'Rezultati':'Results',categories:sr?'Kategorije':'Categories',rules:sr?'Pravila':'Rules',draws:sr?'Žreb i plasman grupa':'Draws and group ranking',registrations:sr?'Prijave':'Registrations',attendance:sr?'Dolasci':'Attendance',cash:sr?'Blagajna':'Cash desk',tables:sr?'Stolovi':'Tables',completion:sr?'Završavanje i otvaranje':'Completion and reopening',tournaments:sr?'Turniri':'Tournaments',players:sr?'Igrači':'Players',trash:sr?'Korpa':'Trash',backups:sr?'Rezervne kopije':'Backups',exports:sr?'Izvoz':'Exports'} as Record<string,string>)[key]??key;}
  function actionLabel(key:string){return ({created:sr?'Dodavanje':'Created',updated:sr?'Izmena':'Updated',deleted:sr?'Uklanjanje':'Removed',recorded:sr?'Sačuvano':'Recorded',corrected:sr?'Ispravka':'Corrected',invalidated:sr?'Poništavanje rezultata':'Result invalidated',archived:sr?'Arhiviranje':'Archived',restored:sr?'Vraćanje':'Restored',completed:sr?'Završavanje':'Completed',reopened:sr?'Ponovno otvaranje':'Reopened',trashed:sr?'Premeštanje u korpu':'Moved to trash',image_changed:sr?'Promena slike':'Image changed',charge:sr?'Zaduženje':'Charge',discount:sr?'Popust':'Discount',payment:sr?'Uplata':'Payment',refund:sr?'Povraćaj':'Refund',allocated:sr?'Raspodela uplate':'Payment allocation',exported:sr?'Izvoz fajla':'File exported'} as Record<string,string>)[key]??key;}
  const fieldNames:Record<string,[string,string]>={name:['Ime / naziv','Name'],club:['Klub','Club'],birth_year:['Godište','Birth year'],city:['Grad','City'],country:['Država','Country'],email:['Email','Email'],phone:['Telefon','Phone'],notes:['Beleške','Notes'],status:['Status','Status'],checked_in:['Dolazak','Attendance'],fee_minor:['Kotizacija','Entry fee'],amount_minor:['Iznos','Amount'],note:['Beleška','Note'],revision:['Verzija zapisa','Record revision'],group_number:['Grupa','Group'],table_number:['Sto','Table'],table_count:['Broj stolova','Table count'],started_at:['Početak','Started'],discipline:['Disciplina','Discipline'],format:['Format','Format'],archived:['Arhivirano','Archived'],has_image:['Slika','Image'],name_snapshot:['Igrač','Player'],club_snapshot:['Klub','Club'],position:['Pozicija','Position'],best_of:['Broj setova','Best of'],points_to_win:['Poena za set','Points to win'],win_by:['Razlika','Winning margin'],group_count:['Broj grupa','Groups'],qualifiers_per_group:['Prolaznika po grupi','Qualifiers per group'],knockout_filling:['Popuna nokauta','Knockout filling'],third_place:['Treće mesto','Third place'],age_enabled:['Starosna grupa','Age restriction'],age_min:['Minimalna starost','Minimum age'],age_max:['Maksimalna starost','Maximum age'],ranking:['Kriterijumi plasmana','Ranking criteria'],first_name:['Prvi učesnik','First participant'],second_name:['Drugi učesnik','Second participant'],winner_name:['Pobednik','Winner']};
  function value(key:string,input:unknown):string{
    if(input===null||input===undefined)return '—';
    if(key.endsWith('_minor')&&typeof input==='number')return formatMoney(input,language);
    if(typeof input==='boolean'||['checked_in','archived','has_image'].includes(key))return input?(sr?'Da':'Yes'):(sr?'Ne':'No');
    if(typeof input==='object')return Array.isArray(input)?input.map(v=>value('',v)).join(', '):Object.entries(input).map(([k,v])=>`${fieldNames[k]?.[sr?0:1]??k}: ${value(k,v)}`).join('; ');
    return ({registered:sr?'Aktivna prijava':'Active registration',withdrawn:sr?'Povučena prijava':'Withdrawn',queued:sr?'Dodeljen':'Assigned',running:sr?'U toku':'Running',released:sr?'Oslobođen':'Released',singles:sr?'Singl':'Singles',doubles:sr?'Dubl':'Doubles',knockout:sr?'Nokaut':'Knockout',groups_knockout:sr?'Grupe → nokaut':'Groups → knockout',bye:'BYE',lucky_loser:sr?'Automatski lucky loser':'Automatic lucky loser',lucky_loser_manual:sr?'Ručni lucky loser':'Manual lucky loser',shared:sr?'Podeljeno treće mesto':'Shared third place',bronze_match:sr?'Meč za treće mesto':'Bronze match',champion_semifinalist:sr?'Poraženi od šampiona':'Semifinalist beaten by champion',head_to_head:sr?'Međusobni meč':'Head to head',set_ratio:sr?'Odnos setova':'Set ratio',point_ratio:sr?'Odnos poena':'Point ratio'} as Record<string,string>)[String(input)]??String(input);
  }
  function entityLabel(item:HistoryItem){
    if(['results','tables'].includes(item.kind)&&/^(ko|group):/.test(item.entity_name)){
      const parts=item.entity_name.split(':');
      return parts[0]==='group'?`${sr?'Grupa':'Group'} ${String.fromCharCode(65+Number(parts[1]))} · ${sr?'Kolo':'Round'} ${Number(parts[2])+1} · ${sr?'Meč':'Match'} ${Number(parts[3])+1}`:`${sr?'Nokaut':'Knockout'} · ${sr?'Kolo':'Round'} ${Number(parts[1])+1} · ${sr?'Meč':'Match'} ${Number(parts[2])+1}`;
    }
    if(item.kind==='players'||item.entity_type==='entry_members'){const data=item.after_data??item.before_data;return playerLabel({name:item.entity_name,club:String(data?.club??data?.club_snapshot??'')});}
    const placeholders=['Prijava / Registration','Pravila / Rules','Žreb / Draw','Plasman grupe / Group ranking','Nokaut mesta / Knockout slots','Stolovi / Tables','Završetak / Completion','Korpa / Trash','Blagajna / Cash'];
    return placeholders.includes(item.entity_name)?item.entity_name.split(' / ')[sr?0:1]:item.entity_name;
  }
  function details(data:Record<string,unknown>|null,item:HistoryItem):{label:string;value:string}[]{
    if(!data)return [];
    const rows=Object.entries(data).filter(([key])=>!['payload','match_key','entity_kind','player_id'].includes(key)&&!key.endsWith('_members')).map(([key,v])=>({label:fieldNames[key]?.[sr?0:1]??key.replaceAll('_',' '),value:key.endsWith('_name')&&Array.isArray(data[key.replace('_name','_members')])?(data[key.replace('_name','_members')] as {name:string;club:string}[]).map(playerLabel).join(' / ')||'—':value(key,v)}));
    if(typeof data.payload==='string'){
      try {
        const payload=JSON.parse(data.payload);
        if(item.kind==='results')rows.push({label:sr?'Setovi':'Sets',value:payload?.sets?.map((s:{first:number;second:number})=>`${s.first}:${s.second}`).join(' · ')??(sr?'Rezultat poništen':'Result cleared')});
        else if(item.kind==='rules'&&payload)for(const [key,v] of Object.entries(payload))rows.push({label:fieldNames[key]?.[sr?0:1]??key.replaceAll('_',' '),value:value(key,v)});
        else if(['draws','completion'].includes(item.kind)&&payload){
          const draw:CategoryDraw|undefined=item.kind==='draws'?payload:payload.draw;
          if(draw?.participants){
            const names=new Map(draw.participants.map(e=>[e.id,e.members.map(m=>playerLabel(m)).join(' / ')]));
            if(item.kind==='draws'){
              rows.push({label:sr?'Režim':'Mode',value:draw.mode==='manual'?(sr?'Ručno':'Manual'):(sr?'Automatski':'Automatic')});
              rows.push({label:sr?'Nosioci':'Seeds',value:draw.seeds.map(id=>names.get(id)??'—').join(', ')||'—'});
              draw.sections.forEach((section,index)=>rows.push({label:draw.format==='groups_knockout'?`${sr?'Grupa':'Group'} ${String.fromCharCode(65+index)}`:(sr?'Kostur':'Bracket'),value:section.map((id,index)=>`${index+1}. ${id?names.get(id)??'—':'BYE'}`).join('\n')}));
            } else if(payload.placements) rows.push({label:sr?'Plasman':'Standings',value:payload.placements.map((p:FinalPlacement)=>`${p.place}${p.place_end!==p.place?'–'+p.place_end:''}. ${names.get(p.entry_id)??'—'}`).join('\n')});
          } else rows.push({label:sr?'Učesnici / mesta':'Participants / slots',value:String(Array.isArray(payload)?payload.length:Object.keys(payload).length)});
        }
      } catch { /* Other immutable metadata remains readable. */ }
    }
    return rows;
  }
  async function load(append=false){
    if(!desktopAvailable)return;
    const version=++request;loading=true;error=null;
    const last=append?items.at(-1):null;
    // A failed filter change must not leave an old cursor attached to new filters.
    if(!append){items=[];hasMore=false;}
    try{const page=await getActionHistory([...selected],tournament||null,category||null,action||null,last?.occurred_at??null,last?.id??null);if(version!==request)return;items=append?[...items,...page.items]:page.items;hasMore=page.has_more;tournaments=page.tournaments;categories=page.categories;}
    catch(cause){if(version===request)error=errorKey(cause);}
    finally{if(version===request)loading=false;}
  }
  $effect(()=>{busy=loading;});
  $effect(()=>{if(active&&desktopAvailable){selected;tournament;category;action;void load();}});
  function historyIcon(kind:string):'check-circle'|'settings'|'layer-group'|'users'|'cash'|'desktop'|'trophy'|'user'|'trash'|'backup'|'history'{
    const icons={results:'check-circle',categories:'layer-group',rules:'settings',draws:'layer-group',registrations:'users',attendance:'users',cash:'cash',tables:'desktop',completion:'check-circle',tournaments:'trophy',players:'user',trash:'trash',backups:'backup',exports:'history'} as const;
    return icons[kind as keyof typeof icons]??'history';
  }
  function day(date:string){return new Date(date).toLocaleDateString(sr?'sr-Latn-RS':'en-GB',{day:'numeric',month:'long',year:'numeric'});}
  function toggle(kind:string){selected=selected.includes(kind)?selected.filter(k=>k!==kind):[...selected,kind];}
</script>

<div class="heading"><div><h1>{sr?'Istorija':'History'}</h1><p class="muted">{sr?'Sve zabeležene akcije, najnovije prvo.':'All recorded actions, newest first.'}</p></div><button class="secondary icon-label" disabled={loading} onclick={()=>load()}><Icon name="restore" size={16}/>{sr?'Osveži':'Refresh'}</button></div>
<section class="panel history-filters">
  <div class="filter-selects">
    <label class="field-label">{sr?'Turnir':'Tournament'}<Select label={sr?'Turnir':'Tournament'} bind:value={()=>tournament,(v)=>{tournament=v;category='';}} options={[{value:'',label:sr?'Svi turniri':'All tournaments'},...tournaments.map(t=>({value:t.id,label:t.name}))]}/></label>
    <label class="field-label">{sr?'Kategorija':'Category'}<Select label={sr?'Kategorija':'Category'} bind:value={category} options={[{value:'',label:sr?'Sve kategorije':'All categories'},...categories.filter(c=>!tournament||c.tournament_id===tournament).map(c=>({value:c.id,label:`${c.name}${!tournament?' — '+(tournaments.find(t=>t.id===c.tournament_id)?.name??''):''}`}))]}/></label>
    <label class="field-label">{sr?'Akcija':'Action'}<Select label={sr?'Akcija':'Action'} bind:value={action} options={[{value:'',label:sr?'Sve akcije':'All actions'},...actions.map(a=>({value:a,label:actionLabel(a)}))]}/></label>
  </div>
  <fieldset><legend>{sr?'Vrste istorije':'History types'}</legend><div class="kind-filters">{#each kinds as kind}<button type="button" class="secondary" class:selected={selected.includes(kind)} aria-pressed={selected.includes(kind)} onclick={()=>toggle(kind)}>{kindLabel(kind)}</button>{/each}</div></fieldset>
  <div class="filter-note"><span class="muted">{selected.length===0?(sr?'Prikazane su sve vrste.':'All types are shown.'):(sr?'Prikazane su izabrane vrste.':'Selected types are shown.')}</span><button class="secondary" onclick={()=>{selected=[];tournament='';category='';action='';}}>{sr?'Ukloni filtere':'Clear filters'}</button></div>
</section>
<p class="muted history-note">{sr?'Starije akcije dostupne su tamo gde je istorija već bila sačuvana.':'Older actions are available where history was already recorded.'}</p>
{#if error}<p class="error" role="alert">{text[error]} <button class="secondary" onclick={()=>load()}>{text.retry}</button></p>{/if}
{#if loading}<p class="muted" role="status">{text.loading}</p>{/if}
{#if !loading&&!items.length&&!error}<p class="muted">{sr?'Nema zabeleženih akcija za ove filtere.':'No recorded actions match these filters.'}</p>{/if}
<div class="history-list">{#each items as item,index (item.id)}
  {#if index===0||day(item.occurred_at)!==day(items[index-1].occurred_at)}<h2 class="history-day">{day(item.occurred_at)}</h2>{/if}
  <article class="history-row">
    <span class="player-avatar history-icon"><Icon name={historyIcon(item.kind)} size={20}/></span>
    <div class="history-body">
    <div class="history-summary">
      <div class="history-subject"><h2>{actionLabel(item.action)}</h2><p><PlayerName label={entityLabel(item)}/></p></div>
      <div class="history-context">{#if item.tournament_name}<span>{item.tournament_name}</span>{/if}{#if item.category_name}<span class="muted">{item.category_name}</span>{/if}</div>
      <div class="history-meta"><span class="pill">{kindLabel(item.kind)}</span><time datetime={item.occurred_at} title={new Date(item.occurred_at).toLocaleString(sr?'sr-Latn-RS':'en-GB')}>{new Date(item.occurred_at).toLocaleTimeString(sr?'sr-Latn-RS':'en-GB',{hour:'2-digit',minute:'2-digit',second:'2-digit'})}</time>{#if item.historical}<small class="muted">{sr?'Prethodno sačuvano':'Previously saved'}</small>{/if}</div>
    </div>
    {#if item.before_data||item.after_data}<details><summary>{sr?'Detalji promene':'Change details'}</summary><div class="history-change">{#each [{label:sr?'Pre':'Before',data:item.before_data},{label:sr?'Posle':'After',data:item.after_data}] as side}<section><h3>{side.label}</h3>{#if side.data}<dl>{#each details(side.data,item) as field}<div><dt>{field.label}</dt><dd><PlayerName label={field.value}/></dd></div>{/each}</dl>{:else}<p class="muted">—</p>{/if}</section>{/each}</div></details>{/if}
    </div>
  </article>
{/each}</div>
{#if hasMore}<button class="secondary" disabled={loading} onclick={()=>load(true)}>{sr?'Učitaj starije':'Load older'}</button>{/if}

<style>
  .history-filters { margin-bottom: 16px; }
  .filter-selects { display: grid; grid-template-columns: repeat(3,minmax(0,1fr)); gap: 16px; }
  .history-filters fieldset { border: 0; padding: 0; margin: 20px 0 16px; }
  .history-filters legend { margin-bottom: 10px; font-size: 12px; color: var(--text-secondary); }
  .kind-filters { display: flex; flex-wrap: wrap; gap: 8px; }
  .kind-filters button { padding: 6px 10px; font-size: 12px; }
  .kind-filters .selected { color: var(--primary); border-color: var(--primary); background: var(--primary-subtle); }
  .filter-note { display: flex; align-items: center; justify-content: space-between; gap: 16px; }
  .filter-note { font-size: 12px; }
  .history-note { margin: 12px 0 24px; font-size: 12px; }
  .history-list { margin-bottom: 20px; }
  .history-day { font-size: 12px; font-weight: 550; color: var(--text-secondary); margin: 24px 0 10px; }
  .history-day:first-child { margin-top: 0; }
  .history-row { display: flex; align-items: flex-start; gap: 12px; padding: 14px 16px; border: 1px solid var(--border-subtle); border-bottom: 0; background: var(--surface); }
  .history-row:has(+.history-day),.history-row:last-child { border-bottom: 1px solid var(--border-subtle); border-radius: 0 0 8px 8px; }
  .history-day+.history-row { border-radius: 8px 8px 0 0; }
  .history-icon { color: var(--primary); background: var(--primary-subtle); margin-top: 2px; }
  .history-body { flex: 1; min-width: 0; }
  .filter-selects label { gap: 10px; min-width: 0; }
  .history-summary { display: grid; grid-template-columns: minmax(0,1.2fr) minmax(0,1fr) auto; align-items: start; gap: 20px; }
  .history-subject { min-width: 0; }
  .history-row h2 { font-size: 13px; font-weight: 550; margin: 0 0 5px; }
  .history-row p { margin: 0; font-size: 13px; overflow-wrap: anywhere; }
  .history-meta { display: flex; flex-direction: column; align-items: flex-end; gap: 6px; }
  .history-row time { font-size: 11px; color: var(--text-muted); white-space: nowrap; font-variant-numeric: tabular-nums; }
  .history-context { font-size: 12px; display: flex; flex-direction: column; gap: 5px; overflow-wrap: anywhere; }
  .history-row small { font-size: 10px; }
  .history-row details { margin-top: 8px; }
  .history-row summary { cursor: pointer; font-size: 12px; color: var(--primary); }
  .history-row summary:focus-visible { outline: 2px solid var(--primary); outline-offset: 4px; }
  .history-change { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; margin-top: 16px; padding-top: 14px; border-top: 1px solid var(--border-subtle); }
  .history-change h3 { font-size: 12px; color: var(--text-secondary); margin: 0 0 12px; }
  .history-change dl { margin: 0; }
  .history-change dl>div { margin-bottom: 12px; }
  .history-change dt { color: var(--text-secondary); font-size: 11px; }
  .history-change dd { margin: 4px 0 0; font-size: 12px; overflow-wrap: anywhere; white-space: pre-wrap; }
  @media(max-width:800px) { .filter-selects { grid-template-columns: 1fr; } }
  @media(max-width:650px) { .history-row { padding: 16px; gap: 10px; }.history-summary { grid-template-columns: minmax(0,1fr) auto; gap: 10px; }.history-context { grid-column: 1; grid-row: 2; }.history-context:empty { display: none; }.history-meta { grid-column: 2; grid-row: 1 / span 2; }.history-meta .pill { white-space: normal; max-width: 130px; text-align: right; }.history-change { grid-template-columns: 1fr; gap: 16px; }.filter-note { align-items: flex-start; flex-direction: column; } }
</style>
