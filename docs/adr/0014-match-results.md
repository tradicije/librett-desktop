# ADR 0014: Match scoring and guarded corrections

Status: accepted

## Scheduling

Matches are projections of a complete saved draw. Incomplete arrangements or
changed registration/group settings disable scoring. Singles and doubles share
entry identifiers; an entrant can represent one player or a pair. Group matches
use the circle method for one round, with 32 matches per page. Group result history also supplies the standings projection. Knockout-only draws resolve first-round byes,
saved winners and subsequent opponents; unsettled feeder matches cannot produce
a bye. Group standings and group-to-knockout qualification are defined in ADR 0015.

## Validation

Category rules determine the odd best-of length, winning points and margin.
A set ends at the first score meeting both point and margin requirements, including
deuce. No additional sets are allowed after a match is won. Played matches require
a winner with the requisite completed sets. Retirement records an explicit winner
and any played sets, with at most one incomplete final set. Walkover records a
winner and no sets. Input points are integers from 0 to 999. Each saved result
includes the scoring rules used; later configuration edits do not rewrite history.

## Persistence and conflicts

Schema 14 appends result revisions scoped to a draw UUID and structural match key.
Draw revisions isolate historical results from regenerated arrangements. A new
arrangement starts with no results; prior revisions remain in SQLite. Requests
carry a UUID, expected result revision, draw UUID and rules revision. Immediate
transactions recheck ownership, registrations, the arrangement, opponents and
rules before saving. Receipts replay the exact result after an uncertain response;
reusing a request UUID with changed fields is rejected.

Changing a knockout winner checks downstream matches along that path. If any
have results, saving requires explicit confirmation. Confirmed corrections append
null result revisions to those matches and save the corrected result atomically.
Changing scores while keeping the same winner preserves subsequent results.
Concurrent corrections produce `match_conflict` and require reloading; uncertain
writes retain immutable request fields and block navigation until retry succeeds.

## Interface

The Matches tab uses cards and a modal editor with set rows, a running set total,
completion indicators, outcome selection and explicit winner selection for
retirement/walkover. Score bindings replace the relevant row explicitly to avoid
nested binding name conflicts. Dirty cancellation asks for confirmation. The
knockout bracket displays set scores, advancing player labels and the champion.
Final placements, match/table scheduling and result export remain separate work.
