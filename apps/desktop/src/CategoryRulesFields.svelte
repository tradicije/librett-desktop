<script lang="ts">
  import Select from './Select.svelte';
  import type { CategoryRules, CompetitionFormat, RankingCriterion } from './api';
  import { messages, type Language } from './i18n';
  let { rules = $bindable(), format, language, disabled = false }: { rules: CategoryRules; format: CompetitionFormat; language: Language; disabled?: boolean } = $props();
  let text = $derived(messages[language]);
  const criteria: RankingCriterion[] = ['head_to_head', 'set_ratio', 'point_ratio'];
</script>
<fieldset disabled={disabled} class="form-group category-rules-fields">
  <legend>{text.categoryRules}</legend>
  <div class="rule-toggles">
  <div class="age-section">
    <label class="age-toggle"><input type="checkbox" role="switch" checked={rules.age_enabled} disabled={disabled} onchange={event => { const enabled=event.currentTarget.checked; rules={...rules,age_enabled:enabled,age_min:enabled ? 0 : null,age_max:enabled ? 130 : null}; }} /><span class="switch-track" aria-hidden="true"></span><span>{language==='sr'?'Starosna grupa':'Age group'}<small>{rules.age_enabled ? language==='sr'?'Ograničen raspon':'Age range enabled' : language==='sr'?'Sva godišta':'All ages'}</small></span></label>
    {#if rules.age_enabled}<div class="form-fields age-fields"><label>{language==='sr'?'Od (godina)':'From (years)'}<input type="number" min="0" max="130" step="1" required disabled={disabled} bind:value={() => rules.age_min ?? undefined, value => rules.age_min=value ?? null} /></label><label>{language==='sr'?'Do (godina)':'To (years)'}<input type="number" min={rules.age_min ?? 0} max="130" step="1" required disabled={disabled} bind:value={() => rules.age_max ?? undefined, value => rules.age_max=value ?? null} /></label></div><p class="field-hint">{language==='sr'?'Starost = tekuća godina − godište. Prijava van raspona je moguća uz potvrdu.':'Age = current year − birth year. Out-of-range entries can be confirmed as exceptions.'}</p>{/if}
  </div>
    <div class="toggle-section">
    <label class="age-toggle"><input type="checkbox" role="switch" checked={rules.knockout_filling!=='bye'} disabled={disabled || format==='knockout'} onchange={event=>rules={...rules,knockout_filling:event.currentTarget.checked?'lucky_loser':'bye'}}/><span class="switch-track" aria-hidden="true"></span><span>Lucky loser<small>{rules.knockout_filling!=='bye' ? language==='sr'?'Automatski ili ručni izbor učesnika':'Automatic or manual participant selection' : language==='sr'?'BYE — slobodan prolaz':'BYE — automatic advance'}</small></span></label>
    {#if rules.knockout_filling!=='bye' && format==='groups_knockout'}<label>{language==='sr'?'Popunjavanje kostura':'Bracket filling'}<Select label={language==='sr'?'Lucky loser režim':'Lucky loser mode'} bind:value={rules.knockout_filling} options={[{value:'lucky_loser',label:language==='sr'?'Automatski Lucky loser':'Automatic lucky loser'},{value:'lucky_loser_manual',label:language==='sr'?'Ručni izbor':'Manual selection'}]} {disabled}/></label>{/if}
    <p class="field-hint">{format==='knockout' ? language==='sr'?'Lucky loser se koristi posle grupa. Mesta u direktnom nokautu menjaj u Žreb → Uredi.':'Lucky losers are used after groups. Edit direct knockout places in Draw → Edit.' : language==='sr'?'U Žreb → Uredi možeš ručno izabrati učesnika ili BYE za svako mesto.':'In Draw → Edit, manually choose a participant or BYE for each place.'}</p>
    </div>
    <div class="toggle-section">
    <label class="age-toggle"><input type="checkbox" role="switch" checked={rules.third_place!=='shared'} disabled={disabled} onchange={event=>rules={...rules,third_place:event.currentTarget.checked?'bronze_match':'shared'}}/><span class="switch-track" aria-hidden="true"></span><span>{language==='sr'?'Odredi jedno treće mesto':'Determine a single third place'}<small>{rules.third_place==='shared' ? language==='sr'?'Polufinalisti dele treće mesto':'Semifinalists share third place' : language==='sr'?'Posebno treće i četvrto mesto':'Separate third and fourth places'}</small></span></label>
    {#if rules.third_place!=='shared'}<label>{language==='sr'?'Kako se određuje treće mesto':'How third place is decided'}<Select label={language==='sr'?'Pravilo trećeg mesta':'Third-place rule'} bind:value={rules.third_place} options={[{value:'bronze_match',label:language==='sr'?'Meč za treće mesto':'Bronze match'},{value:'champion_semifinalist',label:language==='sr'?'Poraženi od pobednika turnira':'Semifinalist beaten by the champion'}]} {disabled}/></label><p class="field-hint">{rules.third_place==='champion_semifinalist' ? language==='sr'?'Treći je polufinalista koji je izgubio od kasnijeg pobednika. Drugi poraženi polufinalista je četvrti.':'Third is the semifinalist beaten by the eventual champion. The other losing semifinalist is fourth.' : language==='sr'?'Poraženi polufinalisti igraju za treće mesto.':'Losing semifinalists play for third place.'}</p>{/if}
    </div>
  </div>
  {#if format === 'groups_knockout'}
    <div class="form-fields"><label>{text.groupCount}<input type="number" min="1" max="2048" step="1" bind:value={rules.group_count} required /></label><label>{text.qualifiersPerGroup}<input type="number" min="1" max="4096" step="1" bind:value={rules.qualifiers_per_group} required /></label></div>
    <p class="field-hint">{text.rulesHint}</p>
  {/if}
  <div class="scoring-fields">
    <label>{text.bestOf}<Select label={text.bestOf} bind:value={rules.best_of} options={[1, 3, 5, 7, 9].map(value => ({ value, label: String(value) }))} {disabled} /></label>
    <label>{text.pointsToWin}<input type="number" min="1" max="99" step="1" bind:value={rules.points_to_win} required /></label>
    <label>{text.winBy}<input type="number" min="1" max="10" step="1" bind:value={rules.win_by} required /></label>
  </div>
  {#if format === 'groups_knockout'}
    <div class="ranking-section"><h3>{text.rankingOrder}</h3><div class="ranking-fields">{#each [0, 1, 2] as index}<label><span>{index + 1}. {language === 'sr' ? 'kriterijum' : 'criterion'}</span><Select label={`${text.rankingOrder} ${index + 1}`} bind:value={rules.ranking[index]} options={criteria.map(value => ({ value, label: text[value] }))} {disabled} /></label>{/each}</div></div>
  {/if}
</fieldset>
<style>
  .age-section, .toggle-section { display: grid; align-content:start; gap: 14px; min-width:0; }
  .rule-toggles { display:grid; grid-template-columns:repeat(3,minmax(0,1fr)); gap:24px; padding-bottom:20px; border-bottom:1px solid var(--border-subtle); }
  .rule-toggles .age-toggle { min-height:52px; }
  .age-fields { gap:12px; }
  .age-toggle { display: flex; align-items: center; gap: 12px; cursor: pointer; position: relative; }
  .age-toggle > input { position: absolute; width: 1px; height: 1px; opacity: 0; }
  .switch-track { width: 36px; height: 20px; flex-shrink: 0; border-radius: 12px; border: 1px solid var(--border); background: var(--background); padding: 2px; }
  .switch-track::after { content: ''; display: block; width: 14px; height: 14px; border-radius: 50%; background: var(--text-muted); }
  input:checked + .switch-track { background: var(--primary); border-color: var(--primary); }
  input:checked + .switch-track::after { transform: translateX(16px); background: white; }
  input:focus-visible + .switch-track { outline: 2px solid var(--primary); outline-offset: 3px; }
  input:disabled + .switch-track { opacity: .45; }
  small { display: block; margin-top: 4px; color: var(--text-muted); font-size: 10px; }
  label { display: grid; gap: 6px; min-width: 0; }
  .scoring-fields, .ranking-fields { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 18px 24px; }
  .field-hint { margin: -4px 0 0; font-size: 11px; color: var(--text-muted); line-height: 1.7; }
  .ranking-section h3 { margin-bottom: 14px; font-size: 12px; font-weight: 550; }
  .ranking-fields label > span { font-size: 11px; color: var(--text-secondary); }
  @media (max-width: 900px) { .ranking-fields { grid-template-columns: 1fr; } }
  @media (max-width: 650px) { .scoring-fields { grid-template-columns: 1fr; } }
@media(max-width:750px){.rule-toggles{grid-template-columns:1fr;}}
</style>
