import { invoke, isTauri } from '@tauri-apps/api/core';

export type Discipline = 'singles' | 'doubles';
export type CompetitionFormat = 'knockout' | 'groups_knockout';
export interface Category { completed: boolean; archived: boolean; fee_minor: number; id: string; name: string; discipline: Discipline; format: CompetitionFormat }
export interface Tournament { completed: boolean; cover: string | null; registered_count: number; id: string; name: string; categories: Category[] }
export interface PlayerProfile { birth_year: number | null; city: string; country: string; email: string; phone: string; notes: string; photo: string | null }
export interface Player extends PlayerProfile { id: string; name: string; club: string }
export type EntryStatus = 'registered' | 'withdrawn';
export interface Entry { id: string; category_id: string; status: EntryStatus; members: (Pick<Player, 'id' | 'name' | 'club'> & { checked_in: boolean })[] }
export const desktopAvailable = isTauri();

export const listTrashedTournaments = () => invoke<Tournament[]>('list_trashed_tournaments');
export const trashTournament = (id: string) => invoke<void>('trash_tournament', { id });
export const restoreTournament = (id: string) => invoke<Tournament>('restore_tournament', { id });
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

export const registrationStarted = (tournamentId: string, categoryId: string) => invoke<boolean>('registration_started', { tournamentId, categoryId });
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
export const saveCategoryDraw = (tournamentId: string, draw: CategoryDraw, expectedRevision: number, confirmRestart = false) => invoke<CategoryDraw>('save_category_draw', { tournamentId, draw, expectedRevision, confirmRestart });

