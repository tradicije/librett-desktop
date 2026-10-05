<script lang="ts">
  import {buildReport,reportDocument,type ReportKind} from './reports';
  import {saveReport,openPrintReport,openDataFolder,desktopAvailable,type Tournament,type Category} from './api';
  import {messages,errorKey,type Language,type MessageKey} from './i18n';
  let {kind,tournament,category,language,busy=$bindable(false),disabled=false}:{kind:ReportKind;tournament:Tournament;category?:Category;language:Language;busy?:boolean;disabled?:boolean}=$props();
  let working=$state(false);let error=$state<MessageKey|null>(null);let path=$state('');let sr=$derived(language==='sr');let text=$derived(messages[language]);$effect(()=>{busy=working;});
  async function generate(format:'csv'|'html'|'print'){if(working)return;working=true;error=null;path='';try{const report=await buildReport(kind,tournament,category,language);if(format==='print')await openPrintReport(report.html);else path=await saveReport(kind,format,format==='csv'?report.csv:reportDocument(report,language));}catch(cause){error=errorKey(cause);}finally{working=false;}}
  async function folder(){try{await openDataFolder('exports');}catch(cause){error=errorKey(cause);}}
</script>
<div class="report-actions"><div class="report-buttons"><button class="secondary" disabled={disabled||working||!desktopAvailable} onclick={()=>generate('print')}>{sr?'Štampa / PDF':'Print / PDF'}</button><button class="secondary" disabled={disabled||working||!desktopAvailable} onclick={()=>generate('csv')}>CSV</button><button class="secondary" disabled={disabled||working||!desktopAvailable} onclick={()=>generate('html')}>HTML</button></div>{#if working}<span class="muted" role="status">{text.loading}</span>{/if}{#if error}<p class="error" role="alert">{text[error]}</p>{/if}{#if path}<p class="notice" role="status">{path}<button class="secondary" onclick={folder}>{sr?'Otvori folder':'Open folder'}</button></p>{/if}</div>
<style>
.report-actions{margin:16px 0;min-width:0;}.report-buttons{display:flex;gap:8px;flex-wrap:wrap;}.report-actions p{overflow-wrap:anywhere;line-height:1.7;}.notice button{margin-left:12px;}
</style>
