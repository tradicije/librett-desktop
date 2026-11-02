import { invoke } from '@tauri-apps/api/core';

export type CashKind = 'charge' | 'discount' | 'payment' | 'refund';

export interface CashRecord { id: string; entry_id: string; kind: CashKind; amount_minor: number; note: string; created_at: string }

export const listCash = (tournamentId: string) => invoke<CashRecord[]>('list_cash', { tournamentId });

export const recordCash = (tournamentId: string, record: CashRecord) => invoke<void>('record_cash', { tournamentId, record });

export interface CashAllocation { record_id: string; player_id: string; amount_minor: number }

export interface CashLedger { records: CashRecord[]; allocations: CashAllocation[] }

export const cashLedger = (tournamentId: string) => invoke<CashLedger>('cash_ledger', { tournamentId });

export interface ExpectedCashAmount { entry_id: string; amount_minor: number }

export const settlePlayerCash = (requestId: string, tournamentId: string, playerId: string, entryIds: string[], paid: boolean, expected: ExpectedCashAmount[]) => invoke<void>('settle_player_cash', { requestId, tournamentId, playerId, entryIds, paid, expected });
