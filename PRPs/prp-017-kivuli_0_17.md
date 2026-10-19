## FEATURE: Rewrite Rapidkoi's UI on kivuli 0.17 (built on 0.18, which made the header bar a node; session 88), every colour from Rapidkoi's theme, and delete the old view.

## OBJECTIVE
Rapidkoi moves from kivuli 0.10 to 0.17 (DECISION-030 A). The window looks as it does after PRP-016: the frame, the rounded address box with its suggestions card, account and menu (disabled), and the current app in the content area. Now every colour, apps' content included, comes from Rapidkoi's dark theme instead of the desktop's (DECISION-031), and the header bar is the frame's colour. The old view (bar, rail, home, info panel, Account and Access pages) is gone from the code; the info panel, Account and Access come back natively in later redesign PRPs.

## CONTEXT

- Starting state: kivuli 0.10 (`lugha.toml`); PRP-014–016 on `prp-016-account_menu_icons`. 15 modules use APIs 0.11–0.17 removed: `.bold`, `.dim`, `.size` on text, `.padding`, `.spacing`, `.width`, `.height`, `.button_style`/`ButtonStyle`, `Surface`, `theme`/`.theme`, `.header_style`, `.header(start, title, end)`.
- Ending state:
  - `lugha.toml`: kivuli `0.17`. `lugha.lock` rewritten by `lughac update` only.
  - `src/themes.la`: the `Theme` struct gains every colour the UI needs (table below); the `Look` enum and `kivuli_theme` are replaced by functions returning `Styling[]` (`frame(t)`, `page(t)`, `address(t)`, `suggestions(t)`, `clear()`, `body(t)`, `dim(t)`, `title(t)`, `heading(t)`, `button(t, kind)`, `field(t)`), each built from `theme.la`'s tokens.
  - `src/theme.la`: tokens stay numbers; small helpers turn them into kivuli values (`px(n)`, `margin(n)`, `padding(n)`, `gap(n)`, `min_size(w, h)`), so no view writes a bare number.
  - `src/render.la`: guests' screens styled from the theme: `TextStyle` → `body/title/heading/dim`, `ButtonLook` → `button(t, kind)`, `Space` → `gap`/`padding`. `screen`, `stopped`, `refused`, `loading` take the theme.
  - `src/layout.la`, `src/addressbox.la`, `src/content.la`, `src/frame.la`: ported. The header is `header_bar(…).style(frame(t))`.
  - `src/suggestions.la`: takes `Card` and `matches` from `home.la`.
  - Deleted: `src/legacy.la`, `src/home.la`; from `src/frame.la` the bar, the rail and their helpers (kept: `icon_button`, `marker`, `marker_icon`, `marker_note`, `file_of`, `icon_of`, `initial`, `label`, `unapproved`); `src/panel.la`'s view and the `page`/`summary` views in `src/account.la` and `src/access.la` (their logic stays: `account_of`, `on_account`, `forget_account`, `access_of`, `on_access`, `looked`, `news`).
  - Tests: see TESTS TO WRITE.
  - `docs/DESIGN.md` rewritten for 0.17 and the theme; CLAUDE.md's tree and STACK; CHANGELOG.md.
