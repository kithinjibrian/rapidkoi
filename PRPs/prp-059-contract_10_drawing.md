## FEATURE: Contract 10: gradients, layers, wrapping rows and fitting rows, so apps can draw what the new tab's target design draws

**Status:** approved (the owner, 2026-11-01: "yes for the two": Q1 the drawing items only, Q2 yes). Decision: DECISION-091.

## OBJECTIVE

An app can draw, with its own nodes:
- a colour washing in from a corner;
- a face with its app's icon on the corner;
- a grid that re-flows as the window changes;
- a line of tiles that ends cleanly at the window's edge.

Today it needs a pre-drawn picture for each. The new tab's replica (`newtab-replica` branch) fakes these with eleven bundle SVGs and a wide filler picture. Under contract 10 most of them become plain nodes:

```
// The small card's picture area: the app's colour from the top-left, its icon on it.
{ "kind": "box", "props": { "style": {
    "height": 84, "corner": 8, "padding": 12,
    "background": "#1b1b1b",
    "gradient": { "ellipse": [0, 0, 1.1, 1.5],
                  "stops": [["#14b8a680", 0], ["#14b8a61f", 0.6], ["#14b8a600", 0.9]] } } },
  "children": [ icon ] }

// A face with the app on its corner.
{ "kind": "box", "props": { "style": { "direction": "layers" } },
  "children": [ face, { "kind": "image", "props": { "src": "asset:board.svg",
                        "style": { "place": "bottom_end", "width": 18, "height": 18 } } } ] }

// Notifications' two equal columns: one column on a narrow window.
{ "kind": "box", "props": { "style": { "direction": "wrap", "equal": true, "per_line": 2, "gap": 16 } },
  "children": [ n1, n2, n3, n4 ] }

// Pinned: as many tiles as fit, the rest hidden from the end.
{ "kind": "box", "props": { "style": { "direction": "fit", "gap": "lg" } }, "children": tiles }
```

## CONTEXT

- **Where it comes from:** `docs/designs/newtab-target.md` → "What it needs, by repository" → "rapidkoi: the contract", items 1–4, on the `newtab-replica` branch. Items 5–7 are left out (Q1).
- **Built on:** kivuli 0.67 (on `main` since `d6f85ea`), checked in `../kivuli/README.md` and its source:
  - `Background { gradient: Gradient.Linear | Gradient.Ellipse }` (0.67, PRP-072 there);
  - `overlay(base, layers)` with `.place(across, down)`;
  - `wrap(children)` with `.equal()` and `.max_per_line(n)`;
  - `fit_row(children, more)`.

  No lugha-gtk or lughac change.
