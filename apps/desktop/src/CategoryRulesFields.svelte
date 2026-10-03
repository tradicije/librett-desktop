<script lang="ts">
  import type { CategoryRules, CompetitionFormat, RankingCriterion } from './api';
  import { messages, type Language } from './i18n';
  let { rules = $bindable(), format, language, disabled = false }: { rules: CategoryRules; format: CompetitionFormat; language: Language; disabled?: boolean } = $props();
  let text = $derived(messages[language]);
  const criteria: RankingCriterion[] = ['head_to_head', 'set_ratio', 'point_ratio'];
</script>
<fieldset disabled={disabled} class="category-rules-fields">
  <legend>{text.categoryRules}</legend>
  {#if format === 'groups_knockout'}
    <p class="muted">{text.rulesHint}</p>
    <div class="rule-fields"><label>{text.groupCount}<input type="number" min="1" max="2048" step="1" bind:value={rules.group_count} required /></label><label>{text.qualifiersPerGroup}<input type="number" min="1" max="4096" step="1" bind:value={rules.qualifiers_per_group} required /></label></div>
  {/if}
  <div class="rule-fields">
    <label>{text.bestOf}<select bind:value={rules.best_of}>{#each [1, 3, 5, 7, 9] as sets}<option value={sets}>{sets}</option>{/each}</select></label>
    <label>{text.pointsToWin}<input type="number" min="1" max="99" step="1" bind:value={rules.points_to_win} required /></label>
    <label>{text.winBy}<input type="number" min="1" max="10" step="1" bind:value={rules.win_by} required /></label>
  </div>
  {#if format === 'groups_knockout'}
    <div class="ranking-fields"><span class="muted">{text.rankingOrder}</span>{#each [0, 1, 2] as index}<label>{index + 1}.<select aria-label={`${text.rankingOrder} ${index + 1}`} bind:value={rules.ranking[index]}>{#each criteria as criterion}<option value={criterion}>{text[criterion]}</option>{/each}</select></label>{/each}</div>
  {/if}
</fieldset>
<style>
  fieldset { margin: 0; padding: 16px; min-width: 0; border: 1px solid var(--border-subtle); border-radius: 6px; }
  legend { padding: 0 6px; font-weight: 550; }
  .rule-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 12px; margin: 12px 0; }
  label { display: grid; gap: 6px; min-width: 0; }
  .ranking-fields { display: grid; gap: 8px; }
  .ranking-fields label { display: grid; grid-template-columns: 20px 1fr; align-items: center; }
</style>
