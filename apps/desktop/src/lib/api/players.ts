import { invoke } from '@tauri-apps/api/core';

export interface PlayerProfile { birth_year: number | null; city: string; country: string; email: string; phone: string; notes: string; photo: string | null }

export interface Player extends PlayerProfile { id: string; name: string; club: string }

export const listPlayers = () => invoke<Player[]>('list_players');

export const savePlayerChecked = (requestId: string, playerId: string, name: string, club: string, profile: PlayerProfile, expected: Player | null) => invoke<Player>('save_player_checked', { requestId, playerId, name, club, profile, expected });

export const getPlayer = (id: string) => invoke<Player>('get_player', { id });

export const deletePlayer = (id: string) => invoke<void>('delete_player', { id });
