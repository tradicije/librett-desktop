<script lang="ts">
  import { onMount } from 'svelte';
  import { desktopAvailable, listPlayers, previewRegistry, refineRegistry, confirmRegistryImport, registrySources, registryPreviews, cancelRegistryPreview, downloadRegistryPhoto, type RegistryPreview, type RegistryDecision, type RegistrySource, type Player } from './api';
  import { messages, errorKey, type Language, type MessageKey } from './i18n';
  import { registryMessages } from './registry-messages';
  import ImageCropDialog from './ImageCropDialog.svelte';
  let {language,busy=$bindable(false),onimported}:{language:Language;busy?:boolean;onimported:()=>Promise<void>}=$props();
  let text=$derived(registryMessages[language]);let common=$derived(messages[language]);
  let url=$state('');let sources=$state<RegistrySource[]>([]);let pending=$state<RegistryPreview[]>([]);let preview=$state<RegistryPreview|null>(null);
  let localPlayers=$state<Player[]>([]);let choices=$state<Record<string,RegistryDecision>>({});let page=$state(0);let reviewed=$state(false);let dirty=$state(false);let notice=$state('');let error=$state<MessageKey|null>(null);
  let crop:ImageCropDialog;
  let rows=$derived(preview?.rows.slice(page*25,page*25+25)??[]);
  let ready=$derived(preview!==null && preview.rows.every(row=>row.skip||row.valid) && !dirty && reviewed);
  async function reload(){[sources,pending,localPlayers]=await Promise.all([registrySources(),registryPreviews(),listPlayers()]);}
  onMount(()=>{if(desktopAvailable)void reload().catch(cause=>error=errorKey(cause));});
  function show(value:RegistryPreview){preview=value;page=0;reviewed=false;dirty=false;choices=Object.fromEntries(value.rows.map(row=>[row.remote_id,{remote_id:row.remote_id,local_id:row.current?row.local_id:null,birth_year:null,name:null,club:null,use_registry:[],skip:row.skip,photo:row.photo}]));}
  function choice(id:string,patch:Partial<RegistryDecision>){choices={...choices,[id]:{...choices[id],...patch}};dirty=true;reviewed=false;}
  function accept(id:string,field:string,checked:boolean){const value=choices[id];const patch:Partial<RegistryDecision>={use_registry:checked?[...value.use_registry.filter(key=>key!==field),field]:value.use_registry.filter(key=>key!==field)};if(checked&&field==='name')patch.name=null;if(checked&&field==='club')patch.club=null;if(checked&&field==='birth_year')patch.birth_year=null;choice(id,patch);}
  async function download(source=url){if(busy)return;busy=true;error=null;notice='';try{url=source;show(await previewRegistry(null,source,[]));await reload();}catch(cause){error=errorKey(cause);}finally{busy=false;}}
  async function file(event:Event){const input=event.currentTarget as HTMLInputElement;const file=input.files?.[0];if(!file||busy)return;busy=true;error=null;notice='';try{if(file.size>32*1024*1024)throw 'invalid_registry';show(await previewRegistry(await file.text(),null,[]));await reload();}catch(cause){error=errorKey(cause);}finally{busy=false;input.value='';}}
  async function refine(){if(!preview||busy)return;busy=true;error=null;try{const next=await refineRegistry(preview.id,Object.values(choices));show(next);await reload();}catch(cause){error=errorKey(cause);}finally{busy=false;}}
  async function confirm(){if(!preview||busy||!ready)return;busy=true;error=null;try{await confirmRegistryImport(preview.id);preview=null;notice=text.done;await reload();await onimported();}catch(cause){error=errorKey(cause);}finally{busy=false;}}
  async function cancel(id:string){if(busy)return;busy=true;error=null;try{await cancelRegistryPreview(id);if(preview?.id===id)preview=null;await reload();}catch(cause){error=errorKey(cause);}finally{busy=false;}}
  async function photo(id:string){if(!preview||busy)return;busy=true;error=null;try{const [encoded,mime]=await downloadRegistryPhoto(preview.id,id);const decoded=atob(encoded);const bytes=new Uint8Array(decoded.length);for(let i=0;i<decoded.length;i++)bytes[i]=decoded.charCodeAt(i);const image=await crop.crop(new File([bytes],'registry-photo',{type:mime}));if(image){choice(id,{photo:image});notice=text.photoReady;}}catch(cause){error=errorKey(cause);}finally{busy=false;}}
