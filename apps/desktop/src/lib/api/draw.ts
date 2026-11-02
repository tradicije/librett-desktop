import { invoke } from '@tauri-apps/api/core';
import type { CompetitionFormat } from '$lib/api/categories';
import type { Entry } from '$lib/api/registrations';
import type { ScheduledMatch } from '$lib/api/matches';

export type DrawMode = 'automatic' | 'manual';

export interface DrawSettings { group_count: number; qualifiers_per_group: number }

export interface CategoryDraw {
  id: string; revision: number; category_id: string; format: CompetitionFormat;
  mode: DrawMode; algorithm_version: number; random_seed: string;
  settings: DrawSettings; participants: Entry[]; seeds: string[];
  sections: (string | null)[][];
}

export const getCategoryDraw = (tournamentId: string, categoryId: string) => invoke<CategoryDraw | null>('get_category_draw', { tournamentId, categoryId });

export const previewCategoryDraw = (tournamentId: string, categoryId: string, mode: DrawMode, settings: DrawSettings, seeds: string[]) => invoke<CategoryDraw>('preview_category_draw', { tournamentId, categoryId, mode, settings, seeds });

export const saveCategoryDraw = (tournamentId: string, draw: CategoryDraw, expectedRevision: number, confirmRestart = false) => invoke<CategoryDraw>('save_category_draw', { tournamentId, draw, expectedRevision, confirmRestart });

export interface StandingRow { entry_id: string; played: number; wins: number; losses: number; sets_for: number; sets_against: number; points_for: number; points_against: number; tied: boolean }

export interface GroupStanding { group: number; rows: StandingRow[]; completed: number; total: number; complete: boolean; resolved: boolean; manual: boolean }

export interface QualificationSlot { lucky_loser: boolean; entry_id: string | null; group: number | null; place: number | null; bye: boolean }

export interface CompetitionState { match_version: number; filler_revision: number; rules_revision: number; fillers: Record<string, FillerChoice>; candidates: LuckyLoserCandidate[]; draw_id: string | null; stale: boolean; groups: GroupStanding[]; slots: QualificationSlot[]; matches: ScheduledMatch[]; order_revisions: number[]; result_versions: number[] }

export interface GroupOrderRequest { request_id: string; tournament_id: string; category_id: string; draw_id: string; group: number; expected_revision: number; expected_result_version: number; order: string[] | null; invalidate_downstream: boolean }

export const getCompetitionState = (tournamentId: string, categoryId: string) => invoke<CompetitionState>('get_competition_state', { tournamentId, categoryId });

export const saveGroupOrder = (request: GroupOrderRequest) => invoke<CompetitionState>('save_group_order', { request });

export type FillerChoice = { kind: 'bye' } | { kind: 'entry'; entry_id: string };

export interface LuckyLoserCandidate { group: number; place: number; standing: StandingRow }

export interface SaveFillersRequest { expected_match_version: number; request_id: string; tournament_id: string; category_id: string; draw_id: string; expected_revision: number; rules_revision: number; order_revisions: number[]; result_versions: number[]; fillers: Record<string,FillerChoice>; invalidate_downstream: boolean }

export const saveKnockoutFillers = (request: SaveFillersRequest) => invoke<CompetitionState>('save_knockout_fillers', { request });
