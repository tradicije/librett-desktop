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
Singles/doubles and category formats are stored settings; draw and match engines
are not implemented. There is no result entry or cash desk,
export/restore UI, or installer yet. Development version `0.1.0` is not a release.

The Players tab shows the shared directory. Add/Edit opens dedicated profile
screens, including photos, and returns to the list after saving. Delete asks for
confirmation and protects profiles with existing registrations. Tournament
registration uses that directory. Entries can be withdrawn/restored, and player
attendance is shared across categories in a tournament. Schema version 4 adds
registration status and attendance; older on-disk databases receive a consistent
`pre-v4-<uuid>.sqlite` backup before migration.

Back/Forward follows screen history. Home returns to the selected module overview,
then mode selection. The mode chooser has no sidebar. Saves lock navigation until
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
