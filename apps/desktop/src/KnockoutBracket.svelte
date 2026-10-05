<script lang="ts">
  import PlayerName from './PlayerName.svelte';
  import Icon from './Icon.svelte';
  import { bracketRounds, roundTitle, groupName, type BracketSlot } from './draw-view';
  import type { ScheduledMatch, MatchResult } from './api';
  import type { Language } from './i18n';
  let { slots, language, progress = [], onslot, editingDisabled = false }: { onslot?: (index:number)=>void; editingDisabled?:boolean; slots: BracketSlot[]; language: Language; progress?: ScheduledMatch[] } = $props();
  let rounds = $derived(bracketRounds(slots, language, progress));
  let width = $derived(rounds.length * 252 + 200);
  let height = $derived(Math.max(180, slots.length / 2 * 100 + 60));
  let finalCenter = $derived(slots.length / 4 * 100 + 44);
  let finalMatch = $derived(rounds.at(-1)?.[0]);
  let champion = $derived(finalMatch?.result ? (finalMatch.result.winner === finalMatch.left.id ? finalMatch.left : finalMatch.right) : null);
  let bronze=$derived(progress.find(match=>match.round===rounds.length-1 && match.position===1));
  function labelFor(id:string|null){return slots.find(slot=>slot.id===id)?.label ?? (language==='sr'?'Čeka polufinalistu':'Awaiting semifinalist');}
  function setsWon(result: MatchResult, side: number) { return result.sets.filter(set => Math.max(set.first, set.second) >= result.rules.points_to_win && Math.abs(set.first - set.second) >= result.rules.win_by && (side === 0 ? set.first > set.second : set.second > set.first)).length; }
