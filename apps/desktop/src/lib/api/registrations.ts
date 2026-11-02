import { invoke } from '@tauri-apps/api/core';
import type { Player } from '$lib/api/players';

export type EntryStatus = 'registered' | 'withdrawn';

export interface Entry { id: string; category_id: string; status: EntryStatus; members: (Pick<Player, 'id' | 'name' | 'club'> & { checked_in: boolean })[] }

export const listEntries = (categoryId: string) => invoke<Entry[]>('list_entries', { categoryId });

export const registerEntry = (tournamentId: string, categoryId: string, playerIds: string[]) =>
  invoke<Entry>('register_entry', { tournamentId, categoryId, playerIds });

export const registrationStarted = (tournamentId: string, categoryId: string) => invoke<boolean>('registration_started', { tournamentId, categoryId });

export const setEntryStatus = (tournamentId: string, entryId: string, status: EntryStatus) =>
  invoke<void>('set_entry_status', { tournamentId, entryId, status });

export const setPlayerAttendance = (tournamentId: string, playerId: string, checkedIn: boolean) =>
  invoke<void>('set_player_attendance', { tournamentId, playerId, checkedIn });

export const registerEntries = (tournamentId: string, categoryId: string, playerGroups: string[][]) => invoke<Entry[]>('register_entries', { tournamentId, categoryId, playerGroups });
