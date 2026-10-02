# Category workspace

- A category row opens details with its own history entry. Registrations, Draw,
  Groups (group formats only) and Bracket are available; planned engines are labeled.
- Check multiple available singles players and submit once. Entries and fees all
  commit or all roll back, including failures during later writes in the batch.
- Doubles requires explicitly choosing each pair; players cannot be repeated in
  queued pairs or existing registrations. Singles/doubles names may be identical.
- Already registered players (including withdrawn entries) stay marked and reserved.
- Delete confirmation can be cancelled without writes. Empty categories are removed;
  used categories are archived. Their IDs, snapshots, attendance and cash survive.
- New registrations in archived categories fail; financial accounts remain visible
  with their category discipline and archive status. Revisiting a removed category
  in history displays an explanation and a way back to the tournament.
- An account with a 1000 RSD fee, 100 RSD discount and 400 RSD received defaults to
  500 RSD payment. Changing accounts resets the amount and note. Paid/free accounts
  do not suggest another payment. Partial payments remain editable.
- Empty cash notes save successfully, overlong notes fail. Uncertain cash requests
  retain their UUID and exact amount during retries; no double payment is created.
- v6/v7 migration preserves all existing entries and fees, creates one pre-v8 backup,
  restores foreign-key enforcement and retains immutable cash triggers.
- Case/whitespace variants of a name in the same discipline are rejected by domain
  and database checks. Different formats do not exempt duplicate names.