</script>
{#if rounds.length}
  <!-- svelte-ignore a11y_no_noninteractive_tabindex (Labeled scroll region supports native keyboard scrolling.) -->
  <div class="bracket-scroll" tabindex="0" role="region" aria-label={language === 'sr' ? 'Nokaut kostur — horizontalni skrol' : 'Knockout bracket — horizontal scrolling'}>
    <div class="bracket-canvas" style:width={`${width}px`} style:height={`${height}px`}>
      <svg class="bracket-lines" width={width} height={height} aria-hidden="true">
        {#each rounds as round, column}
          {#if column > 0}{#each round as match, index}
            {#each [rounds[column - 1][index * 2], rounds[column - 1][index * 2 + 1]] as feeder}
              <path d={`M ${(column - 1) * 252 + 220} ${44 + feeder.center} H ${column * 252 - 16} V ${44 + match.center} H ${column * 252}`} />
            {/each}
          {/each}{/if}
        {/each}
        <path d={`M ${(rounds.length - 1) * 252 + 220} ${finalCenter} H ${rounds.length * 252}`} />
      </svg>
      {#each rounds as round, column}
        <h3 class="round-title" style:left={`${column * 252}px`}>{roundTitle(slots.length / 2 ** column, language)}</h3>
        {#each round as match, matchIndex}
          <div class="bracket-match" style:left={`${column * 252}px`} style:top={`${44 + match.center - 34}px`}>
            <span class="match-number">#{match.number}</span>
            {#each [match.left, match.right] as entry, side}
              <div class="bracket-entry" class:winner={!!entry.id && match.result?.winner === entry.id} class:unresolved={entry.kind !== 'entry'} class:bye={entry.kind === 'bye'} title={`${entry.lucky_loser ? 'LL ' : ''}${entry.group !== undefined && entry.place !== undefined ? `${groupName(entry.group)}${entry.place} ` : ''}${entry.label}`}>
                {#if entry.lucky_loser}<span class="qualification-code">LL</span>{/if}
                {#if entry.group !== undefined && entry.place !== undefined}<span class="qualification-code">{groupName(entry.group)}{entry.place}</span>{:else if entry.seed}<span class="seed-number">{entry.seed}</span>{/if}
                <span class="entry-name"><PlayerName label={entry.label} /></span>
                {#if match.result}<b class="bracket-score">{setsWon(match.result, side)}</b>{/if}
                {#if onslot && column === 0}<button class="slot-edit" disabled={editingDisabled} aria-label={`${language==='sr'?'Promeni mesto':'Change slot'} ${matchIndex*2+side+1}: ${entry.label}`} onclick={()=>onslot?.(matchIndex*2+side)}></button>{/if}
              </div>
            {/each}
          </div>
        {/each}
      {/each}
      <div class="champion" style:left={`${rounds.length * 252}px`} style:top={`${finalCenter - 34}px`}><Icon name="trophy" size={22} /><span><small>{language === 'sr' ? 'Pobednik' : 'Champion'}</small><strong><PlayerName label={champion?.label ?? (language === 'sr' ? 'Čeka se finale' : 'Awaiting the final')} /></strong></span></div>
    </div>
  </div>
  {#if bronze}<section class="bronze-match"><h3>{language==='sr'?'Meč za treće mesto':'Third-place match'}</h3>{#each [bronze.first,bronze.second] as id,side}<div class="bronze-entry" class:winner={!!id && bronze.result?.winner===id}><PlayerName label={labelFor(id)}/>{#if bronze.result}<b>{setsWon(bronze.result,side)}</b>{/if}</div>{/each}</section>{/if}
{:else}<div class="bracket-empty"><Icon name="layer-group" size={28} /><p>{language === 'sr' ? 'Pripremi raspored u podešavanjima kategorije.' : 'Prepare an arrangement in category settings.'}</p></div>{/if}
<style>
  .bracket-scroll { overflow: auto; max-height: 72vh; min-width: 0; padding: 16px 0 20px; }
  .bracket-scroll:focus-visible { outline: 2px solid var(--primary); outline-offset: 3px; }
  .bracket-canvas { position: relative; }
  .bracket-lines { position: absolute; inset: 0; pointer-events: none; }
  path { fill: none; stroke: var(--border); stroke-width: 1.5; }
  .round-title { position: absolute; top: 0; width: 220px; margin: 0; font-size: 12px; font-weight: 600; color: var(--text-secondary); }
  .bracket-match { position: absolute; width: 220px; height: 68px; border: 1px solid var(--border); border-radius: 7px; background: var(--surface); }
  .match-number { position: absolute; top: -14px; right: 2px; font-size: 9px; color: var(--text-muted); }
  .bracket-entry { position: relative; height: 33px; display: flex; align-items: center; gap: 8px; padding: 5px 12px; }
  .bracket-entry + .bracket-entry { border-top: 1px solid var(--border-subtle); }
  .entry-name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 550; }
  small { display: block; font-size: 10px; font-weight: 400; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; }
  .winner { color: var(--primary); background: var(--primary-subtle); }
  .bracket-score { margin-left: auto; font-size: 12px; }
  .seed-number { flex-shrink: 0; color: var(--primary); font-size: 10px; font-weight: 650; }
  .qualification-code { flex-shrink: 0; color: var(--text-secondary); font-size: 10px; font-weight: 600; font-variant-numeric: tabular-nums; }
  .unresolved { color: var(--text-secondary); }
  .bye { color: var(--text-muted); font-size: 10px; }
  .champion { position: absolute; width: 176px; height: 68px; display: flex; align-items: center; gap: 12px; padding: 16px; border: 1px solid var(--border); border-radius: 7px; background: var(--surface); }
  .champion :global(.icon) { color: var(--primary); }
  .champion span { min-width:0; }.champion strong{display:block;font-size:14px;font-weight:600;line-height:1.5;overflow-wrap:anywhere;}.champion small{font-size:10px;margin-bottom:5px;}.champion{height:auto;min-height:68px;}
  .bracket-empty { display: grid; place-items: center; padding: 64px 24px; color: var(--text-muted);  text-align: center; }
.slot-edit{position:absolute;inset:0;width:100%;height:100%;padding:0;border:0;border-radius:3px;background:transparent;cursor:pointer;}.slot-edit:hover:not(:disabled){background:var(--primary-subtle);outline:1px solid var(--primary);}.slot-edit:focus-visible{outline:2px solid var(--primary);outline-offset:1px;}.slot-edit:disabled{cursor:default;}
.bronze-match{max-width:320px;margin-top:20px;}.bronze-match h3{font-size:12px;color:var(--text-secondary);margin:0 0 10px;}.bronze-entry{display:flex;justify-content:space-between;gap:12px;padding:10px 12px;font-size:12px;line-height:1.5;border-bottom:1px solid var(--border-subtle);}
</style>
