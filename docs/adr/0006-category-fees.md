# ADR 0006: Automatic category entry fees

Status: accepted.

The current migration target is v8 with pre-v8 backups; see ADR 0008.

Categories define an RSD fee in integer minor units, zero meaning free entry.
Creating a category requires an explicit fee in the UI. Each successful new
registration adds one immutable charge for that category's fee in the same
transaction as the entry and its members. Doubles uses one fee per pair.
A zero fee creates no cash event. Duplicate or failed registration rolls back
both membership and charge. Automatic charges do not represent payments.

Schema v6 adds a fee defaulting to zero for existing categories. Existing
registrations and manual cash records are not retroactively charged. Category
fee editing and per-person doubles tariffs are later features. Historical
charges retain their amounts. Upgrades create a pre-v6 snapshot.
