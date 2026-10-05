# ADR 0017: Final standings and competition completion

Status: accepted

## Decision

Final standings derive from the same knockout projection used by Draw/Matches.
A valid/current draw, completed and resolved groups, settled qualifier/LL slots
and N−1 valid knockout results for N actual entrants, plus a configured bronze
match when two semifinal losers exist, are required, with a played,
retired or walkover final. Participant coverage and uniqueness are checked.

The winner is first, the losing finalist second, and losing semifinalists share
third place by default. Category rules can instead require a bronze match or
award third place to the semifinalist beaten by the eventual champion; the
other semifinalist is fourth. No unplayed result is invented. Other elimination rounds
share ranges based on actual loser counts. Group non-qualifiers share the
remaining range; no cross-group ranking policy is invented for final standings.
Doubles are entries/pairs. Names/clubs come from the immutable draw snapshot.

Completion is explicit and separate from scoring the final. Category confirmation
stores frozen results and a UTC timestamp. Tournament confirmation requires at
least one active category and all active categories already confirmed. Empty or
unused active categories must be removed/archived before closure.

Completed categories lock entries, category metadata/rules, draws, group orders,
LL choices and match writes. Completed tournaments additionally lock new category
creation/removal and attendance. Cash operations, global player profiles and
tournament cover/name metadata remain available. Reopening requires explicit
confirmation; reopen a tournament before reopening its categories. Opening a
parent does not open children automatically. Existing final snapshots remain in
append-only history; subsequent completion captures a new snapshot.

Schema 17 adds completion revision/timestamp/snapshot columns to categories and
tournaments, plus immutable completion_history. Existing records default open.
On-disk migrations create a pre-v17 consistent SQLite snapshot. Sporting-table
triggers enforce locks in addition to application guards, protecting stale tabs.

Immediate write transactions recompute readiness and compare expected revision
and deterministic version tokens (draw ID, stale state, rules/filler/result/group
order versions, completion and parent revisions). Tournament tokens cover the
active category IDs and each category token. Immutable request UUIDs use the
existing match_writes receipt mechanism; exact retries return the stored response,
UUID reuse with different payloads rejects, and stale dialogs cannot close a
changed result set. Reopen/close history is never overwritten.

## Interface

Category Results shows progress, blockers, podium and standings. Completion and
reopening use explicit modal confirmations with guarded retry behavior.
Tournament Overview shows category progress, winners and result links, plus the
parent completion control. Status badges and workspace refresh update all open
views; mutation controls become read-only without blocking navigation.

## Deferred

Consolation matches, exact ordering within shared ranges, export,
printing, table scheduling and release installers remain separate work.
