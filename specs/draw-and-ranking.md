# Category draws and group ranking

Status: agreed product rules; implementation pending.

## Draw modes

- Every category offers both automatic and manual draw setup, regardless of
  competition format.
- The organizer can review the proposed draw before confirming it.
- The organizer manually marks entries as seeds and orders them by strength.
- Automatic knockout draws award bye positions to the strongest seeded entries.
- The organizer can manually assign bye positions and arrange entries.
- Automatic and manual draws use the same validation for category membership,
  duplicate participants, stage capacity and preservation of started matches.

## Groups

- For a groups-then-knockout category, the organizer chooses the number of
  groups and the number of entries that advance from each group.
- Automatic group allocation keeps group sizes as close as possible.
- Automatic group allocation places seeded entries in separate groups before
  distributing unseeded entries.
- Group ranking criteria are configurable for each category.
- The default order is head-to-head results, then sets, then points.
- Set and point criteria use the ratio of won to lost sets and points.
- A positive number of wins against zero losses is the highest possible ratio;
  zero wins and zero losses remain tied on that criterion.
- For three or more tied entries, head-to-head comparison uses a mini-table
  containing only those tied entries.
- If a mini-table separates some entries but leaves others tied, apply a new
  mini-table only to the remaining tied entries before moving to sets and points.
- If automatic knockout needs more bye positions than there are seeded entries,
  assign the remaining byes randomly. The organizer can review and change them.

## Details still to define

- Whether and how club separation affects automatic draws.
- How to handle an exact tie after every configured ranking criterion.
