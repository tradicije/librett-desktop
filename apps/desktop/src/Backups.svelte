<script lang="ts">
  import {onMount} from 'svelte';
  import Icon from './Icon.svelte';
  import {listBackups,createBackup,automaticBackup,exportBackup,openDataFolder,desktopAvailable,type BackupInfo} from './api';
  import {messages,errorKey,type Language,type MessageKey} from './i18n';
  export type RestoreSource={name:string}|{encoded:string};
  let {language,busy=$bindable(false),onrestore}:{language:Language;busy?:boolean;onrestore:(source:RestoreSource)=>Promise<boolean>}=$props();
  let sr=$derived(language==='sr');let text=$derived(messages[language]);let items=$state<BackupInfo[]>([]);let loading=$state(false);let working=$state(false);let error=$state<MessageKey|null>(null);let notice=$state('');let pending=$state<RestoreSource|null>(null);let sourceName=$state('');let dialog:HTMLDialogElement;let fileInput:HTMLInputElement;const uid=$props.id();
  $effect(()=>{busy=loading||working;});
  async function load(){loading=true;error=null;try{items=await listBackups();}catch(cause){error=errorKey(cause);}finally{loading=false;}}
  onMount(()=>{if(desktopAvailable)void load();});
  async function create(){working=true;error=null;notice='';try{const backup=await createBackup();await load();notice=sr?'Rezervna kopija je sačuvana.':'Backup saved.';sourceName=backup.name;}catch(cause){error=errorKey(cause);}finally{working=false;}}
  async function automatic(){working=true;error=null;try{await automaticBackup();await load();notice=sr?'Automatske kopije su ažurne.':'Automatic backups are up to date.';}catch(cause){error=errorKey(cause);}finally{working=false;}}
  async function exportCopy(name:string){working=true;error=null;try{notice=await exportBackup(name);}catch(cause){error=errorKey(cause);}finally{working=false;}}
  function ask(name:string){pending={name};sourceName=name;dialog.showModal();}
  async function selectFile(event:Event){
    const file=(event.target as HTMLInputElement).files?.[0];if(!file)return;
    error=null;working=true;
    try{
      if(file.size>256*1024*1024)throw 'invalid_backup';
      const bytes=new Uint8Array(await file.arrayBuffer());const chunks:string[]=[];
      for(let i=0;i<bytes.length;i+=32768)chunks.push(String.fromCharCode(...bytes.subarray(i,i+32768)));
      pending={encoded:btoa(chunks.join(''))};sourceName=file.name;dialog.showModal();
    }catch(cause){error=errorKey(cause);}finally{working=false;fileInput.value='';}
  }
  async function restore(){if(!pending||working)return;const source=pending;dialog.close();pending=null;working=true;error=null;try{await onrestore(source);}catch(cause){error=errorKey(cause);}finally{working=false;}}
  async function folder(kind:'backups'|'exports'){try{await openDataFolder(kind);}catch(cause){error=errorKey(cause);}}
</script>
<div class="heading"><div><h1>{sr?'Rezervne kopije':'Backups'}</h1><p class="muted">{sr?'Cela baza: turniri, igrači, slike, rezultati i blagajna.':'The entire database: tournaments, players, images, results and cash desk.'}</p></div><button class="primary" disabled={busy||!desktopAvailable} onclick={create}><Icon name="plus"/>{sr?'Napravi kopiju':'Create backup'}</button></div>
{#if error}<p class="error" role="alert">{text[error]}</p>{/if}{#if notice}<p class="notice" role="status">{notice}</p>{/if}
<section class="backup-policy"><h2>{sr?'Automatska zaštita':'Automatic protection'}</h2><p class="muted">{sr?'Dnevne kopije, najviše 14 automatskih snimaka. Ručne kopije i kopije pre vraćanja se ne brišu rotacijom. Pre zamene podataka automatski čuvamo trenutno stanje.':'Daily backups, retaining up to 14 automatic snapshots. Manual and pre-restore copies are not removed by rotation. Current data is backed up before replacement.'}</p><div class="backup-actions"><button class="secondary" disabled={busy||!desktopAvailable} onclick={automatic}>{sr?'Proveri automatsku kopiju':'Check automatic backup'}</button><button class="secondary" disabled={busy||!desktopAvailable} onclick={()=>folder('backups')}>{sr?'Otvori folder kopija':'Open backup folder'}</button><button class="secondary" disabled={busy||!desktopAvailable} onclick={()=>fileInput.click()}>{sr?'Vrati kopiju iz fajla':'Restore from file'}</button><input hidden bind:this={fileInput} type="file" accept=".sqlite,.db" onchange={selectFile}/></div></section>
{#if loading}<p role="status">{text.loading}</p>{:else if !items.length}<p class="muted">{sr?'Još nema rezervnih kopija.':'No backups yet.'}</p>{/if}
<div class="backup-list">{#each items as item(item.name)}<article class="backup-item"><div><strong>{new Date(item.modified*1000).toLocaleString(sr?'sr-Latn-RS':'en-GB')}</strong><span class="muted">{item.name.includes('-auto-')?(sr?'Automatska':'Automatic'):item.name.includes('-before-restore-')?(sr?'Pre vraćanja':'Before restore'):(sr?'Ručna':'Manual')} · {(item.size/1048576).toFixed(1)} MB</span><small>{item.name}</small></div><div class="backup-actions"><button class="secondary" disabled={busy} onclick={()=>exportCopy(item.name)}>{sr?'Izvezi kopiju':'Export copy'}</button><button class="secondary" disabled={busy} onclick={()=>ask(item.name)}>{sr?'Vrati podatke':'Restore data'}</button></div></article>{/each}</div>
<button class="secondary" disabled={busy||!desktopAvailable} onclick={()=>folder('exports')}>{sr?'Otvori folder izvoza':'Open export folder'}</button>
<dialog class="confirm-dialog" bind:this={dialog} aria-labelledby={`${uid}-restore`}><h2 id={`${uid}-restore`}>{sr?'Vrati rezervnu kopiju':'Restore backup'}</h2><p>{sourceName}</p><p>{sr?'Ova kopija će zameniti celu bazu. Pre zamene čuvamo trenutno stanje. Aplikacija će se ponovo učitati i zatvoriti otvorene tabove.':'This copy replaces the entire database. Current data is saved first. The app will reload and close open tabs.'}</p><div class="dialog-actions"><button class="secondary" onclick={()=>{pending=null;dialog.close();}}>{sr?'Odustani':'Cancel'}</button><button class="primary" onclick={restore}>{sr?'Potvrdi vraćanje':'Confirm restore'}</button></div></dialog>
<style>
.backup-policy{padding:20px 0;border-bottom:1px solid var(--border-subtle);margin-bottom:20px;}.backup-policy h2{font-size:14px;margin:0 0 12px;}.backup-policy p{line-height:1.7;max-width:800px;}.backup-actions{display:flex;gap:8px;flex-wrap:wrap;}.backup-list{display:grid;gap:14px;margin:20px 0;}.backup-item{display:flex;justify-content:space-between;align-items:center;gap:20px;padding:18px;background:var(--surface);border:1px solid var(--border-subtle);border-radius:8px;}.backup-item>div:first-child{display:grid;gap:7px;min-width:0;}.backup-item strong{font-size:13px;}.backup-item small{color:var(--text-muted);font-size:10px;overflow-wrap:anywhere;}@media(max-width:800px){.backup-item{align-items:flex-start;flex-direction:column;}}
</style>