- Related existing code: every module above; `../kivuli/README.md` (Styles), `../kivuli/src/lib.la` (exports), `../kivuli/examples/browser.la` (`header_bar(…)`).
- Lugha spec sections relied on: none new; kivuli needs a lughac from lugha `main`, milestone 100 or later. If the installed one is too old, stop and ask the owner (never build it).
- kivuli API used (all in 0.17's `lib.la`): `Font`, `Text`, `Background`, `Border`, `Side`, `Radius`, `Outline`, `Spacing`, `Edges`, `Sizing`, `Width`, `Size`, `Weight`, `Paint`, `hex`, `When`, `State`, `Shadow`, `header_bar`, `.header(Bar)`, `.style`, `button`, `button_with`, `input`, `text`, `column`, `row`, `overlay`, `scroll`, `icon`.
- Open decisions that must be resolved first: none (DECISION-030 and 031 resolved). DECISION-029 stays open and isn't touched.

## THE DARK THEME (proposed; the owner checks on screen)

| Field | Colour | Used for |
|---|---|---|
| `frame` | `#242424` (as today) | header bar, sidebar |
| `page` | `#161616` (as today) | content area, address box, suggestions card |
| `edge` | `#3a3a3a` (as today) | thin lines: the card, fields |
| `scrim` | `#00000060` (as today) | kept for the panel's return |
| `text` | `#e8e8e8` | all text |
| `dim` | `#e8e8e88c` (text at 55%) | secondary text |
| `control` | `#2e2e2e`, hover `#383838`, pressed `#424242` | normal buttons |
| `accent` | `#3584e4`, hover `#4a90e8`; text `#ffffff` | primary buttons, a focused field's ring |
| `danger` | `#c01c28`, hover `#d42c38`; text `#ffffff` | danger buttons |
| subtle | transparent, hover `#ffffff14` | subtle buttons, header icons |

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Bump kivuli to 0.17 with `lughac update src/main.la`; never edit `lugha.lock` by hand.
- Every colour from `themes.la`; every size, spacing and text size from `theme.la`'s tokens.
- Buttons styled by kind through `.style` (background, text colour, radius, hover and pressed through `When`), never the desktop theme's; a disabled button dimmed (`When(State.Disabled, …)`).
- Fields (the address box): `page` background, `text` colour, the pill radius; the focus ring in `accent` (`When(State.Focus, [Outline …])`).
- The window's look as after PRP-016, plus: header bar in `frame`, text everywhere in `text`/`dim`.
- Guests' `Normal`, `Primary`, `Danger`, `Subtle` buttons and `Body`, `Title`, `Heading`, `Dim` text keep their meaning; the contract doesn't change.
- Each test that got a message by clicking the old view sends that message to `update` instead, and checks the model; what it checked in the old view's text is dropped. Each dropped check is listed in CONTEXT.md.

### Must NOT Do
- Change the guest contract, the model, or `update`'s behaviour (beyond what PRP-014–016 did).
- Port the old view's screens: they're deleted (DECISION-030).
- Follow the desktop's light or dark theme anywhere (DECISION-031).
- Merge to `main` (DECISION-027).

## MODEL AND MESSAGES

- Model fields added or changed: none (`theme` holds the larger `Theme`).
- Messages added: none.

## ERROR HANDLING REQUIREMENTS

- None new. kivuli stops the app on a bad style value (a programmer error): tests build every style function once so a bad value fails a test, not the window.

## SECURITY CONSIDERATIONS

- Apps still can't set colours: their style words choose among Rapidkoi's looks.
- No new dependency: kivuli is already one; its 0.17 brings lugha-gtk 0.1.30 (fetched by lughac).
- The info panel, Account and Access, trust screens, are unreachable until rebuilt. Nothing grants or changes trust meanwhile; signing in and network switches keep their saved state.

## TESTS TO WRITE

Before the port:
- [x] `test/themes.la`: each style function returns the theme's colours (e.g. `button(DARK, Primary)` has `accent`'s background); every function in `DARK` builds without kivuli stopping (a view using all of them renders in `kivuli.testing`).
- [x] `test/render.la` (from `guest.la`'s render checks): each `TextStyle` and `ButtonLook` renders with its theme style; `Space` maps to the tokens; an unknown component says so.
- [x] `test/layout.la`: the header bar is styled `frame`; existing frame, address box and icon tests still pass.

Moved, not rewritten:
- `test/views.la` → `test/frame.la`: `label`, `initial`, `icon_of`, `marker`, `marker_icon`, `marker_note`, `matches`.

Converted (click the old view → send the message to `update`; drop old-view text checks): `test/shell.la`, `opening.la`, `menus.la`, `panel.la`, `account.la`, `access.la`, `effects.la`, `loading.la`, `loader.la`, `remote.la`, `saving.la`, `counter.la`.

Deleted with the old view: tests whose only subject is the old bar, rail, home grid or panel layout (listed in CONTEXT.md as they go).

## ROLLBACK PLAN

- Branch to return to: `prp-016-account_menu_icons` (this PRP's branch, `prp-017-kivuli_0_17`, stacks on it and is deleted unmerged).
- State: PRP-016 on kivuli 0.10.

## ACCEPTANCE CRITERIA
- [ ] The window as after PRP-016, in Rapidkoi's dark theme on a light desktop theme too, the header bar the frame's colour
- [ ] An app's normal, primary, danger and subtle buttons look different and react to hover
- [x] No `.bold`, `.dim`, `.padding`, `.spacing`, `.width`, `.height`, `.button_style`, `ButtonStyle`, `Surface` or `.header_style` left in `src/`
- [x] `lughac test test/` passes
- [x] `lughac build src/main.la -o main` succeeds
- [x] `lughac fmt --check src test` is clean
- [x] Every `public` item has a `/** … */` doc comment
- [x] No file over 500 lines
- [x] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/themes.la test/render.la test/layout.la`, then `lughac test test/`
- `lughac build src/main.la -o main`
- `grep -nE '\.(bold|dim|padding|spacing|width|height|button_style|header_style)\(|ButtonStyle|Surface' src/*.la` finds nothing
- For the user: switch the desktop to a light theme and run `./main`: the window stays dark, the header bar matches the sidebar. Open the counter (`./main guest/rust/target/wasm32-unknown-unknown/release/counter.wasm`): its text is light, its buttons are Rapidkoi's and change on hover. Type in the address box: the card's text is light, the box's focus ring is blue.
