import { invoke } from '@tauri-apps/api/core';
import type { CategoryRules } from '$lib/api/categories';
import type { CategoryDraw } from '$lib/api/draw';

export type MatchOutcome = 'played' | 'retired' | 'walkover';

export interface SetScore { first: number; second: number }

export interface MatchResult { first: string; second: string; winner: string; outcome: MatchOutcome; sets: SetScore[]; rules: CategoryRules }

export interface ScheduledMatch { key: string; round: number; position: number; first: string | null; second: string | null; bye: boolean; result: MatchResult | null; revision: number }

export interface MatchPage { draw: CategoryDraw | null; rules: CategoryRules; rules_revision: number; stale: boolean; round_count: number; total: number; matches: ScheduledMatch[] }

export interface SaveMatchRequest { allow_unconfirmed_start?: boolean; request_id: string; tournament_id: string; category_id: string; draw_id: string; rules_revision: number; key: string; expected_revision: number; result: MatchResult; invalidate_downstream: boolean }

export const getMatchPage = (tournamentId: string, categoryId: string, group: number, round: number, page: number, knockout = false) => invoke<MatchPage>('get_match_page', { tournamentId, categoryId, group, round, page, knockout });

export const saveMatchResult = (request: SaveMatchRequest) => invoke<ScheduledMatch>('save_match_result', { request });
