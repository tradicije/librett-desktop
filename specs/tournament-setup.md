# Tournament setup acceptance scenarios

1. Creating a tournament trims surrounding whitespace and preserves Serbian
   letters. A blank name or a name longer than 120 Unicode scalar values is
   rejected without creating a record.
2. A new tournament has a stable UUID and no categories. Multiple tournaments
   can be created; no product limit is imposed.
3. A tournament can contain a singles category using groups then knockout and
   a doubles category using direct knockout. Category IDs remain distinct.
4. Category names must be unique within their tournament, using trimmed,
   Unicode-lowercased names. The same name can occur in another tournament.
5. Adding a category to a missing tournament fails. Failed validation leaves
   existing tournament data unchanged.
6. Closing and reopening the SQLite repository preserves tournaments, category
   order, disciplines, and formats. Existing schema version 1 is not recreated.
7. UI labels and backend error messages are available in Serbian and English.
   A language change does not translate user-entered names or stored enums.
8. Browser-only preview clearly indicates that desktop persistence is absent;
   it does not create fake tournaments or silently use localStorage as a database.
