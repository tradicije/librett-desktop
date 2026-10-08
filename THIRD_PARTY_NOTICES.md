# Third-party notices

LibreTT is licensed under AGPL-3.0-or-later. Third-party dependencies retain
their own licenses; LibreTT does not relicense them.

## Tabler Icons

- Package: `@tabler/icons-svelte` (version pinned by `package-lock.json`).
- Project: https://github.com/tabler/tabler-icons
- License: MIT.
- Full notice: [tabler-icons-MIT.txt](docs/licenses/tabler-icons-MIT.txt).

The application bundles selected Tabler SVG components. Preserve the MIT
copyright and permission notice with distributed copies.

## Libre Franklin

- Package: `@fontsource-variable/libre-franklin` (version pinned by `package-lock.json`).
- Project: https://github.com/impallari/Libre-Franklin
- License: SIL Open Font License 1.1.
- Full notice: [libre-franklin-OFL.txt](docs/licenses/libre-franklin-OFL.txt).

The variable font, including Latin Extended characters for Serbian, is bundled
locally and works without an internet connection. Preserve its license with
redistributed font files.

This document currently records UI icon and font attribution. A complete dependency
license inventory must be prepared before publishing release installers.

## Registry import dependencies

Direct Rust additions are pinned in Cargo.lock: jsonschema, url, chrono, sha2, reqwest and image. Their license copies are retained below; transitive release-license inventory remains required. Serde/serde_json and base64 were already used. Registry schema and fixtures retain Aleksa Dimitrijević AGPL-3.0-or-later attribution.

- jsonschema 0.58.6: [LICENSE](docs/licenses/jsonschema-0.58.6-LICENSE.txt).
- url 2.5.8: [LICENSE-APACHE](docs/licenses/url-2.5.8-LICENSE-APACHE.txt).
- url 2.5.8: [LICENSE-MIT](docs/licenses/url-2.5.8-LICENSE-MIT.txt).
- chrono 0.4.45: [LICENSE.txt](docs/licenses/chrono-0.4.45-LICENSE.txt).
- sha2 0.10.9: [LICENSE-APACHE](docs/licenses/sha2-0.10.9-LICENSE-APACHE.txt).
- sha2 0.10.9: [LICENSE-MIT](docs/licenses/sha2-0.10.9-LICENSE-MIT.txt).
- reqwest 0.13.5: [LICENSE-APACHE](docs/licenses/reqwest-0.13.5-LICENSE-APACHE.txt).
- reqwest 0.13.5: [LICENSE-MIT](docs/licenses/reqwest-0.13.5-LICENSE-MIT.txt).
- image 0.25.10: [LICENSE-APACHE](docs/licenses/image-0.25.10-LICENSE-APACHE.txt).
- image 0.25.10: [LICENSE-MIT](docs/licenses/image-0.25.10-LICENSE-MIT.txt).
