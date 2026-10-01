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

10. Manage profiles in the Players tab: birth year, location, contact details,
    notes, and photos are optional and persist across restarts; edits keep the ID.
11. A tournament registers existing directory players, with search and independent
    name/club snapshots. Editing profiles does not rewrite past registration names.
12. Version 2 migration preserves players and registrations, creates one readable
    pre-v4 backup, and gives old profiles empty optional details.
13. Reject invalid birth years, oversized profile fields, unsupported or broken
    photo uploads and external photo URLs. Photos remain available after moving
    or deleting the source image file.
14. Mode selection has no sidebar. Back/Forward traverses screen history; Home
    returns first to the module overview, then to mode selection. A new navigation
    branch discards forward history. Navigation cannot interrupt pending writes.

15. The Players tab shows the directory without an inline form. Add opens a blank
    dedicated editor; Edit opens the selected profile on a dedicated screen.
    Saving/cancelling returns to the list, and both editor routes support history.
16. Delete cancellation makes no storage call. Confirmation deletes an unused
    player; attempting to delete a registered player reports an error and retains
    both the player and registrations. Missing IDs report not_found. Reopening
    preserves these outcomes. Revisiting a deleted profile never opens a blank
    form that could recreate it accidentally.
