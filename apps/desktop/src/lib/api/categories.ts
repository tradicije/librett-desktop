import { invoke } from '@tauri-apps/api/core';
import type { Tournament } from '$lib/api/tournaments';

export type Discipline = 'singles' | 'doubles';

export type CompetitionFormat = 'knockout' | 'groups_knockout';

export interface Category { completed: boolean; archived: boolean; fee_minor: number; id: string; name: string; discipline: Discipline; format: CompetitionFormat }

export const addCategory = (tournamentId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number) =>
  invoke<Tournament>('add_category', { tournamentId, name, discipline, format, feeMinor });

export const deleteCategory = (tournamentId: string, categoryId: string) => invoke<Tournament>('delete_category', { tournamentId, categoryId });

export type RankingCriterion = 'head_to_head' | 'set_ratio' | 'point_ratio';

export type ThirdPlaceRule = 'shared' | 'bronze_match' | 'champion_semifinalist';

export type KnockoutFilling = 'bye' | 'lucky_loser' | 'lucky_loser_manual';

export interface CategoryRules { allow_same_club: boolean; third_place: ThirdPlaceRule; knockout_filling: KnockoutFilling; age_enabled: boolean; age_min: number | null; age_max: number | null; group_count: number; qualifiers_per_group: number; best_of: number; points_to_win: number; win_by: number; ranking: RankingCriterion[] }

export interface CategoryConfiguration { category_id: string; revision: number; rules: CategoryRules }

export const defaultCategoryRules = (): CategoryRules => ({ allow_same_club: true, third_place: 'shared', knockout_filling: 'bye', age_enabled: false, age_min: null, age_max: null, group_count: 2, qualifiers_per_group: 2, best_of: 5, points_to_win: 11, win_by: 2, ranking: ['head_to_head', 'set_ratio', 'point_ratio'] });

export const createCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules) => invoke<Tournament>('create_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules });

export const getCategoryRules = (tournamentId: string, categoryId: string) => invoke<CategoryConfiguration>('get_category_rules', { tournamentId, categoryId });

export const saveCategoryRules = (tournamentId: string, categoryId: string, rules: CategoryRules, expectedRevision: number, invalidateDownstream = false) => invoke<CategoryConfiguration>('save_category_rules', { tournamentId, categoryId, rules, expectedRevision, invalidateDownstream });

export const updateCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules, expectedRevision: number, invalidateDownstream = false) => invoke<Tournament>('update_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules, expectedRevision, invalidateDownstream });

export interface CategoryEditorState { category: Category; configuration: CategoryConfiguration; used: boolean }

export const getCategoryEditorState = (tournamentId: string, categoryId: string) => invoke<CategoryEditorState>('get_category_editor_state', { tournamentId, categoryId });
