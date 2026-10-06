import {cashBalance} from './cash-balance';
import {getCategoryReport,cashLedger,listEntries,getSchedule,type Tournament,type Category} from './api';
import {playerLabel} from './player-label';
import {groupName,roundTitle,bracketSlots,bracketRounds,type BracketSlot} from './draw-view';
import type {Language} from './i18n';
export type ReportKind='groups'|'draw'|'results'|'cash'|'schedule';
export interface Report {title:string;html:string;csv:string}
import {escapeHtml,csv} from './report-format';
export {escapeHtml,csvCell,csv} from './report-format';
function table(headers:string[],rows:unknown[][]){return `<table><thead><tr>${headers.map(h=>`<th>${escapeHtml(h)}</th>`).join('')}</tr></thead><tbody>${rows.map(row=>`<tr>${row.map(cell=>`<td>${escapeHtml(cell)}</td>`).join('')}</tr>`).join('')}</tbody></table>`;}
function bracketFigures(slots:BracketSlot[],language:Language,progress:import('./api').ScheduledMatch[]):string{
  const rounds=bracketRounds(slots,language,progress);let pages='';
  // At most 32 entrants per sheet; larger brackets continue in subsequent blocks.
  for(let base=0;base<rounds.length;base+=5){
    const remaining=slots.length/2**base;const columns=Math.min(5,rounds.length-base);
    for(let start=0;start<remaining;start+=32){
      const count=Math.min(32,remaining-start);const width=columns*230;const height=count/2*72+48;
      let body='';
      for(let column=0;column<columns;column++){
        const entriesPerMatch=2**(column+1);const first=start/entriesPerMatch;const total=count/entriesPerMatch;const round=rounds[base+column];
        body+=`<text x="${column*230}" y="16" font-size="12" font-weight="bold">${escapeHtml(roundTitle(remaining/2**column,language))}</text>`;
        for(let local=0;local<total;local++){
          const match=round[first+local];if(!match)continue;const y=40+(local+.5)*72*2**column-24;const x=column*230;
          if(column){for(const side of [-1,1]){const feederCenter=40+(local+.5)*72*2**column+side*18*2**column;body+=`<path d="M ${x-25} ${feederCenter} H ${x-12} V ${y+24} H ${x}" fill="none" stroke="#bbb"/>`;}}
          body+=`<rect x="${x}" y="${y}" width="205" height="48" rx="4" fill="white" stroke="#bbb"/><path d="M ${x} ${y+24} H ${x+205}" stroke="#ddd"/>`;
          for(const [side,entry] of [match.left,match.right].entries()){
            const prefix=entry.group!==undefined&&entry.place!==undefined?`${entry.lucky_loser?'LL ':''}${groupName(entry.group)}${entry.place} `:'';
            body+=`<text x="${x+7}" y="${y+16+side*24}" font-size="10" fill="${match.result?.winner===entry.id?'#117a37':'#222'}"><title>${escapeHtml(prefix+entry.label)}</title>${escapeHtml((prefix+entry.label).slice(0,34))}</text>`;
          }
        }
      }
      pages+=`<section class="bracket-sheet"><h2>${language==='sr'?'Kostur':'Bracket'}${remaining>32?` · ${start+1}–${start+count}`:''}</h2><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${width} ${height}" role="img" aria-label="${escapeHtml(language==='sr'?'Nokaut kostur':'Knockout bracket')}">${body}</svg></section>`;
    }
  }
  return pages;
}
const style=`<style>.report{font-family:Arial,sans-serif;color:#171717;font-size:12px;line-height:1.5}.report h1{font-size:22px;margin:0 0 8px}.report h2{font-size:16px;margin:26px 0 12px}.report p{margin:0 0 14px;color:#555}.report table{width:100%;border-collapse:collapse;margin:12px 0 24px}.report th,.report td{padding:8px;text-align:left;border-bottom:1px solid #ddd;overflow-wrap:anywhere}.report th{background:#f5f5f5;font-weight:600}.report tr{break-inside:avoid}.report .bracket-sheet{break-after:page;margin:16px 0 28px}.report .bracket-sheet svg{width:100%;height:auto;display:block}.report .bracket-sheet h2{margin:0 0 12px}.report thead{display:table-header-group}.report .report-foot{margin-top:24px;font-size:10px}@page{size:A4;margin:14mm}@media print{.report h2{break-after:avoid}}</style>`;
export function reportDocument(report:Report,language:Language){return `<!doctype html><html lang="${language}"><head><meta charset="utf-8"><title>${escapeHtml(report.title)}</title></head><body>${report.html}</body></html>`;}
export async function buildReport(kind:ReportKind,tournament:Tournament,category:Category|undefined,language:Language):Promise<Report>{
  const sr=language==='sr';const title=({groups:sr?'Grupe':'Groups',draw:sr?'Žreb':'Draw',results:sr?'Konačni plasman':'Final standings',cash:sr?'Blagajna':'Cash desk',schedule:sr?'Stolovi':'Tables'})[kind];
  const context=category?`${tournament.name} / ${category.name}`:tournament.name;let sections='';let rows:unknown[][]=[];
  if(kind==='cash'){
    const [ledger,entryGroups]=await Promise.all([cashLedger(tournament.id),Promise.all(tournament.categories.map(c=>listEntries(c.id)))]);
    const entries=new Map(entryGroups.flat().map(e=>[e.id,e]));const categories=new Map(tournament.categories.map(c=>[c.id,c.name]));
    const headers=sr?['Datum','Kategorija','Igrač / par','Vrsta','Iznos (RSD)','Beleška']:['Date','Category','Player / pair','Type','Amount (RSD)','Note'];
    const types=sr?{charge:'Zaduženje',discount:'Popust',payment:'Uplata',refund:'Povraćaj'}:{charge:'Charge',discount:'Discount',payment:'Payment',refund:'Refund'};
    const data=ledger.records.map(r=>{const entry=entries.get(r.entry_id);return [r.created_at,categories.get(entry?.category_id??'')??'',entry?.members.map(playerLabel).join(' / ')??r.entry_id,types[r.kind],(r.amount_minor/100).toFixed(2),r.note];});
    const totals={charge:0,discount:0,payment:0,refund:0};for(const r of ledger.records)totals[r.kind]+=r.amount_minor;
    const balances=entryGroups.flat().flatMap(entry=>entry.members.map(member=>cashBalance(entry,tournament.categories.find(c=>c.id===entry.category_id),ledger,member.id)));
    const totalsHeaders=sr?['Istorijska zaduženja','Popusti','Uplate','Povraćaji','Dugovanje prisutnih','Očekivano od nepotvrđenih']:['Historical charges','Discounts','Payments','Refunds','Outstanding from arrivals','Expected from unconfirmed arrivals'];
    const totalsData=[[totals.charge,totals.discount,totals.payment,totals.refund,balances.reduce((sum,b)=>sum+b.remaining,0),balances.reduce((sum,b)=>sum+b.expected,0)].map(v=>(v/100).toFixed(2))];
    sections=table(totalsHeaders,totalsData)+table(headers,data);rows=[headers,...data,[],totalsHeaders,...totalsData];
  }else if(kind==='schedule'){
    const state=await getSchedule(tournament.id);const headers=sr?['Sto','Kategorija','Prvi učesnik','Drugi učesnik','Status']:['Table','Category','First participant','Second participant','Status'];
    const data=state.assignments.map(a=>[a.table,a.scheduled.category_name,a.scheduled.first_name,a.scheduled.second_name,a.status==='running'?(sr?'U toku':'Running'):(sr?'Dodeljeno':'Assigned')]);sections=table(headers,data);rows=[headers,...data];
  }else{
    if(!category)throw 'not_found';
    const {competition:state,results,rules}=await getCategoryReport(tournament.id,category.id);
    const draw=results.draw;if(!draw)throw 'invalid_draw';
    const names=new Map(draw.participants.map(e=>[e.id,e.members.map(playerLabel).join(' / ')]));
    const label=(id:string|null)=>id?names.get(id)??id:sr?'Čeka učesnika':'Awaiting participant';
    if(state.stale && kind!=='results')throw 'draw_conflict';
    if(kind==='groups'){
      const headers=sr?['Grupa','Plasman','Igrač / par','Mečevi','Pobede','Porazi','Setovi +','Setovi −','Poeni +','Poeni −']:['Group','Place','Player / pair','Played','Wins','Losses','Sets for','Sets against','Points for','Points against'];rows=[headers];
      for(const group of state.groups){const data=group.rows.map((r,i)=>[groupName(group.group),group.resolved?i+1:'—',label(r.entry_id),r.played,r.wins,r.losses,r.sets_for,r.sets_against,r.points_for,r.points_against]);sections+=`<h2>${sr?'Grupa':'Group'} ${groupName(group.group)}</h2><p>${group.completed}/${group.total} ${sr?'mečeva':'matches'}${group.resolved?'':sr?' — plasman nije konačan':' — provisional standings'}</p>`+table(headers.slice(1),data.map(row=>row.slice(1)));rows.push(...data);}
    }else if(kind==='results'){
      if(!results.ready)throw 'competition_incomplete';const headers=sr?['Plasman','Igrač / par','Faza']:['Place','Player / pair','Stage'];const data=results.placements.map(p=>[p.place===p.place_end?`${p.place}.`:p.place===3&&p.stage==='knockout'?'3.':`${p.place}–${p.place_end}.`,label(p.entry_id),({winner:sr?'Pobednik':'Winner',finalist:sr?'Finalista':'Finalist',knockout:sr?'Nokaut':'Knockout',groups:sr?'Grupe':'Groups'})[p.stage]]);sections=`<p>${results.completion.completed_at?(sr?'Potvrđen konačni plasman':'Confirmed final standings'):(sr?'Plasman čeka potvrdu završetka':'Standings await completion confirmation')}</p>`+table(headers,data);rows=[headers,...data];
    }else{
      const size=category.format==='knockout'?draw.sections[0].length:state.slots.length;const finalRound=Math.log2(size)-1;
      const headers=sr?['Faza','Meč','Prvi učesnik','Drugi učesnik','Setovi','Pobednik']:['Round','Match','First participant','Second participant','Set scores','Winner'];
      const data=state.matches.map(m=>[m.round===finalRound&&m.position===1?(sr?'Treće mesto':'Third place'):roundTitle(size/2**m.round,language),m.position+1,m.first?label(m.first):m.bye?'BYE':label(null),m.second?label(m.second):m.bye?'BYE':label(null),m.result?.sets.map(s=>`${s.first}:${s.second}`).join(' / ')??(m.bye?'BYE':''),m.result?label(m.result.winner):m.bye?label(m.first??m.second):'']);sections=bracketFigures(bracketSlots(draw,{...rules,...draw.settings},category.format,language,state.slots),language,state.matches)+table(headers,data);rows=[headers,...data];
    }
  }
  const stamp=new Date().toLocaleString(sr?'sr-Latn-RS':'en-GB');return {title:`${context} — ${title}`,html:style+`<article class="report"><h1>${escapeHtml(title)}</h1><p>${escapeHtml(context)}</p>${sections}<p class="report-foot">LibreTT · ${escapeHtml(stamp)}</p></article>`,csv:csv(rows)};
}
