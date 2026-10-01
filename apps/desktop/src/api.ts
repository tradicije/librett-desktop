import { invoke, isTauri } from '@tauri-apps/api/core';

export type Discipline = 'singles' | 'doubles';
export type CompetitionFormat = 'knockout' | 'groups_knockout';
export interface Category { id: string; name: string; discipline: Discipline; format: CompetitionFormat }
export interface Tournament { id: string; name: string; categories: Category[] }
export interface Player { id: string; name: string; club: string }
export interface Entry { id: string; category_id: string; members: Player[] }
export const desktopAvailable = isTauri();

export const listTournaments = () => invoke<Tournament[]>('list_tournaments');
export const createTournament = (name: string) => invoke<Tournament>('create_tournament', { name });
export const addCategory = (tournamentId: string, name: string, discipline: Discipline, format: CompetitionFormat) =>
  invoke<Tournament>('add_category', { tournamentId, name, discipline, format });

export const listPlayers = () => invoke<Player[]>('list_players');
export const createPlayer = (name: string, club: string) => invoke<Player>('create_player', { name, club });
export const listEntries = (categoryId: string) => invoke<Entry[]>('list_entries', { categoryId });
export const registerEntry = (tournamentId: string, categoryId: string, playerIds: string[]) =>
  invoke<Entry>('register_entry', { tournamentId, categoryId, playerIds });
