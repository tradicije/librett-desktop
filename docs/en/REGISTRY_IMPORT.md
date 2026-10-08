# LibreTT Desktop integration — development

[Srpski](../sr/REGISTRY_IMPORT.md)

Implemented in the sibling [LibreTT Desktop](https://github.com/tradicije/librett-desktop) development source; no integration release is published. See [actual verification](REGISTRY_VERIFICATION_2026_10_08.md).

Registry → Desktop is one way. HTTPS GET/file import never uploads local profiles, contacts, notes, attendance, payments or tournament data. WordPress authority stays separate from local Desktop identities and edits.

Desktop Players offers HTTPS snapshot download, bounded JSON file import, private preview, explicit existing-player mapping, missing-field completion, skip, conflict review and confirmation. Matching names never merge automatically. Local birth year is mandatory and name/club limits are 120 characters; remote optional birth year must be completed when needed. Source club UUID memberships are stored separately and summarized for the existing local club field.

SQLite schema 22 caches approved source data, remote/local mappings, baselines, sticky local overrides, club links and receipts. Existing files get a pre-v22 safety backup. New local UUIDs are independent of remote UUIDs. One local player has one source mapping in this slice. Edits stay local even when later remote values happen to match. Explicitly selecting registry values clears those overrides; absent optional remote values never erase local data. Local deletion leaves a detached source link skipped by default.

Refresh rejects older checkpoints, conflicting equal-checkpoint payloads and regressed/missing known entity revisions/withdrawals. Preview is invalidated by relevant local/source edits and expires after an hour. Confirm is transactional and idempotent. Remote withdrawal updates provenance and preserves local players and historical entry snapshots. This cannot prove first-contact freshness/authenticity: unsigned source metadata is untrusted.

Photos are optional: only an explicit request downloads a descriptor-validated image (hash, size, MIME, dimensions), then the existing crop dialog creates a local JPEG. Existing local photos are retained unless explicitly replaced. Requests are HTTPS-only GET with public pinned DNS addresses, bounded resolver concurrency/timeouts/body, no redirects/proxy/compression; private/reserved hosts are rejected.

JSON bounds: 32 MiB, depth 8, 100,000 aggregate array items, 32 keys/object and 32,768 bytes/string token. Schema and graph checks also enforce typed UUID uniqueness, references, revisions and individual collection bounds. Neither signatures, replica proposals, automatic source migration/discovery nor cached-source re-export are implemented. Use a reviewed new HTTPS URL for a moved source with the same registry UUID.

## Usage

Open Players and expand/import from Registry. Enter the public snapshot HTTPS URL (typically `/wp-json/librett-registry/v1/snapshot`) or choose its JSON file. Inspect dataset/media terms and the unsigned-source notice. Preview pages show 25 records; choose independent local identities or explicitly select an existing player. Complete required local fields or skip records. Choose registry values only for fields whose local override you intend to clear. Request/crop a photo only if wanted.

Apply preview choices before confirmation; changed choices disable confirmation until reviewed again. Confirm the review checkbox and import. Pending previews can be resumed/cancelled; maximum three, valid one hour. Refresh a saved source or explicitly enter its moved URL; registry UUID, not hostname, is identity. Withdrawals preserve local players and tournament history. The existing directory and its registration flows continue to use local profiles.
