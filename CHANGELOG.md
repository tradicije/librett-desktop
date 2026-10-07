# Changelog

Notable changes to LibreTT are recorded here in English.

## Unreleased

### Packaging

- macOS and Windows release ZIPs now include the installer, Serbian/English plain-text installation instructions and installer SHA-256 checksums. Instructions explain Gatekeeper/SmartScreen prompts and preserve system protection.

- Prepared `0.1.0-beta.1` with synchronized npm, Cargo and Tauri metadata and a beta label in About.
- Enabled macOS application/DMG bundles (Apple Silicon and Intel), Windows NSIS setup with offline WebView2 installation, and Linux DEB/AppImage packages. Application identifier and local data location are unchanged.
- Added a manual/tag-triggered GitHub Actions matrix that uploads installers as artifacts without publishing a release, plus release-file collection with platform names and SHA-256 checksums.
- Bundled AGPL, Tabler and Libre Franklin license notices. Documented local builds, manual GitHub pre-release publication and the ad-hoc/unsigned beta distribution limits in English and Serbian.

### Fixed

- Group seeds now follow alternating snake order (two groups: 1/4 and 2/3). Automatically selected lucky losers use maximum matching to minimize opening-round group rematches without changing the selected candidates or manual slots.

- Restarting a draw after a category has started requires explicit confirmation in Settings and server-side authorization; old draw/results stay in history while the new revision starts without scores or table assignments.
- Started categories cannot be deleted or archived; the check and write share the same transaction.
- Changing best-of, point target or winning margin after a category starts requires confirmation in both category editors. Previously saved results retain their original scoring rules; subsequent entries/corrections use the new rules.

- Refresh/reload controls use the circular refresh icon; attendance removal uses a crossed circle. Undo arrows remain for restoration/reopening actions. Removed unused raw component keys from the shared icon map.

- History clears previous filter results and their pagination cursor before reloading, preventing mixed pages after a failed filter change.
- Matches publishes scores, schedule and table numbers together only after all reads succeed, preventing a new match page from displaying old table assignments after a read failure.
- Added history regression checks for deleted-category filters, equal-timestamp pagination and transaction rollback.

- Completing/reopening a category or tournament now snapshots the reactive confirmation draft before cloning it, so confirmation reaches the backend instead of failing on a Svelte proxy. Request preparation errors are caught and displayed in the dialog.

- Cash desk history-filter checkbox uses the standard compact size instead of inheriting full-width text-input dimensions, with panel-aligned spacing on desktop and narrow windows.

- Restore/import validates the migrated SQLite schema against a fresh LibreTT database before replacing live data, rejecting missing/modified tables, unexpected views, indexes and triggers. Staging connections disable trusted schema.
- Saving a match result no longer implicitly saves its table assignment; table changes must be explicitly saved or reverted first.
- Automatic backup failures are visible at startup and during periodic checks, with a retry action; successful checks clear the warning.

- Refined metadata layouts: short category/scoring summaries and position labels stay inline and wrap when needed; player details and group/knockout progress use separate rows. Set scores use compact tiles.

### Added

- Always-accessible Serbian/English tournament guide with twelve steps, common problems and a short glossary; equivalent Markdown guides are available in docs. Sidebar Trash, Backups, History and Guide now sit above local operations in an equal-width 2×2 icon grid with accessible names and hover labels.

- Category switch allowing or separating same-club entries in new automatic groups (allowed by default for compatibility). Separation preserves seed positions and group sizes, considers both doubles members, and refuses an unsuccessful layout rather than silently violating the policy.
- Existing club-name suggestions in the player editor, with explicit acceptance for an unambiguous one-character typo. Club comparison ignores case, spacing/punctuation, Serbian Latin diacritics and STK / Stoni teniski klub prefixes; typo variants are never silently merged.

- Global History in the sidebar, styled with shared theme colors, spaced filter labels, compact event columns, dated event rows, type icons and expandable change details; newest first with dropdown filters for history type, action and tournament, stable paginated loading, contextual labels and before/after details for results, draws/rules, profiles, registrations, attendance, cash, tables, completion, Trash, backups and file exports.
- SQLite schema 20 records committed data changes through immutable transactional audit triggers and preserves names after deletion/renaming without duplicating image data. Existing dated result/draw/cash/completion/Trash records are imported as clearly marked historical entries; previously unrecorded edits are not fabricated. Older databases receive a pre-v20 safety snapshot. Restoring a backup restores its history and records the restore action.

