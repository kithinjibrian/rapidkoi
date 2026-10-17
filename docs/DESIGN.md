# Design System — rapidkoi

rapidkoi draws nothing itself. kivuli renders to GTK 4 widgets, and every colour comes from the user's GTK theme (Adwaita by default). This file fixes the numbers the app passes to kivuli's modifiers and which kivuli styles mean what. See `../kivuli/docs/DESIGN.md` for how kivuli maps them onto GTK.

The values below are a starting scale. Revisit them once DECISION-001 says what the app looks like.

---

## Colour

No colour tokens. Colours come only from the theme, through kivuli:

| Intent | kivuli |
|--------|--------|
| The one main action in a view | `button(…).style(ButtonStyle.Primary)` |
| Deletes or discards | `.style(ButtonStyle.Danger)` |
| Low-emphasis, toolbar-like | `.style(ButtonStyle.Subtle)` |
| Secondary text | `text(…).dim()` |

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

From `docs/source/meetings/2026-10-17-ui-design.md`. Pixels.

| Token | Value | Use |
|-------|-------|-----|
| `RAIL_WIDTH` | 64 | the tab rail on the right |
| `TAB_SIZE` | 44 | one tab button in the rail |
| `TILE_SIZE` | 120 | a Pinned tile on home (square) |
| `TILES_PER_ROW` | 4 | until kivuli has a wrapping grid (DECISION-018) |
| `BANNER_HEIGHT` | 180 | "Where you left off" |
| `PANEL_WIDTH` | 280 | the app info panel |

- Toolbar buttons are icon-only (`button("", msg).icon(name)` with a `.tooltip`), using symbolic theme icons:
  - back `go-previous-symbolic`, home `go-home-symbolic`, forward `go-next-symbolic`
  - info `dialog-information-symbolic`, share `emblem-shared-symbolic`
  - new tab `list-add-symbolic`, settings `emblem-system-symbolic`, more `view-more-symbolic`
- The current tab is `Primary`; the others are `Subtle`.
- The verified mark is `✓`; a local file is marked `📄 … · local file`. Both are drawn only by Rapidkoi, never inside the app's area.
- The art palette for generated app art comes with PRP-003.

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
- No colours, custom CSS, or inline styles.
- Icons are theme icon names, preferably `…-symbolic`.
