# 0005 — Entry cash ledger

Status: Accepted

The first usable cash desk records entry charges, discounts, payments and refunds
as append-only events. Amounts are integer minor units, bounded to 1,000,000,000
per event and account balance component; no floating point enters persistence.
This increment uses RSD (100 minor units per dinar) explicitly. An entry is the
account: a doubles entry is one pair account. Individual doubles tariffs,
automatic category fees, other currencies and payments allocated across several
entries remain later increments, not implied capabilities of this screen.

Discounts cannot exceed charges; refunds cannot exceed received payments.
Overpayments are allowed and displayed as credit. Attendance and withdrawal do
not change financial records. A withdrawal requires an explicit discount and/or
refund if the organizer wants one. Corrections append compensating events with
a reason; neither deletion nor editing of recorded events is exposed.

The repository validates scoped entry ownership and all balances in one SQLite
transaction before insertion. Client request UUIDs make identical retries
idempotent; reusing a request ID with different content is rejected. The UI
keeps the same request across uncertain storage failures until retry succeeds.
No fiscal receipts, accounting compliance or multi-user author attribution are
claimed. Schema v5 upgrades create a pre-v5 SQLite snapshot before mutation.
