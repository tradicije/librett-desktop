![LibreTT](assets/img/logo-dark.png)

[Srpski](README-sr.md)

LibreTT is a free and open-source application in development for organizing
table-tennis tournaments and leagues, with offline operation as its foundation.
It is intended for clubs, organizers, players, and the community that builds
the sport. LibreTT is both the application name and the umbrella project identity.

**Status: early development.** We are currently building the tournament desktop
application. Draws, matches, results, and release installers are not available
yet. See the [development documentation](docs/en/DEVELOPMENT.md) to run it.

## Project philosophy

Table tennis is built by people: players who train, clubs that bring them
together, referees, volunteers, and organizers who make competitions happen.
The basic tools for organizing and recording that work should be available to
everyone, regardless of a club's budget or a tournament's size. **LibreTT starts
from the belief that such tools should be free of charge and open source.**

A simple tool can benefit the sport far beyond its technical scope. Clear
results and schedules can help audiences follow competitions. Statistics kept
for decades can document players' development, clubs' work, and the history of
entire competitions. Accessible, reliable records can make the sport easier to
follow, increase its visibility, and help clubs present their work to potential
sponsors. These are opportunities we want to open to the community, rather than
outcomes that any single application can guarantee.

Other competition tools exist, and some may serve their users very well.
LibreTT's purpose does not depend on being the first or the only one. We want
to offer a choice where free access, open source, offline operation, and lasting
access to data are part of the same commitment to the community. Using the
application and accessing your own history should not require a subscription,
license activation, or purchasing feature tiers.

**Competition data belongs to the community that creates it.** A result is
produced at the table through players' and clubs' work; the application records
it. Competition history must therefore survive a service shutting down,
software maintenance ending, or a subscription becoming unaffordable. What the
community creates today must remain possible to preserve, read, and transfer
decades from now.

That principle sets concrete requirements: operation without mandatory internet
access or accounts, local storage, documented open formats, and archives that
can be read without running LibreTT or relying on its author. A backup is not
enough if reading it requires the same service to remain alive. Open source lets
the community inspect the tool, adapt it, and continue development when its
original maintainers can no longer do so. Open source does not mean publishing
players' personal data; the community holding those records decides how to
share them.

These commitments guide development. Local storage already exists; independent
archival export and a restoration interface are still planned. Preserving
history must be part of the results workflow, rather than an afterthought.

## What works today

- Multiple tournaments and singles/doubles categories with independently selected
  competition formats.
- A shared local player directory with profiles, photos, and search.
- Category workspaces with checkbox batch singles registration and explicit doubles
  pair selection; names may repeat across singles and doubles.
- Category registrations with historical name/club snapshots, withdrawal and
  restoration, and tournament-wide player check-in.
- Category fees in RSD with automatic charges for new registrations; doubles
  fees are per pair, and zero means free entry.
- A searchable cash desk with a row per player, category columns and Paid controls.
  Doubles fees split equally; remaining balance, net received and registered player
  totals account for existing payments and discounts. Financial history is preserved.
- Confirmed deletion of empty categories and archiving of used categories,
  preserving registrations and cash history.
- Serbian and English interfaces, Light/Dark/System themes, and locally bundled
  Libre Franklin fonts and Tabler icons.

Leagues and phone applications come later. Separate per-person doubles tariff
configuration and manual financial adjustment forms remain planned. Local administration
currently has no accounts or enforced user roles. Read the
[security policy](SECURITY.md) before reporting vulnerabilities or sharing databases.

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

## Architecture

The application uses **Tauri 2, Rust, TypeScript/Svelte, and SQLite**. It remains
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
- [Development setup](docs/en/DEVELOPMENT.md)
- [Contributing](CONTRIBUTING.md)
- [Font and icon licenses](THIRD_PARTY_NOTICES.md)
- [Changelog](CHANGELOG.md)

English and Serbian user and technical documentation will grow with the
specification and implementation. The current detailed planning document is
available in Serbian.

## AI-assisted development

LibreTT is developed with AI assistance for programming. The project idea,
architecture, implementation decisions, and tests are authored or reviewed
and verified by Aleksa Dimitrijević. Human review and verification remain
part of development; AI assistance does not replace responsibility for the code.

## Author and license

Copyright (C) 2026 Aleksa Dimitrijević.

LibreTT is licensed under the **GNU Affero General Public License, version 3
or any later version** (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).

LibreTT is provided without warranty; see the license for the full terms.
