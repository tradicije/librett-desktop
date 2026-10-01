# Local players and registration scenarios

1. Trim names and optional clubs, preserve Unicode, and enforce a 120-scalar
   limit. Blank names fail; identical names are allowed with distinct IDs.
2. Search by name or club. UI labels and errors support both languages.
3. Singles requires one player; doubles requires two distinct existing players.
4. Players may enter different categories and tournaments. Duplicate participation
   in one category, even with another doubles partner, fails.
5. Invalid tournament/category ownership and unknown player IDs fail.
6. A failed pair insertion leaves no partial entry or membership records.
7. Entries retain name/club snapshots when source profiles change.
8. Reopening preserves profiles and registrations. Migration retains existing
   tournaments and creates a readable pre-migration backup.
9. Registration does not infer attendance or payment.
