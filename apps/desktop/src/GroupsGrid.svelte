<script lang="ts">
  import GroupRankingDialog from './GroupRankingDialog.svelte';
  import PlayerName from './PlayerName.svelte';
  import { playerLabel } from './player-label';
  import Icon from './Icon.svelte';
  import RoundRobinSchedule from './RoundRobinSchedule.svelte';
  import { groupName } from './draw-view';
  import type { CategoryDraw, Category, Tournament, CompetitionState } from './api';
  import type { Language } from './i18n';
  let { draw, category, tournament, competition, language, busy=$bindable(false), dirty=$bindable(false), onreload }: { draw: CategoryDraw | null; category: Category; tournament:Tournament; competition:CompetitionState|null; language: Language; busy?:boolean;dirty?:boolean;onreload:()=>Promise<void> } = $props();
  let rankingDialog:GroupRankingDialog;
  let openGroups = $state<Record<number, boolean>>({});
  let names = $derived(new Map(draw?.participants.map(entry => [entry.id, entry.members.map(playerLabel).join(' / ')]) ?? []));
  let clubs = $derived(new Map(draw?.participants.map(entry => [entry.id, [...new Set(entry.members.map(member => member.club).filter(Boolean))].join(' / ')]) ?? []));
  function entryName(id: string | null) { return names.get(id ?? '') ?? (language === 'sr' ? 'Neraspoređeno' : 'Unassigned'); }
  function clubName(id: string | null) { return clubs.get(id ?? '') ?? ''; }
