# Changelog

Notable changes to LibreTT are recorded here in English.

## Unreleased

### Changed

- Simplified draw copy and toolbar actions; removed the duplicate bracket heading
  and shortened the pending-qualification notice.

- Removed duplicate club subtitles and the outer frame from the knockout bracket;
  match cards are more compact and winning rows have a subtle highlight.

- Group tables now rebuild wins, sets and points after results, using the tied-player
  mini-table and configured ratios. Completed groups fill the knockout bracket;
  organizers can override the final order or restore automatic ranking.
- Group result/ranking corrections require confirmation before clearing affected
  knockout results. Exact unresolved ties keep qualification pending.
- SQLite schema 15 adds guarded manual group orders with pre-v15 backups.
- Knockout rounds use competition names instead of numbered group rounds.

- Matches now support validated set scores, retirement, walkover and result
  corrections. Knockout winners advance through byes and saved results; changing
  a winner requires confirmation before clearing dependent results. Group
  matches are paged by round.
- SQLite schema 14 preserves immutable result revisions and idempotent write
  receipts, with backups before upgrades.
- Player names show a three-character club suffix in smaller muted type across
  lists, selectors, registrations, attendance, groups, seeds, draws and cash.
- Fixed score field updates and live set totals; completed sets have an indicator
  and save calculates the winner directly from submitted points.
- The sidebar's local-work label stays on one line.

- Added a shared image crop dialog for player photos (1:1) and tournament covers
  (16:9), with drag positioning, zoom, keyboard-accessible position sliders,
  reset and explicit apply/cancel. Crop export preserves image proportions and
  fixes cover distortion caused by the previous crop height calculation.

- Sidebar logos and collapse controls now share the toolbar's center line in
  expanded and collapsed desktop layouts. Expanded branding uses the same
  horizontal inset as sidebar navigation content.
- Tournament Settings is the leftmost tab, matching category navigation.
- Tournament cards are clickable throughout, with 16:9 covers, metadata rows and
  an explicit open button. Tournament Settings supports changing the name and
  cover with optimistic conflict protection and safe retries.

- Tournament directory now has Add Tournament and individual cards in up to three
  columns, with optional 16:9 covers, active registered player/category counts and
  a dedicated editor. Metadata uses separate rows and each card has an open button. Cover images use a user-selected crop exported at 1024×576.
- Round-robin pairings load on expansion, one round and up to 32 pairs at a time.
  Manual arrangement dropdown options are shared across slots.
- SQLite schema 12 adds guarded player-write receipts, expected settlement
  amounts; schema 13 adds tournament covers, with pre-v13 backups for existing databases.

- Redesigned seed and arrangement settings with numbered seed rows, icon actions,
  shared dropdowns, clear draft status and grouped generation/save controls.
  Manual group slots use up to three columns; knockout opponents appear in pairs.

- Category Settings now contains only rules, seeds and arrangement options;
  participant lists and registration forms remain in the Registrations tab.
- Removed the outdated planned WordPress repository rename from both README files.

- Simplified category Settings into one panel with flat rules and
  arrangement sections. Category creation/editing now shares player editor form
  groups, field spacing, dropdowns and action styling.

- Added an Info button at the right of the title bar, opening a themed About
  dialog with the logo, description, version, author and bundled full AGPL license.

- Breadcrumbs navigate to parent screens and support opening destinations in a
  new tab. Sidebar and navigation toolbar sit outside the content scroll region
  so elastic scrolling does not move them.

- README headers include the LibreTT title, a short description and badges for
  the AGPL license, Tauri 2, Svelte 5 and offline support. Logos are centered
  and set to 360px width in both languages.

- Draw now shows only the knockout bracket. Categories using groups have a
  separate Groups tab with participant cards and round-robin pairings arranged
  in up to three columns, reducing to two or one on smaller windows.

- Categories now use a full-width list with Add Category opening a dedicated editor;
  Category cards and Edit/Delete actions share the player directory's layout, styles and icons.
  Registrations are accessed inside each category rather than at tournament root.
- Category rules configure groups, qualifiers, match length, scoring targets and
  tied-player mini-table/set-ratio/point-ratio order. Settings contains rules and
  ordered seeds and automatic/manual arrangements; Groups shows round-robin
  pairings, while Draw shows a complete horizontally scrollable knockout bracket.
  Group qualifiers remain placeholders until match results are implemented.
- Working tab labels show the immediate parent and page, retaining full-context tooltips.
- SQLite schema 11 stores versioned category rules, preserves existing group settings
  and creates pre-v11 migration backups. Category metadata/rules save atomically;
  fee edits affect future registrations, while used categories retain discipline/format.

- Added persistent tournament-level tabs for Overview, Categories
  and Cash desk. Draw, Matches and Results are category-level tabs; Matches and
  Results are clearly marked as in development.

- Replaced the tournament cash desk's account/form/history blocks with a searchable
  player table, separate category columns, initially empty selection checkboxes and Collect/Refund buttons. Only selected categories
  are settled; each paid category shows its own status. Confirmed refunds return only selected player/category payments
  and preserve financial history, including withdrawn registrations.
- Cash summaries show outstanding balance after payments, net received and unique
  active registered players; doubles fees split equally between both members.
- Unified hover, active and keyboard-focus styling across controls, navigation,
  category rows, checkboxes and lists in both themes.

- Cash notes are optional. Account selection prefills the remaining payable balance
  and still supports partial payments and idempotent retries after uncertain writes.
