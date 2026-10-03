# 0013: Guard edits, financial confirmation and image decoding

Status: accepted

## Decision

Workspace navigation, browser/native history, editor cancellation and explicit
reloads ask before discarding local forms or draw drafts. Closing a window or
using Quit checks all mounted workspaces. Browser reload uses beforeunload.
Pending writes block these operations; force termination is outside this guard.

Player writes carry a stable player UUID, request UUID and the complete original
profile snapshot. An immediate SQLite transaction compares the original with
current data, writes the new profile and records the request/result together.
An identical retry returns the original result even after subsequent edits.
Conflicting edits must reload; editor reload requires discard confirmation.
Request history is cascaded away when an unused player is deleted.

Financial settlement carries an expected amount for every selected entry. The
transaction checks all current amounts and rolls back the whole operation on
any mismatch. The UI refreshes balances and requires a new action/confirmation.
Request identity includes expected amounts. Unallocated legacy payment/refund
writes are rejected once an entry has player allocations; negative historical
player balances are rejected rather than automatically charged. Existing
immutable financial records are preserved.

Complete manual knockout drafts reject empty-versus-empty opening matches.
Incomplete manual drafts remain permitted. Round-robin pairings are mounted only
when expanded and generated one round at a time, with at most 32 visible pairs.
Participant labels and dropdown options are shared instead of rebuilt per slot.

Image files are capped at 10 MB. Encoded dimensions are inspected before browser
decoding; dimensions must be at most 6000 per side and 16 million total pixels.
Animated PNG and extended/animated WebP are rejected. Uploaded images are
converted to JPEG: player photos at 512×512, tournament covers exactly
1024×576. A shared modal lets users select the crop through drag positioning,
zoom and keyboard-accessible position sliders, with reset and apply/cancel.
The source rectangle always matches the output aspect ratio. Cancelling
preserves the existing image. Backend validation checks encoded limits,
reads dimensions and fully decodes JPEG with a bounded decoder buffer.

Tournament creation carries a stable UUID, so retrying matching metadata does
not create another tournament. Cards show optional covers, names, active unique
player counts and active category counts in separate rows, with an open button at
the bottom, in at most three columns. Existing 4:3 covers remain readable and are
cropped visually to 16:9.

## Storage

Schema 12 adds player write receipts and expected settlement amounts. Schema 13
adds tournament cover data and accepts either development variant of schema 12. Older schemas receive a pre-v13 backup before migration. Existing
profiles, financial history and draw revisions remain intact.

Tournament Settings edits the name and optional cover. Updates compare the
original metadata atomically, reject conflicting edits, and accept retrying an
already-applied update. The entire tournament card supports mouse and keyboard
navigation; its bottom button also supports opening a new workspace tab.