</script>
<GroupRankingDialog {language} {category} {tournament} {names} bind:busy bind:dirty {onreload} bind:this={rankingDialog} />
{#if draw}
  <p class="ranking-policy muted">{language==='sr' ? 'Predaja i nedolazak: za tabelu se preostali setovi i ciljni poeni pripisuju pobedniku. Ručni plasman je dostupan po završetku grupe.' : 'Retirement and walkover: remaining sets and target points are awarded to the winner for standings. Manual ranking is available once the group finishes.'}</p>
  <div class="groups-grid">
    {#each draw.sections as group, index}
      {@const standing=competition?.groups[index]}
              <section class="group-card">
                <div class="group-heading"><h3>{language === 'sr' ? 'Grupa' : 'Group'} {groupName(index)}</h3><span>{group.filter(Boolean).length} / {group.length}</span></div>
                {#if standing}
                  <div class="standings-scroll"><table><thead><tr><th>#</th><th>{language==='sr'?'Igrač':'Player'}</th><th title={language==='sr'?'Odigrano':'Played'}>M</th><th title={language==='sr'?'Pobede':'Wins'}>W</th><th title={language==='sr'?'Porazi':'Losses'}>L</th><th>{language==='sr'?'Setovi':'Sets'}</th><th>{language==='sr'?'Poeni':'Points'}</th></tr></thead><tbody>
                  {#each standing.rows as row,place}<tr class:qualifying={standing.complete && standing.resolved && place<draw.settings.qualifiers_per_group}><td>{place+1}{#if row.tied && !standing.manual}<span title={language==='sr'?'Izjednačeni':'Tied'}>≈</span>{/if}</td><th scope="row"><PlayerName label={entryName(row.entry_id)} /></th><td>{row.played}</td><td>{row.wins}</td><td>{row.losses}</td><td>{row.sets_for}:{row.sets_against}</td><td>{row.points_for}:{row.points_against}</td></tr>{/each}
                  </tbody></table></div>
                  <div class="standings-status"><span><span class="metadata-line">{standing.completed}/{standing.total} {language==='sr'?'mečeva':'matches'}</span><span class="metadata-line muted">{standing.manual ? language==='sr'?'Ručno':'Manual' : language==='sr'?'Automatski':'Automatic'}</span></span>{#if standing.complete}<button class="secondary" disabled={category.completed || tournament.completed || busy || competition?.stale} onclick={()=>{if(competition)rankingDialog.open(competition,index);}}>{language==='sr'?'Plasman':'Ranking'}</button>{/if}</div>
                  {#if standing.complete && !standing.resolved}<p class="tie-note">{language==='sr'?'Potpuno izjednačenje — potvrdi ručni plasman da bi se odredili prolaznici.':'Exact tie — confirm manual ranking to resolve qualifiers.'}</p>{/if}
                {:else}
                  <ol class="group-entries">{#each group as id}<li><span class="entry-position"><Icon name={category.discipline==='doubles'?'users':'user'} size={15}/></span><span class="group-player"><PlayerName label={entryName(id)} />{#if clubName(id)}<small>{clubName(id)}</small>{/if}</span></li>{/each}</ol>
                {/if}
                <div class="group-qualifiers"><Icon name="arrow-right" size={14} />{draw.settings.qualifiers_per_group} {language === 'sr' ? 'prolaze u nokaut' : 'advance to knockout'}</div>
                {#if group.every(id => id !== null)}
                  <details ontoggle={event => openGroups[index] = event.currentTarget.open}><summary>{language === 'sr' ? 'Ko sa kim igra' : 'Round-robin pairings'}</summary>{#if openGroups[index]}<RoundRobinSchedule ids={group.filter((id): id is string => id !== null)} {names} {language} />{/if}</details>
                {/if}
              </section>
    {/each}
  </div>
{:else}
  <div class="group-empty"><Icon name="users" size={28} /><p>{language === 'sr' ? 'Grupe će se prikazati posle pripreme rasporeda.' : 'Groups will appear after the layout is prepared.'}</p></div>
{/if}
<style>
  .ranking-policy{font-size:11px;margin:0 0 16px;}
  .standings-scroll{overflow-x:auto;}table{width:100%;border-collapse:collapse;font-size:11px;min-width:350px;}th,td{padding:10px 8px;border-bottom:1px solid var(--border-subtle);text-align:left;white-space:nowrap;}thead th{font-size:10px;font-weight:500;color:var(--text-muted);}tbody th{white-space:normal;min-width:120px;font-weight:550;}td{font-variant-numeric:tabular-nums;}tr.qualifying{background:var(--primary-subtle);} .standings-status{padding:10px 14px;display:flex;align-items:center;justify-content:space-between;gap:8px;font-size:10px;color:var(--text-muted);} .standings-status button{font-size:10px;min-height:28px;padding:4px 8px;} .tie-note{padding:0 14px;color:var(--text-secondary);font-size:11px;}
  .groups-grid { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 20px; align-items: start; }
  .group-card { border: 1px solid var(--border); border-radius: 8px; background: var(--surface); overflow: hidden; }
  .group-heading { display: flex; justify-content: space-between; align-items: center; padding: 12px 14px; border-bottom: 1px solid var(--border-subtle); }
  .group-heading h3 { margin: 0; font-size: 12px; }
  .group-heading > span { color: var(--text-muted); font-size: 11px; }
  .group-entries { padding: 0; margin: 0; list-style: none; }
  .group-entries li { display: flex; gap: 10px; align-items: center; padding: 10px 14px; border-bottom: 1px solid var(--border-subtle); }
  .entry-position { width: 22px; flex-shrink: 0; color: var(--text-muted); }

  .group-player { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 550; }
  small { display: block; font-size: 10px; font-weight: 400; color: var(--text-muted); }
  .group-qualifiers { display: flex; align-items: center; gap: 6px; padding: 10px 14px; color: var(--text-secondary); font-size: 10px; }
  details { border-top: 1px solid var(--border-subtle); padding: 10px 14px; font-size: 11px; }
  summary { cursor: pointer; color: var(--text-secondary); }
  .group-empty { display: grid; place-items: center; color: var(--text-muted); padding: 32px 16px; border: 1px dashed var(--border); border-radius: 8px; text-align: center; }
  @media (max-width: 1100px) { .groups-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
  @media (max-width: 650px) { .groups-grid { grid-template-columns: minmax(0, 1fr); } }
</style>
