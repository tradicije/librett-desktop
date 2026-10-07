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

Grouped categories offer one BYE/Lucky loser selector in the shared rules form.
Lucky loser places form a flat section in Settings, with two columns of per-place
selectors, Automatic / eligible player / BYE options and All automatic / Save
places actions. Candidate statistics live in an expandable table. A short notice
explains pending cutoff ties or invalidated manual choices. Result-impact dialogs
require explicit confirmation and explain preservation of result history.
Bracket origins display LL C3 before the player name, keeping club typography.

Results uses a flat progress/status strip, concise blocker list, winner/finalist/
shared-bronze cards and a standings table. Completion lives below the standings;
reopening uses a separate explicit confirmation. Tournament Overview shows up
to three category cards per row, completion counts and result links. Closed
sporting forms disable mutation controls, while navigation and the cash desk
remain available. Completed badges appear on category and tournament cards.


Short category and scoring summaries stay inline and wrap on narrow windows. Player details and group/knockout progress use separate labeled rows where this improves readability. Each played set has its own score tile.

## Bracket editing and third place

Open a category’s **Draw → Edit**, then click a player or BYE in the opening round. Choose from this category’s registrations; selecting an already placed participant swaps positions. Save changes explicitly. Later rounds follow match winners and cannot be assigned independently. Direct knockout edits create a new draw revision, retaining previous results in history. Group knockout edits are available after all groups finish and their standings are resolved; changes that invalidate dependent results require confirmation.

Category creation/settings display three switches in one row (stacked on narrow windows): age group, Lucky loser/BYE, and a single third place. Lucky loser is available for groups → knockout; direct knockout uses BYE. With the third-place switch off, semifinalists share third place. With it on, choose a bronze match or award third to the semifinalist beaten by the eventual champion. The other semifinalist is fourth. A bronze match is shown beside the final round in Matches and below the bracket; completion waits for its result. If there is only one semifinal loser because of BYEs, that participant is third without an extra match.

Enabling the Lucky loser switch reveals automatic or manual filling. Manual mode leaves vacant places awaiting a participant or BYE. Player directory cards show only name/club and age (current year minus birth year); other details are in the profile.

The bracket participant picker uses an inline dropdown whose list height adapts to the viewport; the list owns scrolling without expanding an absolutely positioned menu beyond the dialog.

Player age numbers use the theme’s primary text color, with a muted year/years unit. Serbian uses godina for endings 1 and 5–9/0, godine for endings 2–4, and godina for 11–14.

Backups are in the sidebar. Match editors expose a table number and independent save; report buttons are next to Groups/Draw, Results and Cash Desk. Print previews have a separate native window.

## Action history

History uses the shared heading, panel, Select, secondary button, pill and player-name components. One filter panel contains tournament/history-type/action dropdowns. Events are grouped by local calendar date, with a compact type icon, explicit history type, action, subject and tournament/category context. Before/after details expand inside the row instead of nesting more bordered panels. Theme tokens cover both light/dark modes; narrow windows stack filters, timestamps and change columns. Older history is identified without presenting invented actions or operator identities. Dropdown labels have explicit spacing above their controls. Compact event rows place the action and subject, tournament/category context, and type/time in three columns; dates stay in group headings. On narrow windows the context moves below the subject.

Refresh and reload controls use the circular refresh icon. Undo arrows identify restoring or reopening an item, while a crossed circle marks removal of attendance confirmation.

Started categories cannot be removed. Saving a replacement draw in Settings explains that scores and table assignments do not carry into the new revision and requires confirmation. Scoring changes after the category starts explain that saved scores keep their original rules and subsequent entries/corrections use the new rules; both category editing screens present this confirmation.

Group categories include a Same club in a group switch in creation/settings, enabled by default. Disabling it applies to new automatic layouts. Failed club separation explains that seeds/groups or a manual layout can be adjusted. The player club field offers existing names and an explicit correction button for a similar name; it never silently replaces a typo.

The main sidebar lists Tournaments, Players, Backups, History and finally Trash as vertically stacked labeled rows. Guide remains at the bottom above the divider and local operations. Collapsed navigation shows icons without button outlines, with accessible names and hover labels. Guide uses numbered sections, a jump index, common-problem disclosures and a glossary; jumps scroll without adding browser history entries.

A subtle horizontal divider separates sidebar utilities from local operations, with 18px of spacing above and below the divider in expanded and collapsed modes.

Sidebar navigation starts directly with Tournaments, without a workspace caption above it.


## Free-software notice

The initial tournament/league selection screen includes a restrained notice below the mode cards: LibreTT is free forever and anyone who paid a seller for the program should request a refund. It displays the official `https://librett.org` and `https://github.com/tradicije/librett-desktop` links. Desktop opens these two fixed destinations in the system browser; preview uses normal external links. The notice follows the selected Serbian/English language.

The notice spans the full width of the mode-card grid with compact spacing. Links are labeled **LibreTT Website** and **Source Code**. Its semantic section avoids inheriting global sidebar styles.

The notice uses a 22px heading, separate primary free-software and secondary refund paragraphs, and a bottom row with official-link labels on the left and named links on the right. Narrow windows stack the links row.

The initial screen centers the heading, both mode cards and the notice together within the content area. A single flex column with a minimum height matching the scroll area centers all sections together and grows when content needs scrolling; legacy top-offset rules are removed.
