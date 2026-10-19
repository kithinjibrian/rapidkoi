## FEATURE: Two icons at the right of the header bar: account and menu.

## OBJECTIVE
The next redesign step (DECISION-027), the owner's request (session 82). The header bar's right end shows an account icon and a menu icon, in that order, before GTK's − □ ×. In this step they're shown disabled, each with a tooltip saying what it will be, like today's other placeholders (DECISION-018). What each opens comes in its own PRP.

## CONTEXT

- Starting state: PRP-015's header bar: only the address box (`src/layout.la`, `.header(none, addressbox.field(m), none)`).
- Ending state:
  - `src/layout.la`: the header's end views are the two icons.
  - `src/frame.la`: `icon_button` becomes public, so the layout reuses it (same look and disabled behaviour as the old bar's).
  - `test/layout.la`: new tests. CHANGELOG.md, docs/DESIGN.md.
- Related existing code: `src/frame.la` (`icon_button`, today's disabled share button), `../kivuli/README.md` (`.header`, `.disabled`, `.icon`, `.tooltip`).
- kivuli API used: `button`, `.icon`, `.tooltip`, `.disabled`, `.button_style(Subtle)`; `kivuli.testing.is_disabled`, `tooltip`. All in kivuli 0.10.
- Icons: `avatar-default-symbolic` (account) and `open-menu-symbolic` (menu), both in Adwaita.
- Open decisions that must be resolved first: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Account: `avatar-default-symbolic`, tooltip "Account (coming soon)", disabled.
- Menu: `open-menu-symbolic`, tooltip "Menu (coming soon)", disabled.
- Order: account, then menu, at the header bar's end, so menu sits next to − □ ×.
- Subtle (flat) icon buttons, as the old bar's.

### Must NOT Do
- Open anything. Account and the menu each get their own PRP. The menu will need a kivuli change: kivuli 0.10 opens menus only by a right-click or long press, never a left click on a button.
- Touch the address box or the card.

## MODEL AND MESSAGES

- Model fields added or changed: none.
- Messages added: none.

## ERROR HANDLING REQUIREMENTS

- None: nothing can fail.

## SECURITY CONSIDERATIONS

- None: no input, capability or dependency.

## TESTS TO WRITE

`test/layout.la`, no display:
- [x] The view has an "Account (coming soon)" button, disabled.
- [x] The view has a "Menu (coming soon)" button, disabled.
- [x] Both are there on home and on an app page.

## ROLLBACK PLAN

- Branch to return to: `prp-015-address_box` (the PRP's branch, `prp-016-account_menu_icons`, stacks on it and is deleted unmerged).
- State: PRP-015's header.

## ACCEPTANCE CRITERIA
- [ ] The header bar shows the account and menu icons at its right, before − □ ×, greyed out, with their tooltips
- [x] `lughac test test/` passes
- [x] `lughac build src/main.la -o main` succeeds
- [x] `lughac fmt --check src test` is clean
- [x] Every `public` item has a `/** … */` doc comment
- [x] No file over 500 lines
- [x] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/layout.la`, then `lughac test test/`
- `lughac build src/main.la -o main`
- For the user: `./main`. At the header bar's right, before − □ ×: a person icon, then a ☰ menu icon, both greyed out; hovering each says what it will be.
