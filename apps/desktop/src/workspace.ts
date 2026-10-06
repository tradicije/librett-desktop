import type { CategoryTab } from './CategoryDetail.svelte';
export type TournamentTab = 'overview' | 'categories' | 'cash' | 'settings';
export type Route = {
  view: 'trash' | 'backups' | 'dashboard' | 'tournaments' | 'tournament-create' | 'players' | 'player-create' | 'player-edit' | 'tournament' | 'category' | 'category-create' | 'category-edit';
  id?: string; categoryId?: string; tournamentTab?: TournamentTab; categoryTab?: CategoryTab;
};
export interface WorkspaceStatus { title: string; context?: string; busy: boolean; dirty: boolean; view: Route['view'] }
