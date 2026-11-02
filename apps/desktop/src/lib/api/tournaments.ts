import { invoke } from '@tauri-apps/api/core';
import type { Category } from '$lib/api/categories';

export interface Tournament { completed: boolean; cover: string | null; registered_count: number; id: string; name: string; categories: Category[] }

export const listTrashedTournaments = () => invoke<Tournament[]>('list_trashed_tournaments');

export const trashTournament = (id: string) => invoke<void>('trash_tournament', { id });

export const restoreTournament = (id: string) => invoke<Tournament>('restore_tournament', { id });

export const listTournaments = () => invoke<Tournament[]>('list_tournaments');

export const createTournamentWithCover = (id: string, name: string, cover: string | null) => invoke<Tournament>('create_tournament_with_cover', { id, name, cover });

export const updateTournamentDetails = (id: string, name: string, cover: string | null, expectedName: string, expectedCover: string | null) => invoke<Tournament>('update_tournament_details', { id, name, cover, expectedName, expectedCover });
