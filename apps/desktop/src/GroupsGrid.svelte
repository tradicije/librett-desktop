<script lang="ts">
  import Icon from './Icon.svelte';
  import { groupName, roundRobin } from './draw-view';
  import type { CategoryDraw, Category } from './api';
  import type { Language } from './i18n';
  let { draw, category, language }: { draw: CategoryDraw | null; category: Category; language: Language } = $props();
  function entryName(id: string | null) { return draw?.participants.find(entry => entry.id === id)?.members.map(member => member.name).join(' / ') ?? (language === 'sr' ? 'Neraspoređeno' : 'Unassigned'); }
  function clubName(id: string | null) { return [...new Set(draw?.participants.find(entry => entry.id === id)?.members.map(member => member.club).filter(Boolean))].join(' / '); }
</script>
{#if draw}
  <div class="groups-grid">
    {#each draw.sections as group, index}
              <section class="group-card">
                <div class="group-heading"><h3>{language === 'sr' ? 'Grupa' : 'Group'} {groupName(index)}</h3><span>{group.filter(Boolean).length} / {group.length}</span></div>
                <ol class="group-entries">{#each group as id}<li><span class="entry-position">{#if id && draw.seeds.includes(id)}<span class="group-seed">#{draw.seeds.indexOf(id) + 1}</span>{:else}<Icon name={category.discipline === 'doubles' ? 'users' : 'user'} size={15} />{/if}</span><span class="group-player" title={entryName(id)}>{entryName(id)}{#if clubName(id)}<small>{clubName(id)}</small>{/if}</span></li>{/each}</ol>
                <div class="group-qualifiers"><Icon name="arrow-right" size={14} />{draw.settings.qualifiers_per_group} {language === 'sr' ? 'prolaze u nokaut' : 'advance to knockout'}</div>
                {#if group.every(id => id !== null)}
                  <details><summary>{language === 'sr' ? 'Ko sa kim igra' : 'Round-robin pairings'}</summary><div class="round-robin">{#each roundRobin(group.filter((id): id is string => id !== null)) as round, roundIndex}<h4>{language === 'sr' ? 'Kolo' : 'Round'} {roundIndex + 1}</h4>{#each round as [a, b]}<p><span>{entryName(a)}</span><span class="versus">vs</span><span>{entryName(b)}</span></p>{/each}{/each}</div></details>
                {/if}
              </section>
    {/each}
  </div>
{:else}
  <div class="group-empty"><Icon name="users" size={28} /><p>{language === 'sr' ? 'Grupe će se prikazati posle pripreme rasporeda.' : 'Groups will appear after the layout is prepared.'}</p></div>
{/if}
<style>
  .groups-grid { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 20px; align-items: start; }
  .group-card { border: 1px solid var(--border); border-radius: 8px; background: var(--surface); overflow: hidden; }
  .group-heading { display: flex; justify-content: space-between; align-items: center; padding: 12px 14px; border-bottom: 1px solid var(--border-subtle); }
  .group-heading h3 { margin: 0; font-size: 12px; }
  .group-heading > span { color: var(--text-muted); font-size: 11px; }
  .group-entries { padding: 0; margin: 0; list-style: none; }
  .group-entries li { display: flex; gap: 10px; align-items: center; padding: 10px 14px; border-bottom: 1px solid var(--border-subtle); }
  .entry-position { width: 22px; flex-shrink: 0; color: var(--text-muted); }
  .group-seed { color: var(--primary); font-weight: 650; font-size: 10px; }
  .group-player { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 550; }
  small { display: block; font-size: 10px; font-weight: 400; color: var(--text-muted); }
  .group-qualifiers { display: flex; align-items: center; gap: 6px; padding: 10px 14px; color: var(--text-secondary); font-size: 10px; }
  details { border-top: 1px solid var(--border-subtle); padding: 10px 14px; font-size: 11px; }
  summary { cursor: pointer; color: var(--text-secondary); }
  .round-robin h4 { font-size: 10px; color: var(--text-muted); margin: 14px 0 6px; }
  .round-robin p { display: grid; grid-template-columns: minmax(0, 1fr) 16px minmax(0, 1fr); gap: 4px; font-size: 10px; }
  .round-robin p span { overflow-wrap: anywhere; }
  .versus { color: var(--text-muted); text-align: center; }
  .group-empty { display: grid; place-items: center; color: var(--text-muted); padding: 32px 16px; border: 1px dashed var(--border); border-radius: 8px; text-align: center; }
  @media (max-width: 1100px) { .groups-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
  @media (max-width: 650px) { .groups-grid { grid-template-columns: minmax(0, 1fr); } }
</style>
