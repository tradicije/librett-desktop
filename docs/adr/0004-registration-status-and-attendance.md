# ADR 0004: Registration status and tournament attendance

Status: accepted.

Registration, attendance, and payment are independent. The next desktop slice
adds registration withdrawal/restoration and tournament-wide player attendance.
Cash-desk accounting follows separately: dues, discounts/waivers, partial payments,
refunds, currency in integer minor units, and allocation across categories.

An entry is registered or withdrawn. Withdrawal retains its ID, member snapshots,
and category membership; restoration reuses the same entry. Membership remains
reserved for withdrawn entries, so withdrawing is not a way to register a new
pair or bypass duplicate-player constraints. Future lineup corrections require
an explicit use case. Existing player-deletion protection remains in force.

Attendance belongs to a player within a tournament, not to a category or pair.
A doubles entry shows both players separately; a player entered in multiple
categories shares one attendance state. Absence does not withdraw registrations,
withdrawal does not erase attendance, and neither state infers payment.

Mutations validate tournament ownership. Attendance can only be recorded for a
player with a registration in that tournament. Schema version 4 defaults existing
entries to registered and players to not checked in, with a consistent pre-v4
backup before migrating existing databases. Full operator audit logs, draw locks,
fee accounting, and match participation rules remain future work.
