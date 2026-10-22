## FEATURE: The sidebar catches up with kivuli 0.54: a × on any hovered tab, + right after the last tab, and the current tab scrolled into view.

## OBJECTIVE
From chat with the owner (sessions 145, 148; kivuli shipped the requests, session 160). Three changes to the sidebar (PRP-023), each waiting on kivuli until now:
1. **× on hover** (DECISION-036): every open tab, app or empty, shows its × while the pointer is over it (or the keyboard focus is in it); the current tab keeps its × always, as today.
2. **Chrome's +** (DECISION-037): + sits right after the last tab; once the tabs overflow the strip, they scroll and + stays at the bottom.
3. **Scroll into view** (DECISION-037): whenever the current tab changes (selected, opened, +, closed to a neighbour), the sidebar scrolls as little as it can to show it. The keyboard focus stays where it was.

## CONTEXT

- Starting state: kivuli 0.51; `src/ui/sidebar.la` puts the × only on the current tab (`close_button`, with Ctrl+W), `scroll(…).expand()` then + at the bottom; nothing scrolls a tab into view.
- Ending state:
  - `lugha.toml`: kivuli `0.54`; `lughac update src/main.la` rewrites `lugha.lock` (never by hand).
  - `src/ui/sidebar.la`: each open tab's button is a group (`Group.mark()`) and named `tab-{id}`; every open tab has a ×, hidden at rest unless current, shown by `When(GroupHover…)` and `When(GroupFocusWithin…)`; only the current tab's × carries Ctrl+W (one shortcut, one button). Closed pins have no × (nothing to close). `scroll_fit(…)` then +, no spacer.
  - `src/ui/frame_looks.la`: `tab_close` gains the hidden-at-rest look (`Effects.opacity`), for tabs that aren't current.
  - `src/shell.la`: after any update that changes `current` to a tab, the command also carries `Cmd.scroll_to("tab-{id}")` (batched with what it already returns).
  - DECISION-036 and DECISION-037 resolved; CLAUDE.md stack line (kivuli 0.54); DESIGN.md; CHANGELOG.md.
- Related existing code: `../kivuli/README.md` (group states, `scroll_fit`, `Cmd.scroll_to`), `../kivuli/src/styles/groups.la`, `../kivuli/src/core/effects.la`.
- kivuli API used: `Group.mark()`, `State.GroupHover`, `State.GroupFocusWithin`, `scroll_fit`, `Cmd.scroll_to`, `Cmd.batch`, `.name`; confirmed in kivuli 0.54's README and source.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- A hidden × is still a real button: it closes its own tab when clicked (so hovering then clicking works), and its tooltip is "Close {name}".
- Closing a pinned app's tab with its × keeps the pin (as the menu's Close does).
- `kivuli.ui` components hold kivuli 0.52's `Sheet.group` field: any `Sheet { … }` Rapidkoi writes in full gets it.
- Nothing scrolls when the current tab doesn't change (typing, cards, a guest's message).

### Must NOT Do
- Give every × Ctrl+W (only the current tab's).
- Move the focus to the sidebar when scrolling.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new.

## ERROR HANDLING REQUIREMENTS

- None new: `Cmd.scroll_to` with a name that isn't drawn does nothing (kivuli).

## SECURITY CONSIDERATIONS

- None: no guest-facing change.

## TESTS TO WRITE

`test/sidebar.la`:
- [ ] Every open tab, app or empty, has a × button ("Close {name}") that sends `CloseTab(id)`; a closed pin has none.
- [ ] Only the current tab's × answers Ctrl+W.
- [ ] A pinned app's × closes its tab, and the pin stays.

`test/shell.la` (or `test/one_tab.la`):
- [ ] Selecting a tab, + and opening a file return a command holding `Cmd.scroll_to("tab-{id}")` for the new current tab (found with `kivuli.testing`, as `Cmd.focus` is).
- [ ] Closing the current tab scrolls to the neighbour now current.
- [ ] A `Draft` (typing) returns no scroll.

The hover itself, the scroll position and + following the last tab are seen by hand (a window).

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits (and `lughac update` back to 0.51).

## ACCEPTANCE CRITERIA
- [ ] Hovering any tab shows its ×; the current one always shows it
- [ ] + follows the last tab until they overflow
- [ ] Selecting a tab out of sight scrolls the sidebar to it; the address box keeps the focus
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean; no file over 500 lines
- [ ] DECISIONS.md, CLAUDE.md, DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`, open a few apps. Hover each: its × appears and closes it. With few tabs, + is right under the last. Open enough to overflow: the tabs scroll, + stays at the bottom; scroll the current one out of sight, press Ctrl+T or open an app: the sidebar scrolls to it, and typing still goes into the address box.
