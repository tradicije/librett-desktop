import { invoke, isTauri } from '@tauri-apps/api/core';

export type Discipline = 'singles' | 'doubles';
export type CompetitionFormat = 'knockout' | 'groups_knockout';
export interface Category { archived: boolean; fee_minor: number; id: string; name: string; discipline: Discipline; format: CompetitionFormat }
export interface Tournament { id: string; name: string; categories: Category[] }
export interface PlayerProfile { birth_year: number | null; city: string; country: string; email: string; phone: string; notes: string; photo: string | null }
export interface Player extends PlayerProfile { id: string; name: string; club: string }
export type EntryStatus = 'registered' | 'withdrawn';
export interface Entry { id: string; category_id: string; status: EntryStatus; members: (Pick<Player, 'id' | 'name' | 'club'> & { checked_in: boolean })[] }
export const desktopAvailable = isTauri();

export const listTournaments = () => invoke<Tournament[]>('list_tournaments');
export const createTournament = (name: string) => invoke<Tournament>('create_tournament', { name });
export const addCategory = (tournamentId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number) =>
  invoke<Tournament>('add_category', { tournamentId, name, discipline, format, feeMinor });

export const listPlayers = () => invoke<Player[]>('list_players');
export const createPlayer = (name: string, club: string) => invoke<Player>('create_player', { name, club });
export const listEntries = (categoryId: string) => invoke<Entry[]>('list_entries', { categoryId });
export const registerEntry = (tournamentId: string, categoryId: string, playerIds: string[]) =>
  invoke<Entry>('register_entry', { tournamentId, categoryId, playerIds });

export const savePlayerProfile = (id: string | null, name: string, club: string, profile: PlayerProfile) =>
  invoke<Player>('save_player_profile', { id, name, club, profile });

export const getPlayer = (id: string) => invoke<Player>('get_player', { id });
export const deletePlayer = (id: string) => invoke<void>('delete_player', { id });

export const setEntryStatus = (tournamentId: string, entryId: string, status: EntryStatus) =>
  invoke<void>('set_entry_status', { tournamentId, entryId, status });
export const setPlayerAttendance = (tournamentId: string, playerId: string, checkedIn: boolean) =>
  invoke<void>('set_player_attendance', { tournamentId, playerId, checkedIn });

export type CashKind = 'charge' | 'discount' | 'payment' | 'refund';
export interface CashRecord { id: string; entry_id: string; kind: CashKind; amount_minor: number; note: string; created_at: string }
export const listCash = (tournamentId: string) => invoke<CashRecord[]>('list_cash', { tournamentId });
export const recordCash = (tournamentId: string, record: CashRecord) => invoke<void>('record_cash', { tournamentId, record });

export const deleteCategory = (tournamentId: string, categoryId: string) => invoke<Tournament>('delete_category', { tournamentId, categoryId });
export const registerEntries = (tournamentId: string, categoryId: string, playerGroups: string[][]) => invoke<Entry[]>('register_entries', { tournamentId, categoryId, playerGroups });

export interface CashAllocation { record_id: string; player_id: string; amount_minor: number }
export interface CashLedger { records: CashRecord[]; allocations: CashAllocation[] }
export const cashLedger = (tournamentId: string) => invoke<CashLedger>('cash_ledger', { tournamentId });
export const settlePlayerCash = (requestId: string, tournamentId: string, playerId: string, entryIds: string[], paid: boolean) => invoke<void>('settle_player_cash', { requestId, tournamentId, playerId, entryIds, paid });

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
export const saveCategoryDraw = (tournamentId: string, draw: CategoryDraw, expectedRevision: number) => invoke<CategoryDraw>('save_category_draw', { tournamentId, draw, expectedRevision });

export type RankingCriterion = 'head_to_head' | 'set_ratio' | 'point_ratio';
export interface CategoryRules { group_count: number; qualifiers_per_group: number; best_of: number; points_to_win: number; win_by: number; ranking: RankingCriterion[] }
export interface CategoryConfiguration { category_id: string; revision: number; rules: CategoryRules }
export const defaultCategoryRules = (): CategoryRules => ({ group_count: 2, qualifiers_per_group: 2, best_of: 5, points_to_win: 11, win_by: 2, ranking: ['head_to_head', 'set_ratio', 'point_ratio'] });
export const createCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules) => invoke<Tournament>('create_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules });
export const getCategoryRules = (tournamentId: string, categoryId: string) => invoke<CategoryConfiguration>('get_category_rules', { tournamentId, categoryId });
export const saveCategoryRules = (tournamentId: string, categoryId: string, rules: CategoryRules, expectedRevision: number) => invoke<CategoryConfiguration>('save_category_rules', { tournamentId, categoryId, rules, expectedRevision });

export const updateCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules, expectedRevision: number) => invoke<Tournament>('update_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules, expectedRevision });
