import { invoke, isTauri } from '@tauri-apps/api/core';

export type Discipline = 'singles' | 'doubles';
export type CompetitionFormat = 'knockout' | 'groups_knockout';
export interface Category { id: string; name: string; discipline: Discipline; format: CompetitionFormat }
export interface Tournament { id: string; name: string; categories: Category[] }
export const desktopAvailable = isTauri();

export const listTournaments = () => invoke<Tournament[]>('list_tournaments');
export const createTournament = (name: string) => invoke<Tournament>('create_tournament', { name });
export const addCategory = (tournamentId: string, name: string, discipline: Discipline, format: CompetitionFormat) =>
  invoke<Tournament>('add_category', { tournamentId, name, discipline, format });
