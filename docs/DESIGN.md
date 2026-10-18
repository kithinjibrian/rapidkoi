# Design System — rapidkoi

rapidkoi draws nothing itself. kivuli renders to GTK 4 widgets, and every colour comes from the user's GTK theme (Adwaita by default). This file fixes the numbers the app passes to kivuli's modifiers and which kivuli styles mean what. See `../kivuli/docs/DESIGN.md` for how kivuli maps them onto GTK.

The values below are a starting scale. Revisit them once DECISION-001 says what the app looks like.

---

## Colour

**App content** has no colour tokens. Colours come only from the GTK theme, through kivuli:

| Intent | kivuli |
|--------|--------|
| The one main action in a view | `button(…).style(ButtonStyle.Primary)` |
| Deletes or discards | `.style(ButtonStyle.Danger)` |
| Low-emphasis, toolbar-like | `.style(ButtonStyle.Subtle)` |
| Secondary text | `text(…).dim()` |

**Rapidkoi's own parts** take colours from a **Rapidkoi theme** (`src/themes.la`), the owner's choice (2026-10-17, sessions 10 and 12; inspiration: `docs/source/meetings/2026-10-17-ui/inspiration-opera-frame.png`).
- **The frame is one surface:** the top bar and the rail are the same colour, with no shadow or line between them.
- **The page (home or an app) sits flush inside it**: it meets the header bar above, the rail on its right, and the window's edges. **Only its top-right corner curves** (`PAGE_RADIUS`), where the frame wraps it (the owner, session 14; screenshot `2026-10-17-ui/screenshot-4-after-header-look.png`).
- Dark is first and the default:

| Field | Dark | Look |
|-------|------|------|
| `frame` | `#242424`, flat (no shadow or edge line) | `Look.Frame`: the header bar (`.header_style`), the window around the page, and the rail on it |
| `page` | `#161616` | `Look.Page`: the page; only its top-right corner curves, `PAGE_RADIUS` (12 px) |

Only backgrounds and the one corner are set; text and borders stay the GTK theme's. Light and other themes come later, with settings. Hex values live only in `src/themes.la`.

## Spacing (pixels: `.padding`, `.spacing`)

| Token | Value | Use |
|-------|-------|-----|
| `SPACE_XS` | 4 | inside tight groups (icon + label) |
| `SPACE_SM` | 8 | between related controls |
| `SPACE_MD` | 12 | between rows of a form |
| `SPACE_LG` | 24 | window padding, between sections |
| `SPACE_XL` | 36 | between major regions |

## Typography (points: `.size`, `.bold`)

| Token | Value | Use |
|-------|-------|-----|
| (none) | theme default | body text: don't call `.size` |
| `TEXT_TITLE` | 18 | a view's title, with `.bold()` |
| `TEXT_HEADING` | 14 | section headings, with `.bold()` |

## The frame (Rapidkoi's own UI)

From `docs/source/meetings/2026-10-17-ui-design.md`. Pixels. The bar sits in the window's header bar (one row with the window controls), and the window opens maximized.

| Token | Value | Use |
|-------|-------|-----|
| `FRAME_THICKNESS` | 50 | the rail's whole width, equal to the header bar's height: the frame is an even L (session 15) |
| `RAIL_WIDTH` | 34 | the rail's column (`FRAME_THICKNESS` less `SPACE_SM` padding each side) |
| `TAB_SIZE` | 34 | one tab button in the rail: the header's icon-button size |
| `TAB_ICON` | 22 | an app's icon on its rail tab |
| `TILE_ICON` | 64 | an app's icon on its home tile |
| `TILE_SIZE` | 120 | a Pinned tile on home (square) |
| `BANNER_HEIGHT` | 180 | "Where you left off" |
| `PANEL_WIDTH` | 360 | the app info panel, fixed: its text never widens it (session 31) |
| `PANEL_CHARS` | 30 | the most characters a wrapped panel line asks for (`.wrap_at`), kept well under the width so the panel never shifts between pages; the name wraps at 14, the header's file name is cut at 18, paths are cut with … and shown whole on hover |
| `PAGE_RADIUS` | 12 | the page's top-right curve |
| `WINDOW_WIDTH` × `WINDOW_HEIGHT` | 960 × 700 | the window's default size |

- Toolbar buttons are icon-only (`button("", msg).icon(name)` with a `.tooltip`), using symbolic theme icons:
  - back `go-previous-symbolic`, home `go-home-symbolic`, forward `go-next-symbolic`
  - info `dialog-information-symbolic`, share `emblem-shared-symbolic`
  - new tab `list-add-symbolic`, settings `emblem-system-symbolic`, more `view-more-symbolic`
