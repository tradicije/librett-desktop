# Changelog

Notable changes to LibreTT are recorded here in English.

## Unreleased

### Fixed

- Keep the desktop sidebar anchored to the viewport while main content scrolls.

- Set the Linux GLib program name before GTK startup so the Wayland window ID
  matches `org.librett.desktop` and KWin can resolve the title-bar icon.
- Removed the pending leagues tab from navigation during tournament development.

### Added

- English and Serbian security policies covering vulnerability reporting,
  supported development versions, local data, and current trust boundaries.
- Tournament-wide per-player attendance, independent doubles check-in, and
  registration withdrawal/restoration with status filtering and category totals.
- SQLite schema version 4 with pre-migration backups and regression tests for
  attendance ownership, cross-category consistency, and registration preservation.
- Dark LibreTT logo at the top of both README files.

- List-only Players tab with icon-labeled Add, Edit, and Delete actions and
  dedicated create/edit screens integrated with Back/Forward navigation.
- Confirmed deletion of unused player profiles, with existing registrations
  protected and missing/deleted profiles handled when revisiting editor history.

- Shared Players tab with editable birth year, location, contact details, notes,
  and offline profile photographs, independent of tournament registration.
- SQLite schema version 3 with transactional profile migration and consistent
  pre-v3 backups for existing databases, preserving registration snapshots.
- Back/Forward screen history and contextual Home navigation; the mode-selection
  screen now has no sidebar and module navigation has no dashboard item.

- Initial bilingual mode-selection dashboard with tournament access and a disabled
  leagues card, plus navigation back to the dashboard from the tournament module.
- Enabled GTK application identity and added Linux development icon registration
  so Wayland desktops can match running windows to the LibreTT icon.
- Locally bundled Libre Franklin variable font for offline typography.
- Bilingual README disclosure of AI-assisted programming and human authorship,
  review, and verification.
- Theme-colored selection menus with keyboard navigation and typeahead, replacing
  system-rendered dropdown popups; consistent icon/text alignment.
- Shared light/dark design tokens using the LibreTT palette across the interface.
- Persisted Light/Dark/System theme selection, live system-theme tracking, and
  theme-specific LibreTT logo variants.
- Native desktop icons generated from the supplied LibreTT app icon, replacing
  the temporary development artwork.
- Theme-aware Tabler Icons for navigation, categories, player actions, search,
  preferences, and tournament controls, bundled locally under their MIT license.
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

- Browser verification passed for both palettes and logos, Tabler rendering,
  theme persistence, live system-theme changes, explicit theme overrides,
  localized preference labels, and absence of JavaScript errors.
- Linux desktop compilation check passed with the supplied native app icon.
- Frontend type/accessibility checks and production build passed.
- Seven Rust domain/storage tests passed, including database reopening,
  rollback, historical snapshots, and migration backup verification.
- Core Clippy and Linux native desktop compilation checks passed. Graphical
  desktop interaction remains a manual check.
- Windows, macOS, and Linux desktop qualification remains pending.
