# Changelog

Notable changes to LibreTT are recorded here in English.

## Unreleased

### Added

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
draws, players, payments, results, archival exports, or release installers.

### Validation

- Frontend type/accessibility checks and production build passed.
- Rust tests, native desktop build, and end-to-end SQLite operation remain
  unverified until Rust and native system dependencies are available.
- Windows, macOS, and Linux desktop qualification remains pending.
