## FEATURE: The window shows only the bare frame: an empty header bar, an empty sidebar on the right, and an empty content area.

## OBJECTIVE
The first step of the redesign (DECISION-027). The window shows three areas and nothing else: the header bar across the top, with only GTK's minimise, maximise and close buttons; the sidebar down the right edge; and the content area filling the rest, meeting the window's left and bottom edges, with only its top-right corner rounded. The header and sidebar are one colour (the frame), and the content area is the page colour. No tabs, address box, home, buttons or text.

## CONTEXT

- Starting state: `src/shell.la`'s `view` composes the notice, the page (home or an app, with the info panel over it), the rail (`frame.rail`) and the header bar (`frame.bar`). Many tests drive that view: `test/shell.la`, `opening.la`, `panel.la`, `account.la`, `access.la`, `effects.la`, `loading.la`, `menus.la`, `loader.la`, `remote.la`, `saving.la`, `counter.la`.
- Ending state:
  - `src/layout.la` (new, pure): `view(m: Model): View<Msg>`, the bare frame.
  - `src/legacy.la` (new): today's `shell.view` with its helpers `card` and `app_view`, moved unchanged and renamed `legacy.view`. The old tests keep using it, so they keep guarding `update`'s behaviour until later redesign PRPs replace each part, and the last one deletes it.
  - `src/shell.la`: `view` calls `layout.view`. Its imports shrink to what remains.
  - `test/layout.la` (new); the old tests' `shell.view`/`view(` calls point at `legacy.view`.
  - `docs/DESIGN.md`, CLAUDE.md's file tree, CHANGELOG.md.
- Related existing code: `src/shell.la:223–297`, `src/themes.la` (`Look.Frame`, `Look.Page`: top-right radius already), `src/theme.la` (`FRAME_THICKNESS`, `WINDOW_WIDTH/HEIGHT`), `../kivuli/README.md` (`.header`, `.header_style`, `.window`, `.maximized`, `.theme`, `.width`, `.expand`), `../kivuli/src/testing.la` (`texts`, `click`).
- Related source files: the owner's sketch (session 72): header on top, sidebar on the right, content area inset in the frame.
- Lugha spec sections relied on: none new.
- kivuli API used: `column`, `row`, `.expand`, `.width`, `.style`, `.window`, `.maximized`, `.theme`, `.header(start, title, end)` with empty views, `.header_style`; `kivuli.testing.texts`, `click`. All exist in kivuli 0.10.
- Open decisions that must be resolved first: none. DECISION-027 is resolved.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Header bar: `.header([], <empty title>, [])` with `Look.Frame`. Its title area shows nothing, not the window title, which stays set for the taskbar (`title_of(m)`). GTK draws − □ ×.
- Sidebar: an empty `Look.Frame` column, `FRAME_THICKNESS` wide, on the right, so the frame stays an even L.
- Content area: an empty `Look.Page` column that expands, meeting the window's left and bottom edges, top-right corner rounded (`PAGE_RADIUS`, already in `Look.Page`).
- The window keeps its size, maximised start and theme.
- `model`, `update`, `subscriptions` and `boot` are unchanged: the app keeps loading what it saved, it just isn't shown.

### Must NOT Do
- Delete `frame.la`, `home.la`, `panel.la`, `account.la` or `access.la`, or any `update` logic: later redesign PRPs move each into the new frame or into a bundled app.
- Add any new colour, token or look: the existing `Frame` and `Page` looks are the sketch.
- Merge to `main` (DECISION-027).

## MODEL AND MESSAGES

- Model fields added or changed: none.
- Messages added: none.

## ERROR HANDLING REQUIREMENTS

- None: the view is pure and can't fail.

## SECURITY CONSIDERATIONS

- No new input, dependency or capability. Guests still run (restored pages load) but nothing they draw is shown.

## TESTS TO WRITE

`test/layout.la`, with no display:
- [ ] View: a fresh model's view has no text (`texts(v)` is empty).
- [ ] View: a model with tabs open, a pin and a notice still has no text, and no "New tab" or "Close" button (`click` returns `Err`).
- [ ] View: `shell.view(m)` and `layout.view(m)` show the same (the shell wires the new frame).
- The old view tests keep passing against `legacy.view`.

## ROLLBACK PLAN

- Branch to return to: `redesign` (the PRP's branch, `prp-014-bare_frame`, is deleted unmerged).
- State: as at session 72's open.

## ACCEPTANCE CRITERIA
- [ ] The window shows only the header with − □ ×, the empty sidebar on the right and the empty content area with one rounded corner
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/layout.la`, then `lughac test test/`
- `lughac build src/main.la -o main`
- For the user: `./main`. The window shows the header bar with only − □ ×, a strip down the right in the same colour, and the darker content area meeting the left and bottom edges, curved only at its top-right corner. Nothing else.
