# rapidkoi-rust request: contract 10, drawing (`v0.4.0-pre.1`)

From Rapidkoi, PRP-059 (DECISION-091). Built in `../rapidkoi-rust` at the owner's word (2026-11-01: "yes for the two"). Rapidkoi's side is on its branch `prp-059-contract_10_drawing`; it waits on this tag to move its built-in apps and merge. Notes and the counter on the VM need rebuilding on it too, or they stop opening once Rapidkoi moves.

## What changed in the contract

Contract 10 is contract 9 with four looks, and Rapidkoi runs **only** contract 10 (`rapidkoi_version` 10; 9 and older are refused with "This app needs to be rebuilt for this Rapidkoi"). The full text: `docs/specs/contract-10.md` in Rapidkoi. Nothing else changes: no asks, events or nodes are added or removed.

1. **`gradient`**, a style key drawn over `background`:
   - `{ "linear": 0–359, "stops": [[colour, at], …] }`: degrees, 0 up and 90 right;
   - `{ "ellipse": [x, y, across, down], "stops": […] }`: its centre as fractions of the box (−1 to 2), its radii against the box's width and height (above 0, at most 4).

   Stops: 2–8 of them, each `at` from 0 to 1, in order; a colour is any contract colour. Allowed inside states.
2. **`direction: "layers"`**: the first child is the base and sizes the box; later children are drawn on top, each where its own `place` says:
   - `top_start`, `top`, `top_end`, `start`, `center`, `end`, `bottom_start`, `bottom`, `bottom_end`, or `fill` (the default), which covers the base;
   - start and end follow the text's direction.
3. **`direction: "wrap"`**: children side by side, a new line when one is full, with `gap` across and down. `equal: true` makes every child as wide as the widest, and `per_line` (1–32) caps a line.
4. **`direction: "fit"`**: as many children as fit from the start; the rest are hidden from the end.

## What the SDK needs

1. **`__CONTRACT_VERSION` = 10** (`abi.rs`, and its test).
2. **Styles** (`style.rs`):
   - `Background::linear(angle, stops)` and `Background::ellipse(x, y, across, down, stops)`, stops as `(Color, f64)` pairs;
   - `Layout::layers()`, `Layout::wrap()`, `Layout::fit()`;
   - `Layout::equal()` and `Layout::per_line(n)`;
   - `Layout::place(Place)`, with `Place` holding the nine places and `Fill`.
3. **Builders** (`view.rs`), beside `row` and `column`: `layers(children)`, `wrap(children)` and `fit(children)`.
4. **Docs:** contract 10 in the crate's description and `lib.rs`.
5. **Tests:** each style encodes as above; the builders set their direction.
6. **Tag** `v0.4.0-pre.1` (a breaking change: every app rebuilds), asking the owner before pushing it.

## Then, in Rapidkoi

Done: `v0.4.0-pre.1` tagged and pushed (2026-11-01); `guest/rust` moved to it, the built-ins repacked and `./main` rebuilt. Republishing Notes and the counter on the VM is asked for separately.
