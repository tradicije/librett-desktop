import { invoke } from '@tauri-apps/api/core';

export interface HistoryItem { id:number; occurred_at:string; kind:string; action:string; entity_type:string; entity_id:string; entity_name:string; tournament_id:string|null; tournament_name:string|null; category_id:string|null; category_name:string|null; before_data:Record<string,unknown>|null; after_data:Record<string,unknown>|null; historical:boolean }

export interface HistoryOption {id:string;name:string;tournament_id:string|null}

export interface HistoryPage {items:HistoryItem[];has_more:boolean;tournaments:HistoryOption[];categories:HistoryOption[]}

export const getActionHistory = (kinds:string[],tournamentId:string|null,categoryId:string|null,action:string|null,beforeAt:string|null,beforeId:number|null) => invoke<HistoryPage>('get_action_history',{kinds,tournamentId,categoryId,action,beforeAt,beforeId});
