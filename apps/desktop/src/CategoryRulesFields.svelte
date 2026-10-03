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
  label { display: grid; gap: 6px; min-width: 0; }
  .scoring-fields, .ranking-fields { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 18px 24px; }
  .field-hint { margin: -4px 0 0; font-size: 11px; color: var(--text-muted); line-height: 1.7; }
  .ranking-section h3 { margin-bottom: 14px; font-size: 12px; font-weight: 550; }
  .ranking-fields label > span { font-size: 11px; color: var(--text-secondary); }
  @media (max-width: 900px) { .ranking-fields { grid-template-columns: 1fr; } }
  @media (max-width: 650px) { .scoring-fields { grid-template-columns: 1fr; } }
</style>
