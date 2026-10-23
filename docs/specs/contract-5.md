# Contract 5: boxes, texts, styles and roles

PRP-037 (part 1 of contract 5; PRP-038 and PRP-039 complete it), DECISION-049. Contract 4 (`contract-4.md`) with a new screen description: apps build every screen from three kinds of node, style them freely, and say what each piece is. Events, state, links and asks are contract 4's.

Contract 5 is open until PRP-039 lands. Nothing outside this repository targets it before then.

## Nodes

Every node is `{ "kind", "props", "children"? }`, as before.

| Kind | Props | Children |
|---|---|---|
| `box` | `style`, `role`, `checked`, `selected`, `disabled`, `label`, `on_press`, and `title` on the root | yes |
| `text` | `text` (required), `style`, `role`, `label` | no |
| `input` | `value` (required), `placeholder`, `lines` (1–20; more than 1 is a box of several lines), `on_change` (none: disabled), `on_submit` (Enter, on one line; sent with the text as `value`), `style` | no |

- **A box with `on_press`** is pressable: focusable, and pressed by a click, Enter or Space. It sends `on_message` with its message.
- **`disabled: true`** greys a pressable box out, and it sends nothing.
- **Any other kind** (contract 4's `button`, `column`, `row` included) is shown as an unsupported placeholder, its subtree still counted against the limits.

Limits are those of earlier contracts: 4 MB, 10,000 nodes, 64 deep, and each string's length.

## Style

One map, every key optional, at most 32 keys. A known key with a bad value stops the app with its sentence. An unknown key is ignored, so later versions can add keys.

| Key | Value |
|---|---|
| `direction` | `"column"` (default) or `"row"` |
| `gap`, `padding` | `"xs"`…`"xl"` or 0–256 px; `padding` also `[vertical, horizontal]` |
| `align` | `"start"`, `"center"`, `"end"`, `"stretch"` |
| `grow` | `true`: take the spare room along the parent's direction |
| `width`, `height`, `min_width`, `max_width`, `min_height`, `max_height` | 0–4096 px |
| `background`, `color` | a colour |
| `border` | `{ "width": 0–16, "color": colour }` |
| `outline` | `{ "width": 0–8, "color": colour }` |
| `corner` | `"small"`, `"medium"`, `"large"`, `"round"`, or 0–256 px |
| `shadow` | `"none"`, `"sm"`, `"md"`, `"lg"` |
| `opacity` | 0–1 |
| `size` | `"title"`, `"heading"`, `"body"`, `"small"`, `{ "pt": 6–72 }` or `{ "rem": 0.5–6 }` |
| `weight` | `"normal"`, `"medium"`, `"bold"` |
| `font` | `"body"`, `"mono"`, `"serif"`, `"sans"` |
| `italic` | `true` |
| `hover`, `pressed`, `focus`, `disabled`, `checked` | a style map without these keys |

The `checked` look is laid over the base when the node is `checked: true`.

**A colour** is one of three things:
- one of Rapidkoi's names: `background`, `surface`, `text`, `text_dim`, `accent`, `on_accent`, `border`, `danger`, `on_danger`;
- the app's own: `#rrggbb` or `#rrggbbaa`;
- `{ "mix": [colour, colour, 0–1] }`, the first moved towards the second, with no mix inside a mix.

Rapidkoi's names, its spacing and corner sizes, and its text sizes are resolved when Rapidkoi draws. The app never learns their values, and they follow Rapidkoi's theme. Raw values stay as written.

A pressable box starts with no look at all (no fill, edge or shadow) apart from an accent ring when it has the keyboard. A text starts in the theme's text colour. A typing box starts in Rapidkoi's field look.

## Roles

`role`: `button`, `checkbox`, `switch`, `tab`, `tab_list`, `heading`, `list`, `list_item`, `group`, `separator` (AccessKit's names).

State:
- `checked`: a checkbox or switch only;
- `selected`: a tab or list item only;
- `disabled`;
- `label`: what a screen reader says when the text isn't enough.

Rapidkoi uses roles for focus, pressing by keyboard and the checked look. Screen readers hear them once kivuli passes them on (DECISION-050).

## Rust

- `rapidkoi::raw` has the primitives: `boxed()`, `text()`, `field()`, `Role`, `.style([...])`, `.role()`, `.checked()`, `.on_press()`, `.on_submit()`, `.lines()`.
- `rapidkoi::style` has the values: `Background`, `Text`, `Font`, `Border`, `Outline`, `Spacing`, `Sizing`, `Shadow`, `Effects`, `Layout`, `When(State, …)`, `Color::system`/`rgb`/`rgba`/`.mix`.
- The widgets (`text().title()`, `button().primary()`, `checkbox`, `tabs`, `spacer`, `input`, `text_area`, `column`, `row`) are built from them, and `.style([...])` restyles any of them.