- Financial account labels and category deletion prompts include discipline to
  distinguish singles/doubles categories sharing a name.

- Redesigned desktop screens with a compact sidebar, persistent context toolbar,
  restrained typography, consistent controls and denser competition/player lists.
- Grouped player editing fields, added initial avatars and a search toolbar,
  clarified registration summaries and highlighted selected cash accounts.
- Consolidated theme styles while retaining the LibreTT palette, local font/icons,
  keyboard focus, responsive layouts and reduced-motion support.

- Restructured both README files around the project's philosophy, current features
  and roadmap, removed the redundant title beneath the logo, and expanded the
  commitment to free access, community-owned data and long-term sporting history.

### Fixed

- Confirm before discarding dirty forms/drafts through navigation, history, editor
  cancellation, explicit draw reload, window closing or Quit.
- Reject player edits against stale profile snapshots; matching request retries
  cannot create duplicate players or replay updates. Editors can reload after conflicts.
- Reject cash settlements whose amounts changed after display/confirmation;
  require refreshed confirmation and prevent mixing unallocated payment/refund
  writes with player allocations. Preserve existing financial records.
- Reject empty-versus-empty pairs in complete manual knockout arrangements.
- Check image dimensions before browser decoding and fully decode bounded JPEG
  uploads in the backend, preventing oversized or malformed images being stored.

- Isolate workspace scrolling below the title bar so macOS elastic overscroll
  cannot move the tabs. Preserve each workspace's scroll position using its own
  scroll container instead of document scrolling.

- Keep the workspace tab bar fixed to the viewport through the end of long
  pages and subtract its height from workspace/sidebar height to avoid an
  unnecessary vertical scrollbar on short pages.
- Overlay the macOS title bar, retain native window controls and center their
  actual AppKit frames on the tab row, including after resize/focus changes.

- Connect screen navigation to webview history, including mouse back/forward
  buttons. Enable native macOS trackpad navigation gestures and restore the
  current screen when native navigation occurs during a pending write.

- Keep the desktop sidebar anchored to the viewport while main content scrolls.

- Set the Linux GLib program name before GTK startup so the Wayland window ID
  matches `org.librett.desktop` and KWin can resolve the title-bar icon.
- Removed the pending leagues tab from navigation during tournament development.

### Added

- Collapsible sidebar shared across working tabs and saved as a local preference.
  The compact 68px menu shows navigation icons with labels/tooltips and uses the
  supplied dark/light square brand icons; expanded mode keeps the full logo with
  the collapse control beside it with additional spacing. Clicking the compact brand icon expands the
  menu; brand images do not navigate to Home.

- Independent workspace tabs that retain screen state, searches, selections,
  draft edits, focus, scroll position and their own Back/Forward history.
- A fixed top bar with a pinned Home button, uniformly 180px-wide tabs, short page
  labels that fade at the right only when overflowing, and full-context
  tooltips, tab closing and right-click Open in new tab. Cmd/Ctrl-click and
  middle-click also open navigation destinations in new tabs; Cmd/Ctrl+T
  opens Home to choose a workspace, Cmd/Ctrl+W closes the active working tab,
  and Ctrl+Tab cycles Home and working tabs.
- Confirmation before discarding unsaved tab edits and protection against
  switching/closing tabs during pending writes. Read-only data refreshes when
  returning to a workspace while retaining valid selections and dirty drafts.

- Editable category draw drafts for singles and doubles: ordered manual seeds,
  automatic/manual group allocation, organizer-selected group counts and
  qualifiers, and knockout slots with byes awarded to the strongest seeds.
- SQLite schema 10 with pre-v10 backups, immutable draw revisions, stale-registration
  validation, optimistic revision checks and idempotent save retries.

- Atomic, idempotent player settlements across selected categories, with per-player
  doubles allocations and confirmed refunds when clearing Paid.
- SQLite schema 9 with pre-v9 backups and immutable financial allocations/requests;
  historical partial payments, discounts and ledger records remain preserved.

- Dedicated category workspaces with Registrations, Draw, Matches and Results
  sections; unfinished competition-engine tabs clearly show their planned status.
- Checkbox batch singles registration and explicit doubles-pair queues, saved
  atomically with member snapshots and automatic category charges.
- Confirmed category removal: delete empty categories and archive used categories
  while preserving registrations, attendance and cash accounts.
- SQLite schema versions 7/8 with pre-v8 backups, category archiving and category
  names unique per tournament and discipline (same name allowed for singles/doubles).

- Category fees configured in RSD during creation; new singles and doubles
  registrations automatically receive one charge, atomically with entry creation.
- Zero-fee categories, fee display, payment-first cash desk and SQLite schema 6
  with pre-v6 backups; existing registrations are not retroactively charged.
- Tests for exact fee amounts, pair charging, free entry, rollback on charge
  failure and v5 migration preserving manual financial records.

- Entry cash desk in Serbian and English, with RSD charges, discounts, partial
  payments, refunds, outstanding balances, credit and chronological history.
- Append-only financial records, integer minor-unit amounts, scoped transactional
  balance validation and idempotent requests for safe retries after uncertain writes.
- SQLite schema version 5 with pre-v5 backups, cash ledger regression tests and
  documented first-increment limits (manual entry accounts, shared doubles accounts).

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

### Planned

- Match generation and results, category-specific group ranking by head-to-head
  mini-table, set ratio and point ratio, including re-evaluation of partial ties.
  Draw confirmation and advancement to the knockout stage are still pending.
