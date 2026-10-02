![LibreTT](assets/img/logo-dark.png)

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

The interface supports Light, Dark, and System themes, remembers the selection,
and uses the corresponding LibreTT logo. Tabler Icons are bundled locally and
work offline. Their MIT license is preserved separately from LibreTT's AGPL
license; see [third-party notices](THIRD_PARTY_NOTICES.md).

The app opens on a mode-selection dashboard without a sidebar. Tournaments opens
its module; Leagues is disabled until implemented. Back/Forward follows screen
history. Home returns to the module overview, then to mode selection on a second
click. The Players tab shows the shared directory, with Add/Edit opening dedicated
profile screens. Delete asks for confirmation; players with registrations are
retained to preserve competition history. Profiles have editable birth year,
location, contact details, notes, and locally stored photos; tournaments register
participants from that directory. The local operator manages profiles; user
accounts and enforced administrator permissions are not implemented yet.

Tournament entries support withdrawal/restoration and individual player check-in
shared across categories in the same tournament. The Cash desk records charges,
discounts, partial payments and refunds with entry balances and immutable history.
This first increment uses RSD and a shared account per doubles pair; automatic
fees, per-person doubles tariffs and payments split across categories are planned. Read the [security policy](SECURITY.md) before reporting vulnerabilities
or sharing local databases.

## AI-assisted development

LibreTT is developed with AI assistance for programming. The project idea,
architecture, implementation decisions, and tests are authored or reviewed
and verified by Aleksa Dimitrijević. Human review and verification remain
part of development; AI assistance does not replace responsibility for the code.

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
