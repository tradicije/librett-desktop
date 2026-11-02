import { invoke } from '@tauri-apps/api/core';
import type { ThirdPlaceRule } from '$lib/api/categories';
import type { CategoryDraw } from '$lib/api/draw';

export interface CompletionState { revision: number; completed_at: string | null }

export interface FinalPlacement { entry_id: string; place: number; place_end: number; stage: 'winner' | 'finalist' | 'knockout' | 'groups'; round: number | null }

export interface CategoryResults { third_place: ThirdPlaceRule; category_id: string; category_name: string; completion: CompletionState; tournament_completion: CompletionState; version: string; draw: CategoryDraw | null; ready: boolean; blockers: string[]; group_completed: number; group_total: number; knockout_completed: number; knockout_total: number; placements: FinalPlacement[] }

export interface TournamentProgress { tournament_id: string; completion: CompletionState; version: string; ready: boolean; categories: CategoryResults[] }

export interface CompletionRequest { request_id: string; tournament_id: string; category_id: string | null; expected_revision: number; expected_version: string; complete: boolean }

export const getCategoryResults = (tournamentId: string, categoryId: string) => invoke<CategoryResults>('get_category_results', { tournamentId, categoryId });

export const getTournamentProgress = (tournamentId: string) => invoke<TournamentProgress>('get_tournament_progress', { tournamentId });

export const changeCompletion = (request: CompletionRequest) => invoke<TournamentProgress>('change_completion', { request });