export type RankingCriterion = 'head_to_head' | 'set_ratio' | 'point_ratio';
export type ThirdPlaceRule = 'shared' | 'bronze_match' | 'champion_semifinalist';
export type KnockoutFilling = 'bye' | 'lucky_loser' | 'lucky_loser_manual';
export interface CategoryRules { third_place: ThirdPlaceRule; knockout_filling: KnockoutFilling; age_enabled: boolean; age_min: number | null; age_max: number | null; group_count: number; qualifiers_per_group: number; best_of: number; points_to_win: number; win_by: number; ranking: RankingCriterion[] }
export interface CategoryConfiguration { category_id: string; revision: number; rules: CategoryRules }
export const defaultCategoryRules = (): CategoryRules => ({ third_place: 'shared', knockout_filling: 'bye', age_enabled: false, age_min: null, age_max: null, group_count: 2, qualifiers_per_group: 2, best_of: 5, points_to_win: 11, win_by: 2, ranking: ['head_to_head', 'set_ratio', 'point_ratio'] });
export const createCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules) => invoke<Tournament>('create_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules });
export const getCategoryRules = (tournamentId: string, categoryId: string) => invoke<CategoryConfiguration>('get_category_rules', { tournamentId, categoryId });
export const saveCategoryRules = (tournamentId: string, categoryId: string, rules: CategoryRules, expectedRevision: number, invalidateDownstream = false) => invoke<CategoryConfiguration>('save_category_rules', { tournamentId, categoryId, rules, expectedRevision, invalidateDownstream });

export const updateCategoryWithRules = (tournamentId: string, categoryId: string, name: string, discipline: Discipline, format: CompetitionFormat, feeMinor: number, rules: CategoryRules, expectedRevision: number, invalidateDownstream = false) => invoke<Tournament>('update_category_with_rules', { tournamentId, categoryId, name, discipline, format, feeMinor, rules, expectedRevision, invalidateDownstream });

export interface CategoryEditorState { category: Category; configuration: CategoryConfiguration; used: boolean }
export const getCategoryEditorState = (tournamentId: string, categoryId: string) => invoke<CategoryEditorState>('get_category_editor_state', { tournamentId, categoryId });
export const createTournamentWithCover = (id: string, name: string, cover: string | null) => invoke<Tournament>('create_tournament_with_cover', { id, name, cover });

export const updateTournamentDetails = (id: string, name: string, cover: string | null, expectedName: string, expectedCover: string | null) => invoke<Tournament>('update_tournament_details', { id, name, cover, expectedName, expectedCover });

export type MatchOutcome = 'played' | 'retired' | 'walkover';
export interface SetScore { first: number; second: number }
export interface MatchResult { first: string; second: string; winner: string; outcome: MatchOutcome; sets: SetScore[]; rules: CategoryRules }
export interface ScheduledMatch { key: string; round: number; position: number; first: string | null; second: string | null; bye: boolean; result: MatchResult | null; revision: number }
export interface MatchPage { draw: CategoryDraw | null; rules: CategoryRules; rules_revision: number; stale: boolean; round_count: number; total: number; matches: ScheduledMatch[] }
export interface SaveMatchRequest { allow_unconfirmed_start?: boolean; request_id: string; tournament_id: string; category_id: string; draw_id: string; rules_revision: number; key: string; expected_revision: number; result: MatchResult; invalidate_downstream: boolean }
export const getMatchPage = (tournamentId: string, categoryId: string, group: number, round: number, page: number, knockout = false) => invoke<MatchPage>('get_match_page', { tournamentId, categoryId, group, round, page, knockout });
export const saveMatchResult = (request: SaveMatchRequest) => invoke<ScheduledMatch>('save_match_result', { request });

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

export interface CompletionState { revision: number; completed_at: string | null }
export interface FinalPlacement { entry_id: string; place: number; place_end: number; stage: 'winner' | 'finalist' | 'knockout' | 'groups'; round: number | null }
export interface CategoryResults { third_place: ThirdPlaceRule; category_id: string; category_name: string; completion: CompletionState; tournament_completion: CompletionState; version: string; draw: CategoryDraw | null; ready: boolean; blockers: string[]; group_completed: number; group_total: number; knockout_completed: number; knockout_total: number; placements: FinalPlacement[] }
export interface TournamentProgress { tournament_id: string; completion: CompletionState; version: string; ready: boolean; categories: CategoryResults[] }
export interface CompletionRequest { request_id: string; tournament_id: string; category_id: string | null; expected_revision: number; expected_version: string; complete: boolean }
export const getCategoryResults = (tournamentId: string, categoryId: string) => invoke<CategoryResults>('get_category_results', { tournamentId, categoryId });
export const getTournamentProgress = (tournamentId: string) => invoke<TournamentProgress>('get_tournament_progress', { tournamentId });
export const changeCompletion = (request: CompletionRequest) => invoke<TournamentProgress>('change_completion', { request });

export interface BackupInfo { name:string; size:number; modified:number }
export const listBackups=()=>invoke<BackupInfo[]>('list_backups');
export const createBackup=()=>invoke<BackupInfo>('create_backup');
export const automaticBackup=()=>invoke<void>('automatic_backup');
export const restoreBackup=(name:string)=>invoke<void>('restore_backup',{name});
export const importBackup=(encoded:string)=>invoke<void>('import_backup',{encoded});
export const exportBackup=(name:string)=>invoke<string>('export_backup',{name});
export const openDataFolder=(kind:'backups'|'exports')=>invoke<void>('open_data_folder',{kind});
export interface ReadyMatch {category_id:string;category_name:string;draw_id:string;key:string;phase:'groups'|'knockout';round:number;first:string;second:string;first_name:string;second_name:string;players:string[]}
export interface TableAssignment {table:number;status:'queued'|'running';scheduled:ReadyMatch;started_at:string|null}
export interface ScheduleState {tournament_id:string;table_count:number;version:string;assignments:TableAssignment[];waiting:ReadyMatch[];truncated:boolean}
export type ScheduleAction={kind:'remove';draw_id:string;key:string}|{kind:'configure';table_count:number}|{kind:'assign';draw_id:string;key:string;table:number}|{kind:'start'|'clear';table:number};
export interface ScheduleRequest {request_id:string;tournament_id:string;expected_version:string;action:ScheduleAction}
export const getSchedule=(tournamentId:string)=>invoke<ScheduleState>('get_schedule',{tournamentId});
export const changeSchedule=(request:ScheduleRequest)=>invoke<ScheduleState>('change_schedule',{request});
export const saveReport=(kind:string,format:'csv'|'html',content:string)=>invoke<string>('save_report',{kind,format,content});
export const openPrintReport=(content:string)=>invoke<void>('open_print_report',{content});
export const getPrintReport=(token:string)=>invoke<string>('get_print_report',{token});
export const printReport=()=>invoke<void>('print_report');

export const getMatchTables=(tournamentId:string,drawId:string)=>invoke<Record<string,number>>('get_match_tables',{tournamentId,drawId});

export const getCategoryReport=(tournamentId:string,categoryId:string)=>invoke<{results:CategoryResults;competition:CompetitionState;rules:CategoryRules}>('get_category_report',{tournamentId,categoryId});

export interface HistoryItem { id:number; occurred_at:string; kind:string; action:string; entity_type:string; entity_id:string; entity_name:string; tournament_id:string|null; tournament_name:string|null; category_id:string|null; category_name:string|null; before_data:Record<string,unknown>|null; after_data:Record<string,unknown>|null; historical:boolean }
export interface HistoryOption {id:string;name:string;tournament_id:string|null}
export interface HistoryPage {items:HistoryItem[];has_more:boolean;tournaments:HistoryOption[];categories:HistoryOption[]}
export const getActionHistory = (kinds:string[],tournamentId:string|null,categoryId:string|null,action:string|null,beforeAt:string|null,beforeId:number|null) => invoke<HistoryPage>('get_action_history',{kinds,tournamentId,categoryId,action,beforeAt,beforeId});
