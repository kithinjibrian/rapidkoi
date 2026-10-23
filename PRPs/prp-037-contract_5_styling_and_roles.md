## FEATURE: Contract 5, part 1: apps build their screens from boxes and text, style them freely through one door with Rapidkoi's names that follow the person, and say what each piece is (roles); the Rust library rebuilds its widgets on these primitives.

## OBJECTIVE
From chat with the owner (sessions 197–198), from the basics: "let people style their apps however they want", while five things stay in the person's hands (bigger text; light or dark and contrast; less motion; knowing which app they're in; using any app without seeing it or without a mouse).

Done looks like this. A contract 5 app sends a tree of `box`, `text` and `input` nodes. Each node has a `style` (colours, font, size, borders, corners, shadow, spacing, sizes, and looks on hover, press, focus and when disabled), a `role` (button, checkbox, tab, heading…) and its state (checked, selected). Rapidkoi draws exactly that. Where the app used Rapidkoi's names (`accent`, `surface`, `title`, `md`…) instead of raw values, it follows Rapidkoi's theme. A box with `on_press` is reachable with Tab and pressed with Enter or Space. In Rust, `button("Add").primary()` still works: the library now builds it from a styled box with role button, and `.style([...])` restyles any widget.

## WHY (the owner's decisions, recorded)
- **Rapidkoi is the engine, the Rust library is the framework** (the web's and Flutter's split, from the session 198 research). The contract carries primitives, styles and roles; widgets live in the library, where they can change freely.
- **The contract only gains what an app can't build without painting:** typing, scrolling and long lists, things on top, pictures, pickers. This PRP adds only typing (`on_submit`, several lines), which the tasks app needs.
- **Apps style freely, through one door** (`.style([...])`, kivuli's categories), within what Rapidkoi can draw. This replaces DECISION-031's "apps never set colours". The desktop's GTK theme stays ignored, and the frame stays Rapidkoi's.
- **Nothing about looks goes in the manifest** (the owner: "an overreach").
- **Rapidkoi's names** (colours, text sizes, fonts, spacing, corners) can be used without the app learning their values. That's CSS's system colours: nothing to fingerprint, always current.
- **Roles from AccessKit's list,** never invented ones, and never named after an app's subject (no `task`, no `message`).
- **Follow by default, force as a last resort.** The person's settings and the emergency switch are part 2 (PRP-038).

## THE SPLIT (to approve with this PRP)
Contract 5 is one bump, built in three PRPs. It stays open until the last lands, and nothing outside this repository targets it before then (as contract 3 was, DECISION-039):
- **PRP-037 (this):** the primitives, the style door, Rapidkoi's names, roles, pressing, `grow`, typing with Enter and several lines; the Rust library's raw builders and its widgets rebuilt on them.
- **PRP-038:** the person's controls: Rapidkoi's settings for text size, light/dark and contrast, and motion; apps reading them; the per-app emergency switch.
- **PRP-039:** composing apps: link fields as paths into nested state (`editor.task=42`), nested state encoded as named fields, and typed messages per part with `.map` in the library.
- **PRP-040:** the tasks app (step 3).

## CONTEXT

- Starting state: contract 4 (PRP-036). Nodes are `text`, `button`, `input`, `column` and `row`, with fixed looks (`TextStyle`, `ButtonLook`, `Space`): `src/guest/nodes.la`, `props.la`, `contract.la`, `src/ui/render.la`, `themes.la`. The Rust library is `guest/rust/rapidkoi/src/view.rs`.
- Ending state:
  - **`src/guest/styles.la` (new, pure):** the style vocabulary decoded and checked from CBOR into a typed `Look`.
  - **`src/guest/roles.la` (new, pure):** roles and their states.
  - **`src/guest/nodes.la`:** contract 5's nodes `Box`, `Txt` and `Field` beside the old ones.
  - **`src/guest/contract.la`:** reads contract 5 trees by version. It's at 443 lines, so the contract 5 reader goes in `src/guest/tree5.la`.
  - **`src/ui/paint.la` (new, pure):** a `Look` turned into kivuli `Styling[]`, with Rapidkoi's names resolved from `themes.la` (colours, and colour mixing computed here) and DESIGN.md's tokens.
  - **`src/ui/render.la`:** draws contract 5 nodes. A box with `on_press` is `button_with`; `grow` is `.expand()`; a several-line field is `text_area`.
  - **`src/guest/limits.la`:** `CONTRACT_VERSION` 5 and the style limits.
  - **The Rust library:**
    - `raw` (`boxed`, `text`, `field`, `Role`, `.style([...])` with kivuli-like typed values);
    - `ui` (`column`, `row`, `text().title()`, `button(…).primary()`, `checkbox`, `tabs`, `spacer`, `input(…).on_submit`, `text_area`), rebuilt on `raw` and producing contract 5;
    - `export!` reports 5.
  - The counter, quote and the new tab move to contract 5 (`newtab.rkb` repacked).
  - **Docs:**
    - `docs/specs/contract-5.md`;
    - DECISIONS.md: DECISION-049 (styling and contract 5), DECISION-050 (kivuli accessible roles);
    - CLAUDE.md: the DESIGN RULE amended for apps' content, and the stack line;
    - MEMORY.md, DESIGN.md, CHANGELOG.md.
- Related existing code: the files above; `../kivuli/src/styles/` (`boxes.la`, `fonts.la`, `drawing.la`, `theming.la`, `values.la`), `../kivuli/src/view/widgets.la` (`button_with`, `text_area`), `views.la` (`.expand()`, `.name`), `mapping.la` (`on_submit`), `core/effects.la` (`Cmd.focus`).
- Related source files: `docs/source/stakeholder/vision-2026-10-17.md` ("The interface"); the session 198 research (the Extensible Web Manifesto, Flutter's layers, the first rule of ARIA, AccessKit).
- Lugha spec sections relied on: enums and structs, `match`, string parsing (`parse_i64`, `parse_f64`), `Result`. Each is confirmed with `lughac spec` while building.
- kivuli API used: `Background.color`, `Border.all/rounded`, `Outline.ring`, `Spacing`, `Sizing`, `Font.size/scale/weight/family`, `Text.color`, `Shadow.sm…xl`, `Effects.opacity`, `When(State.Hover | Press | Focus | Disabled | Checked, …)`, `Paint.Color`, `button_with`, `text_area`, `.expand()`, `.on_submit`. All exist in kivuli 0.54 (checked in session 198). **Missing: accessible roles** (DECISION-050).
- Open decisions that must be resolved first: none for this PRP's scope. DECISION-050 only delays the screen-reader half of roles (below).

## THE CONTRACT 5 TREE (exact in `docs/specs/contract-5.md`)

```
{ "kind": "box",
  "style": { … },                       // optional
  "role": "button", "checked": true,    // optional
  "label": "Add a task",                // optional: what a screen reader says, when the text isn't enough
  "on_press": <msg>,                    // optional: makes the box pressable
  "children": [ … ] }

{ "kind": "text", "text": "Count: 3", "style": { … }, "role": "heading" }

{ "kind": "input", "value": "…", "placeholder": "…", "lines": 1 | 2..20,
  "on_change": <msg>, "on_submit": <msg>, "style": { … } }
```

**Style** (one map; every key optional):

| Key | Value |
|---|---|
| `direction` | `"column"` (default) or `"row"` |
| `gap`, `padding` | a space: `"xs"`…`"xl"` or px 0–256; `padding` also `[vertical, horizontal]` |
| `align` | `"start"`, `"center"`, `"end"`, `"stretch"` |
| `grow` | `true`: take the spare room along the parent's direction |
| `width`, `height`, `min_width`, `max_width`, `min_height`, `max_height` | px 0–4096 |
| `background`, `color` | a colour (below) |
| `border` | `{ "width": 0–16, "color": colour }` |
| `corner` | `"small"`, `"medium"`, `"large"`, `"round"`, or px 0–256 |
| `outline` | `{ "width": 0–8, "color": colour }` |
| `shadow` | `"none"`, `"sm"`, `"md"`, `"lg"` |
| `opacity` | 0.0–1.0 |
| `size` | `"title"`, `"heading"`, `"body"`, `"small"`, `{ "px": 6–96 }` or `{ "rem": 0.5–6.0 }` |
| `weight` | `"normal"`, `"medium"`, `"bold"` |
| `font` | `"body"`, `"mono"`, `"serif"`, `"sans"` |
| `italic` | `true` |
| `hover`, `pressed`, `focus`, `disabled`, `checked` | a style map without these keys (no nesting) |

**A colour** is one of three things:
- a Rapidkoi name: `"background"`, `"surface"`, `"text"`, `"text_dim"`, `"accent"`, `"on_accent"`, `"border"`, `"danger"`, `"on_danger"`;
- raw: `"#rrggbb"` or `"#rrggbbaa"`;
- a mix: `{ "mix": [colour, colour, 0.0–1.0] }`, with no nested mixes.

**Roles:** `button`, `checkbox`, `switch`, `tab`, `tab_list`, `heading`, `list`, `list_item`, `group`, `separator`. States: `checked`, `selected`, `disabled` (booleans). The names follow AccessKit's.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **Contracts 2–4 run unchanged,** with their own trees and looks. A contract 5 app's tree is read only by contract 5's reader.
- **Every style value is hostile input:** checked against the table's kinds and ranges, at most 32 keys per style map, and a known key with a bad value is a contract error that stops the app with its sentence. An unknown key is ignored, so later versions can add keys.
- **Rapidkoi's names resolve at draw time** from `themes.la` (colours) and DESIGN.md's tokens (sizes, spacing, corners). The app never receives their values.
- **Pressing:** a box with `on_press` is focusable and pressed by click, Enter or Space, and sends its message as `on_message`.
- **Roles are checked and kept:**
  - `checkbox` and `switch` get kivuli's `Checked` state for styling;
  - a box with a role and no `on_press` isn't focusable;
  - a pressable box with no role is treated as a button.
- **The Rust library produces the same screens as today's** for the counter, quote and the new tab: their views are unchanged in source, and their trees are now contract 5.
- **Limits:** everything in contract 4's view limits applies to every node (count, depth, string length), and a style map counts toward the tree's size.

### Must NOT Do
- The person's settings, preference reads, or the emergency switch (PRP-038).
- Link paths, nested named state, or typed messages and `.map` (PRP-039).
- Long lists, things on top, pictures, pickers, gradients, animations, custom font files.
- Anything about looks in the manifest.
- Working around DECISION-050: no fake accessibility through labels in text.
- Merging into `redesign` or `main`.

## SCREEN READERS (DECISION-050)
kivuli 0.54 can't set a GTK accessible role or state on a node. Until it can:
- roles drive what Rapidkoi can do itself: focus, pressing by keyboard, and the `Checked` look;
- a screen reader hears what GTK infers from the widget (a pressable box is a button), not "checkbox, ticked".

The request is written as `docs/specs/kivuli-accessible-roles.md`. When it lands, roles, states and labels reach the screen reader with no contract change.

## MODEL AND MESSAGES
- No shell `Model` or `Msg` changes: contract 5 nodes render through `ui.render` like today's, and presses arrive as `Event` like today's.

## ERROR HANDLING REQUIREMENTS
- `ContractError.Invalid(path, problem)` gains sentences for each style and role rule ("root > box[2]: style.corner must be small, medium, large, round or 0–256 px").
- A bad tree stops the app on its error screen, as today. Rapidkoi never crashes on one.

## SECURITY CONSIDERATIONS
- **The guest contract changes (contract 5): owner review.** No new host function: styles and roles ride in `view`'s existing result.
- **Every value is bounded** (the table), so a hostile app can't ask for a 100,000 px shadow or blur its way into slow drawing. There are no blur or filter effects in this PRP.
- **Free styling makes lookalikes easier:** an app can draw what looks like a Rapidkoi dialog inside its content. The defence stays where it is. The address, the shield and Rapidkoi's own dialogs live in the frame and above the content, where apps can't draw. DESIGN.md says so.
- No new dependencies. The Rust library keeps minicbor and rapidkoi-derive.

## TESTS TO WRITE

`test/styles.la` (new):
- [ ] Every key and value kind reads: names, px, rem, the three colour forms, state looks.
- [ ] Each bad value says why: out of range, an unknown name, a nested state look, a nested mix, over 32 keys.
- [ ] An unknown key is ignored.

`test/roles.la` (new):
- [ ] Each role and state reads; an unknown role is refused; `checked` on a role without it is refused.

`test/paint.la` (new):
- [ ] Rapidkoi's names become the theme's values.
- [ ] A raw colour stays as written.
- [ ] A mix computes.
- [ ] Sizes become tokens and px.
- [ ] State looks become `When(…)`.

`test/render.la`:
- [ ] A pressable box is a button that sends its message (`kivuli.testing`: click by label).
- [ ] `grow` expands.
- [ ] A several-line input is a text area.
- [ ] `on_submit` sends.

`test/contract.la` (at its limit, so in `test/tree5.la`):
- [ ] A contract 5 tree reads.
- [ ] Contract 4's kinds in a contract 5 tree are refused.
- [ ] Limits apply.

`test/guest.la`, `test/tabs.la`:
- [ ] Contract 5 runs.
- [ ] 6 is refused.
- [ ] Contracts 2–4 unchanged.

`guest/rust` (`cargo test`):
- [ ] The raw builders encode exactly the spec's CBOR.
- [ ] `button(…).primary()`, `checkbox`, `tabs` and `spacer` build the expected boxes, styles and roles.
- [ ] `.style([...])` on a widget layers over its defaults.
- [ ] The counter's, quote's and the new tab's views encode as contract 5.

## ROLLBACK PLAN
- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits. Contract 5 is then unknown, and its bundles say "needs a newer version".

## ACCEPTANCE CRITERIA
- [ ] The counter, quote and the new tab look and behave as before, as contract 5 apps.
- [ ] An app restyling a button (its own green, round corners) shows it; one using `accent` follows Rapidkoi's theme.
- [ ] A pressable box works by click, Tab plus Enter, and Space.
- [ ] Contracts 2–4 apps run as before.
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines.
- [ ] DECISIONS.md (049, 050), CLAUDE.md's DESIGN RULE, `docs/specs/contract-5.md`, DESIGN.md, MEMORY.md, CHANGELOG.md updated.

## VALIDATION
- `lughac fmt --check src test`, `lughac test test/`, `lughac build src/main.la -o main`, `cargo test --manifest-path guest/rust/Cargo.toml`
- For the user:
  1. Rebuild the counter and open it: same look, now contract 5.
  2. Open a small styled example, kept in `guest/rust/` for this PRP: a green rounded button and an `accent` one. Each looks as written.
  3. Tab to Add and press Space: it counts.
