## FEATURE: Empty tabs are real tabs, and ⌂ is the app's own home.

## OBJECTIVE
From chat with the owner (sessions 148–149). Two changes:
1. **Empty tabs.** + adds an empty tab to the sidebar and selects it; pressing it again adds another. An empty tab shows the new-tab page (the empty content area, later the bundled new-tab app, DECISION-027) with the cursor in the address box. Opening an address, a pinned app or "where you left off" from an empty tab opens it **in that tab**. An empty tab closes like any other (its × and Ctrl+W, its menu's Close). The window always has a tab: it starts with one empty tab, and closing the last tab leaves one empty tab.
2. **⌂ is the app's home**, not Rapidkoi's: it takes the current app to its home screen, as ← and → move through its screens. That needs routes (links, build step 2), so ⌂ is disabled ("Home (comes with links)") like ← and → until then. Because apps are cached and kept running, this, not a refresh, is how people get back to an app's start (the owner).

This reverses "home isn't a tab" (the old rail's design): an empty tab is the home.

## CONTEXT

- Starting state: `Model.tabs: Tab[]` (each an app), `current: Option<i64>`, `None` meaning home; `Msg.NewTab` and `Msg.Home` both go home (`window.go_home`); `navigate.open_file` opens into the current tab's app, or a new tab from home; the sidebar (PRP-023) lists app tabs; ⌂ in the header sends `Msg.Home`.
- Ending state:
  - `src/window/window.la`: `Model.blanks: i64[]`, the open empty tabs' ids, sharing `next` with app tabs, so app and empty tabs keep one creation order. `current` is an app tab's id, an empty tab's id, or `None` only while there are no tabs at all (never shown: see below). `page_of` is `Home` on an empty tab. New: `new_blank(m)`, `fill(m, id, page)` (an empty tab becomes an app tab, same id); `close` handles empty tabs and leaves one empty tab when the last tab closes; `go_home` no longer exists as Rapidkoi's home (see Messages).
  - `src/window/navigate.la`: opening from an empty tab fills that tab (`open_file`, `reopen`, opening a pin).
  - `src/shell.la`: `NewTab` opens a new empty tab; `start` begins with one empty tab when no app is given.
  - `src/ui/sidebar.la`: empty tabs among the open apps in creation order, each a placeholder icon (`tab-new-symbolic`), tooltip "New tab" (+ becomes "Add a tab"), ringed and with its × when current, a menu with Close.
  - `src/ui/header.la`: ⌂ disabled, "Home (comes with links)".
  - Tests: `test/tabs_empty.la` (new); every test using `Msg.Home` or `current == None` for home follows (see Messages).
  - DESIGN.md, CHANGELOG.md.
- Open decisions: none. Supersedes the old "home isn't a tab".

## MODEL AND MESSAGES

- `Model.blanks: i64[]`: the empty tabs' ids.
- `Msg.NewTab`: a new empty tab, selected, the address box focused.
- `Msg.Home`: kept as the message "show an empty tab": the current tab if it's empty, else a new one. Nothing in the window sends it once ⌂ is the app's home; tests and menu code that meant "go home" keep it.
- `Msg.CloseTab(id)`, `Select(id)`: work on empty tabs too.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- One id sequence for app and empty tabs; the sidebar orders unpinned tabs by id (creation order).
- Filling an empty tab keeps its id and its place.
- Closing the current tab selects a neighbour (the next, else the previous), empty or not; closing the last leaves a fresh empty tab, selected.
- "Where you left off" and `last` behave as before for app tabs; empty tabs are never "left off".
- Empty tabs aren't saved (open tabs aren't restored on launch yet, CLAUDE.md known issues).

### Must NOT Do
- Give ⌂ any behaviour before links (it would mean restarting the app, losing its state).
- Merge into `redesign` or `main` (session 90).

## ERROR HANDLING REQUIREMENTS

- None new: a failed open fills the empty tab with its failed page, as a new tab did.

## SECURITY CONSIDERATIONS

- None new.

## TESTS TO WRITE

`test/tabs_empty.la`:
- [x] A fresh window has one empty tab, current; the sidebar shows it ("New tab"), no app.
- [x] + (`NewTab`) twice: two more empty tabs, the last current.
- [x] Opening a file from an empty tab fills that tab: same id, no extra tab; from an app tab it replaces that app as today.
- [x] Opening a pin and Continue from an empty tab fill it.
- [x] Closing an empty tab selects a neighbour; closing the last tab leaves one empty tab.
- [x] Ctrl+W and the × close the current empty tab.
- [x] Unpinned tabs, app and empty, show in creation order.
- [x] `Home` on an empty tab stays; on an app tab it opens a new empty tab.

`test/header.la`: ⌂ is disabled, "Home (comes with links)".
Existing tests that read home as `current == None` follow: home is now `page_of(m) == Home` on an empty tab.

## ROLLBACK PLAN

- `git revert` the PRP's commits on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] + adds empty tabs to the sidebar; typing an address in one opens the app there; closing works; the window is never tabless
- [ ] ⌂ disabled with its tooltip
- [ ] The tests above and `lughac test test/` pass (the owner runs them)
- [x] `lughac build src/main.la -o main`; `lughac fmt --check src test` clean; no file over 500 lines
- [x] DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`: one empty tab in the sidebar, ringed. + twice: three. Type `rapidkoi://quote.dafifi.net` in the last: it becomes the quote app in place. Close tabs down to none: one empty tab remains. ⌂ greyed: "Home (comes with links)".
