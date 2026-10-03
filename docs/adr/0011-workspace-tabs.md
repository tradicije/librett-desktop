# 0011: Keep independent workspaces mounted behind a pinned Home screen

Status: accepted

## Context

Organizers need Cash desk, Registrations and Draw open simultaneously without
losing search, selection or unsaved work. macOS title-bar space should be usable,
with real window controls retained. Native history must follow the active tab.

## Decision

App owns a pinned Home workspace and a keyed collection of working tabs. Home
is not listed or closable; selecting a tournament from Home opens a working tab.
Workspace owns its route history, local forms and child components. Sidebar
collapse is a shared, locally persisted preference; compact mode uses the supplied
theme-specific square brand icons and labeled icon navigation. The compact brand
icon expands the sidebar; brand images are not Home navigation links. Inactive
workspaces remain mounted and hidden. App shares tournaments, language and theme,
remembers scroll/focus, and guards switches/closing while writes or modals are active.
Known unsaved forms and drafts require confirmation before a tab is discarded.

Tabs are fixed at 180px; overflowing labels fade at the right after measuring
the actual text width, without an ellipsis. Tab labels show the immediate parent and current page; tooltips carry the full context. Navigation
supports Cmd/Ctrl-click, middle-click and a contextual Open in new tab command.
Opening a duplicate copies the destination, not unsaved form data. + / Cmd/Ctrl+T
return to Home for choosing another workspace. Working tabs use Cmd/Ctrl+W and
Ctrl+Tab, plus accessible tab-list keyboard controls.

Native webview history is kept centered between backward and forward entries.
Popstate dispatches to the active workspace's history and restores the center.
This prevents histories from mixing when a user changes tabs. Toolbar/mouse
controls use the same workspace travel operation. Native navigation is blocked
during writes and modals. macOS trackpad navigation remains enabled.

The 48px workspace row is fixed. Content/sidebar height subtracts that row so
short pages do not force a scrollbar. macOS Overlay/hiddenTitle keep native window
buttons alongside tabs; their actual AppKit frames are centered using coordinate
conversion at startup and on resize/focus. Other systems retain native chrome.
Blank parts of the row allow native dragging through a narrowly scoped capability.

Read-only lists refresh when a tab becomes active, preserving valid selections.
Dirty draw drafts remain local and rely on membership/revision checks when saved.

## Consequences

Several mounted workspaces use more memory than one screen; closing a tab frees
its state. IDs for repeated dialogs are instance-specific. Tabs are session-local
and restart does not restore unsaved forms. Window closing and Quit check unsaved work in every workspace. Routing inside a tab still replaces its current child screen;
retention applies while switching between working tabs, not every history entry.


Workspace scrolling uses a bounded container below the 48px title bar. The root
document does not scroll, and vertical overscroll cannot chain into it. Sidebars and navigation toolbars sit outside the content scroll region; tab
switching restores the content container scroll position. Breadcrumb buttons
use workspace navigation and support the same new-tab actions as other links. This isolates the tab row from macOS elastic scrolling.
