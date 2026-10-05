# ADR 0016: Lucky loser knockout filling

Status: accepted

## Decision

Grouped categories choose BYE (serde-compatible default) or Lucky loser in
versioned rules. Direct qualification determines bracket size. Spare positions
are the BYEs from the original seed layout; existing same-group separation
only swaps qualifier positions, preserving spare-position identities.

Automatic candidates require every group to be complete and resolved. Exclude
direct qualifiers; compare group place ascending, then wins/played, sets for/
against and points for/against descending. Ratios use integer cross products
with existing infinity/zero semantics. These cross-group comparisons intentionally
do not use head-to-head. An exact cutoff tie leaves the entire cutoff cohort
unselected until the organizer chooses. Ties wholly within the selected cohort
retain stable group order for placement, without deciding qualification by chance.

Per-position overrides are explicit BYE or an eligible entry UUID. Missing keys
mean automatic. Reserve manual entries before automatic selection, enforcing
uniqueness and excluding direct qualifiers. Exhausted candidates become BYEs;
unresolved and invalid manual selections remain unsettled. Eligible manual
choices survive scoring/ranking changes. Switching modes retains dormant choices;
a new draw has none. LL group/place metadata propagates through knockout rounds.

Schema 16 adds `knockout_fillers`, keyed by immutable draw ID with a revision
and JSON choices. On-disk upgrades create a pre-v16 consistent SQLite backup.
No backfill changes previous entrants; old rules/results default to BYE.

Writes are immediate transactions using existing immutable `match_writes`
receipts. Requests guard draw ID, rules/filler revisions, group result/order
versions and the sum of all current match revisions. Replays return the stored
response; mismatched UUID reuse and stale state reject. Group/scoring/rules
changes recompute the projection. When participant changes invalidate recorded
knockout results, explicit confirmation is required before appending null result
revisions in the same transaction. Historical results and receipts remain.

## Interface

Category create/edit/settings share the mode selector. Settings exposes each
spare position, All automatic, candidate statistics and an impact dialog.
Incomplete groups and cutoff ties remain visible as pending slots in the bracket,
never automatic walkovers. Draw and match projections share the same domain
filling function; participants cannot disagree between these views.

## Limits

This is an organizer policy rather than a published federation regulation.
The existing 4096-entry category draw limit remains. No global tournament
player limit or maximal-size performance qualification is introduced.
