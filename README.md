# LibreTT

[Srpski](README-sr.md)

LibreTT is a planned free and open-source, offline-first application for
organizing table-tennis tournaments and leagues. LibreTT is both the application
name and the umbrella project identity.

**Status: early development.** The first slice creates and lists local tournaments
and adds singles/doubles categories with independently selected formats. It also
provides local players and category registrations with historical snapshots. The
full tournament workflow and release installers are not available yet. See
[development setup](docs/en/DEVELOPMENT.md) for running and checking the project.

## Purpose

**Competition data belongs to the community that creates it.** A result produced
by a player or club must remain available even if LibreTT is no longer maintained
or stops working. The application preserves that history; access to it must not
depend on the application's survival.

Local ownership, documented open exports, and independently readable archives
are architectural requirements. Backups alone are not enough: the community
must be able to read, transfer, and recover records without running LibreTT or
a central service. Archival export and restoration are planned, not yet implemented.

Give clubs and organizers tools they can operate locally, without mandatory
accounts, internet access, license activation, or paid feature tiers. Keep
competition data portable through documented formats and backups.

## Planned scope

The application has two modules: **Tournaments** and **Leagues**. Development
starts with tournaments on **Linux, macOS, and Windows**. Phone companions for
referees and spectators come later.

- Manage multiple tournaments, each containing multiple categories: singles,
  doubles, veterans, age groups, and organizer-defined categories.
- Choose direct single-elimination or round-robin groups followed by
  single-elimination independently for each category.
- Maintain a local player directory and register players in multiple categories.
- Track advance registrations, attendance, fees, payments, and outstanding balances.
- Generate seeded groups and brackets automatically, arrange them manually, or
  edit a generated draw before confirming it.
- Manage results, table assignments, standings, advancement, and final placements.
- Provide backups, restoration, import/export, printable material, and a display
  for a second screen.
- Support Serbian and English throughout the user interface and output.

Future integration with a player directory on **stoni.rs** will allow online
search and download for subsequent offline use. That directory and its API do
not exist yet. Local operation will remain independent of that service.

## Architecture proposal

The proposed stack is **Tauri 2, Rust, TypeScript/Svelte, and SQLite**. It remains
subject to an early cross-platform prototype, including offline installation,
printing, backup restoration, and second-screen support.

Competition rules belong in a platform-independent domain layer. Desktop and
future companion adapters invoke the same application use cases. Confirmed
results are the source of truth; standings are rebuildable projections.

LibreTT's existing WordPress project and DimiPress Rally inform the design.
This repository describes a new standalone application. The author intends to
rename the existing WordPress project to `librett-wordpress`.

## Documentation and contributions

- [Desktop application plan (Serbian)](docs/DESKTOP_TOURNAMENT_PLAN_SR.md)
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

English and Serbian user and technical documentation will grow with the
specification and implementation. The current detailed planning document is
available in Serbian.

## Author and license

Copyright (C) 2026 Aleksa Dimitrijević.

LibreTT is licensed under the **GNU Affero General Public License, version 3
or any later version** (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).

LibreTT is provided without warranty; see the license for the full terms.
