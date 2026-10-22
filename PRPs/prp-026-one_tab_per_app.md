## FEATURE: One tab per app, and at most one empty tab.

## OBJECTIVE
From chat with the owner (session 155). An app runs in one tab at a time: opening an app that's already open (by typing its address, from a pin, from "where you left off") switches to its tab instead of starting a second copy. The same for empty tabs: there's at most one; + with an empty tab open switches to it.

Why: an app's whole state is its screen today, saved per app, not per tab, so two copies silently overwrite each other's saved state, and a pinned app open twice hid its second tab from the sidebar. Several tabs of one app, each with its own screen and sharing the app's data, come with links (build step 2) and storage (step 4).

## CONTEXT

- Starting state: `navigate.open_file` opens into the current app tab (replacing its app), else into the current empty tab (PRP-024), else a new tab; `window.new_blank` always adds an empty tab; `window.go_home` shows the current empty tab or adds one; the sidebar shows a pinned app once, by its first tab (`sidebar.tab_of`), hiding any second.
- Ending state:
  - `src/window/window.la`: `open_tab_of(m, path): Option<i64>`, the tab already showing the same app; `new_blank` switches to the empty tab if there is one, else adds one; `go_home` the same.
  - `src/window/navigate.la`: `open_file` (and `reopen`) switch to the app's open tab when there is one, leaving the current tab as it was (an empty tab stays empty; an app tab keeps its app).
  - Tests: `test/one_tab.la` (new); `test/tabs_empty.la` changes where it made several empty tabs.
  - DESIGN.md, CHANGELOG.md.
- "The same app": the same address (host and port; `rapidkoi://` or not, a trailing `/` or not), or the same file path as typed. A signed app reached two ways (its site, a `.rkb` of it) stays two apps here: one tab per address or file.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Switching to an open app shows its tab as `Select` does (the box empties, cards close, it becomes "where you left off" when left).
- An address still loading counts as open: opening it again switches to the loading tab.
- With at most one empty tab, + is "go to the empty tab" when there is one.
- The sidebar's hidden second tab can't happen any more (one tab per app); no sidebar change needed.

### Must NOT Do
- Merge two copies already open (the window can't have them after this).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new.

## ERROR HANDLING REQUIREMENTS

- None new.

## SECURITY CONSIDERATIONS

- None: switching tabs runs nothing new.

## TESTS TO WRITE

`test/one_tab.la`:
- [ ] Opening a file that's open in another tab switches to it; the tab count doesn't change; the current tab keeps what it had.
- [ ] The same for an address, typed with or without `rapidkoi://` and a trailing `/`.
- [ ] Opening a pin, and Continue, switch to the app's open tab.
- [ ] + with an empty tab open switches to it; with none, adds one.
- [ ] An address still loading counts as open.

`test/tabs_empty.la`: + twice leaves one empty tab.

## ROLLBACK PLAN

- `git revert` the commit on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] An app never opens in a second tab; there's never more than one empty tab
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean
- [ ] DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: open `rapidkoi://quote.dafifi.net`; + and type it again: the quote tab is shown, no second one. Press + twice: one empty tab.
