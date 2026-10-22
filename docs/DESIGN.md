# Design System — rapidkoi

rapidkoi draws nothing itself: kivuli (0.18) renders to GTK 4 widgets. Every colour comes from **Rapidkoi's theme** (`src/ui/themes.la`), never the desktop's GTK theme, light or dark (DECISION-031). Every size, spacing and text size comes from the tokens in `src/ui/tokens.la`. Both reach kivuli through `.style([...])`, its one way to style a node.

The screens of the old view (bar, rail, home, info panel, Account and Access pages) were deleted in PRP-017 (DECISION-030); their designs are in this file's git history, for when the redesign brings each back.

---

## Colour

**Everything** takes Rapidkoi's theme: its own parts and apps' content alike. Apps never set colours: they name a text style (`body`, `title`, `heading`, `dim`) or a button kind (`normal`, `primary`, `danger`, `subtle`), and the theme decides the look. Hex values live only in `src/ui/themes.la`.

Dark is first and the default (the frame and page are the owner's colours, 2026-10-17; the rest proposed in PRP-017):

| Field | Dark | Used for |
|-------|------|----------|
| `frame` | `#242424` | the header bar, the sidebar and the window behind the page, one flat surface (back from the gradient, session 118); cards and popovers (`Token.Card`, `Popover`) |
| `accent_end` | `#ff2e63` | the brand gradient's other end (`#ff7700 → #ff2e63`, session 112): kept, worn by nothing for now |
| `on_frame` | `#ffffff` | icons, text and the window controls' glyphs on the frame (session 116) |
| `frame_hover` | `#ffffff26` | a window control's round hover on the frame (`window_controls(t)`; close turns `danger`) |
| `page` | `#161616` | the content area, the address box, the suggestions card, text fields |
| `edge` | `#3a3a3a` | thin lines: the suggestions card, text fields |
| `scrim` | `#00000060` | kept for the info panel's return |
| `text` | `#e8e8e8` | all text |
| `dim` | `#e8e8e88c` | secondary text (text at 55%) |
| `control` / `_hover` / `_press` | `#2e2e2e` / `#383838` / `#424242` | normal buttons |
| `accent` / `accent_hover`, `on_accent` | `#ff7700` / `#ff8c2e`, `#161616` | **Rapidkoi's brand colour** (the owner, session 111): primary buttons, focus rings, a focused field's edge; dark text on it (white reads at 2.66:1) |
| `danger` / `danger_hover`, `on_danger` | `#c01c28` / `#d42c38`, `#ffffff` | danger buttons |
| `subtle_hover` | `#ffffff14` | subtle buttons under the pointer (transparent at rest) |
| `tooltip` | `#000000` | behind every tooltip's text (`tooltips(t)` on the root, padded `SPACE_MD` × `SPACE_SM`, rounded `BUTTON_RADIUS`, session 146) |
| `clear` | `#00000000` | nothing: the click-catching layer |

**Looks** are functions in `ui/themes.la` returning `Styling[]`: `frame`, `page`, `field`, `address`, `suggestions`, `clear`, `body`, `dim`, `title`, `heading`, `button(t, kind)`.
- Buttons: the kind's colour, no border, `BUTTON_RADIUS` corners, the hover and pressed colours through `When(State.Hover | Press, …)`, half opacity while disabled.
- Fields: the page's colour, an `EDGE_WIDTH` edge in `edge`, turning `accent` while focused.
- The page: only its top-right corner curves (`PAGE_RADIUS`), where the frame wraps it.

## Spacing (pixels)

| Token | Value | Use |
|-------|-------|-----|
| `SPACE_XS` | 4 | inside tight groups (icon + label) |
| `SPACE_SM` | 8 | between related controls |
| `SPACE_MD` | 12 | between rows of a form |
| `SPACE_LG` | 24 | the page's margin, between sections |
| `SPACE_XL` | 36 | between major regions |

Written through `ui/tokens.la`'s helpers: `margin(n)` (room outside a node), `padding(n)` (inside its border), `gap(n)` (between children), `at_least(w, h)`, `at_most(w)`. The contract's `padding` is a margin.

## Typography (points)

| Token | Value | Use |
|-------|-------|-----|
| (none) | theme default | body text |
| `TEXT_TITLE` | 18 | a view's title, bold (`title(t)`) |
| `TEXT_HEADING` | 14 | section headings, bold (`heading(t)`) |

## The frame

The window opens maximized (`WINDOW_WIDTH` × `WINDOW_HEIGHT`, 960 × 700, when not).
- **The header bar** (PRP-019, `src/ui/header.la`, in `frame(t)`): ← back, ⌂ home, → forward on an app page (none on an empty tab, session 110); ⌂ is the app's own home screen, disabled with ← and → until links give apps routes (PRP-024); the address box; 👤 account and ☰ menu; then the window controls, styled by `kivuli.ui`'s `WindowControls.look()` then `window_controls(t)`: white glyphs at `HEADER_ICON` (session 126), `SPACE_XS` either side, a round hover on each 24 × 24 icon box padded `SPACE_XS` (32 across, `CIRCLE`), red for close, the close button's own red cleared (session 119) (DECISION-035, session 116). Every header button has `SPACE_XS` either side. Each control is a `kivuli.ui` ghost icon button (`ui.button_with` + `Button.ghost()` + `Button.icon()`, 36 px) holding one of Rapidkoi's Iconsax icons (`src/ui/icons.la`, recoloured to the theme's `text`) at `HEADER_ICON` (20, the owner's call, session 125: every icon matches), capped with `square(n)` (DECISION-034), each button centred down the bar and each icon centred in its button. The shield and ☆ inside the address box are `ADDRESS_ICON` (20, session 125); the address box's icon buttons are circles, `SPACE_XS` around the icon, with a round hover, and the shield and ⚙ sit in one row with no gap between their buttons (session 145). Back, forward, account and menu are disabled with a tooltip saying when they come. No reload (the owner, session 101).
- **The sidebar:** empty, `FRAME_THICKNESS` (50) wide, the frame's colour: the frame is an even L.
- **The sidebar** (PRP-023, `src/ui/sidebar.la`): pinned apps (open or not), a line, the other tabs, app and empty, in the order they were made (an empty tab: `tab-new-symbolic`, "New tab", PRP-024), scrolling when they don't fit, then + ("Add a tab", Ctrl+T) fixed at the bottom (Chrome's way and scrolling an app into view wait for kivuli, DECISION-037). Each app a round ghost icon button (`app_button`: a faint wash on hover), its icon at `HEADER_ICON` or its initial; the current one ringed in the accent (`app_current`, `RING_WIDTH` 2) with a small × at its top-right (`tab_close`, `TAB_CLOSE` 16, the desktop's `window-close-symbolic` at `TAB_CLOSE_ICON` 10; red on hover; Ctrl+W). Right-click: Close / Pin or Unpin / Forget this app… (`Open` for a pin that isn't open). A × on any hovered app waits for kivuli's group hover (DECISION-036).
- **The content area:** the current app, `SPACE_LG` from its edges, scrolling; empty on home until the new tab is a bundled app.
- **The address box** (PRP-015, PRP-019, `src/ui/addressbox.la`): a `kivuli.ui` `input_group`, `ADDRESS_HEIGHT` (30) tall exactly on every tab (`fixed_height`, session 152), `SPACE_SM` above and below it in the bar (session 107), and at least `ADDRESS_WIDTH` (560) wide, rounded by `CIRCLE` (a pill at any height: GTK caps a radius at half the box, session 121); no focus glow, only a thin ring edge (session 117). At its start, on every app page, the shield (`shield-tick`, "Site information") in its look: white when verified, the accent orange for caution (a development address, a key not confirmed yet, a new publisher), `faint` grey when unsigned (PRP-021). Clicking it opens its card, a `kivuli.ui` popover hanging from the shield (its rows' hover rounded `BUTTON_RADIUS`, the app's icon `CARD_ICON` 20, session 136), `CARD_WIDTH` (360) on every page (session 134), its rows padded `SPACE_MD` above and below, `SPACE_SM` at the sides (sessions 135, 144): the app's icon and name, then "Connection secure" and "Signed app" rows (their words follow the state; words at the left, the `arrow-right4` icon in `faint` at the right, session 137; a pill hover), each opening a page with back (the `arrow-left4` icon, a round hover, session 138), then its facts (address; publisher and key, since when, version), no explaining sentences (session 143). Then site settings (`settings`, disabled "Site settings (coming soon)" until the settings exist, PRP-020) on any app page; at its end ☆ (`star`, Pin / Unpin) on an app page. In the middle one field on every tab (PRP-025): "Enter Address" on an empty tab, the app's address on an app, typed in straight away. At the end: share (`export-circle2`, greyed "Share (comes with links)"), then ☆ on an app. Its suggestions card is gone until its redesign (session 108); a failed open's reason stays in the model (`notice`) meanwhile.

