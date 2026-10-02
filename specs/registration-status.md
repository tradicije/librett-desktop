# Registration status and attendance scenarios

1. New and migrated entries are registered; all players initially have unconfirmed
   arrival. Migration retains IDs, player details, snapshots, and memberships.
2. Check-in applies to a player within a tournament, across all categories;
   the same player in another tournament is unchanged. Doubles members are separate.
3. Undo check-in leaves registrations unchanged. Neither attendance nor withdrawal
   records or infers any payment.
4. Withdrawal retains the entry, snapshots, members, and arrival state. Restore
   uses the same entry. Withdrawn membership remains reserved; use Restore rather
   than trying to duplicate that registration with a new pair.
5. Unknown entries, wrong tournament ownership, and attendance without a
   registration in the tournament fail without modifying records.
6. Reopening preserves attendance and status. Repeated writes of the same state
   are valid. A version 3 migration produces one readable pre-v6 backup.
7. Filter all/active/withdrawn entries. Totals describe the selected category's
   active entries, withdrawn entries, and checked-in players in active entries.
8. Saves disable navigation and mutation controls. Failed saves leave the previous
   displayed state intact. All labels/statuses/errors support Serbian and English.
