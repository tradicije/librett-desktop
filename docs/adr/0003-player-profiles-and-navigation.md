# ADR 0003: Shared player administration and screen navigation

Status: accepted.

The mode chooser is a standalone screen. Within the tournament module, a Players
tab owns the shared local directory; a tournament owns only its category entries.
Stable player IDs are retained during edits. Entries store independent name/club
snapshots, rather than contact details or photographs.

Profile fields are optional except the name: club, birth year, city, country,
email, phone, notes, and photo. Birth years range from 1900 to the current year.
Profile text is trimmed and bounded. The UI decodes JPEG/PNG/WebP files up to
10 MiB and 6000 pixels per dimension, reduces them to at most 512 pixels per
side, and encodes JPEG. The backend bounds the stored data URI to 350,000 bytes,
validates base64 and JPEG boundary signatures, and rejects external photo URLs.
Photos reside in SQLite with the profile, so their persistence does not depend
on the original file or an online service. File reading uses data URLs compatible
with the desktop image CSP. Full image decoding occurs in the UI.

Version 3 migrates transactionally, preserving older records. Before migrating
an existing version 1 or 2 database, VACUUM INTO creates a consistent
`pre-v3-<uuid>.sqlite` backup beside the database. Backup failure blocks migration;
new databases do not need a backup and current databases are not backed up again.

Back/Forward traverses screens, with forward history discarded after a new
navigation branch. This is navigation history, not database undo. Home returns
from a tournament or the Players tab to the module overview, and from the module
overview to mode selection. Navigation is disabled while a save, photo read, or
registration is pending. Forms may be reset on navigation; saved data persists.

The local operator administers the directory. There is no authentication or
role-enforcement boundary in the current desktop app. Future companion access
must define separate authenticated permissions before exposing administration.

The Players tab displays only the directory and row actions. Add/Edit opens
separate create/edit routes, included in screen history. Editors load a player
by stable ID and return to the list after saving or cancellation. Revisiting a
deleted profile displays a missing-player message and no editable form.

Deleting an unused profile requires UI confirmation and removes it from SQLite.
A conditional DELETE refuses players referenced by entry_members with a typed
player_in_use error. Existing registrations and snapshots remain unchanged;
this is not a withdrawal action. Missing IDs report not_found.
