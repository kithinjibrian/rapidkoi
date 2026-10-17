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

## In code

The tokens become `public` constants in one module (`src/theme.la`), created by the first PRP that writes view code. Views use the names, never the numbers.

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
