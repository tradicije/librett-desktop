# ADR 0001: Desktop foundation

Status: accepted for the initial development slice; platform qualification pending.

LibreTT uses Tauri 2, Rust, Svelte/TypeScript, and SQLite for the first desktop
implementation. Domain and application crates are independent of Tauri and SQL.
Only backend adapters access storage. UUIDs are generated locally. Enumerated
states and errors use stable, untranslated transport values.

The first slice creates and lists tournaments and adds singles/doubles categories
with independently selected knockout or groups-then-knockout formats. It does
not implement draws, players, payments, match results, or leagues yet.

SQLite schema changes are versioned and transactional. A database newer than
the application supports is rejected. The desktop database belongs in the OS
application data directory, outside the checkout. No browser storage substitutes
for SQLite; the browser-only preview disables persistence controls explicitly.

The interface starts in Serbian, supports English, and stores only its language
preference in localStorage. Future competition and export specifications must
define snapshots, revisions, backups, and rule versions before those features
are implemented. This first migration is for new development databases only;
backup and upgrades of populated databases require their own implementation.

Installers, printing, second-screen support, offline installation, Windows,
macOS, and Linux runtime behavior still require platform qualification. This
decision does not claim that these operational checks have passed.

## Community data continuity

Competition records belong to the community producing them. Their lifetime must
not depend on LibreTT maintenance, executables, an account, or a hosted service.
Before results are shipped, specify versioned open source-record exports,
human-readable archival output, and a documented way to recover and interpret
records independently of the application. Export stable IDs, rule versions,
confirmed results, correction history, and the metadata needed to interpret
them. Public archives exclude private player/contact/payment data by default.
Validate continuity by reading an exported archive without LibreTT. SQLite
storage alone is not the finished archival or interoperability solution.
