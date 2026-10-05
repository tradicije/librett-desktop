# Beta installers and GitHub releases

[Srpski](../sr/RELEASE.md)

The first beta is **0.1.0-beta.1**. Packaging does not publish a release.
Referee/phone companions are outside this beta. Real-tournament validation and
manual installation checks on each supported platform are still required.

## Packages

| Platform | Release file | Installation |
| --- | --- | --- |
| macOS Apple Silicon | `LibreTT_0.1.0-beta.1_macos-arm64.dmg` | Open DMG and drag LibreTT into Applications |
| macOS Intel | `LibreTT_0.1.0-beta.1_macos-x64.dmg` | Open DMG and drag LibreTT into Applications |
| Windows x64 | `LibreTT_0.1.0-beta.1_windows-x64.exe` | Run the setup wizard |
| Linux x64 | `LibreTT_0.1.0-beta.1_linux-x64.deb` | Install using the distribution package manager |
| Linux x64 | `LibreTT_0.1.0-beta.1_linux-x64.AppImage` | Make executable and run |

Windows embeds the offline WebView2 installer, increasing the download size but
allowing installation without fetching the runtime. Linux packages still depend
on compatible system libraries; CI builds on Ubuntu 22.04. Only x64 Linux and
Windows are packaged initially. macOS packages are separate for the two CPU types.

macOS uses an **ad-hoc signature**, without Apple Developer ID/notarization.
Windows packages have no publisher certificate. Downloaded beta packages may
therefore be blocked or show OS trust prompts. Document this in the release;
on macOS use the OS **Privacy & Security → Open Anyway** flow for a trusted beta.
Do not instruct users to disable system protection globally. Distribution
signing/notarization needs separate certificates and credentials.
See [Tauri macOS signing](https://v2.tauri.app/distribute/sign/macos/) and
[Windows signing](https://v2.tauri.app/distribute/sign/windows/).

## Build locally

Install the development prerequisites described in [DEVELOPMENT.md](DEVELOPMENT.md).
From the repository root on the target OS:

```sh
npm ci
npm run desktop:build -- --ci -- --locked
```

Native packages are written to `target/release/bundle/`. Copy them into a
release-ready folder and generate SHA-256 checksums, choosing the matching CPU:

```sh
npm run desktop:collect -- target/release/bundle macos-arm64
```

Other collection labels are `macos-x64`, `windows-x64`, and `linux-x64`.
Collected files are in `release-assets/<platform>/`, which is ignored by Git.
macOS and Windows also produce a release ZIP containing the installer,
`README-SR.txt`, `README-EN.txt` and installer checksums. Packaging uses `zip` on
macOS and PowerShell Compress-Archive on Windows; Linux keeps its standalone packages.
Collection requires exactly one package per expected format and matching npm,
Cargo and Tauri versions. If old bundles remain, use a clean checkout/build
directory rather than mixing versions. Builds with `--target <triple>` write to
`target/<triple>/release/bundle/`; pass that directory to the collector.

## Build all platforms with GitHub Actions

1. Commit and push the packaging changes to GitHub.
2. Open **Actions → Build beta installers → Run workflow** and select the branch.
3. Wait for all four platform jobs to finish. Each uploads an artifact named
   `LibreTT-<platform>` containing the installer(s), checksums, and a ready-made
   release ZIP with bilingual instructions for macOS/Windows.
4. Download and extract each artifact. Artifacts are retained for 30 days.
   Run installation checks before publishing; successful CI only confirms builds.

Pushing a `v*` tag also triggers the workflow. Use a tag matching the version
shown in the application, for example `v0.1.0-beta.1`. The workflow has read-only
repository permissions and does not create, edit or publish GitHub releases.
Runner/platform availability follows
[GitHub's runner documentation](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

## Publish manually

Create a tag for the exact commit used by the successful builds, then open
**Releases → Draft a new release**:

- Select `v0.1.0-beta.1` and title it `LibreTT 0.1.0-beta.1`.
- Mark **Set as a pre-release**.
- Extract the downloaded Actions artifact ZIPs. For macOS/Windows, attach the
  inner `LibreTT_<version>_<platform>.zip` release ZIP, which includes bilingual
  instructions beside the installer. For Linux, attach the DEB/AppImage and
  checksum file. Do not publish the outer Actions artifact ZIP as the release package.
- Include the beta limitations, installation instructions and changelog summary.
- Keep the tag/source archives available for the corresponding AGPL source code.

Suggested release summary:

> First public beta for offline table-tennis tournament administration. Includes
> players, categories, registration, draws, group/knockout scoring and standings,
> final placements, cash desk, manual table assignment, backups and print/export.
> Referee/phone companions are not included. Real-tournament validation is pending.
> macOS is ad-hoc signed and not notarized; Windows is unsigned.

The application identifier stays `org.librett.desktop`: installing this beta uses
the same local data directory as development builds. Stop the development app
and make a manual backup before switching. Installers contain application files
and license notices, not your local database. On the first tournament, keep a
backup and parallel record until real-event behavior has been validated.

For the next beta, update `package.json`, `apps/desktop/package.json`,
`package-lock.json`, workspace/Cargo lock versions, and Tauri's version together.

