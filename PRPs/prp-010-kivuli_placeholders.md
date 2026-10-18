## FEATURE: Retiring four placeholders with kivuli 0.10: the info panel over the app, right-click menus on tabs and tiles, a wrapping home grid, and panel rows clickable anywhere

## OBJECTIVE
Four things the owner has been living with since PRP-002 become what the sketches meant:
- **The panel:** ⓘ opens the info panel *over* the app, with the app dimmed behind it, and closes on a click outside or Escape.
- **Menus:** right-clicking a tab offers Reload, Pin, About, Close and Forget. Home's tiles offer Open, Unpin and Forget.
- **Home:** the pinned apps re-flow in a grid as the window resizes.
- **Panel rows:** each one is clickable across its whole width.

Designed in chat, session 47: `docs/source/meetings/2026-10-18-kivuli-placeholders.md` (topics 1–4, all settled).

## CONTEXT

- **Starting state:**
  - kivuli 0.6.
  - `shell.view` draws `row([page, panel])` when the panel is open.
  - `home.grid` makes rows of `TILES_PER_ROW` (4).
  - Panel rows: only the › arrow is a button (`panel.line`).
  - Tab actions live in the panel only.
- **Ending state:**
  - `lugha.toml`: kivuli `version = "0.10"`; `lughac update src/main.la`.
  - `src/shell.la`:
    - the middle is `overlay(page, layers)`; when the panel is open, the layers are a scrim (a `Look.Scrim` button, Msg.TogglePanel, Escape) and the panel placed `(Align.End, Align.Fill)`
    - the page keeps its full width
  - `src/themes.la`: `Look.Scrim`, the dark theme's dim layer (a translucent black, from the theme only).
  - `src/frame.la`: each rail tab gets `.menu([...])`.
  - `src/home.la`: tiles in `wrap(...).equal()` with a `.menu`; the "where you left off" card gets a `.menu`. `TILES_PER_ROW` is removed.
  - `src/panel.la`: `line` becomes one `button_with(row([label, spacer, summary, ›]), Msg.PanelGo(page))` with `ButtonStyle.Subtle`.
  - `src/actions.la`: `Msg.OnTab(i64, TabAction)` with `TabAction { Reload, TogglePin, About, Close, Forget }`, and `Msg.OnPin(string, PinAction)` with `PinAction { Open, Unpin, Forget }`.
  - `src/shell.la` (`handle_msg`): `OnTab` selects the tab, then runs the action with the existing code (`Reload`, `TogglePin`, `PanelGo(About)`, `CloseTab`, `Ask(Confirm.Forget)` with the panel open). `OnPin` does `Open` = `OpenPinned`, `Unpin` = remove the pin, and `Forget` = open it, then the panel at the question.
  - Docs: CLAUDE.md (known issues: four placeholders gone; the file dialog remains), DECISIONS.md (020: clickable rows, overlay and context menus done; 018 updated), docs/DESIGN.md (the overlay, the scrim, the menus, the grid), CHANGELOG.md.
