# Tournament simulation before beta 2

Date: October 7, 2026.

## Result

All 41 core tests passed (5 domain and 36 storage tests), together with the existing cash-calculation and report-format checks. No new application defect was found in the executed scenarios. Two repeatable scenarios were added to `crates/storage-sqlite/src/workflow_tests.rs`.

## Combined tournament

- 12 players with Serbian characters in their names, four clubs and three categories.
- Open: three groups of four, club separation, two direct qualifiers per group and two automatic lucky losers, followed by a complete knockout.
- Veterans: seven players with BYE, semifinals, final and bronze match. This category name is a fixture label; the scenario does not test age exceptions.
- Doubles: six pairs, an odd 500.01 RSD entry fee, walkover and complete knockout.
- Attendance, multi-category payments, idempotent payment retries, refund and recollection.
- A group result and backup, SQLite connection close/reopen, exact competition-state and cash-ledger preservation.
- Retirement during play; a semifinal winner correction rejected without consent and accepted with downstream result invalidation, followed by replayed finals.
- Completion of categories and tournament, reopening and completion again.
- Final backup imported into a second fresh database: identical players, placements and cash, preserved audit history, and SQLite `integrity_check` returning `ok`.

A second scenario produces tied lucky-loser candidates at the cutoff. Two places remain unresolved, premature completion is rejected, manual selection succeeds and the tournament can finish.

The full suite additionally covers cross-category table/player conflicts, all third-place policies, registration locks, Trash, older migrations and invalid-backup rejection. Report checks cover Serbian text, CSV quoting/formula protection and HTML escaping, not physical printing.

## Reproduce

```sh
cargo test --workspace --exclude librett-desktop --offline
cargo test -p librett-storage-sqlite beta2_full_event_simulation_on_disk --offline -- --nocapture
npm run test:cash
npm run test:reports
```

## Limits

The simulation calls the real application and SQLite code using temporary data. The user's live database was neither used nor changed. Desktop UI interaction, installer qualification on all operating systems, abrupt process termination during writes and physical printing were not performed. Importing into a second database checks data transfer, but does not constitute a test on another OS. Closing/reopening a SQLite connection verifies persistence rather than a complete desktop application restart.
