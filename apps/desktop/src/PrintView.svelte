<script lang="ts">
  import {onMount} from 'svelte';
  import {getPrintReport,printReport} from './api';
  import {savedLanguage,messages,errorKey} from './i18n';
  const language=savedLanguage();const sr=language==='sr';let content=$state('');let error=$state('');let loading=$state(true);
  onMount(()=>{const token=new URLSearchParams(window.location.search).get('print')??'';getPrintReport(token).then(value=>content=value).catch(cause=>error=messages[language][errorKey(cause)]).finally(()=>loading=false);});
  async function print(){try{await printReport();}catch(cause){error=messages[language][errorKey(cause)];}}
</script>
<div class="print-preview"><header><strong>LibreTT</strong><span>{sr?'Štampa / PDF':'Print / PDF'}</span><button disabled={loading||!content} onclick={print}>{sr?'Štampaj / Sačuvaj PDF':'Print / Save PDF'}</button></header>{#if error}<p role="alert">{error}</p>{/if}{#if loading}<p>{messages[language].loading}</p>{/if}<main>{@html content}</main></div>
<style>
.print-preview{height:100dvh;overflow:auto;background:white;color:#171717;}header{position:sticky;top:0;display:flex;align-items:center;gap:18px;padding:16px 24px;background:#f4f4f4;border-bottom:1px solid #ddd;z-index:1;}header span{font-size:12px;}header button{margin-left:auto;background:#fff;color:#171717;border:1px solid #ccc;}main{padding:24px;min-height:0;}@media print{.print-preview{height:auto;overflow:visible;}header{display:none;}main{padding:0;} :global(html),:global(body),:global(#app){height:auto!important;overflow:visible!important;background:white!important;}}
</style>
