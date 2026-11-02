import { describe, expect, it } from 'vitest';
import type { CashLedger, CashRecord } from '$lib/api/cash';
import type { Category } from '$lib/api/categories';
import type { Entry } from '$lib/api/registrations';
import { cashBalance } from './cash-balance';

function fixture() {
  const category: Category = { id: 'category', name: 'Doubles', discipline: 'doubles', format: 'knockout', fee_minor: 501, completed: false, archived: false };
  const entry: Entry = { id: 'entry', category_id: category.id, status: 'registered', members: [
    { id: 'a', name: 'A', club: '', checked_in: false },
    { id: 'b', name: 'B', club: '', checked_in: false },
  ] };
  const record = (id: string, kind: CashRecord['kind'], amount_minor: number): CashRecord => ({ id, entry_id: entry.id, kind, amount_minor, note: '', created_at: '' });
  const ledger: CashLedger = { records: [record('fee', 'charge', 501)], allocations: [] };
  const balance = (id: string) => cashBalance(entry, category, ledger, id);
  function allocate(id: string, kind: 'payment' | 'refund', amount: number) {
    ledger.records.push(record(id, kind, amount));
    ledger.allocations.push({ record_id: id, player_id: 'a', amount_minor: amount });
  }
  return { category, entry, ledger, record, balance, allocate };
}

describe('cashBalance', () => {
  it('splits an odd doubles fee into estimates before arrival', () => {
    const { balance } = fixture();
    expect(balance('a')).toMatchObject({ remaining: 0, expected: 251 });
    expect(balance('b')).toMatchObject({ remaining: 0, expected: 250 });
  });

  it('collects only the arrived partner’s share and preserves the other estimate', () => {
    const { entry, balance, allocate } = fixture();
    entry.members[0].checked_in = true;
    expect(balance('a')).toMatchObject({ remaining: 251, expected: 0 });
    expect(balance('b')).toMatchObject({ remaining: 0, expected: 250 });
    allocate('paid', 'payment', 251);
    expect(balance('a')).toMatchObject({ remaining: 0, net: 251 });
    expect(balance('b').expected).toBe(250);
  });

  it('keeps payments refundable after withdrawal and restores debt on registration', () => {
    const { entry, balance, allocate } = fixture();
    entry.members[0].checked_in = true;
    allocate('paid', 'payment', 251);
    entry.status = 'withdrawn';
    expect(balance('b')).toMatchObject({ expected: 0, remaining: 0 });
    expect(balance('a')).toMatchObject({ eligible: false, net: 251 });
    allocate('refund', 'refund', 251);
    expect(balance('a')).toMatchObject({ remaining: 0, net: 0 });
    entry.status = 'registered';
    expect(balance('a').remaining).toBe(251);
    expect(balance('b').expected).toBe(250);
  });

  it('excludes archived categories from both debt and estimated income', () => {
    const { category, entry, balance } = fixture();
    entry.members[0].checked_in = true;
    category.archived = true;
    expect(balance('a').remaining).toBe(0);
    expect(balance('b').expected).toBe(0);
    category.archived = false;
    expect(balance('a').remaining).toBe(251);
    expect(balance('b').expected).toBe(250);
  });

  it('preserves integer splitting for historical payments without allocations', () => {
    const { entry, ledger, record, balance } = fixture();
    entry.members[0].checked_in = true;
    ledger.records.push(record('historical', 'payment', 101));
    expect(balance('a')).toMatchObject({ net: 51, remaining: 200 });
    expect(balance('b')).toMatchObject({ net: 50, expected: 200 });
    entry.members[0].checked_in = false;
    expect(balance('a')).toMatchObject({ remaining: 0, expected: 200 });
  });
});
