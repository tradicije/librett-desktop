# Player cash matrix

- Summary: 40,000 RSD charged and 35,000 RSD received shows 5,000 RSD outstanding,
  35,000 RSD net received. Registered counts unique active players, not entries.
- One player has one row with name, club, one column per category, total outstanding
  and a Pay button. Search covers name, club and category; it does not filter summary totals.
- Category checkboxes select payment scope. Missing registrations have no checkbox.
  All start unchecked. Selection does not remove debt or modify registration.
  Clearing all disables Pay; fully paid categories show Paid and cannot be selected.
- A 500 RSD doubles account charges 250 RSD to each player. Paying one preserves
  the other's balance. Pair totals are counted once in the tournament summary.
- Existing unallocated pair balances split deterministically; an odd minor unit
  belongs to the first member. New allocations identify the actual paying member.
- Pay writes selected remaining balances in one transaction, including all member
  allocations. Late failure rolls back every payment; unchanged UUID retries write once.
- A refreshed or already-paid selection adds no duplicate payment. Partial previous
  payments and discounts reduce the amount collected automatically.
- Successful payment clears the selection and marks only settled categories Paid.
  Reloading preserves these statuses from the ledger. No Paid checkbox or refund
  toggle is exposed; financial records are never deleted or rewritten.
- The obsolete account form and selected-entry history are absent. History remains
  readable through the ledger backend. Manual adjustments are not exposed here.
- New schema v9 preserves historical records and creates one pre-v9 backup before
  migration. Allocation/request updates and deletes are rejected.
- All category columns remain separate in smaller windows via horizontal table
  scrolling, with row/column headers, keyboard controls and both language/theme modes.
- Hover states never resize controls; disabled controls do not appear actionable.
