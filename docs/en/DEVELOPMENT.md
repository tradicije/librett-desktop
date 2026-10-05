# Development

[Srpski](../sr/DEVELOPMENT.md)

## Requirements

- Node.js 22 LTS and npm.
- Stable Rust with Cargo, rustfmt, and Clippy.
- The operating-system development dependencies listed in
  [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

The core crates can be tested without Linux WebKit/GTK development libraries.
The actual desktop application requires them. Windows and macOS need their
respective native toolchains. Cross-platform qualification is still pending.

## Install and run

From the repository root:

```sh
npm ci
npm run check
npm run build
npm run test:core
npm run desktop -- dev
```

`npm run dev` provides a browser-only UI preview on `http://127.0.0.1:1420`.
Database operations are disabled there; use the Tauri desktop command for
persistent tournaments. The desktop stores `librett.sqlite` in the OS application
data directory resolved by Tauri for `org.librett.desktop`, outside the checkout.
It needs no network connection after development dependencies are installed.

The current slice supports tournaments, categories, local player profiles,
name/club search, and singles/doubles category registration.
Singles/doubles matches support set scoring, retirement and walkover. Group
standings and group-to-knockout qualification update automatically. There is no
export/restore UI or installer yet. Development version `0.1.0` is not a release.

The Players tab shows the shared directory. Add/Edit opens dedicated profile
screens, including photos, and returns to the list after saving. Delete asks for
confirmation and protects profiles with existing registrations. Tournament
registration uses that directory. Entries can be withdrawn/restored, and player
attendance is shared across categories in a tournament. Schema version 16 includes immutable draw drafts and versioned category rules; older on-disk databases receive a consistent
`pre-v16-<uuid>.sqlite` backup before migration.

Back/Forward follows the active workspace’s screen history. The top Home button
returns to the pinned mode-selection workspace. The mode chooser has no sidebar. Saves lock navigation until
completion. Local administration currently has no login or enforced user roles.

## Themes and icons

The theme selector supports Light, Dark, and System; System follows live OS
changes. Theme and language preferences are local to the UI. Tournament data
remains in SQLite. The supplied `assets/img/logo-light.png` and `logo-dark.png`
are bundled into the application, so logos work offline.

The desktop icon source is `assets/img/app-icon.png`. Tauri-generated PNG, ICO,
and ICNS files live in `apps/desktop/src-tauri/icons/` and are application assets,
not development caches. The native window icon requires restarting the desktop.

On Linux/Wayland the shell resolves the icon through a `.desktop` entry matching
`org.librett.desktop`, rather than the embedded window image. GTK application ID
registration is enabled, and the GLib program name is set before GTK startup
so the Wayland window ID also matches. GTK permits one application instance per session under
that ID. For a development checkout, register the icon once (and again after
changing the icon):

```sh
npm run desktop:install-icon
```

This copies PNGs into `$XDG_DATA_HOME/icons/hicolor` and installs a hidden
`org.librett.desktop.desktop` entry in `$XDG_DATA_HOME/applications` (default:
`~/.local/share`). It refreshes KDE's desktop cache when available. These are
local system integration files, outside the repository. The entry matches running
windows; it does not add a launcher. Stop and restart the desktop dev command.
Existing non-development desktop entries are left untouched. Release installers
must provide the same desktop identity and icon independently of this dev helper.
See [Tauri GTK application ID configuration](https://v2.tauri.app/reference/config/#enablegtkappid).

Icons use the official `@tabler/icons-svelte` package. Only selected components
are imported, rendered using `currentColor`, and bundled for offline use.
Decorative icons accompany visible labels. There is no external icon-library
folder requirement. Preserve [Tabler's MIT notice](../../docs/licenses/tabler-icons-MIT.txt)
when distributing the application. Restart the dev command after installing
new dependencies if Vite does not pick them up.

## Checks

```sh
cargo fmt --all -- --check
cargo test --workspace --exclude librett-desktop
cargo clippy --workspace --exclude librett-desktop -- -D warnings
```

After native requirements are installed, also check the desktop adapter:

```sh
npm run build
cargo check -p librett-desktop
```

Manual desktop smoke check: create a tournament with Serbian letters, add a
singles category with groups then knockout and a doubles category with knockout,
reject a duplicate category, restart, and verify persistence. Switch languages
and verify labels/errors change while names remain intact. Disconnect networking
and repeat. These are required checks, not a claim they have already passed.

## Linux graphics troubleshooting

If compilation succeeds but the window exits with `Error 71 (Protocol error)
dispatching to Wayland display`, try a per-run WebKitGTK workaround:

```sh
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run desktop -- dev
```

This disables the faster DMABUF rendering path for that process. It is not a
global application default. See [Tauri Linux graphics guidance](https://v2.tauri.app/develop/debug/linux-graphics/).
If the failure persists, collect graphics-driver/session details before choosing
another workaround. A Vite/esbuild `EPIPE` after shutdown may be a consequence
of the stopped process, rather than an independent TypeScript problem.

## Repository hygiene

Commit source, migrations, specifications, documentation, and dependency lock
files. Do not commit dependencies, generated output, local databases, caches,
credentials, or editor/agent settings. `.gitignore` excludes these artifacts.
Use external temporary directories for research and one-off tooling. Dependency
installation creates an ignored `node_modules/`; builds create ignored output.
Keep transient cache/build paths outside the checkout where convenient:

```sh
npm ci --cache /tmp/librett-npm-cache
CARGO_TARGET_DIR=/tmp/librett-target cargo test --workspace --exclude librett-desktop
```

See [ADR 0001](../adr/0001-desktop-foundation.md) and
[setup scenarios](../../specs/tournament-setup.md).

## Cash desk

Open a tournament and choose Cash desk. The summary shows remaining outstanding
balance, net received and distinct active registered players. The searchable table
has one row per player and one column per category, followed by total outstanding
and buttons for collection and refunds. Categories without a registration show a dash. Archived/withdrawn entries
retain their financial balances and are labeled; they do not add to the active count.

Category checkboxes start unchecked and select unpaid accounts to settle. Clicking
Collect records their remaining
balances atomically, without entering amounts or notes. Doubles charges split equally:
500 RSD per pair means 250 RSD per player. Paying one member does not pay the other.
Settled categories display Paid and remain selectable for refunds. A successful payment
clears the selection; payment status remains visible after reopening this screen.

Refund displays selected category amounts and a total for confirmation, then
returns only that player’s received payments. Cancellation leaves the ledger
unchanged. Refunds do not withdraw registrations; manage withdrawal separately.
Withdrawn entries remain available for refunds.

Existing partial payments, discounts and refunds affect amounts shown. The simplified
screen does not expose manual charge/discount/partial-payment forms or ledger history;
those records remain preserved in the backend. Uncertain writes retain their UUID
and lock controls until a retry confirms the outcome. Smaller windows scroll the
category columns horizontally. New registrations still receive category fees
atomically; zero means free participation.

## Category workspace

Click a category row to open its Registrations tab. Check several singles players
and register them together, or select two doubles players and add each pair to the
queue before submitting. Already registered players remain marked and unavailable.
Batch registration is atomic, including automatic charges. Draw offers editable
group/knockout drafts; Matches and Results remain placeholders.

Delete asks for confirmation. Empty categories are removed; categories with any
entry are archived and retain accounts in Cash desk. History routes to removed
categories explain their unavailable state. Category names are unique within a
discipline, so Singles and Doubles may share the same name.

## Tournament workspace

Opening a tournament shows persistent Overview, Categories, Registrations and Cash
desk tabs. Categories contains the active category list and the new-category form.
Registrations opens a selected category's workspace for managing entries, attendance
and registration status. Each category has its own Registrations, Draw, Matches and
Results tabs; Matches and Results are marked as in development. Cash desk shows the
existing player payment matrix. Overview is the tournament's starting screen.
Moving between tabs participates in Back/Forward history, as does opening a category.


## Category draw drafts

Open a category’s Draw tab, choose Automatic or Manual, and add seeds in order
of strength. For group formats, choose the group count and qualifiers per group.
Create arrangement produces balanced groups or first-round knockout slots;
automatic byes go to the strongest seeds, then randomly to unseeded entries.
The slot selectors allow manual edits and swap entries that are already placed.
Changing generation settings applies to the next arrangement; changing seeds
also updates the displayed draft. Save draft persists a new revision. Manual
drafts may be incomplete. Save before navigating away; unsaved edits are local
to the open draw screen. Reload discards local edits and fetches current entries.

A changed registration list requires regeneration. Concurrent revisions reject
a stale save. An uncertain write keeps its request UUID and blocks navigation
until a retry confirms the outcome. Existing databases receive a pre-v16 backup.
Complete current draws supply the Matches tab. Set results and knockout
advancement, group standings and qualification are implemented; explicit
competition lifecycle remains pending.

## Native navigation

Toolbar arrows, mouse side buttons and native webview back/forward share screen
history, including tabs. macOS enables WKWebView horizontal trackpad navigation
(the system’s swipe-between-pages preference must permit it). Normal horizontal
scrolling remains native. Pending writes prevent screen changes even when
navigation is triggered outside the toolbar. History is reset on application reload.


## Workspace tabs

Home is pinned behind the top-left Home button and never appears in the tab list.
Choosing a tournament from Home opens a working tab. The tab label shows the immediate parent and current page (e.g. Veterans | Draw);
its tooltip contains the full context.
Home preserves open working tabs. The + button and Cmd/Ctrl+T open Home to choose
another workspace; they do not create an empty Home tab.

Click normally to navigate inside the current working tab. Cmd/Ctrl-click,
middle-click, or right-click → Open in new tab opens a destination separately.
Right-clicking a working tab opens its current destination in another tab; it does
not copy unsaved form contents. Close with ×, middle-click, or Cmd/Ctrl+W. Home
cannot be closed. Ctrl+Tab / Ctrl+Shift+Tab cycle Home and working tabs. Arrow keys,
Home and End navigate the focused working-tab list.

Inactive workspace components remain mounted and hidden, retaining current forms,
searches, selections, draft edits, focus and scroll. Each owns its route history;
mouse buttons and macOS gestures act on the active workspace. Native History API
entries serve as a centered bridge for gestures so histories never mix across tabs.
Pending writes and open modal dialogs block tab changes. Closing a tab with unsaved
player profiles, registration queues, tournament/category forms or draw edits asks
for confirmation. Tabs exist for the current app session; restart does not restore
them or unsaved forms. Window closing and Quit require confirmation when any workspace contains unsaved work.

Cash, registrations, directory and draw data refresh when returning to a tab.
Valid selections remain; unsaved draw layouts are preserved and stale membership
or revisions are rejected by the existing save validation.

The 48px top bar is fixed to the viewport. Workspace and sidebar heights subtract
that row, so short pages do not gain a scrollbar. On macOS, Overlay title-bar style
and hiddenTitle put the native window buttons beside Home/tabs; AppKit coordinates
center the actual button frames on the row after startup, resize and focus. Window
chrome changes require restarting the desktop process. Other systems retain their
native title bar above the workspace row. Blank row areas support native dragging.

Working tabs are a fixed 180px wide regardless of label length. Labels fade at the
right only when their measured text overflows; they do not use an ellipsis. The
full page/context is available in the tooltip, and the tab strip scrolls horizontally.


## Collapsible sidebar

Use the button beside the logo to collapse the sidebar to 68px; the expanded menu
uses 208px (184px in smaller desktop layouts). Compact mode shows only navigation
icons with accessible labels and tooltips, plus `icon-dark.png` in dark UI or
`icon-light.png` in light UI. Click the compact brand icon to expand; there is no
separate expansion icon. Neither the full logo nor the compact icon navigates to Home. The preference
is shared by Home and every working tab and saved under `librett.sidebarCollapsed`
in local storage, including across application restarts. Theme changes also switch
the compact brand icon. Collapse/expand changes layout without replacing screen state.


## Category configuration and visual draw

Tournament tabs are Overview, Categories and Cash desk. Categories provides Add,
Edit and Delete actions matching the player directory. Add/Edit open a dedicated
editor; category rows open their Registrations page. Used categories cannot change
discipline or format, and fee edits affect only future registrations.

Category Settings combines saved rules and ordered seeds and automatic
or manual arrangements. Rules include group count, qualifiers, best-of sets,
points, winning margin and the tied-player mini-table/set-ratio/point-ratio order.
Categories using groups have a separate Groups tab with round-robin pairings
and cards in up to three columns (two or one on smaller windows). Draw shows only
the complete horizontally scrollable knockout bracket. Group qualifiers
remain labeled placeholders until their groups finish and their order is resolved.
Knockout-only draws show saved scores, advancing winners and the champion.

Schema 11 saves versioned rules and migrates existing group draft settings after
a pre-v11 backup. See [ADR 0012](../adr/0012-category-rules-and-draw-view.md).


Workspace scrolling uses a bounded container below the 48px title bar. The root
document does not scroll, and vertical overscroll cannot chain into it. Sidebars and navigation toolbars sit outside the content scroll region; tab
switching restores the content container scroll position. Breadcrumb buttons
use workspace navigation and support the same new-tab actions as other links. This isolates the tab row from macOS elastic scrolling.


The Info button at the far right of the title bar opens a themed About dialog.
It shows the logo, version from package metadata, author, description and full
bundled AGPL license, available offline. Escape or Close dismisses the dialog.


## Guarded writes and tournament covers

Navigation, Back/Forward, cancellation and draw reload confirm before discarding
local edits. Window closing and Quit inspect all workspaces. Player writes use
stable IDs, atomic snapshot checks and idempotent request receipts; conflicts
can be recovered through a confirmed reload. Financial actions include expected
amounts and require a new confirmation when balances change.

The tournament directory uses up to three cards per row. Add Tournament opens a
dedicated editor with an optional user-cropped 16:9 cover. Registered counts include
unique active players across active categories. Groups generate pairings only
when opened, one round at a time, with 32 pairs per page. Uploaded images are
bounded before decoding and verified with a full backend JPEG decode.
See [ADR 0013](../adr/0013-guarded-writes-and-bounded-images.md).

Tournament Settings edits the name and optional cover. Updates compare the
original metadata atomically, reject conflicting edits, and accept retrying an
already-applied update. The entire tournament card supports mouse and keyboard
navigation; its bottom button also supports opening a new workspace tab.

`ImageCropDialog.svelte` imports images through `readBoundedImage` before
opening an interactive crop preview. Player photos export at 512×512 (1:1),
tournament covers at 1024×576 (16:9), using identical source and output ratios.
Dragging and position sliders share bounded crop coordinates; zoom ranges from
1× to 4×. Apply exports a bounded JPEG; Cancel keeps the existing photo/cover.
File inputs reset after closing so the same file can be selected again.

## Group standings and qualification

The Groups tab shows played/won/lost matches, set and point totals and live
ranking. The configured tie criteria use a mini-table restricted to tied entries.
A completed resolved group supplies its configured qualifiers to the bracket.
Exact ties require the group Ranking action; organizers can also override any
completed group's order or restore Automatic. New group results restore the
automatic order. The Matches stage selector includes the resulting knockout.
Dependent knockout results are cleared only after confirmation.

Schema 15 stores guarded manual group orders. See
[ADR 0015](../adr/0015-group-standings-and-qualification.md) for ranking
statistics, non-played endings, qualification, correction and refresh behavior.

## Category age groups

Player create/edit forms require a birth year; the guarded desktop save command
returns `birth_year_required` for missing years. Existing profiles without a year
remain readable and can be completed when edited. Legacy internal Rust helpers
retain their compatibility behavior; they are not exposed as desktop commands.

`CategoryRules` includes `age_enabled`, `age_min` and `age_max`. The switch is
off by default (all ages); enabled ranges require inclusive integer bounds
0–130 with minimum ≤ maximum. Disabled ranges have null bounds. The shared rules
form exposes the switch and From/To fields in category creation, editing and
settings. JSON defaults keep old rules, draws and result snapshots compatible;
there is no schema migration or backfill of invented birth years.

Registration refreshes category rules and player profiles before submitting the
selected singles/pairs. Age uses the local current calendar year minus birth year,
not a birthday or tournament date. `AgeWarningDialog` lists out-of-range and
unknown-year players and allows a confirmed exception. Cancel preserves the
selection. Restoring withdrawn registrations follows the same warning flow.
The backend intentionally does not reject entries based on age, and changing a
category age range does not remove existing entries.

## Lucky loser knockout filling

In category creation/editing or Settings, choose BYE (the compatible default)
or Lucky loser under Vacant knockout places. Lucky loser fills the next power
of two already determined by direct qualifiers: 5 → 8, 7 → 8, 9 → 16. It does
not enlarge the bracket based on the number of candidate entries.

After saving a group arrangement, Settings shows Lucky loser places. Automatic
fills every available place after all group standings are complete and resolved.
Use All automatic to remove overrides, or select a non-qualifier/BYE for each
place and save. A candidate cannot appear twice or be a direct qualifier.
Insufficient candidates leave real BYEs; unresolved places never award a bye.

Candidates rank by group place ascending, then win percentage, set ratio and
point ratio descending, using exact integer comparisons. When an exact tie
straddles the automatic cutoff, all affected places wait for manual selection.
This is the application's agreed policy, not a claim about a federation's rules.
The candidate table shows the statistics used for selection. Bracket labels
include LL and group/place, carried into later rounds.

Manual choices belong to a saved draw. An eligible choice survives standings
changes; one that becomes ineligible blocks that place until reviewed. A new
draw starts with no overrides. Switching to BYE retains inactive overrides so
switching back can restore them, subject to current eligibility.

`save_knockout_fillers` uses an immutable request UUID, expected filler/rules
revisions, group order/result versions and the version of all match results.
Stale edits reject with `match_conflict`; uncertain writes must retry the same
request. Result-impact confirmation appends null revisions to affected knockout
results atomically with the choices; previous results remain in history.
Category rule writes also require confirmation when the projected participants
would invalidate recorded knockout results. Schema 16 and the selection policy
are documented in [ADR 0016](../adr/0016-lucky-loser-knockout-filling.md).

Tournament totals and registration do not have a fixed player cap. A draw
supports 2–4096 entries in one category (a doubles entry is a pair), independently
of other categories. This is a code limit, not a performance qualification.
