import type { CategoryTab } from './CategoryDetail.svelte';
export type TournamentTab = 'overview' | 'categories' | 'registrations' | 'cash';
export type Route = {
  view: 'dashboard' | 'tournaments' | 'players' | 'player-create' | 'player-edit' | 'tournament' | 'category';
  id?: string; categoryId?: string; tournamentTab?: TournamentTab; categoryTab?: CategoryTab;
};
export interface WorkspaceStatus { title: string; context?: string; busy: boolean; dirty: boolean; view: Route['view'] }
