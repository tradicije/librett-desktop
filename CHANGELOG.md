# Changelog

Notable changes to LibreTT are recorded here in English.

## Unreleased

### Added

- Documented a per-run Linux WebKitGTK workaround for the reported Wayland
  protocol error; graphical startup confirmation remains pending.
- Local player directory with optional club names and name/club search.
- Singles/doubles registration using existing players, duplicate participation
  protection, and historical name/club snapshots.
- Transactional entries and schema version 2 migration with a consistent
  pre-migration backup for existing version 1 databases.
- Player-registration scenarios, ADR 0002, regression tests, a development
  application icon, and the Cargo dependency lock file.
- Initial Tauri/Rust desktop adapter, Svelte/TypeScript bilingual interface,
  platform-independent domain/application crates, and SQLite persistence.
- Tournament creation/listing and singles/doubles categories with independently
  selected knockout or groups-then-knockout settings.
- Versioned transactional initial schema, stable local UUIDs, name validation,
  duplicate-category checks, and core persistence regression tests.
- Development instructions in English and Serbian, repository ignores, and
  tournament setup acceptance scenarios.
- Initial desktop application plan for Linux, macOS, and Windows.
- Tournament and league module boundaries, with tournaments as the first focus.
- Planned tournament categories, singles and doubles, registrations, attendance,
  fee tracking, manual and automatic draws, and category-specific formats.
- Future companion and stoni.rs player-directory integration requirements.
- English and Serbian README and contribution guides.
- GNU AGPL version 3 license text and AGPL-3.0-or-later project licensing notice.

### Established

- Community ownership and long-term accessibility of competition records as a
  core principle, with independent archival export required before results ship.
- LibreTT as the application name and umbrella project identity.
- Aleksa Dimitrijević as the author.
- Serbian and English as the initial product languages.
- Tauri 2, Rust, TypeScript/Svelte, and SQLite as the proposed stack, pending
  cross-platform validation.

No application release exists yet. The initial implementation does not include
draws, attendance, payments, results, archival exports, or release installers.

### Validation

- Frontend type/accessibility checks and production build passed.
- Seven Rust domain/storage tests passed, including database reopening,
  rollback, historical snapshots, and migration backup verification.
- Core Clippy and Linux native desktop compilation checks passed. Graphical
  desktop interaction remains a manual check.
- Windows, macOS, and Linux desktop qualification remains pending.
