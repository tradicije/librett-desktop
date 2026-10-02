# 0010: Persist category draw drafts as immutable revisions

Status: accepted

## Context

Organizers need automatic and manual group/knockout layouts, ordered seeds,
reviewable byes and locally saved work before the match engine is introduced.

## Decision

The domain creates and validates a CategoryDraw containing active entry snapshots,
ordered seed IDs, fixed group capacities or first-round knockout slots, settings,
a generation version and UUID random seed. Automatic proposals use balanced
groups or separated knockout seeds and award byes to the strongest seeds.
Manual proposals can remain incomplete and reuse the same structural validation.
Both singles and doubles use registration IDs as participants.

Application use cases check category ownership and canonicalize snapshots.
SQLite schema 10 appends immutable JSON revisions with a unique category/revision
pair. Saving checks the expected revision and active membership inside a write
transaction. Request UUIDs make identical save retries idempotent. Existing
schemas receive a consistent pre-v10 backup before migration.

The category Draw screen previews, edits and saves drafts. Confirmation, match
records, scoring, group ranking and advancement are separate future work.

## Consequences

History is preserved and stale layouts cannot silently replace a newer revision.
Manual drafts may have unassigned slots and are not playable matches. Registration
changes require regeneration. Unsaved edits are local to the open draw screen.
The draft payload is versioned by generation algorithm; later engines must validate
completed layouts before creating matches.