- Attendance-aware cash desk separates estimates for unconfirmed arrivals from outstanding debt. Collecting payment requires an active registration and confirmed arrival in both UI and transactional backend checks; doubles partners pay independently. Withdrawn/archived registrations stop contributing debt or estimates, while original charges and payments remain in the audit ledger and refunds remain available.
- Registrations default to active entries, with withdrawn history accessible through filters. Once a category has started, registration changes are blocked to preserve its draw and results; retirements/walkovers are recorded in Matches.
- Prepared draws remain available before arrival checks. The first match result requires confirmed arrivals or an explicit organizer override, recorded in the write receipt without confirming attendance or enabling payment. Cash reports use the same effective debt/estimate calculations as the desk.

- Tournament Trash in the sidebar: confirmed removal from the active list, searchable saved tournaments and restoration with registrations, results, completion history and cash records intact. SQLite schema 19 blocks stale writes while tournaments are in Trash and preserves Trash in backups. Existing databases receive a pre-v19 snapshot before migration.

- Database backups: daily consistent SQLite snapshots with 14-copy rotation, manual copies, export, validated restore/import and a safety copy before replacing data. Restore reloads workspaces to prevent stale writes.
- Manual table assignment in Matches → Edit match, independently of score entry. Table/player conflicts are checked across tournament categories and doubles; assignments can be moved/removed and completed matches release tables while retaining their table number.
- Print/PDF previews and UTF-8 CSV/HTML export for groups, bracket, final standings and cash desk. Brackets split into blocks of up to 32 entrants; category reports use one SQLite read snapshot. Export escapes HTML and protects spreadsheet cells from formula injection.
- Workflow tests cover odd draws, scoring, completion/reopening locks, bronze/champion-based third place, automatic/manual lucky losers, cross-category table conflicts and backup restoration. Updated migration expectations and added report-format checks.
- Schema 18 stores table assignments and idempotent scheduling receipts, with a consistent pre-v18 snapshot before upgrading existing databases.

### Changed

- Collapsed sidebar stacks Trash, Backups, History and Guide in a single column; expanded sidebar retains its equal-width 2×2 grid.

- History type now uses a dropdown with an All types option; removed the tournament-category dropdown and type toggle buttons.

- Draw → Edit now keeps the bracket visible and lets organizers select opening-round participants or BYE by clicking a slot. Existing participants swap positions; group knockout overrides support any registered category entry after resolved groups, with guarded result invalidation.
- Category creation/settings show age, Lucky loser/BYE and third-place switches in one responsive row. Enabling Lucky loser reveals automatic versus manual filling; manual mode leaves places unresolved until chosen. Third place can be shared, decided by a bronze match, or awarded to the semifinalist beaten by the eventual champion.
- Bronze matches appear in Draw and Matches, affect final standings and must finish before category completion when both semifinal losers exist.
- Player directory cards now show only the name with club suffix and age underneath, calculated as current year minus birth year, with Serbian godina/godine plural forms (muted unit, primary text for the number); contact/location details remain in the player profile.
- The bracket participant picker keeps its dropdown inside the dialog with one bounded, independently scrolling list instead of nested dialog/list scrollbars.
- Champion cards emphasize the player name with a smaller caption and muted club suffix. Dropdown selection closes the menu and isolates selection events from surrounding controls.


- Implemented category Results with winner/finalist/bronze summaries, complete
  standings and progress/blocker explanations. Losing semifinalists share third
  place; other knockout losers share elimination-stage ranges. Group
  non-qualifiers share the remaining range without invented cross-group ranks.
- Categories can be completed only after valid draws, resolved groups/qualifiers
  and all required knockout results, including the final. Confirmation stores
  a frozen standings snapshot and completion timestamp.
- Tournament Overview shows category progress, winners and completed counts.
  Tournament completion requires at least one active category and confirmation
  of every active category. Unused categories can be removed beforehand.
- Completion makes sporting operations read-only in both UI and SQLite guards.
  Reopening requires confirmation; reopen a tournament before reopening its
  categories. Cash operations and global player profiles remain available.
- Completion requests use immutable write identities and version guards; stale
  confirmations reject and uncertain writes retry safely. Completion/reopen
  history is append-only. Schema 17 upgrades create a pre-v17 SQLite backup.

- macOS Dock icons now use an 824px artwork region centered on a transparent
  1024px canvas, preventing full-bleed source artwork from appearing oversized.
  The desktop icon generator reapplies this margin automatically to ICNS assets.

- Refreshed Linux PNG, Windows ICO/Appx and macOS ICNS assets from the latest
  application icon design, including its latest visual revision.

