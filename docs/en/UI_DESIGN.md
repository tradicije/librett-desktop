# Desktop interface direction

Registrations rely on the category tab divider; the registration section has no
extra top border or offset, regardless of intervening dialog elements.

Group qualifiers display their origin before their name in the knockout bracket
(A1, B2, etc.). The prefix follows the entrant through later rounds; unresolved
places show the same prefix with a short awaiting-qualifier label.

LibreTT is an organizer's working tool. Its interface should help a person find
an entry, confirm attendance or record a payment quickly during a busy tournament.
The visual hierarchy follows those tasks rather than promotional content.

## Principles

- Keep the provided LibreTT light/dark palette, Libre Franklin and Tabler icons.
  Primary color identifies an action, selection or focus; it is not a decoration.
- Use a persistent, compact context toolbar and a viewport-height sidebar.
  The initial competition-mode selection retains its sidebar-free layout.
- Give names, account amounts and actionable controls clear priority. Use quieter
  secondary text for supporting details and tabular numerals for financial totals.
- Favor rows, separators and modest surfaces. Reserve panels for meaningful groups
  such as a list beside an input form. Use a small, consistent corner radius.
- Keep hover, disabled, selected, focus, error and busy states visible. Do not rely
  on color alone: selected accounts expose aria-pressed and status uses text.
- Group player fields by profile, contact and additional information. Keep full
  details in the editor; show a concise summary in the directory.
- Preserve native scrolling, keyboard controls, navigation history, translated
  accessible labels and the protection against leaving uncertain cash writes.
- Stack forms and lists on narrow windows. Honor reduced motion and keep decoration
  out of the way of real information.

## References

Linear, Raycast, Figma, GitHub Desktop, Notion and Arc are direction references
requested by the project author, not templates or sources of copied assets.
The interpretation for LibreTT is restrained chrome, readable working lists,
clear context and compact actions. For examples of organizing information through
lists and views, see [Linear display options](https://linear.app/docs/display-options)
and [custom views](https://linear.app/docs/custom-views).

The implementation has no gradients, decorative statistics, promotional hero
sections or ornamental illustrations. Mode cards represent actual module choices;
counts and financial figures always come from current data.

## Current coverage

Mode selection, tournament-level tabs, category tabs, the player directory,
player create/edit screens and the cash desk share this system. Tournament tabs
keep Settings, Overview, Categories and Cash desk available in the tournament
context. Draw, Matches and Results live inside each category; unfinished competition
sections state their development status.
The backend model and existing application workflows are unchanged by this redesign.
Browser checks use temporary synthetic data outside the repository; production
screens contain no seeded demonstration records.


## Native history

Screen and tab history uses the webview History API. Toolbar arrows and mouse
back/forward buttons share it; macOS also enables native trackpad navigation.
Pending writes restore the current history position before changing the screen.


## Workspace row

The fixed 48px row starts with pinned Home, followed by independent working tabs
with short page labels and full-context tooltips. Back/Forward stays in the toolbar
below. Home never appears as an ordinary tab. macOS native window controls share
the row and are centered using AppKit coordinates. Linux and Windows use
application-drawn controls in the same row, invoking native window actions.
Windows controls are aligned on the right with rectangular buttons.
Content heights account for this row, with overflow only when
content exceeds the viewport. Hidden workspaces preserve their local state.

Working tabs are a fixed 180px wide regardless of label length. Labels fade at the
right only when their measured text overflows; they do not use an ellipsis. The
full page/context is available in the tooltip, and the tab strip scrolls horizontally.


The sidebar supports expanded and 68px compact modes. Expanded mode places the
collapse control beside the full logo with a 16px gap; compact mode uses the theme-specific square
brand icon as the expansion button and icon-only navigation with accessible
names/tooltips. Brand images do not navigate to Home. The shared
layout preference persists across workspaces and restarts.

The expanded sidebar footer shows LibreTT for Windows, LibreTT for MacOS or
LibreTT for Linux according to the platform, between the local-work label and
the copyright. This label keeps the same wording in both interface languages
and is hidden in compact mode.


Category rows share player profile card spacing, avatar size, typography and
responsive action layouts. Edit/Delete use the same icon-labeled secondary buttons.
Category creation/editing has a dedicated screen; Groups has its own tab with a maximum of three card columns; Draw shows only the bracket.


Category Settings uses one outer panel, with flat sections separated by spacing
and dividers. Participant lists and registration forms belong only to Registrations. Category
creation/editing shares the player editor form groups, fields and action styles.


Seed and arrangement settings use numbered seed rows with move/remove actions,
shared dropdowns and separate generation/save action rows. Draft status sits
above the manual slots. Group slots use up to three columns; knockout opponents
are paired side by side, stacking on narrow windows.

## Match scoring and player identity

The Matches tab filters by group and round, with up to 32 matches per page.
Cards show both entrants, set scores and explicit completion/retirement/walkover
status. A single dialog handles set entry and corrections, with separate
confirmation when changing a knockout winner clears downstream results.
Completed sets have an icon and the running set total updates during input.

Player labels append the first three club letters/digits, uppercased, in
parentheses. `PlayerName.svelte` renders that suffix in smaller muted type.
Lists, registration/attendance, seeds/selectors, groups/pairings, brackets,
match dialogs and cash confirmations use the same display. Stored names and
profile name inputs remain plain; players without clubs have no suffix.

Group tables include live ranking and a manual/automatic Ranking dialog.
Knockout round selectors use competition names rather than group round numbers.

The bracket scroll region has no enclosing panel. Match cards and connecting
lines provide structure; player club codes appear only beside names.

The Draw toolbar uses a short rules summary, icon-only Refresh and Edit.
A brief pending-qualifier notice replaces duplicate headings and explanatory copy.

Category age restrictions use an off-by-default switch and inline From/To
fields. Out-of-range registration uses one confirmation dialog for the selected
players, with their club labels, ages and an explicit Register as exception action.
