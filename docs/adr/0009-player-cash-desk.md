# ADR 0009: Player-oriented tournament cash desk

Status: accepted.

The cash desk becomes a searchable player list, with category checkboxes and a
Pay button per player. Summary values are remaining outstanding balance, net
received, and the number of distinct active registered players. They are computed
from financial records, independently of the search or category selection.

Category selections start empty. Clicking Pay records actual payments for selected
categories' remaining balances, clears selection and marks each settled category Paid.
Paid categories cannot be selected again. Existing
partial payments, discounts, refunds and overpayments remain part of the ledger.
A request UUID makes the entire multi-category settlement atomic and idempotent.
Amounts are calculated from persisted records inside an immediate transaction,
not accepted from UI totals. Financial records remain immutable.

The previous manual account form and selected-entry history are removed from this
screen. The underlying ledger remains available for future reporting and exports.

Doubles fees are split equally between the two entry members, as requested by the
project author (500 RSD per pair = 250 RSD each). A player-specific allocation
links each new payment/refund to its member. Existing unallocated pair payments
are split as historical balances; the extra minor unit goes to the first member
for odd amounts so totals stay exact. Category columns form a semantic table,
with horizontal scrolling in smaller windows. Selection affects settlement only,
not a player's displayed total or tournament summary.

Schema v9 adds immutable allocation and settlement records with pre-v9 backups.
The ledger still supports partial payments and discounts in existing data and
backend operations; this simplified screen only settles selected remaining balances. It does not provide manual adjustment entry.
