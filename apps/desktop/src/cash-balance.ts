import type { CashLedger, Category, Entry } from './api';

function share(amount: number, position: number, count: number) {
  return Math.trunc(amount / count) + (position < Math.abs(amount % count) ? Math.sign(amount) : 0);
}

/** Historical charges stay in the ledger; current status determines collectable debt. */
export function cashBalance(entry: Entry, category: Category | undefined, ledger: CashLedger, playerId: string) {
  let due = 0, net = 0, allocatedNet = 0;
  for (const record of ledger.records.filter(r => r.entry_id === entry.id)) {
    if (record.kind === 'charge') due += record.amount_minor;
    if (record.kind === 'discount') due -= record.amount_minor;
    const sign = record.kind === 'payment' ? 1 : record.kind === 'refund' ? -1 : 0;
    const allocations = ledger.allocations.filter(a => a.record_id === record.id);
    if (allocations.length) allocatedNet += sign * allocations.filter(a => a.player_id === playerId).reduce((sum, a) => sum + a.amount_minor, 0);
    else net += sign * record.amount_minor;
  }
  const position = entry.members.findIndex(m => m.id === playerId);
  due = share(due, position, entry.members.length);
  net = share(net, position, entry.members.length) + allocatedNet;
  const active = entry.status === 'registered' && !!category && !category.archived;
  const arrived = !!entry.members.find(m => m.id === playerId)?.checked_in;
  const eligible = active && arrived;
  const unpaid = active ? Math.max(0, due - net) : 0;
  return { due, net, active, arrived, eligible, remaining: eligible ? unpaid : 0, expected: active && !arrived ? unpaid : 0 };
}