- The Cargo binary and default run target are now named LibreTT on all desktop
  platforms (LibreTT.exe on Windows), avoiding the macOS development fallback
  to the former executable name. The internal package remains librett-desktop.
- Linux sets its GLib application display name to LibreTT, retaining the stable
  GTK application ID, and the development desktop entry targets Exec=LibreTT.
- Documented cross-platform branding and icon refresh: Linux PNG, Windows
  ICO/Appx and macOS ICNS share the same source and native rebuild tracking.

- macOS startup sets the native process display name from Tauri's product name
  before AppKit initializes. Development runs now use LibreTT instead of the
  internal Cargo executable name, librett-desktop.
- Added macOS CFBundleName/CFBundleDisplayName metadata and native build tracking
  for Info.plist so display-name updates are embedded on the next build.

- The native build explicitly tracks the generated `icons` directory so changing
  desktop icons recompiles Tauri's embedded icon resources instead of reusing
  a stale executable after a development restart.

- Regenerated desktop PNG, ICO and ICNS assets from the updated application
  icon. Added `npm run desktop:generate-icons` to refresh desktop assets from
  `assets/img/app-icon.png` without adding generated mobile icon folders.
- Documented icon regeneration and full native-app restart, including the
  macOS development Dock icon's use of `icons/icon.icns`.

- Categories with groups can choose BYE or Lucky loser for spare knockout
  places. Automatic selection waits for all groups to finish and resolves
  candidates by group place, win percentage, set ratio and point ratio.
  Exact ties at the cutoff await the organizer's choice.
- Category settings include per-place Automatic, eligible player and BYE
  choices, an All automatic action and candidate statistics. Direct qualifiers
  and duplicate entrants cannot be selected as lucky losers.
- Brackets label lucky losers with LL and their group/place (e.g. LL C3),
  retaining the origin through subsequent rounds. Ineligible saved manual
  choices await review rather than silently changing to another player.
- Lucky loser edits guard draw, rules, standings, filler and match revisions;
  retries preserve write identity. Participant changes and rule edits require
  confirmation before clearing dependent knockout results, retaining history.
- Schema 16 stores versioned knockout filler choices. Existing databases
  receive a consistent pre-v16 backup before migration. Existing rules and
  result snapshots default to BYE; migration does not alter their participants.

- Removed the redundant top separator and spacing from category Registrations;
  its layout no longer depends on being adjacent to the category tabs.
- Player create/edit screens and guarded desktop profile saves require a birth
  year. Existing profiles without a year remain readable.
- Categories can enable an inclusive minimum/maximum age range (0–130) through
  a switch, disabled by default. Age rules are saved with versioned category
  rules and appear in create/edit and category settings.
- Registration and restoration warn about out-of-range players or unknown birth
  years in restricted categories. Organizers can confirm exceptions; both doubles
  partners are checked. Age is the current calendar year minus birth year.

- Windows now shares the 48px workspace title bar with tabs, Info and
  application-drawn minimize/maximize/close controls styled for Windows.
  Native actions preserve guarded close, title-bar dragging, double-click
  maximize and edge/corner resizing. Linux and Windows share the controls
  component and platform-scoped permissions; macOS keeps native AppKit buttons.

- The expanded sidebar footer shows LibreTT for Windows, LibreTT for MacOS or
  LibreTT for Linux according to the platform, below the local-work label and
  above the copyright. The platform label is hidden in compact mode.

- Added a theme-colored 1px Linux window outline, hidden when maximized/fullscreen.
  Window controls use smaller Tabler icons, compact rounded buttons and consistent
  hover, pressed and keyboard-focus states.

- Linux uses one title bar for workspace tabs, Info and application-drawn window
  controls. Minimize, maximize/restore and guarded close use native window actions;
  drag regions, double-click maximize and edge/corner resizing remain available.
  macOS retains its native AppKit traffic lights and Windows retains its system frame.

- Added a Rust stable shields.io badge to both README headers, matching the
  label/value format of the other badges.
- Knockout entrants retain group/place prefixes (A1, B2, etc.) beside their names
  as they advance. Pending qualifiers use the same compact labels.
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

- Restore opaque Linux window rendering after transparent surfaces left the
  application invisible or painted only hovered controls on NVIDIA/Wayland.
  Keep native transparency disabled and remove misleading CSS frame rounding.
  The Linux frame uses square corners, a themed outline and compact Tabler controls;
  native corner shaping remains deferred until a portable rendering solution is verified.

- Prevent Linux/Windows startup navigation from remaining locked while waiting
  for a native-history event. The history bridge is scoped to macOS WKWebView
  trackpad gestures; toolbar and mouse navigation remain available on all platforms.

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
