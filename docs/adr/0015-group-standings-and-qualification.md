# ADR 0015: Group standings and knockout qualification

Status: accepted

Group tables are rebuilt from the latest result revisions for the current draw.
Rows contain matches played, wins/losses, sets and points for/against. Wins are
the primary order; equal wins use the category's configured criteria. The
head-to-head criterion builds a mini-table restricted to the tied entries;
subsequent set and point ratios use that mini-table. Ratios compare integer
cross-products, with nonzero/zero treated as infinity and 0/0 as zero.

Retirement and walkover count as wins/losses. For ranking statistics the winner
receives the remaining sets and target points; recorded played points remain.
These projected statistics never rewrite the stored match score. The organizer
can override the final group order manually. Unresolved exact ties are marked,
and qualifiers stay pending until the organizer resolves the order. No player
is selected arbitrarily when all configured criteria tie.

A group becomes eligible for qualification when every pair has a result and its
final order is resolved. The saved qualifier count determines bracket places.
First qualifiers are separated by seed positions, with a same-group opening
opponent adjustment matching the existing structural bracket. Real byes are
separate from pending group places, so an unfinished group cannot grant a bye.
The Matches tab exposes Groups and Knockout phases; knockout rounds use their
competition names (round of 32/16, quarterfinals, semifinals and final).

Match correction projects the new group order and knockout opponents within the
write transaction. Any saved knockout result with changed opponents, or whose
feeder result is invalidated, requires explicit confirmation before a null
revision is appended. Unaffected knockout results are preserved. Editing a group
result restores that group's automatic order, making an earlier manual override
explicitly obsolete. Result history remains available in SQLite.

Schema 15 adds versioned manual group orders. Requests carry an immutable UUID,
draw ID, expected order revision and a version sum of group result revisions.
An automatic/manual ranking action uses the same transactional correction and
retry protections as match results. Upgrades create a pre-v15 SQLite backup.

The group view and bracket read a shared CompetitionState projection. Successful
writes notify other mounted workspaces, which refresh when no local edit is open.
Views also reload on activation and include a Refresh action. Final category
placements and archival export remain separate work.
