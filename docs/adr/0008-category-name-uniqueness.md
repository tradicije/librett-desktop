# ADR 0008: Category names are unique per discipline

Status: accepted.

An organizer may create Singles / Apsolutna and Doubles / Apsolutna within the
same tournament. Duplicate validation uses trimmed, Unicode-lowercase name plus
discipline and tournament. Formats do not create a separate uniqueness scope.
Archived categories retain identity and reserve their name within their discipline.

SQLite schema v8 replaces the category table's tournament/name uniqueness with
(tournament_id, discipline, name_key), preserving IDs, fees, archive flags and
ordering. Foreign keys are disabled outside the rebuild transaction, all existing
references are checked before commit, and enforcement is restored afterward.
Existing databases receive a consistent pre-v8 backup. Migration tests exercise
entries, historical snapshots, attendance, financial records and immutable triggers.
Cash labels and deletion confirmations include discipline to distinguish names.
