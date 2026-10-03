# 0012: Separate category configuration from the visual draw

Status: accepted

## Decision

Tournament navigation contains Overview, Categories and Cash desk. Categories is
an action list; Add and Edit open a dedicated editor using the same navigation
and unsaved-change guards as player editing. Registrations belong to categories.

Creation stores category metadata and validated rules atomically. Rules specify
round-robin group count, qualifiers per group, best-of sets, points to win, winning
margin and ranking order. Defaults rank tied participants by their mini-table,
then set ratio and point ratio. These are configuration values; actual scoring,
rankings and progression will be implemented with the match engine.

Category Settings contains rules, registrations, ordered seeds and the existing
automatic/manual arrangement editor. Organizers save rules before generating a
new layout. Automatic knockout placement awards byes to the strongest seeds;
manual positioning remains available. Saved draw revisions remain immutable.

Categories using groups have a separate Groups tab for participants and
round-robin pairings, arranged in up to three columns (two or one on smaller
windows). Draw displays only the full knockout bracket with connectors and
horizontal scrolling, including for categories using groups. Group qualifier slots are labeled projections, never inferred winners.
Stale arrangements retain their saved group counts and show a warning until a
new revision is generated. Unknown match winners remain placeholders.

SQLite schema 11 stores category rule revisions. Migration preserves group counts
from existing draw drafts and supplies defaults elsewhere, after a pre-v11 backup.
Optimistic revisions protect edits across workspaces; retries of matching writes
are idempotent. Draw writes recheck membership and saved group rules inside the
transaction. Metadata and rules update together. Discipline/format changes are
blocked once any registration exists. Fee changes affect future registrations;
existing financial history is preserved.

## Consequences

Category setup is separate from presentation. Participants and seeds can be
prepared after category creation without crowding the categories list. The draw
can be inspected before results exist, but it cannot yet determine actual group
qualifiers or final standings. Changes in rules or registrations require a fresh
arrangement before saving a new draw.