- The current tab is `Primary`; the others are `Subtle`. Home isn't a tab: the rail lists apps only.
- The verified mark is `✓`; a local file is marked `📄 … · local file`. Both are drawn only by Rapidkoi, never inside the app's area.
- The art palette for generated app art comes with PRP-003.

## Text alignment (session 28)

kivuli centers text by default. Rapidkoi puts text at the **start of its line** (`.align(Align.Start)`): an app's text nodes, the error screen, home's headings and messages, and everything in the info panel. In a row, the label sits at the start and a `spacer()` takes the room, never an expanded label, which centers its text. Tile names stay centered under their tiles.

## The bar and the info panel (sessions 26–27)

- **The bar:** `‹ ⌂ › ⟳ [address ☆] ⓘ ⤴`.
  - ⟳ (`view-refresh-symbolic`) reloads, keeping the app's state.
  - ☆ (`non-starred-symbolic`, ★ `starred-symbolic` when pinned) pins.
- **The info panel** (the owner's sketch, `docs/source/meetings/2026-10-17-ui/sketch-3-info-panel.png`):
  - a header (a 48 px icon, the name in title style, `Version …` dim, and the marker as a subtle button opening About)
  - the description
  - the panel on the frame's colour at a fixed `PANEL_WIDTH` (the owner preferred it to a page-coloured panel, session 29; fixed whatever its text, session 31)
  - **Your settings** and **On this device**, each a small dim heading over a `Look.Section` card (the frame's `#242424`, `PAGE_RADIUS` corners) of rows: the label, a dim summary, and a › arrow
  - **Forget this app** (danger) and **Close**, side by side in the bottom row
- Rows open pages with a ‹ Back arrow. **Start over** and **Forget** always ask first, in place, with [Cancel] and the danger action.

## The panel, menus and home's grid (PRP-010)

- **The info panel sits over the app** at `PANEL_WIDTH`, placed at the end and the full height. The app keeps its width. A dim `Look.Scrim` layer (the theme's `scrim`, `#00000060` in dark) covers the rest of the app; clicking it ("Click to close the panel") or Escape closes the panel.
- **A tab's right-click menu:** Reload (greyed while loading), Pin to home / Unpin, About this app, separator, Close tab, Forget this app…. The tab is selected first. Forget opens the panel at its question.
- **A pinned tile's menu:** Open, Unpin, separator, Forget this app…. The "where you left off" card's Continue: Open, separator, Forget this app….
- **Home's pinned tiles** wrap, all one width (`wrap(...).equal()`, `SPACE_MD`).
- **Panel rows** are one `Subtle` button each (`button_with`): label, spacer, dim summary, ›.

## Where an app came from (PRP-006)

- **Markers**, drawn by Rapidkoi only, in the bar and the panel's header:
  - `✓ counter.dafifi.net · verified`: a remote signed site over HTTPS, every check passed (PRP-009)
  - `✓ localhost:8000 · verified (development)`: the same, from this machine
  - `✓ host`: a signed file whose key is trusted this session
  - `⚠ host · not confirmed`: a signed file whose key Rapidkoi hasn't met (it runs, as a stranger)
  - `· new publisher` after a ✓ when a changed key was opened as a separate app
  - `🛠 localhost:8000 · development`, `📦 file · local bundle`, `📄 file · local file` as before
- **A failed check:** "Rapidkoi didn't run this app" (title style) over the reason (dim). No Restart and no run-anyway; ⟳ tries again.
- **A changed key:** "HOST is now signed by a different key" (title style), "This can mean the site changed owners." and both fingerprints (dim), then **Keep using my version** (primary; "Don't open it" when nothing was open) and **Open as a separate app**.
- **About** for a signed app: Publisher `host · key xxxx-xxxx-xxxx-xxxx`, Trust, Since ("Trusted since 18 Oct 2026, first use"), From, Version, Contract, Build `sha256` + 16 hex digits (all 64 on hover), Format.

## In code

The tokens are `public` constants in `src/theme.la`. Views use the names, never the numbers.

## Component Rules

- Correct:

      column([...]).padding(SPACE_LG).spacing(SPACE_MD)
      button("Delete", Msg.Delete).style(ButtonStyle.Danger)

- Wrong:

      // a bare number: drifts from the scale
      column([...]).padding(20)
      // custom CSS colour: breaks dark and high-contrast themes
      button("Delete", Msg.Delete).css("color: #c01c28")

## What Is Forbidden

- No numbers in `.padding`, `.spacing` or `.size` other than the tokens above.
- No colours, custom CSS or inline styles, except Rapidkoi's themes in `src/themes.la`, applied as kivuli looks to Rapidkoi's own parts. Apps never get colours.
- Icons are theme icon names, preferably `…-symbolic`.