- **Related code:** `src/shell.la` (view), `src/frame.la` (rail), `src/home.la` (grid, card), `src/panel.la` (line), `src/themes.la`, `../kivuli/README.md` (Layout, Modifiers, Testing), `../kivuli/examples/frame.la` (the Info panel overlay).
- **Lugha spec relied on:** nothing new.
- **kivuli API used** (all in 0.10.0's README):
  - `overlay`, `.place`
  - `wrap`, `.equal`
  - `.menu`, `item`, `menu_separator`, `item(...).disabled`
  - `button_with`
  - `ButtonStyle.Subtle`, `.shortcut`
  - testing `pick`, `click`
- **Open decisions:** none. DECISION-018 and -020 are updated by this PRP.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**The overlay (topic 1)**
- **When the panel is open:**
  - the page: as today, full width
  - layer 1: a full-cover `button("", Msg.TogglePanel).style([Look.Scrim]).tooltip("Close the panel").shortcut("Escape")`
  - layer 2: `panel.view(...)`, placed `(Align.End, Align.Fill)`, at `PANEL_WIDTH`
- **When it's closed:** `overlay(page, [])`, so the page is never rebuilt by opening or closing.
- **The scrim's colour** comes from `themes.la` only (the design rule): a translucent black over the page in the dark theme.
- **The panel's own look** is unchanged (frame colour, its sections, Forget and Close in one row).

**Menus (topic 2)**
- **A rail tab:** `.menu([item("Reload", OnTab(id, Reload)), item(pin label, OnTab(id, TogglePin)), item("About this app", OnTab(id, About)), menu_separator(), item("Close tab", OnTab(id, Close)), item("Forget this app…", OnTab(id, Forget))])`.
  - The pin label is "Unpin" when that tab's app is pinned, else "Pin to home".
  - Reload is `.disabled(true)` while the tab is loading in the background.
- **`OnTab(id, action)`** selects tab `id`, then runs the action exactly as the bar or panel would:
  - About opens the panel at About
  - Forget opens the panel at Main with `confirm: Some(Confirm.Forget)` (the existing question)
- **A pinned tile:** `.menu([item("Open", OnPin(path, Open)), item("Unpin", OnPin(path, Unpin)), menu_separator(), item("Forget this app…", OnPin(path, Forget))])`.
- **The "where you left off" card:** `.menu([item("Open", Continue), menu_separator(), item("Forget this app…", ...)])`. Forget opens it (Continue), then the panel at the question.
- **`OnPin(path, Unpin)`** removes that pin, as ☆ does, and the pins file is saved by `persist` as today.

**The grid (topic 3)**
- Pinned tiles in `wrap(tiles).equal().spacing(SPACE_MD)`. Tiles are as today; search filtering is unchanged.
- `TILES_PER_ROW` and the hand-made rows are gone.

**Rows (topic 4)**
- `panel.line(label, summary, opens)` is `button_with(row([text(label), spacer(), text(summary).dim(), icon("go-next-symbolic")]).spacing(SPACE_SM), Msg.PanelGo(opens)).button_style(ButtonStyle.Subtle).tooltip(label)`.
- The header's marker button is unchanged.

### Must NOT Do
- No file dialog (kivuli has none yet; typing a path stays).
- No change to what the panel, tiles or tabs show, only where they sit and how they're reached.
- No action reachable from a menu that isn't reachable today. Forget always asks first.
- No colours outside `themes.la`.

## MODEL AND MESSAGES
- **Model:** unchanged.
- **Messages:**
  - `OnTab(i64, TabAction)`: a tab's menu item
  - `OnPin(string, PinAction)`: a home tile's menu item, by path
- **Enums:** `TabAction { Reload, TogglePin, About, Close, Forget }`, `PinAction { Open, Unpin, Forget }`.

## ERROR HANDLING REQUIREMENTS
- `OnTab` for a tab that's gone, or `OnPin` for a path that's no longer pinned, changes nothing.
- Nothing new can fail: every action reuses an existing path, and its errors show as they do today.

## SECURITY CONSIDERATIONS
- Menus add no capability: each item is an existing action. Forget still asks, in the panel.
- The scrim stops clicks reaching the app while the panel is open, so Start over and Forget act on an app that can't change under them.

## TESTS TO WRITE
- [ ] **overlay** (`kivuli.testing`):
  - with the panel open, "Close the panel" sends TogglePanel, and Escape (`press`) does too
  - the app's own buttons aren't the click target while it's open (the scrim is above). Kivuli's tests own layering, so this is checked as the scrim being present.
  - with it closed, there's no scrim
- [ ] **tab menu** (`pick(view, tab tooltip, item)`):
  - every item's message
  - "Unpin" when pinned
  - Reload disabled while loading (`Silent`)
  - `OnTab(id, X)` selects the tab, then does X: Reload, pin, About (the panel at About), Close (the tab closes), Forget (the panel's Forget question shows)
  - `OnTab` for a closed tab changes nothing
- [ ] **tile menus:**
  - Open opens the pin
  - Unpin removes it, and the pins file is saved (a test store)
  - Forget opens it with the question
  - the card's Open is Continue
- [ ] **grid:** all pinned tiles present, with any number (no rows of 4); search hides non-matches (texts).
- [ ] **rows:** clicking a row by its label (`click(view, "Account")`) sends `PanelGo(Account)`, for each of the five rows.
- [ ] **existing tests** pass with the new layout; tests that clicked the › arrow by tooltip click the row instead.

## ROLLBACK PLAN
- Branch: `prp-010-kivuli_placeholders`, from main after PRP-009.
- Each of the four is its own commit after the kivuli bump, so any one can be dropped.

## ACCEPTANCE CRITERIA
- [ ] ⓘ opens the panel over the app with the app dimmed; a click outside, or Escape, closes it *(the owner)*
- [ ] Right-click a tab: the menu, and each item does what it says *(the owner)*
- [ ] Home's grid re-flows when the window is resized *(the owner)*
- [ ] Panel rows highlight and open from anywhere on the row *(the owner)*
- [ ] Every test above
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner:
  1. `./main`, pin two or three apps, and resize the window.
  2. Open one, press ⓘ, click outside the panel; press ⓘ, then Escape.
  3. Right-click its tab and try each item.
  4. Click a panel row anywhere.