- **Decisions:** DECISION-056 (one contract; older apps are refused), DECISION-088 (contract 8), DECISION-087 (contract 9). The open entries are DECISION-044 (Continue), DECISION-081 and DECISION-082 (audiences), and none of them touches drawing. DECISION-091 is this one.
- **Files to read first:**
  - `docs/specs/contract-9.md` (the Style section);
  - `src/guest/styles.la` (409 lines: reading a style);
  - `src/guest/nodes.la` (376: `Look`, `Direction`, `Place`);
  - `src/ui/paint.la` (354: a look to kivuli's values);
  - `src/ui/render.la` (338: `boxed`);
  - `src/guest/limits.la` (`CONTRACT_VERSION`);
  - `test/styles.la` and `test/render*.la`;
  - `test/support.la` (WAT stand-ins export `rapidkoi_version`).
- **Creates:**
  - `src/guest/gradients.la`: reading a `gradient`, kept out of `styles.la`, which would pass 500 lines;
  - `test/gradients.la`;
  - `test/render_layout.la`: layers, wrap and fit drawn.
- **Changes:**
  - code: `src/guest/styles.la`, `src/guest/nodes.la`, `src/ui/paint.la`, `src/ui/render.la`, `src/guest/limits.la` (10), `test/support.la` (stand-ins say 10);
  - the spec: `docs/specs/contract-9.md` → `contract-10.md`;
  - the built-ins: `guest/rust/*` (the SDK tag), `assets/apps/*.rkb` (repacked);
  - DECISIONS.md, CHANGELOG.md and CLAUDE.md (the contract line).

## REQUIREMENTS

### Must Do

- **Contract 10.** `CONTRACT_VERSION` is 10. Contract 9 apps are refused with today's sentence, "This app needs to be rebuilt for this Rapidkoi". Everything in contract 9 carries over unchanged.
- **`gradient`**, a new style key, drawn over `background` (the colour beneath it) and under the content:
  - `{ "linear": 0–359, "stops": [...] }`: along an angle, in degrees; 0 points up, 90 right.
  - `{ "ellipse": [x, y, across, down], "stops": [...] }`: from an ellipse's centre out. `x` and `y` are its centre as fractions of the box (−1 to 2, so a glow may start beyond an edge), and `across` and `down` are its radii against the box's width and height (above 0, at most 4).
  - `stops`: 2–8 pairs of `[colour, at]`, each `at` from 0 to 1 and in order. A colour is any contract colour, so Rapidkoi's names follow the theme.
  - Physical, as CSS's are: `x` runs from the left in every language.
  - Allowed in `hover`, `pressed`, `focus`, `disabled` and `checked` states.
- **`direction: "layers"`**: the first child is the base and gives the box its size; each later child is drawn on top, in order. A layer sits where its own **`place`** says:
  - `top_start`, `top`, `top_end`, `start`, `center`, `end`, `bottom_start`, `bottom`, `bottom_end`, or `fill`, which covers the base (the default);
  - start and end follow the text's direction, as Rapidkoi's rows do;
  - a layer takes the pointer only where it is, so a pressable base stays pressable around a badge;
  - `place` on a box that isn't a layer is ignored.
- **`direction: "wrap"`**: children side by side, a new line when one is full, re-flowing as the window changes. `gap` is the room across and down. Two companion keys, both ignored on other directions:
  - **`equal: true`** makes every child as wide as the widest;
  - **`per_line`** (1–32) caps how many go on a line.
- **`direction: "fit"`**: children side by side, as many as fit from the start at their natural widths; the rest are hidden from the end. It never makes the window wider. Nothing tells the app how many are hidden (not needed by the design; a later version can add it).
- **Every new value is checked** as today's are. A known key with a bad value stops the app with its sentence:
  - "This app's gradient needs 2 to 8 stops";
  - "This app's gradient stops are out of order";
  - "This app put a gradient's centre outside −1 to 2";
  - "This app gave a gradient a radius outside 0 to 4";
  - "This app's place isn't one Rapidkoi knows";
  - "This app put more than 32 on a line".
  An unknown key is ignored.
- **The spec** becomes `docs/specs/contract-10.md`: contract 9's text, plus these keys and their sentences.
- **rapidkoi-rust is tagged first** (CLAUDE.md, STACK): contract 10's version, `Gradient` styles, `Direction::Layers | Wrap | Fit`, `Place`, `equal` and `per_line`. It's written as a request (`docs/specs/rapidkoi-rust-contract-10.md`) and built there under that repository's rules (Q2). Then the built-ins (`guest/rust/newtab`, `settings`) move to the tag and are repacked.

### Must NOT Do

- **Items 5–7** of the design's list: a place's `summary` and `picture`, pictures Rapidkoi hands over, and an app's colour in answers (Q1).
- **A "more" for fitting rows, or how many are hidden**: nothing asks for it yet.
- **Margins, negative offsets, or a layer placed in pixels.** Places are enough for the design's badges; offsets would let an app draw over the edges of its own boxes.
- **Gradient text, conic or repeating gradients.** GTK's CSS can't fill text with a gradient; the others aren't asked for.
- **Accepting contract 9 apps alongside 10.** One contract (DECISION-056).
- **Changing the new tab here.** The replica moves onto contract 10's nodes in its own branch, after this merges.

## DESIGN

- **`Direction`** gains `Layers`, `Wrap` and `Fit`. **`Look`** gains:
  - `gradient: Option<Gradient>`, Rapidkoi's own enum (`Linear(i64, Stop[])`, `Ellipse(f64, f64, f64, f64, Stop[])`, with `Stop { colour: Colour, at: f64 }`);
  - `place: Option<Spot>` (`Spot`: the nine places and `Fill`);
  - `equal: bool`;
  - `per_line: Option<i64>`.

  `nodes.la` grows by about 30 lines, to about 406.
- **Reading** (`src/guest/gradients.la`): numbers are CBOR integers or floats. Each check returns its `ContractError` with the sentence above, and `styles.la` calls it for `gradient`.
- **Drawing** (`paint.la`):
  - The gradient becomes kivuli's `Background { gradient: Gradient.Linear(angle, stops) | Gradient.Ellipse(Ellipse { x, y, across, down } as f32, stops) }`.
  - Stops' colours resolve as today's colours do: Rapidkoi's names through the theme, and "Use my colours" turning the app's own colours into the theme's text colour.
  - Because `background-image` is one property in kivuli, the gradient merges into the same `Background` as the colour.
- **`render.la`**:
  - `Layers` builds `overlay(base, layers)`, each layer given `.place(across, down)` from its `place`, with `fill` meaning `Align.Fill` both ways.
  - `Wrap` builds `wrap(inside)`, adding `.equal()` and `.max_per_line(n)`.
  - `Fit` builds `fit_row(inside, column([]))`, an empty "more".
  - A layers box with no children draws nothing; one child is just the base.
  - `boxed` stays under 500 lines by moving the layout choice into a function of its own.
- **Limits:** unchanged. Layers, lines and fitted children count as nodes, within today's 10,000 nodes and 64 deep.
- **Security:** no new host function and no new capability: these are looks. A gradient can't name a URL or a file, only colours and numbers. Every number is range-checked before kivuli sees it, and kivuli checks again: radii positive, every number finite.
- **What the person notices:** published contract 9 apps stop with the rebuild sentence until they're republished. Today those are counter.dafifi.net and notes.dafifi.net on the test VM, and Notes is its own repository. Republishing them is outward-facing, so it's asked for separately.

## QUESTIONS FOR THE OWNER

- **Q1. Which items go into contract 10?** *Recommended:* the four drawing items only. Items 5–7 (a place's card, pictures Rapidkoi hands over, an app's colour) would have nothing reading them until Recent places and the new tab's real data are built, and Recent waits on DECISION-044, which is open. The cost: a second version bump later, which means republishing counter and notes again. *Or:* all seven now, so there's one bump, but items 5–7 would sit unused.
- **Q2. May I write in `../rapidkoi-rust` for its part,** under its own rules (its process, then the tag `v0.4.0-pre.1`)? *Recommended:* yes, as with kivuli. Otherwise it goes in as a request and waits for an SDK session.

## WHAT CHANGED WHILE BUILDING

Settled in the build, each smaller than the PRP's scope:
- **A gradient goes only on a box's own look**, not inside `hover` and the other states as written above. kivuli draws it as a background image, and its README says a background image can't change per state. The error is "style.hover.gradient only goes on a box's own look", as `scroll`'s is.
- **The error wording** follows the existing style errors ("style.gradient must have 2 to 8 stops", "style.per_line must be 1–32", and so on) rather than the sample sentences in Must Do.
- **Use my colours** drops a gradient that holds any of the app's own colours, as the app's own fills go; a gradient of Rapidkoi's names only is drawn. (The design note's "the app's own colours into the theme's text colour" fits text and SVGs, not fills.)
- **The new tab and settings** were first tested against the local SDK through an uncommitted `.cargo/config.toml` patch, then rebuilt from the `v0.4.0-pre.1` tag (pushed 2026-11-01) and tested again: 735 of 735.