</script>
<section class="panel registry-import">
  <h2>{text.title}</h2><p class="muted">{text.intro}</p><p class="muted">{text.unsigned}</p>
  <label>{text.source}<input type="url" placeholder="https://…/wp-json/librett-registry/v1/snapshot" bind:value={url} maxlength="2048" disabled={busy||!desktopAvailable} /></label>
  <div class="buttons"><button disabled={busy||!desktopAvailable||!url} onclick={()=>download()}>{text.fetch}</button><label>{text.file}<input type="file" accept=".json,application/json" onchange={file} disabled={busy||!desktopAvailable} /></label></div>
  {#if error}<p class="error" role="alert">{common[error]}</p>{/if}{#if notice}<p role="status">{notice}</p>{/if}
  <details><summary>{text.sources} ({sources.length})</summary>{#each sources as source(source.registry_id)}<p>{source.name} · {text.checkpoint} {source.checkpoint} <button disabled={busy||!source.source_url} onclick={()=>download(source.source_url!)}>{text.refresh}</button><button disabled={busy} onclick={()=>url=source.source_url??''}>{text.source}</button></p>{/each}{#if !sources.length}<p>{text.noSource}</p>{/if}</details>
  {#if pending.length}<details open><summary>{text.pending}</summary>{#each pending as item(item.id)}<p>{item.registry_name} · {item.checkpoint} <button disabled={busy} onclick={()=>show(item)}>{text.resume}</button><button disabled={busy} onclick={()=>cancel(item.id)}>{text.cancel}</button></p>{/each}</details>{/if}
  {#if preview}
    <h3>{text.preview}: {preview.registry_name}</h3><p>{preview.registry_id} · {text.checkpoint} {preview.checkpoint} · {preview.rows.length}</p><p class="muted">{text.omitted}</p><p>{text.withdrawals} ({preview.withdrawals})</p>
    <details><summary>{text.policy}</summary><pre>{JSON.stringify(preview.policy,null,2)}</pre></details>
    {#each rows as row(row.remote_id)}
      <article class="registry-row"><h4>{row.source_name}</h4><details><summary>{text.fields}</summary><pre>{JSON.stringify(row.remote,null,2)}</pre></details>
        <label>{text.mapping}<select disabled={busy} value={choices[row.remote_id].local_id??''} onchange={event=>choice(row.remote_id,{local_id:event.currentTarget.value||null})}><option value="">{text.defaultMapping}</option>{#each localPlayers as player(player.id)}<option value={player.id}>{player.name} · {player.birth_year??'—'} · {player.club} · {player.id.slice(0,8)}</option>{/each}</select></label>
        {#if row.current}<p>{text.local}: {row.current.name} · {row.current.birth_year??'—'} · {row.current.club}</p>{/if}
        <label>{text.name}<input maxlength="120" disabled={busy} value={choices[row.remote_id].name??row.proposed.name} oninput={event=>choice(row.remote_id,{name:event.currentTarget.value})} /></label>
        <label>{text.club}<input maxlength="120" disabled={busy} value={choices[row.remote_id].club??row.proposed.club} oninput={event=>choice(row.remote_id,{club:event.currentTarget.value})} /></label>
        <label>{text.year}<input type="number" min="1900" max={new Date().getFullYear()} disabled={busy} value={choices[row.remote_id].birth_year??row.proposed.birth_year??''} oninput={event=>choice(row.remote_id,{birth_year:event.currentTarget.value?Number(event.currentTarget.value):null})} /></label>
        <p>{text.country}: {row.proposed.country||'—'}</p>
        {#if row.conflicts.length}<p>{text.changes}: {row.conflicts.map(field=>field==='birth_year'?text.year:field==='country'?text.country:field==='name'?text.name:field==='club'?text.club:text.photo).join(', ')}</p>{/if}
        {#each ['name','club','birth_year','country'] as field}<label class="check"><input type="checkbox" disabled={busy} checked={choices[row.remote_id].use_registry.includes(field)} onchange={event=>accept(row.remote_id,field,event.currentTarget.checked)} />{text.useRegistry}: {field==='birth_year'?text.year:field==='country'?text.country:field==='name'?text.name:text.club}</label>{/each}
        {#if row.remote.photo_id}<button disabled={busy} onclick={()=>photo(row.remote_id)}>{text.photo}</button>{/if}{#if choices[row.remote_id].photo}<img class="prepared-photo" alt="" src={choices[row.remote_id].photo??''} />{/if}
        <label class="check"><input type="checkbox" disabled={busy} checked={choices[row.remote_id].skip} onchange={event=>choice(row.remote_id,{skip:event.currentTarget.checked})} />{text.skip}</label>
        {#if !row.valid&&!choices[row.remote_id].skip}<p class="error">{text.invalid}</p>{/if}
      </article>
    {/each}
    <div class="buttons"><button disabled={busy||page===0} onclick={()=>page--}>{text.previous}</button><span>{page+1} / {Math.max(1,Math.ceil(preview.rows.length/25))}</span><button disabled={busy||(page+1)*25>=preview.rows.length} onclick={()=>page++}>{text.next}</button><button disabled={busy||!dirty} onclick={refine}>{text.refine}</button></div>
    <label class="check"><input type="checkbox" disabled={busy||dirty} bind:checked={reviewed} />{text.reviewed}</label>
    <div class="buttons"><button class="primary" disabled={busy||!ready} onclick={confirm}>{text.confirm}</button><button disabled={busy} onclick={()=>cancel(preview!.id)}>{text.cancel}</button></div>
  {/if}
</section>
<ImageCropDialog {language} bind:this={crop} />
<style>
  .registry-import{margin-bottom:24px}.registry-import label{display:block;margin:10px 0}.registry-import input:not([type=checkbox]),select{display:block;width:100%;margin-top:6px}.buttons{display:flex;align-items:center;gap:12px;flex-wrap:wrap}.registry-row{border-top:1px solid var(--border);padding:18px 0}.check{display:flex!important;gap:8px;align-items:center}pre{white-space:pre-wrap;overflow-wrap:anywhere;max-height:240px;overflow:auto}.prepared-photo{width:80px;height:80px;object-fit:cover}
</style>
