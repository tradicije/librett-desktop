# ADR 0019: One-way public player-registry import

Date: 2026-10-08. Status: Accepted implementation direction at the user's request.

Registry snapshot v1 remains unsigned. Validate the bundled 2020-12 schema and semantic graph with strict bounded Serde parsing before staging. HTTPS requests use GET only, public pinned DNS addresses, no redirects/proxies/compression, bounded responses and timeouts. File and online imports share validation.

Keep registry/remote UUIDs and club relationships separate from fresh local player/club IDs. Do not infer identity from equal names. Stage a private preview; confirm an atomic guarded transaction. Missing or invalid birth years require completion or explicit skipping. Keep optional omissions, local contacts/notes/photos and historical entry snapshots. Track sticky per-field local overrides through SQL triggers covering existing edit paths; a reviewed registry choice can clear them. Remote withdrawal changes provenance and never deletes local players. A local deletion retains a detached source link and is skipped until explicitly imported again.

Schema 22 adds source cache, mappings, club links, staging and receipts. Existing files receive a consistent pre-v22 backup. Source checkpoint and semantic digest are checked at confirmation. Decimal counters remain strings at the UI boundary. Unsigned counters are declared metadata, not authentication or trustworthy first-contact freshness. No registry write/proposal/account channel is introduced.

Photo download is a separate explicit request validated against its descriptor; the existing crop dialog produces validated local JPEG bytes. Source terms/rights are operator declarations. Testing and platform limitations are documented separately; no release/version change is made.
