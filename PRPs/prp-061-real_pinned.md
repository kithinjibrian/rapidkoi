## FEATURE: The new tab's Pinned shows your real pins and opens them; the address box's ☆ is filled in the brand colour while the app is pinned

**Status:** approved (the owner, 2026-11-01: "approved, go ahead").

## OBJECTIVE

- **The new tab's Pinned section is real.**
  - It shows your pins in your order, each with its icon (`icon:PLACE`, drawn by Rapidkoi) and its name.
  - Pressing one opens it.
  - With no pins, the section isn't there.
  - It's still one line of as many as fit, with "See all" at the heading's end, inert until the pins page exists (a later PRP).
- **The address box's ☆ (Pin/Unpin)** is filled in the brand colour (`accent`) while the app shown is pinned, and is the plain outline in the text colour otherwise.

## CONTEXT

- **Where it comes from:**
  - the owner, 2026-11-01: "let's build the real pinned section. change color of bookmark icon to fill brand color", "when app is bookmarked";
  - `docs/designs/newtab-target.md`: Pinned, and the shell's list, item 9 (the new tab on real data).
- **What exists:**
  - The new tab already declares `read_pins` and `open_apps`.
  - The `pins` answer names each pin's icon as `icon:PLACE` (PRP-060).
  - `open_pin(index)` opens a pin by its place in your pins as they are now (`guest/home.la`'s `pin_msg`).
  - Rapidkoi tells the new tab when your pins change with `on_change` (PRP-031).
  - The ☆ is `ui/addressbox.la:227`, always `icon(Icon.Star, t.text)`; `window.pins.is_pinned` says whether the app shown is pinned.
- **No contract change, no SDK release, no new permission.**
- **Decisions:** none open touch it.
- **Changes:**
  - `guest/rust/newtab/src/lib.rs` (and `assets/apps/newtab.rkb`, repacked);
  - `src/ui/icons.la`, `src/ui/addressbox.la`;
  - `test/icons.la`, `test/addressbox.la`;
  - CHANGELOG.md.

## REQUIREMENTS

### Must Do

- **The new tab:**
  - It asks for `pins` with `session` each time it's shown (`on_open`, `on_resume`) and when told something changed (`on_change`).
  - It keeps each pin's name and icon source in its state.
  - Each tile is a 56 px icon over its name, as the replica's are, and pressing it asks `open_pin(i)`.
  - No pins: no Pinned section at all.
  - The placeholder pins go: `notes.svg`, `counter.svg`, `trip.svg` and `settings.svg` stay only for the parts still placeholder (notifications, Recent).
- **The ☆:**
  - `icons.filled(i, colour)` draws an outline icon filled and stroked in `colour`, keyed apart from the outline one.
  - The address box draws the ☆ filled in `t.accent` when `is_pinned`, and outlined in `t.text` otherwise.
  - The tooltip stays "Pin" or "Unpin".

### Must NOT Do

- **The pins page** ("See all"), reordering pins from the new tab, or unpinning there.
- **Any other part of the new tab** (notifications, Recent).
- **Recolouring other icons.**

## TESTS

- [ ] guest/rust: shown or changed, the new tab asks `pins` and `session`; a `pins` answer becomes its tiles' names and icons; pressing tile 1 asks `open_pin(1)`; no pins means no Pinned section.
- [ ] `test/icons.la`: `filled` fills and strokes in the colour, keyed apart from the outline.
- [ ] `test/addressbox.la`: on a pinned app the ☆ is the accent-filled star; on one that isn't, the text-coloured outline.
- [ ] Shell, end to end: with a pin saved, the real new tab shows its name; pressing it opens it.

## ACCEPTANCE

- [ ] The new tab shows your pins and opens them; with none, there's no Pinned.
- [ ] The ☆ is filled orange on a pinned app.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main` and guest/rust's tests pass.
- [ ] For the owner, by eye: pin two apps, open a new tab, and see them, each with its icon; press one. On a pinned app the ☆ is filled orange; unpin it and it's an outline again.
