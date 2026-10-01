# Contributing to LibreTT

[Srpski](CONTRIBUTING-sr.md)

Contributions are welcome: real tournament examples, rules, workflow feedback,
documentation, translations, bug reports, and implementation when development
begins. The project is in early development; see
[development setup](docs/en/DEVELOPMENT.md) for requirements and check commands.

## Before making a change

Read the [README](README.md) and the
[application plan](docs/DESKTOP_TOURNAMENT_PLAN_SR.md). Describe the problem,
the proposed behavior, and a concrete example. Label future features clearly;
do not describe planned functionality as implemented.

For business rules, specify valid cases, invalid cases, and edge cases before
implementation. Record changes to architecture in an architecture decision
record under `docs/adr/` before implementing them.

## Design principles

- Core tournament operation must work without internet access or an account.
- Domain rules must not depend on Tauri, SQL, HTTP, or the user interface.
- Desktop and companion entry points must use the same validated use cases.
- Keep confirmed results and correction history; rebuild derived standings.
- Preserve the rules and player information used by historical competitions.
- Treat registration, attendance, and payment as separate concerns.
- Support manual and automatic draws through the same structural validation.
- Keep data portable and document migrations, backups, and recovery behavior.
- Preserve community access to records independently of LibreTT's survival.

## Documentation and languages

Keep `README.md` and `README-sr.md` consistent. Maintain contribution guides
and affected user documentation in English and Serbian. Write `CHANGELOG.md`
entries in English under `Unreleased` until a release is made.

All user-facing functionality must support Serbian and English, including
validation messages and printed output. Use stable translation keys rather
than storing translated domain states. User-entered names are not translations.

## Verification and review

Once implementation starts, add meaningful tests for business rules and
regressions. Verify persistence changes with migration and recovery scenarios.
Platform-specific changes require checks on the affected operating systems.
Development setup and exact check commands must be documented when the tooling
is introduced.

A proposed change should explain its purpose, resulting behavior, verification,
and any remaining limitations. Include relevant documentation and changelog
updates. Keep unrelated changes separate.

When sharing examples or reporting an issue, use synthetic or anonymized
player data and include steps, expected behavior, actual behavior, and relevant
platform/version information.

## License and attribution

LibreTT is authored by Aleksa Dimitrijević and licensed under
`AGPL-3.0-or-later`; see [LICENSE](LICENSE). Contributions must be compatible
with that license. Preserve existing attribution and identify the source and
license of any third-party material.