## Text alignment

kivuli centers text by default. Rapidkoi puts text at the **start of its line** (`.align(Align.Start)`): an app's text nodes, the error screens.

## Where an app came from (PRP-006)

- **Markers**, drawn by Rapidkoi only, in the address box:
  - `[security-high-symbolic] HOST`: a signed site whose checks all passed, or a signed file whose key is trusted
  - `[dialog-warning-symbolic] HOST · not confirmed`: a signed file whose key Rapidkoi hasn't met
  - `· new publisher` after the host when a changed key was opened as a separate app
  - `🛠 localhost:8000 · development`, `📦 file · local bundle`, `📄 file · local file`, `HOST · loading…` in words, no icon
  - The address field's hover text says it in words ("HOST · verified", "· verified (development)", "· trusted")
- **A failed check:** "Rapidkoi didn't run this app" (title) over the reason (dim). No Restart and no run-anyway.
- **A changed key:** "HOST is now signed by a different key" (title), "This can mean the site changed owners." and both fingerprints (dim), then **Keep using my version** (primary; "Don't open it" when nothing was open) and **Open as a separate app** (normal).

## Component rules

- Correct:

      column([...]).style([margin(SPACE_LG), gap(SPACE_MD)])
      button("Delete", Msg.Delete).style(button(m.theme, ButtonLook.Danger))
      text(reason).style(dim(m.theme))

- Wrong:

      // a bare number: drifts from the scale
      column([...]).style([Spacing { gap: Width.Px(20), ..Spacing.theme() }])
      // a colour outside themes.la
      text(reason).style([Text { color: Paint.Color(hex("#999")), ..Text.theme() }])

## What is forbidden

- No numbers in a `Spacing`, `Sizing`, `Font` or `Border` other than the tokens above.
- No colours outside `src/ui/themes.la`; nothing follows the desktop theme; apps never get colours.
- Icons are theme icon names, preferably `…-symbolic`.