## TESTS

- [ ] `test/gradients.la`:
  - a linear gradient and an ellipse are read;
  - each sentence above, with its bad value;
  - stops of 2 and of 8 pass, and 1 and 9 fail;
  - a centre at −1 and at 2 passes, and −1.1 and 2.1 fail;
  - a radius of 0 and of 4.1 fail;
  - a named colour resolves through the theme, and "Use my colours" turns an own colour into the text colour.
- [ ] `test/styles.la`:
  - `direction` reads `layers`, `wrap` and `fit`, and refuses others;
  - `place`, `equal` and `per_line` are read, and refused when bad;
  - `gradient` is read inside `hover`;
  - an unknown key is still ignored.
- [ ] `test/render_layout.la`, through `kivuli.testing`:
  - a layers box shows its base and its layers in order, a layer's place set as written, and a pressable base still pressed through a badge's empty room;
  - a wrap shows every child, equal and per line passed on;
  - a fit row holds its children;
  - an empty layers box draws nothing.
- [ ] The contract: a stand-in exporting version 9 is refused with the rebuild sentence; every other stand-in says 10 and runs.
- [ ] The built-ins, on the rapidkoi-rust tag, open: the new tab and settings.

## ACCEPTANCE

- [ ] A contract 10 app can use every key and direction above; a contract 9 app is refused with the rebuild sentence.
- [ ] `docs/specs/contract-10.md` describes contract 10 whole; DECISION-091 is resolved; CHANGELOG.md says it.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye, once the new tab moves to contract 10 on its own branch: the small cards' washes, the faces' badges and Notifications' columns are drawn by nodes, not pictures, and narrowing the window re-flows Notifications to one column.
