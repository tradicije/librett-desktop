# ADR 0007: Category workspace and batch registration

Status: accepted.

The current migration target is v8 with pre-v8 backups; see ADR 0008.

Categories open a dedicated workspace with Registrations, Draw, Groups (when
applicable) and Bracket tabs. Competition engines remain unimplemented; their
tabs say so explicitly. Registration operates on the selected category only.
Singles accepts checked players; doubles queues explicitly chosen pairs.
A batch (up to 1000 entries) validates discipline, ownership, existing membership
and duplicate players, then writes every entry, member and fee in one transaction.
Failure leaves no partial registrations or charges.

Deleting an empty category removes it permanently. A category with any entry is
archived: it disappears from active tournament views and rejects new entries,
but its entries, attendance and cash history remain readable. Cash desk includes
archived accounts. The confirmation explains this before the operation. Schema
v7 adds archived=false with a pre-v7 backup; historical IDs remain stable.

Cash notes are optional (trimmed, at most 500 characters). Selecting an account
prefills its outstanding balance, considering payments, discounts and refunds,
not just the original category fee. Paid/free accounts default to no amount.
The organizer may override the amount for partial payments. Switching transaction
kind clears non-payment amounts; switching accounts clears old notes. Uncertain
writes retain the original request and lock inputs until confirmed.
