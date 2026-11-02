import { invoke } from '@tauri-apps/api/core';
import type { CategoryResults } from '$lib/api/results';
import type { CompetitionState } from '$lib/api/draw';
import type { CategoryRules } from '$lib/api/categories';

export const saveReport=(kind:string,format:'csv'|'html',content:string)=>invoke<string>('save_report',{kind,format,content});

export const openPrintReport=(content:string)=>invoke<void>('open_print_report',{content});

export const getPrintReport=(token:string)=>invoke<string>('get_print_report',{token});

export const printReport=()=>invoke<void>('print_report');

export const getCategoryReport=(tournamentId:string,categoryId:string)=>invoke<{results:CategoryResults;competition:CompetitionState;rules:CategoryRules}>('get_category_report',{tournamentId,categoryId});
