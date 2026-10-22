## FEATURE: The sidebar: pinned apps, open apps, the active one ringed, closing and adding.

## OBJECTIVE
Designed in chat with the owner (session 145). The sidebar down the window's right holds Rapidkoi's apps as icon buttons, top to bottom: pinned apps (open or not), a line, the other open apps, then +. The active app's icon has a round orange ring (the accent); any app's icon shows a faint round wash on hover. An app closes from its right-click menu, with Ctrl+W (the current one), or with a small × on the active app. + and Ctrl+T open a new tab (home). Hovering an app shows its name and screen title.

The × on any hovered app needs kivuli's group hover (DECISION-036, requested in `docs/specs/kivuli-group-hover.md`); until then it's on the active app only.

## CONTEXT

- Starting state: the sidebar is an empty `FRAME_THICKNESS`-wide strip (`src/ui/layout.la`). The model has `tabs`, `current`, `pinned`; messages `Select`, `CloseTab`, `NewTab`, `OpenPinned`, `OnTab(id, TabAction)`, `OnPin(path, PinAction)` (PRP-010's menus' actions, kept through the redesign).
- Ending state:
  - `src/ui/sidebar.la` (new, pure): the sidebar's view.
  - `src/ui/layout.la`: the sidebar from `sidebar.view(m)`.
  - `src/ui/themes.la`: the ring and hover looks; `src/ui/tokens.la`: sizes as needed.
  - Tests: `test/sidebar.la` (new); `test/layout.la` (it checked there's no New tab or tab menu).
  - DESIGN.md, CLAUDE.md's tree, CHANGELOG.md.
- Related existing code: `src/ui/frame.la` (`label`, `initial`, `icon_of`), `src/ui/header.la` (icon buttons), the old rail in git (`src/frame.la` before `80ed95c`: tab buttons, their menus).
- kivuli API used: `kivuli.ui`'s `button_with`, `Button.ghost()/icon()`, `.menu`, `item`, `menu_separator`, `overlay` with `.place`, `.shortcut`, `.tooltip`, `ui.separator`, `image`.
- Open decisions: DECISION-036 (deferred: the × on any hovered app).

## THE SIDEBAR

| Part | Shows | Click | Right-click |
|---|---|---|---|
| A pinned app, open | its icon (or initial); the ring when it's current | `Select(id)` | Close, Unpin, Forget this app… (`OnTab`) |
| A pinned app, not open | its icon (or initial) | `OpenPinned(path)` | Open, Unpin, Forget this app… (`OnPin`) |
| (a line between pinned and the rest, when both are there) | | | |
| An open app, not pinned | its icon (or initial); the ring when current | `Select(id)` | Close, Pin, Forget this app… (`OnTab`) |
| The current app's × (top-right of its icon) | a small × (text: there's no close icon in `assets/icons/` yet) | `CloseTab(id)`; Ctrl+W | |
| + | the `+` | `NewTab`; Ctrl+T | |

Tooltips: an app's `label` ("Notes · Sprint 3"); a pinned app that isn't open: "Open Notes"; ×: "Close Notes"; +: "New tab".

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Pinned apps first in the pins' order, each matched to an open tab by path; then open apps not pinned, in tab order; then +.
- The ring: a 2 px accent border, round (`CIRCLE`); hover: a faint round wash (`frame_hover`), as the window controls.
- Every icon `HEADER_ICON` (20); buttons `kivuli.ui` ghost icon size (36), centred in the 50 px strip.
- Every colour from `themes.la`, every size a token.

### Must NOT Do
- Put the × on every app before kivuli has group hover (DECISION-036).
- Reorder by dragging (kivuli has no drag and drop).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new.

## ERROR HANDLING REQUIREMENTS

- None new: opening a pin that fails shows in its tab, as today.

## SECURITY CONSIDERATIONS

- None new: apps' icons are bytes kivuli checks (PNG, JPEG, SVG within limits); names are text.

## TESTS TO WRITE

`test/sidebar.la`:
- [x] Open apps appear by their label and send `Select(id)`.
- [x] A pinned app that isn't open sends `OpenPinned(path)`; one that's open appears once, as its tab.
- [x] Only the current app has a × ("Close Notes" → `CloseTab(id)`); Ctrl+W sends it; home has neither.
- [x] + sends `NewTab`; Ctrl+T too.
- [x] Each menu's items send their `OnTab`/`OnPin` action.

`test/layout.la`: the sidebar is there (New tab), no longer empty.

## ROLLBACK PLAN

- `git revert` the PRP's commit on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] The sidebar as above; the ring on the current app; × on it; menus; Ctrl+W, Ctrl+T
- [ ] `lughac test test/` passes (the owner runs it)
- [x] `lughac build src/main.la -o main`; `lughac fmt --check src test` clean
- [x] DESIGN.md, CLAUDE.md, CHANGELOG.md updated

## VALIDATION
- For the user: open two apps and pin one; the pinned one at the top, a line, the other below, + last; the current one ringed with a ×; right-click each; Ctrl+W, Ctrl+T.

## AMENDMENT 1 (session 148)

The owner asked whether the sidebar scrolls and whether + stays reachable with many apps, then chose Chrome's way: + right after the last app until the apps overflow, then stuck at the bottom; and scrolling an app into view when it's opened or selected. Both need kivuli (a scroll with natural height; `Cmd.scroll_to`): requested in `docs/specs/kivuli-scroll-fit-and-scroll-to.md`, DECISION-037. Meanwhile the apps scroll and + is fixed at the bottom of the strip.
