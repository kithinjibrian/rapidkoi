# Retiring four placeholders with kivuli 0.10 (PRP-010) — design in chat, session 47, 2026-10-18

The owner and Claude, one topic at a time; each answered "yes". Feeds PRP-010. kivuli 0.7–0.10 brought `wrap`/`keyed_wrap` (`.equal`, `.max_per_line`), `overlay`/`.place`, `.menu`/`item`/`menu_separator` (and `testing.pick`), and `button_with`. Still missing: a file dialog (that placeholder stays).

## 1. The info panel as an overlay — settled

- It covers the app's right side, between the page and the tab rail, at `PANEL_WIDTH`, looking exactly as today (frame colour, cards, Forget and Close at the bottom). Only its place changes.
- The app underneath keeps its full width: nothing reflows when the panel opens or closes.
- A dim layer over the rest of the app while it's open; clicking it closes the panel. Escape closes it too, as do Close and ⓘ.
- The app isn't clickable while the panel is open (Start over and Forget act on it).

## 2. A tab's right-click menu — settled

- Reload; Pin to home / Unpin; About this app; separator; Close tab; Forget this app… (opens the panel at its question, so it still asks first).
- Picking an item selects that tab first, then acts: one set of actions for the bar and the menu.
- Right-click, long press, or Menu / Shift+F10 (kivuli gives all three).
- Home's tiles (corrected in chat): Open; Unpin (pins); Forget this app… (opens the app with the panel at the question).

## 3. Home's tiles in a wrapping grid — settled

- Pinned apps re-flow with the window's width, as many per line as fit, every tile one width (`wrap` + `.equal()`), replacing rows of 4.
- Tiles unchanged; "Where you left off" stays its own card above; search filtering unchanged (the grid closes up).

## 4. Panel rows clickable anywhere — settled

- Each row (label, dim summary, ›) is one `Subtle` `button_with`: click anywhere, hover highlight, Tab and Enter. Same look at rest.
- The header's publisher marker stays a subtle button.
- DECISION-020's clickable rows are done.
