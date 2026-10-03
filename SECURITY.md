# Security policy

[Srpski](SECURITY-sr.md)

## Supported versions

LibreTT is in early development. There are no supported stable releases or
published installers yet. Security fixes are applied to the current `main`
branch; update development checkouts before reporting an already-fixed issue.
The package version `0.1.0` does not imply a stable or audited release.

## Reporting a vulnerability

Use **Report a vulnerability** on the repository's
[Security page](https://github.com/tradicije/librett-desktop/security) when
GitHub private vulnerability reporting is enabled. Availability depends on the
repository settings; this policy does not enable it or guarantee that it is on.
If the private reporting button is unavailable, open a minimal public issue
requesting a private contact from Aleksa Dimitrijević, without vulnerability
details, exploit code, or private data. No separate security email is published.

Include the affected commit/version, operating system, reproducible steps using
synthetic data, expected and observed behavior, and the potential impact. Share
proofs of concept privately. Do not attach real player photos, contact details,
SQLite databases, backups, credentials, or personal filesystem paths to public
issues. Reports in Serbian or English are welcome. There is no guaranteed
response time or bug bounty program at this stage.

## Current security boundary

The desktop application is offline-first and trusts the local operator and OS
account. It has no authentication, enforced administrator roles, database
application-layer encryption, or implemented LAN/companion/cloud service.
The Vite development server is a development tool, bound to loopback by default;
it is not an administration service to expose on a public or club network.

Profiles and photographs are stored in the local SQLite database, outside the
checkout. Pre-migration backups contain the same personal data. Protect those
files with OS account permissions, appropriate device storage protection, and
controlled backups. The repository ignores local databases and secrets.

Production web content has a configured Content Security Policy, and SQL writes
use bound parameters. Domain/application validation protects names, player
membership, duplicate registrations, and profile bounds. These controls do not
constitute a security audit or protection from a compromised OS account.

Image uploads have file and encoded dimension/pixel limits before browser
decoding. Photos and covers are resized/cropped into bounded JPEG data URLs;
the backend verifies dimensions and performs a full JPEG decode with memory
limits. Animated PNG and extended/animated WebP are rejected. External photo URLs are not accepted. Treat future file
imports and remote synchronization as new trust boundaries requiring validation.

Before companion access or release distribution, review authentication and
permissions, dependencies, packaging/signing, untrusted imports, and disclosure
handling. Planned capabilities are not current guarantees.

[GitHub private vulnerability reporting documentation](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately).

## Dependency audit notes (2026-10-03)

The npm audit returned no known vulnerabilities. The Rust dependency audit found
[an iterator soundness issue in glib 0.18.5](https://rustsec.org/advisories/RUSTSEC-2024-0429.html),
used by the Linux GTK dependency chain; the upstream fix requires glib 0.20 or later.
The application does not call the affected iterator, but transitive use has not
been ruled out. This remains an upstream limitation for Linux builds.
[proc-macro-error 1.0.4 is unmaintained](https://rustsec.org/advisories/RUSTSEC-2024-0370.html)
and remains a transitive build dependency. These findings are not resolved by
the application fixes.
