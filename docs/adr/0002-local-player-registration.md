# ADR 0002: Local players and category entries

Status: accepted.

A player profile belongs to the workspace. Entries reference one player for
singles or two distinct players for doubles. A player can enter different
categories but cannot appear in two entries in the same category. Identical
names remain distinct; names are not identity keys.

Application use cases validate tournament/category ownership, player existence,
member count, and duplicate participation. SQLite foreign keys and a unique
category/player constraint protect storage. Entries and members are saved
transactionally, preserving name/club snapshots independently of profile updates.

Schema version 2 creates players and registrations. Opening a version 1 database
first creates a consistent `VACUUM INTO` snapshot beside it named
`pre-v2-…sqlite`, then migrates transactionally. Backup failure blocks migration.
Backup retention and the restore UI are not implemented.

Profile editing is implemented in ADR 0003. Withdrawals, attendance, payments,
external directories, and draws are future use cases. Registration implies
neither attendance nor payment. SQLite accessibility does not replace the
required independent archival format for results.

Schema version 3 supersedes the migration target and backup naming described
above; see [ADR 0003](0003-player-profiles-and-navigation.md).
