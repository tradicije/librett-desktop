import { invoke, isTauri } from '@tauri-apps/api/core';

export type Discipline = 'singles' | 'doubles';
export type CompetitionFormat = 'knockout' | 'groups_knockout';
export interface Category { archived: boolean; fee_minor: number; id: string; name: string; discipline: Discipline; format: CompetitionFormat }
export interface Tournament { cover: string | null; registered_count: number; id: string; name: string; categories: Category[] }
export interface PlayerProfile { birth_year: number | null; city: string; country: string; email: string; phone: string; notes: string; photo: string | null }
export interface Player extends PlayerProfile { id: string; name: string; club: string }
export type EntryStatus = 'registered' | 'withdrawn';
export interface Entry { id: string; category_id: string; status: EntryStatus; members: (Pick<Player, 'id' | 'name' | 'club'> & { checked_in: boolean })[] }
export const desktopAvailable = isTauri();

export const listTournaments = () => invoke<Tournament[]>('list_tournaments');
export const addCategory = (tournamentId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number) =>
  invoke<Tournament>('add_category', { tournamentId, name, discipline, format, feeMinor });

export const listPlayers = () => invoke<Player[]>('list_players');
export const listEntries = (categoryId: string) => invoke<Entry[]>('list_entries', { categoryId });
export const registerEntry = (tournamentId: string, categoryId: string, playerIds: string[]) =>
  invoke<Entry>('register_entry', { tournamentId, categoryId, playerIds });

export const savePlayerChecked = (requestId: string, playerId: string, name: string, club: string, profile: PlayerProfile, expected: Player | null) => invoke<Player>('save_player_checked', { requestId, playerId, name, club, profile, expected });

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
export interface ExpectedCashAmount { entry_id: string; amount_minor: number }
export const settlePlayerCash = (requestId: string, tournamentId: string, playerId: string, entryIds: string[], paid: boolean, expected: ExpectedCashAmount[]) => invoke<void>('settle_player_cash', { requestId, tournamentId, playerId, entryIds, paid, expected });

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
export interface CategoryRules { age_enabled: boolean; age_min: number | null; age_max: number | null; group_count: number; qualifiers_per_group: number; best_of: number; points_to_win: number; win_by: number; ranking: RankingCriterion[] }
export interface CategoryConfiguration { category_id: string; revision: number; rules: CategoryRules }
export const defaultCategoryRules = (): CategoryRules => ({ age_enabled: false, age_min: null, age_max: null, group_count: 2, qualifiers_per_group: 2, best_of: 5, points_to_win: 11, win_by: 2, ranking: ['head_to_head', 'set_ratio', 'point_ratio'] });
export const createCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules) => invoke<Tournament>('create_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules });
export const getCategoryRules = (tournamentId: string, categoryId: string) => invoke<CategoryConfiguration>('get_category_rules', { tournamentId, categoryId });
export const saveCategoryRules = (tournamentId: string, categoryId: string, rules: CategoryRules, expectedRevision: number) => invoke<CategoryConfiguration>('save_category_rules', { tournamentId, categoryId, rules, expectedRevision });

export const updateCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules, expectedRevision: number) => invoke<Tournament>('update_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules, expectedRevision });

export interface CategoryEditorState { category: Category; configuration: CategoryConfiguration; used: boolean }
export const getCategoryEditorState = (tournamentId: string, categoryId: string) => invoke<CategoryEditorState>('get_category_editor_state', { tournamentId, categoryId });
export const createTournamentWithCover = (id: string, name: string, cover: string | null) => invoke<Tournament>('create_tournament_with_cover', { id, name, cover });

export const updateTournamentDetails = (id: string, name: string, cover: string | null, expectedName: string, expectedCover: string | null) => invoke<Tournament>('update_tournament_details', { id, name, cover, expectedName, expectedCover });

export type MatchOutcome = 'played' | 'retired' | 'walkover';
export interface SetScore { first: number; second: number }
export interface MatchResult { first: string; second: string; winner: string; outcome: MatchOutcome; sets: SetScore[]; rules: CategoryRules }
export interface ScheduledMatch { key: string; round: number; position: number; first: string | null; second: string | null; bye: boolean; result: MatchResult | null; revision: number }
export interface MatchPage { draw: CategoryDraw | null; rules: CategoryRules; rules_revision: number; stale: boolean; round_count: number; total: number; matches: ScheduledMatch[] }
export interface SaveMatchRequest { request_id: string; tournament_id: string; category_id: string; draw_id: string; rules_revision: number; key: string; expected_revision: number; result: MatchResult; invalidate_downstream: boolean }
export const getMatchPage = (tournamentId: string, categoryId: string, group: number, round: number, page: number, knockout = false) => invoke<MatchPage>('get_match_page', { tournamentId, categoryId, group, round, page, knockout });
export const saveMatchResult = (request: SaveMatchRequest) => invoke<ScheduledMatch>('save_match_result', { request });

export interface StandingRow { entry_id: string; played: number; wins: number; losses: number; sets_for: number; sets_against: number; points_for: number; points_against: number; tied: boolean }
export interface GroupStanding { group: number; rows: StandingRow[]; completed: number; total: number; complete: boolean; resolved: boolean; manual: boolean }
export interface QualificationSlot { entry_id: string | null; group: number | null; place: number | null; bye: boolean }
export interface CompetitionState { draw_id: string | null; stale: boolean; groups: GroupStanding[]; slots: QualificationSlot[]; matches: ScheduledMatch[]; order_revisions: number[]; result_versions: number[] }
export interface GroupOrderRequest { request_id: string; tournament_id: string; category_id: string; draw_id: string; group: number; expected_revision: number; expected_result_version: number; order: string[] | null; invalidate_downstream: boolean }
export const getCompetitionState = (tournamentId: string, categoryId: string) => invoke<CompetitionState>('get_competition_state', { tournamentId, categoryId });
export const saveGroupOrder = (request: GroupOrderRequest) => invoke<CompetitionState>('save_group_order', { request });
