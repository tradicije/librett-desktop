# Development verification — 2026-10-08

[Srpski](../sr/REGISTRY_VERIFICATION_2026_10_08.md)

Authorized implementation/testing of all six requested catalogue/Desktop steps. No production release was made. Tests use isolated synthetic databases/files; no installed user Desktop database is opened.

## Results

| Check | Actual result |
| --- | --- |
| Registry PHPUnit unit | 47 tests / 80 assertions passed |
| Registry WordPress/MariaDB integration | 41 tests / 142 assertions passed |
| PHPStan | Maximum level, all src classes, no errors |
| PHP CS Fixer | 120 PHP files, zero changes required; supported PER-CS 3.0 ruleset, PER 3.1 remains the reviewed target |
| Composer | Strict manifest/lock validation passed; update/install audit reported no known advisories at execution |
| Gettext | Serbian catalogue msgfmt --check compiled successfully |
| Desktop Rust workspace | 51 tests passed: application 2, domain 5, SQLite 43, native HTTPS guard 1; doc-tests also passed |
| Desktop native Linux | cargo check passed with DBus/GTK/WebKit native dependencies in Fedora 44 rootless container |
| Desktop TypeScript/Svelte | 0 errors / 0 warnings |
| Desktop Vite | Production frontend build passed |
| Desktop Rustfmt | Applied and checked across workspace |
| Desktop Clippy | Workspace/all-targets exits successfully; existing tournament/domain/backup lint warnings remain. Strict -D warnings initially failed on these pre-existing warnings; new registry modules have no remaining Clippy warnings |
| Cross-project transfer | Actual synthetic WordPress export imported into Desktop, missing birth year completed, same-name identities kept separate, memberships retained, restart and SQLite backup restore passed |

Environment: PHP 8.5.11 / PHPUnit 13.4.1 / PHPStan 2.3.0 / PHP CS Fixer 3.95.27 / Composer 2.10.3 / gettext 0.21; WordPress 7.1.3 single-site / MariaDB 10.11.19 InnoDB / GD. Rust/Cargo/Clippy/Rustfmt 1.98.1 in Fedora 44; Vite 6.4.3 with existing locked frontend dependencies.

## Commands

Registry root:

```sh
tools/dev/run php vendor/bin/phpunit
tools/dev/run php vendor/bin/phpunit -c phpunit.integration.xml
tools/dev/run php vendor/bin/phpstan analyse --debug --memory-limit=512M
tools/dev/run php vendor/bin/php-cs-fixer check --sequential
tools/dev/run composer validate --strict --no-check-publish
tools/dev/run msgfmt --check --output-file=languages/librett-player-registry-sr_RS.mo languages/librett-player-registry-sr_RS.po
```

Desktop: `cargo test --locked --workspace`, `cargo check -p librett-desktop --locked`, `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets` in the native-dependency container; `npm run check` and `npm run build` in apps/desktop. The tracked `tools/dev/registry-check` wrapper reproduces the container workflow; tests here used the same image with the existing Cargo cache mounted.

## Meaning of coverage

Identity/setup, private CRUD validation/permissions, stale revisions, revision exhaustion, audit rollback, bounded search, archive/restore and initial/draft migrations are exercised. Catalogue cases cover feature schema resume/checksum, POST/nonce/anonymous denial, UUID relationships, rollback on invalid links, policy/age authorization, private projection exclusion, no-op public revisions, withdrawal and protected media/license behavior. JSON cases execute schema fixtures and malformed/duplicate/UTF-8/depth/string/collection/reference checks; import cases exercise explicit mapping, receipts, repeated import/local changes, expiry/staleness and transaction failure.

Desktop exercises local overrides/omissions, manual profiles, duplicate names, local deletion, stale previews/checkpoints, source URL changes, known-entity regression/lost withdrawal rejection, atomic rollback, actual immutable entry-member snapshots and registry cache backup/restart. Native network tests verify reserved/private/documentation address rejection; no live HTTPS service or redirect/decompression attack harness was exercised. Address policy was reviewed against the [IANA IPv6 special registry](https://www.iana.org/assignments/iana-ipv6-special-registry/).

## Limits and delivery

No interactive browser/GUI acceptance or macOS/Windows native checks here. Host native compilation initially lacked DBus development files; container compilation resolves this development prerequisite. MySQL 8.4, multisite, production performance and operator data/rights approval remain unverified. No signature/replica/recovery, purge/private restore, orphan cleanup, source trust/freshness guarantee or production installer is claimed.

Local source changes are ready for review/commit in both repositories; no commit, push, version change, publication or user-data migration was performed by this task. Suggested commits: registry `feat: implement single-site player catalogue and public snapshot workflows`; Desktop `feat: add one-way player registry import and refresh`.
