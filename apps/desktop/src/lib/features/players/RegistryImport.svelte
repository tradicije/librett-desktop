<script lang="ts">
  import { onMount } from 'svelte';
  import { desktopAvailable } from '$lib/api/core';
  import { listPlayers, type Player } from '$lib/api/players';
  import { previewRegistry, refineRegistry, confirmRegistryImport, registrySources, registryPreviews, cancelRegistryPreview, downloadRegistryPhoto, type RegistryPreview, type RegistryDecision, type RegistrySource } from '$lib/api/registry';
  import { messages, errorKey, type Language, type MessageKey } from '$lib/i18n';
  import { registryMessages } from '$lib/features/players/registry-messages';
  import Icon from '$lib/components/Icon.svelte';
  import { IconChevronDown } from '@tabler/icons-svelte';
  import Select from '$lib/components/Select.svelte';
  import ImageCropDialog from '$lib/dialogs/ImageCropDialog.svelte';
  let {language,busy=$bindable(false),onimported}:{language:Language;busy?:boolean;onimported:()=>Promise<void>}=$props();
  let text=$derived(registryMessages[language]);let common=$derived(messages[language]);
  let url=$state('');let sources=$state<RegistrySource[]>([]);let pending=$state<RegistryPreview[]>([]);let preview=$state<RegistryPreview|null>(null);
  let localPlayers=$state<Player[]>([]);let choices=$state<Record<string,RegistryDecision>>({});let page=$state(0);let reviewed=$state(false);let dirty=$state(false);let notice=$state('');let error=$state<MessageKey|null>(null);
  let crop:ImageCropDialog;
  let fileInput:HTMLInputElement;
  let expanded=$state(false);
  const uid=$props.id();
  let mappingOptions=$derived([{value:'',label:text.defaultMapping},...localPlayers.map(player=>({value:player.id,label:`${player.name} · ${player.birth_year??'—'} · ${player.club} · ${player.id.slice(0,8)}`}))]);
  function fieldLabel(field:string){return field==='birth_year'?text.year:field==='country'?text.country:field==='name'?text.name:field==='club'?text.club:text.photo;}
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
<section class="panel registry-import" aria-labelledby={`${uid}-title`} aria-busy={busy}>
  <h2 class="registry-heading" id={`${uid}-title`}>
    <button class="registry-toggle" aria-expanded={expanded} aria-controls={`${uid}-content`} onclick={()=>expanded=!expanded}>
      <span class="registry-title"><span class="row-icon"><Icon name="globe" size={19} /></span><span>{text.title}</span></span>
      <span class="registry-chevron" class:expanded><IconChevronDown size={18} stroke={1.75} aria-hidden="true" /></span>
    </button>
  </h2>
  <div id={`${uid}-content`} hidden={!expanded}>
  <p class="registry-intro muted">{text.intro}</p>
  <div class="source-form">
    <label class="field-label" for={`${uid}-url`}>{text.source}</label>
    <div class="source-controls">
      <input id={`${uid}-url`} type="url" placeholder="https://…/wp-json/librett-registry/v1/snapshot" bind:value={url} maxlength="2048" disabled={busy||!desktopAvailable} />
      <button class="primary" disabled={busy||!desktopAvailable||!url.trim()} onclick={()=>download()}><Icon name="arrow-down" size={17} />{text.fetch}</button>
      <button class="secondary" disabled={busy||!desktopAvailable} onclick={()=>fileInput.click()}><Icon name="plus" size={17} />{text.file}</button>
      <input hidden bind:this={fileInput} type="file" accept=".json,application/json" onchange={file} disabled={busy||!desktopAvailable} />
    </div>
  </div>
  <p class="source-note"><Icon name="info" size={16} /><span>{text.unsigned}</span></p>
  {#if error}<p class="error" role="alert">{common[error]}</p>{/if}
  {#if notice}<p class="notice import-notice" role="status"><Icon name="check-circle" size={17} />{notice}</p>{/if}
  <details class="registry-details">
    <summary>{text.sources}<span class="pill">{sources.length}</span></summary>
    {#each sources as source(source.registry_id)}
      <div class="saved-row"><div class="saved-info"><strong>{source.name}</strong><span class="muted">{text.checkpoint} {source.checkpoint}</span>{#if source.source_url}<small>{source.source_url}</small>{/if}</div><div class="buttons"><button class="secondary" disabled={busy||!source.source_url} onclick={()=>download(source.source_url!)}><Icon name="refresh" size={16} />{text.refresh}</button><button class="secondary" disabled={busy||!source.source_url} onclick={()=>url=source.source_url??''}>{text.source}</button></div></div>
    {:else}<p class="empty-source muted">{text.noSource}</p>{/each}
  </details>
  {#if pending.length}
    <details class="registry-details" open><summary>{text.pending}<span class="pill">{pending.length}</span></summary>
      {#each pending as item(item.id)}<div class="saved-row"><div class="saved-info"><strong>{item.registry_name}</strong><span class="muted">{text.checkpoint} {item.checkpoint}</span></div><div class="buttons"><button class="secondary" disabled={busy} onclick={()=>show(item)}><Icon name="list" size={16} />{text.resume}</button><button class="secondary" disabled={busy} onclick={()=>cancel(item.id)}>{text.cancel}</button></div></div>{/each}
    </details>
  {/if}
  {#if preview}
    <div class="preview-section">
      <div class="section-heading"><div><p class="eyebrow">{text.preview}</p><h3>{preview.registry_name}</h3></div><span class="pill">{preview.rows.length}</span></div>
      <div class="preview-meta"><span>{text.checkpoint} <strong>{preview.checkpoint}</strong></span><span class="registry-id">{preview.registry_id}</span></div>
      <p class="muted preview-help">{text.omitted}</p>
      {#if preview.withdrawals}<p class="source-note"><Icon name="info" size={16} /><span>{text.withdrawals} ({preview.withdrawals})</span></p>{/if}
      <details class="registry-details technical-details"><summary>{text.policy}</summary><pre>{JSON.stringify(preview.policy,null,2)}</pre></details>
      <div class="registry-rows">
        {#each rows as row(row.remote_id)}
          <article class="registry-row" class:skipped={choices[row.remote_id].skip}>
            <div class="profile-heading"><div class="profile-identity"><span class="player-avatar" aria-hidden="true">{row.source_name.split(/\s+/).slice(0,2).map(part=>part[0]).join('').toLocaleUpperCase()}</span><div><h4>{row.source_name}</h4><span class="muted">{row.proposed.country||'—'}</span></div></div><label class="check skip-control"><input type="checkbox" disabled={busy} checked={choices[row.remote_id].skip} onchange={event=>choice(row.remote_id,{skip:event.currentTarget.checked})} />{text.skip}</label></div>
            <div class="mapping-field"><span class="field-label">{text.mapping}</span><Select label={`${text.mapping}: ${row.source_name}`} options={mappingOptions} disabled={busy} bind:value={()=>choices[row.remote_id].local_id??'',value=>choice(row.remote_id,{local_id:value||null})} /></div>
            {#if row.current}<p class="local-profile"><span>{text.local}</span><strong>{row.current.name} · {row.current.birth_year??'—'} · {row.current.club||'—'}</strong></p>{/if}
            <div class="profile-fields">
              <label class="field-label">{text.name}<input maxlength="120" disabled={busy} value={choices[row.remote_id].name??row.proposed.name} oninput={event=>choice(row.remote_id,{name:event.currentTarget.value})} /></label>
              <label class="field-label">{text.club}<input maxlength="120" disabled={busy} value={choices[row.remote_id].club??row.proposed.club} oninput={event=>choice(row.remote_id,{club:event.currentTarget.value})} /></label>
              <label class="field-label">{text.year}<input type="number" min="1900" max={new Date().getFullYear()} disabled={busy} value={choices[row.remote_id].birth_year??row.proposed.birth_year??''} oninput={event=>choice(row.remote_id,{birth_year:event.currentTarget.value?Number(event.currentTarget.value):null})} /></label>
            </div>
            {#if row.conflicts.length}<p class="conflict-note"><Icon name="exclamation-circle" size={16} /><span>{text.changes}: {row.conflicts.map(fieldLabel).join(', ')}</span></p>{/if}
            <fieldset class="registry-choices"><legend>{text.useRegistry}</legend><div class="choice-options">{#each ['name','club','birth_year','country'] as field}<label class="check"><input type="checkbox" disabled={busy} checked={choices[row.remote_id].use_registry.includes(field)} onchange={event=>accept(row.remote_id,field,event.currentTarget.checked)} />{fieldLabel(field)}</label>{/each}</div></fieldset>
            <div class="profile-footer"><details class="registry-details technical-details"><summary>{text.fields}</summary><pre>{JSON.stringify(row.remote,null,2)}</pre></details>
              {#if row.remote.photo_id||choices[row.remote_id].photo}<div class="photo-actions">{#if choices[row.remote_id].photo}<img class="prepared-photo" alt={text.photoReady} src={choices[row.remote_id].photo??''} />{/if}{#if row.remote.photo_id}<button class="secondary" disabled={busy} onclick={()=>photo(row.remote_id)}><Icon name="user" size={16} />{text.photo}</button>{/if}</div>{/if}
            </div>
            {#if !row.valid&&!choices[row.remote_id].skip}<p class="error profile-error">{text.invalid}</p>{/if}
          </article>
        {/each}
      </div>
      <div class="preview-toolbar"><div class="pagination"><button class="secondary" disabled={busy||page===0} onclick={()=>page--} aria-label={text.previous}><Icon name="arrow-left" size={16} /></button><span>{page+1} / {Math.max(1,Math.ceil(preview.rows.length/25))}</span><button class="secondary" disabled={busy||(page+1)*25>=preview.rows.length} onclick={()=>page++} aria-label={text.next}><Icon name="arrow-right" size={16} /></button></div><button class="secondary" disabled={busy||!dirty} onclick={refine}><Icon name="refresh" size={16} />{text.refine}</button></div>
      <div class="import-confirmation"><label class="check review-check"><input type="checkbox" disabled={busy||dirty} bind:checked={reviewed} />{text.reviewed}</label><div class="buttons"><button class="primary" disabled={busy||!ready} onclick={confirm}><Icon name="check-circle" size={17} />{text.confirm}</button><button class="secondary" disabled={busy} onclick={()=>cancel(preview!.id)}>{text.cancel}</button></div></div>
    </div>
  {/if}
  </div>
</section>
<ImageCropDialog {language} bind:this={crop} />
<style>
  .registry-import { margin-bottom:24px; }
  .registry-heading { margin:0; }
  .registry-toggle { width:100%; justify-content:space-between; padding:0; border:0; background:transparent; text-align:left; font-size:inherit; font-weight:inherit; }
  .registry-toggle:hover:enabled { background:transparent; color:var(--primary); }
  .registry-title { display:flex; align-items:center; gap:12px; min-width:0; }
  .registry-title .row-icon { flex-shrink:0; }
  .registry-chevron { display:flex; flex-shrink:0; color:var(--text-muted); }
  .registry-chevron.expanded { transform:rotate(180deg); }
  .registry-intro { margin:16px 0 20px; max-width:760px; font-size:11px; line-height:1.7; }
  .source-form { display:grid; gap:7px; }
  .source-controls { display:flex; align-items:center; gap:8px; flex-wrap:wrap; }
  .source-controls input[type=url] { flex:1 1 280px; }
  .source-controls button { flex-shrink:0; min-height:36px; }
  .source-note { display:flex; align-items:flex-start; gap:8px; margin:12px 0 18px; font-size:11px; color:var(--text-muted); line-height:1.65; }
  .source-note :global(.icon), .conflict-note :global(.icon) { flex-shrink:0; margin-top:1px; }
  .buttons { display:flex; align-items:center; gap:8px; flex-wrap:wrap; }
  .import-notice { display:flex; align-items:center; gap:8px; }
  .registry-details { min-width:0; border-top:1px solid var(--border-subtle); }
  summary { padding:13px 0; cursor:pointer; font-size:11px; font-weight:550; color:var(--text-secondary); overflow-wrap:anywhere; }
  summary:hover { color:var(--primary); }
  summary:focus-visible { outline:2px solid var(--primary); outline-offset:3px; border-radius:var(--radius); }
  summary .pill { margin-left:8px; vertical-align:middle; }
  .saved-row { display:flex; align-items:center; justify-content:space-between; gap:16px; padding:12px 0; border-top:1px solid var(--border-subtle); }
  .saved-info { display:grid; gap:5px; min-width:0; font-size:11px; overflow-wrap:anywhere; }
  .saved-info strong { font-size:12px; font-weight:550; }
  .saved-info small { color:var(--text-muted); }
  .empty-source { margin:0; padding:0 0 14px; font-size:11px; }
  .preview-section { border-top:1px solid var(--border-subtle); margin-top:8px; padding-top:24px; }
  .preview-section .section-heading { margin-bottom:12px; }
  .eyebrow { font-size:10px; color:var(--text-muted); margin:0 0 7px; }
  h3 { margin:0; overflow-wrap:anywhere; }
  .preview-meta { display:flex; flex-wrap:wrap; gap:8px 20px; color:var(--text-secondary); font-size:10px; overflow-wrap:anywhere; }
  .preview-meta strong { font-weight:550; }
  .registry-id { color:var(--text-muted); }
  .preview-help { font-size:11px; line-height:1.7; margin:12px 0 16px; }
  pre { white-space:pre-wrap; overflow-wrap:anywhere; max-height:240px; overflow:auto; padding:12px; border:1px solid var(--border-subtle); border-radius:var(--radius); background:var(--background); color:var(--text-secondary); font-size:10px; line-height:1.7; margin:0 0 14px; }
  .registry-rows { display:grid; gap:16px; margin-top:8px; }
  .registry-row { min-width:0; border:1px solid var(--border-subtle); border-radius:8px; padding:18px; }
  .registry-row.skipped { border-style:dashed; }
  .profile-heading { display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px; margin-bottom:18px; }
  .profile-identity { display:flex; gap:12px; align-items:center; min-width:0; }
  .profile-identity .player-avatar { flex-shrink:0; width:36px; height:36px; font-size:12px; }
  h4 { font-size:12px; font-weight:550; margin:0 0 5px; overflow-wrap:anywhere; }
  .profile-identity .muted { font-size:10px; }
  .mapping-field { display:grid; gap:7px; }
  .local-profile { display:grid; gap:5px; font-size:11px; line-height:1.6; padding:10px 12px; background:var(--background); border-radius:var(--radius); overflow-wrap:anywhere; }
  .local-profile span { color:var(--text-muted); font-size:10px; }
  .local-profile strong { font-weight:500; }
  .profile-fields { display:grid; grid-template-columns:minmax(0,1fr) minmax(0,1fr) minmax(110px,.45fr); gap:16px; margin-top:16px; }
  .check { display:flex; gap:8px; align-items:flex-start; font-size:11px; color:var(--text-secondary); line-height:1.6; cursor:pointer; }
  .check input { width:14px; height:14px; min-height:14px; flex:0 0 14px; padding:0; margin:2px 0 0; accent-color:var(--primary); }
  .skip-control { font-size:10px; }
  .registry-choices { border:0; border-top:1px solid var(--border-subtle); margin:18px 0 0; padding:12px 0 0; min-width:0; }
  .registry-choices legend { padding:0 8px 0 0; font-size:10px; color:var(--text-muted); max-width:100%; }
  .choice-options { display:flex; flex-wrap:wrap; gap:12px 24px; }
  .conflict-note { display:flex; gap:8px; font-size:11px; line-height:1.6; color:var(--primary); margin:14px 0 0; }
  .profile-footer { margin-top:16px; }
  .photo-actions { display:flex; align-items:center; gap:12px; flex-wrap:wrap; margin-top:4px; }
  .prepared-photo { width:48px; height:48px; object-fit:cover; border-radius:var(--radius); border:1px solid var(--border-subtle); }
  .profile-error { margin:14px 0 0; }
  .preview-toolbar { display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px; padding:18px 0; }
  .pagination { display:flex; align-items:center; gap:12px; }
  .pagination span { font-size:11px; color:var(--text-secondary); font-variant-numeric:tabular-nums; }
  .pagination button { min-width:32px; padding:6px; }
  .import-confirmation { display:grid; gap:16px; border-top:1px solid var(--border-subtle); padding-top:18px; }
  @media(max-width:800px) { .profile-fields { grid-template-columns:1fr; gap:12px; } .saved-row { align-items:flex-start; flex-direction:column; gap:12px; } }
  @media(max-width:540px) { .source-controls input[type=url] { flex-basis:100%; } .registry-row { padding:14px; } }
</style>
