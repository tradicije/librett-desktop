# ADR 0018: Bracket editing and placement rules

Status: accepted

## Decision

Draw editing keeps the knockout bracket visible. Only opening-round slots are
assigned; later rounds derive from winners. Selecting an already placed entry
swaps positions. Direct knockout editing saves a new immutable draw revision
and explicitly confirms the result reset. All registered draw participants must
remain assigned before saving.

After completed, resolved groups, organizers can override any opening-round
qualifier or vacant place with a registered category entry or BYE. Overrides
reuse knockout_fillers, optimistic versions and immutable request receipts.
Dependent results are invalidated only with explicit confirmation. Overrides
reserve identities before automatic filling to prevent duplicate participants.

The category switch selects BYE when off. When on, a dropdown selects automatic
Lucky loser or manual selection. Manual filling keeps unspecified vacant slots
pending, including when there are too few candidates; the organizer must choose
entries or BYE. Direct knockout has no eliminated group pool and keeps BYE.

The third-place switch defaults off (shared third). When enabled, a dropdown
selects a bronze match or third place for the semifinalist beaten by the eventual
champion. Bronze participants derive from semifinal losers. Its key is the final
round's ko:<round>:1; the championship final remains ko:<round>:0. Semifinal
corrections invalidate dependent bronze results through the existing projection
and history mechanism. Final placements and completion include the bronze result.
If only one semifinal loser exists because of BYEs, no extra match is invented.

These rules extend JSON configuration with serde defaults. Existing stored
configurations/results continue to deserialize without a database migration.
