## FEATURE: The header bar redesigned on kivuli.ui, with Rapidkoi's palette and the Iconsax icons.

## OBJECTIVE
The first step of the full redesign on `kivuli.ui` (sessions 99–101). The header bar reads, left to right: ← back, ⌂ home, → forward; the address box as one framed group with a 🛡 trust shield at its start and a ☆ pin at its end; 👤 account and ☰ menu; then GTK's − □ ×. Every icon is the owner's Iconsax SVG in the theme's text colour, and the buttons are `kivuli.ui` ghost icon buttons. Colours come from a dark `kivuli.ui` palette made of Rapidkoi's theme. Nothing else on screen changes: the sidebar, content area and suggestions card stay as they are.

## CONTEXT

- Starting state: `src/ui/layout.la` builds the header (`header_bar([], addressbox.field(m), [account, menu])`), `src/ui/addressbox.la` the rounded field or the marker button, `src/ui/frame.la`'s `icon_button` the theme-icon buttons; colours from `src/ui/themes.la`. kivuli 0.48.
- Ending state:
  - `src/ui/icons.la` (new, pure): each Iconsax SVG as a string constant (`arrow-left4`, `home`, `arrow-right4`, `shield-tick`, `star`, `profile-circle`, `menu`; also `settings`, `rotate-right` kept for later), and `icon(name, colour): Image`, the SVG with its `#ffffff` fill swapped for `colour`, as `Image.Bytes` keyed by name and colour.
  - `src/ui/header.la` (new, pure): `bar(m): Bar<Msg>`, the whole header.
  - `src/ui/addressbox.la`: `field` becomes a `kivuli.ui` `input_group`: shield at the start (signed apps), the field or the address in the middle, ☆ at the end (app pages).
  - `src/ui/themes.la`: `palette(t): Palette` (Rapidkoi's dark colours as shadcn tokens, below); `Theme` keeps `frame` and the colours apps' content still uses.
  - `src/ui/layout.la`: `.header(header::bar(m))`, `.palette(palette(t))` and `.scheme(Scheme.Dark)` on the root.
  - `src/ui/frame.la`: `icon_button` removed (replaced by the header's).
  - Tests: `test/icons.la`, `test/header.la` (new); `test/addressbox.la`, `test/layout.la` updated.
  - `docs/DESIGN.md`, CLAUDE.md's tree, CHANGELOG.md.
- Related existing code: `../kivuli/src/ui/buttons.la` (`ui.button_with`, `Button.ghost()`, `Button.icon()`), `../kivuli/src/ui/input_groups.la` (`input_group`, `group_input`, `group_button_with`), `../kivuli/src/styles/palettes.la` (`Palette.zinc().with(token, light, dark)`), `../kivuli/README.md` (Components, Palettes).
- Lugha spec: no compile-time file embedding, so the SVGs live in a module as strings (escapes `\"`, section 2); `s.split(sep)`, `xs.join(sep)` for the recolouring.
- kivuli API used: `kivuli.ui { button_with, Button, input_group, group_input, group_button_with }`, `image`, `Image.Bytes`, `Palette`, `Token`, `Scheme`, `.palette`, `.scheme`, `header_bar`, `.header`.
- Open decisions that must be resolved first: none.

## THE HEADER

| Control | Icon | What it does in this PRP |
|---|---|---|
| ← back | `arrow-left4` | disabled, "Back (comes with links)": history is build step 2 |
| ⌂ home | `home` | `Msg.Home`, as today |
| → forward | `arrow-right4` | disabled, "Forward (comes with links)" |
| address box | | `input_group`; behaviour as PRP-015 (typing, the card, Enter, Escape, Ctrl+K) |
| 🛡 (group start) | `shield-tick` | only for a signed app whose key is verified or trusted; its tooltip is today's marker note; it sends nothing until app info returns. Not confirmed: no shield, the words "· not confirmed" stay in the address. Unsigned: no shield. |
| ☆ (group end) | `star` | app pages only: `Msg.TogglePin`, tooltip "Pin" / "Unpin" |
| 👤 account | `profile-circle` | disabled, "Account (coming soon)" |
| ☰ menu | `menu` | disabled, "Menu (coming soon)" |

No reload (the owner, session 101). Buttons: `ui.button_with([icon], msg)` with `Button.ghost()` and `Button.icon()` (36 px squares); icons 20 px (a token, `HEADER_ICON`).

## THE PALETTE (dark only; the desktop is ignored, DECISION-031)

`Palette.zinc()` with every token below set, light and dark alike, and `.scheme(Scheme.Dark)`:

| Token | Colour | From |
|---|---|---|
| `Background` / `Foreground` | `#161616` / `#e8e8e8` | page / text |
| `Card`, `Popover` / `…Foreground` | `#242424` / `#e8e8e8` | frame / text |
| `Primary` / `PrimaryForeground` | `#3584e4` / `#ffffff` | accent |
| `Destructive` / `…Foreground` | `#c01c28` / `#ffffff` | danger |
| `Secondary`, `Muted` / `…Foreground` | `#2e2e2e` / `#e8e8e8`, `#e8e8e88c` | control / text, dim |
| `Accent` / `AccentForeground` | `#383838` / `#e8e8e8` | control hover (a ghost button's hover) / text |
| `Border`, `Input` | `#3a3a3a` | edge |
| `Ring` | `#3584e4` | accent |

Hex values stay in `themes.la`. The frame's `#242424` header and sidebar keep `themes.frame(t)`.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Icons come only from `ui/icons.la`, recoloured to the theme's `text` (disabled ones dim through the button's disabled look).
- A test keeps each constant identical to its file in `assets/icons/`, so the two can't drift.
- Every size is a token (`HEADER_ICON`; kivuli.ui's own sizes are its design).
- The header keeps the frame's colour (`header_bar(…).style(frame(t))`).

### Must NOT Do
- Restyle the sidebar, content area, suggestions card or apps' content: later PRPs of the redesign.
- Add reload, settings or any new behaviour.
- Read `assets/` at run time.
- Merge into `redesign` or `main`, or open a branch (session 90).

## MODEL AND MESSAGES

- Model fields added or changed: none.
- Messages added: none (`Home`, `TogglePin`, `EditAddress`, `Draft`, `Go`, `Dismiss` exist).

## ERROR HANDLING REQUIREMENTS

- None new. An SVG kivuli can't draw shows nothing (kivuli's rule); the test against `assets/` keeps them valid.

## SECURITY CONSIDERATIONS

- The icons are Rapidkoi's own, compiled in; apps can't reach them. The shield is drawn only by Rapidkoi, in the header, where apps can't paint.
- Iconsax's licence must cover shipping the icons before a release (the owner's to check).

## TESTS TO WRITE

`test/icons.la`:
- [x] Each icon constant equals its file in `assets/icons/`.
- [x] `icon(name, colour)` puts `colour` where `#ffffff` was, and nowhere else changes; its key names the icon and the colour.
- [x] (Changed while building: an `Icon` enum, not names, so an unknown icon can't compile; Lugha tests can't expect a panic.)

`test/header.la` (through `kivuli.testing`):
- [x] Order: Back, Home, Forward, the address box, Account, Menu (by tooltip).
- [x] Back, Forward, Account, Menu are disabled; Home sends `Msg.Home`.
- [x] On home: no ☆ and no shield; the field takes typing.
- [x] On an unsigned app: ☆ "Pin" sends `TogglePin`, and after pinning it reads "Unpin"; no shield.
- [x] On a verified signed app: the shield is there, its tooltip the marker note; on a not-confirmed one, no shield.

`test/addressbox.la` and `test/layout.la`: keep passing (the field is now inside the group).

## ROLLBACK PLAN

- `git revert` the PRP's commit on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] The header shows ← ⌂ →, the framed address box with 🛡 and ☆ where they apply, 👤 ☰, in white Iconsax icons on the frame's colour
- [ ] The tests above pass, and `lughac test test/` passes (the owner runs them)
- [x] `lughac build src/main.la -o main` succeeds; `lughac fmt --check src test` is clean
- [x] Every `public` item has a `/** … */` doc comment; no file over 500 lines
- [x] CHANGELOG.md, DESIGN.md and CLAUDE.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/icons.la test/header.la test/addressbox.la test/layout.la`, then `lughac test test/` (the owner)
- `lughac build src/main.la -o main`
- For the user: `./main`: ← ⌂ → at the left (← → greyed), the address box framed in the middle, 👤 ☰ greyed at the right. Open `rapidkoi://counter.dafifi.net`: the shield at the box's start, ☆ at its end; click ☆, hover it: "Unpin".
