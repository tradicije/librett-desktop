# Desktop interface direction

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

Mode selection, tournaments, categories, registrations, the player directory,
player create/edit screens, cash accounts and financial history share this system.
The backend model and existing application workflows are unchanged by this redesign.
Browser checks use temporary synthetic data outside the repository; production
screens contain no seeded demonstration records.
