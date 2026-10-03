<script lang="ts">
  import Icon from './Icon.svelte';
  import { bracketRounds, roundTitle, type BracketSlot } from './draw-view';
  import type { Language } from './i18n';
  let { slots, language }: { slots: BracketSlot[]; language: Language } = $props();
  let rounds = $derived(bracketRounds(slots, language));
  let width = $derived(rounds.length * 252 + 200);
  let height = $derived(Math.max(180, slots.length / 2 * 100 + 60));
  let finalCenter = $derived(slots.length / 4 * 100 + 44);
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
        {#each round as match}
          <div class="bracket-match" style:left={`${column * 252}px`} style:top={`${44 + match.center - 38}px`}>
            <span class="match-number">#{match.number}</span>
            {#each [match.left, match.right] as entry}
              <div class="bracket-entry" class:unresolved={entry.kind !== 'entry'} class:bye={entry.kind === 'bye'} title={[entry.label, entry.club].filter(Boolean).join(' · ')}>
                {#if entry.seed}<span class="seed-number">{entry.seed}</span>{/if}
                <span class="entry-name">{entry.label}{#if entry.club}<small>{entry.club}</small>{/if}</span>
              </div>
            {/each}
          </div>
        {/each}
      {/each}
      <div class="champion" style:left={`${rounds.length * 252}px`} style:top={`${finalCenter - 38}px`}><Icon name="trophy" size={22} /><span>{language === 'sr' ? 'Pobednik' : 'Champion'}<small>{language === 'sr' ? 'Čeka se finale' : 'Awaiting the final'}</small></span></div>
    </div>
  </div>
{:else}<div class="bracket-empty"><Icon name="layer-group" size={28} /><p>{language === 'sr' ? 'Pripremi raspored u podešavanjima kategorije.' : 'Prepare an arrangement in category settings.'}</p></div>{/if}
<style>
  .bracket-scroll { overflow: auto; max-height: 72vh; min-width: 0; padding: 20px; border: 1px solid var(--border-subtle); border-radius: 8px; background: var(--background); }
  .bracket-scroll:focus-visible { outline: 2px solid var(--primary); outline-offset: 3px; }
  .bracket-canvas { position: relative; }
  .bracket-lines { position: absolute; inset: 0; pointer-events: none; }
  path { fill: none; stroke: var(--border); stroke-width: 1.5; }
  .round-title { position: absolute; top: 0; width: 220px; margin: 0; font-size: 12px; font-weight: 600; color: var(--text-secondary); }
  .bracket-match { position: absolute; width: 220px; height: 76px; border: 1px solid var(--border); border-radius: 7px; background: var(--surface); }
  .match-number { position: absolute; top: -14px; right: 2px; font-size: 9px; color: var(--text-muted); }
  .bracket-entry { height: 37px; display: flex; align-items: center; gap: 8px; padding: 5px 12px; }
  .bracket-entry + .bracket-entry { border-top: 1px solid var(--border-subtle); }
  .entry-name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 550; }
  small { display: block; font-size: 10px; font-weight: 400; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; }
  .seed-number { flex-shrink: 0; color: var(--primary); font-size: 10px; font-weight: 650; }
  .unresolved { color: var(--text-secondary); }
  .bye { color: var(--text-muted); font-size: 10px; }
  .champion { position: absolute; width: 176px; height: 76px; display: flex; align-items: center; gap: 12px; padding: 16px; border: 1px solid var(--border); border-radius: 7px; background: var(--surface); }
  .champion :global(.icon) { color: var(--primary); }
  .champion span { font-weight: 600; }
  .bracket-empty { display: grid; place-items: center; padding: 64px 24px; color: var(--text-muted); border: 1px dashed var(--border); border-radius: 8px; text-align: center; }
</style>
