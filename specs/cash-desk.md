# Entry cash desk

1. Charge 1,000.01 RSD, discount 100 RSD, receive 250.50 RSD: due is
   900.01 RSD, net received is 250.50 RSD, outstanding is 649.51 RSD.
2. Payments may exceed the fee. Display credit independently from other entries'
   outstanding balances; tournament totals must not offset those two quantities.
3. Discounts above accumulated charges and refunds above received payments fail
   without a write. Zero, negative, out-of-bound amounts fail; notes are optional and limited to 500 characters.
4. Repeating the same request UUID and content writes once; conflicting reuse fails.
   A write that succeeds before an IPC/refresh failure can be retried safely.
5. Another tournament cannot access or write this entry's account.
6. Withdrawn registrations retain their records; check-in does not modify money.
7. Events survive restart with stable IDs, optional note and timestamp. SQL updates and
   deletions are rejected by triggers. Corrections use additional events.
8. A v4 database receives one readable pre-v8 backup before migration. Existing
   registrations and player data survive, and an unchanged reopen makes no backup.
9. The UI supports Serbian/English and all themes. Busy writes lock navigation;
   uncertain failures retain the request and lock form inputs until retry confirms.

Current scope: RSD entry accounts with automatic category fees, doubles charged as a pair. Payments
allocated across entries, per-person doubles tariffs and financial
export are future increments. Cash desk is an organizer's ledger, not a fiscal or
accounting system.

## Automatic category fees (ADR 0006)

- Configure 1,000.01 RSD at category creation; each new singles registration has
  one 100001-minor-unit charge and no payment. Doubles creates one charge per pair.
- A zero-fee category creates no cash event. Invalid fees cannot create a category.
- Duplicate participation creates neither another entry nor another charge.
- If inserting the automatic charge fails, the entry and members roll back.
- Migration from v5 defaults existing category fees to zero without changing
  existing manual charges or charging historical entries again.
- Decimal input accepts comma/dot with at most two decimal places; reject exponent
  notation, negative amounts, overflow and extra decimals before invoking backend.
