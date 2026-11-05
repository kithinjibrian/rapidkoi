## FEATURE: Places on a widget card: up to four parts of a card that each open the app somewhere

**Status:** approved (the owner, 2026-11-04: "approved"). Asked 2026-11-04: "yes record it and write up the places", after widgets were made output only (DECISION-097).

## OBJECTIVE

- **A card can hold up to four places.** Each is a part of the card (a row, a "+") that opens the app at its own place when clicked. Notes' card could be "+" for a new note, then the three newest notes, each opening itself:

  ```
  ┌──────────────────────────────┐
  │ ▣ Notes · 2 min ago   [ + ]  │  ← + opens ?new=1
  │ Shopping list                │  ← opens ?note=7
  │ Garden plan                  │  ← opens ?note=4
  │ Trip to Naivasha             │  ← opens ?note=2
  └──────────────────────────────┘
  ```

- **A place looks pressable.** It's the app's own box, with Rapidkoi's hover and focus over it, as a button's are.
- **A card with places isn't a button itself** (DECISION-098: kivuli holds no button in a button). Its top line (icon, name, when) opens the card's own place; blank space opens nothing. A card without places is one button, as before.
- **Still output only (DECISION-097):** every click opens the app, visibly, at a place. Nothing is sent to the app and nothing runs without a window.
- **The keyboard:** Tab reaches each place, then Enter opens it.

## CONTEXT

- **Builds on PRP-073:**
  - `guest/cards.la` (the card's rules);
  - `ui/widgets.la` (drawing the row and previews);
  - `window/widgetchoices.la` (`opened`);
  - `render.Msg.OpenWidget`;
  - the place's fields as the `widget` ask's `fields` are (`notifyasks.fields_of`);
  - `navigate.open_place` (local places since the local place fix).
- **The contract:** an addition to contract 11, `open` on a box inside a card. Screens (not cards) refuse it, so nothing changes for an app's own screen.
- **kivuli:** a `button_with` holds nothing interactive (`../kivuli/src/view/walk.la`, `check_content`), so a place can't sit inside the card's button. Found on screen 2026-11-05 and settled by DECISION-098: a card with places isn't a button.
- **SDK:** rapidkoi-rust gets `View::open(fields)`, a box that opens this place inside a card (`.open([("note", 7.into())])`, or `.open([])` for the app's start), and is tagged `v0.5.0-pre.8` after asking.
- **Decisions:** DECISION-097 (output only), resolved. None open.
- **Security, for the owner's review:** a place is the same hostile input a card's `fields` already are. It opens only the card's own app, and Rapidkoi takes it from the kept card, never from the click.

## REQUIREMENTS

### Must Do

- **The prop:** `open` on a box in a card is a map of a place's fields (at most 16, each a simple value of at most 1 KB, as `fields` are), or an empty map for the app's start.
- **The card's rules:** at most 4 boxes with `open` in a card, and none inside another. A box with `open` has no `on_press` or `menu`, like every card box. One breaking these makes the `widget` ask stop the app with its sentence.
- **On a screen** (not a card), `open` is refused as an unknown use: "open only goes on a box in a widget's card".
- **Drawing:**
  - on the new tab, each place is a button holding the app's box, with Rapidkoi's hover (`control_hover`) and focus ring, and the tooltip "Open";
  - a card with places is a frame, not a button: its top line is a button that opens the card's own place (tooltip "Open {app}"), and its right-click menu stays on the whole card (DECISION-098);
  - in previews (App info, ⚙ and "Replace a widget?"), places are drawn but not pressable, as the card isn't.
- **Clicking a place** opens the card's app at that place's address (`address_of` with those fields), as clicking the card opens its own. Rapidkoi reads it from the kept card by its index, never from the message.
- **The file:** places are in the card's kept bytes already, so the `widgets` file doesn't change. They're re-checked when read, as cards are.
- **The spec:** `docs/specs/contract-11.md` › Widgets says what `open` is and its limits.

### Must NOT Do

- **A place that sends the app a message,** or anything that runs it without a window (DECISION-097).
- **A place opening another app or an address:** only the card's own app, at fields.
- **More than four places,** or places inside places.

## DESIGN

- **`guest/nodes.la`:** a new `Node.Place(fields, Self)`, the box the place wraps. `guest/tree.la` reads a box with `open` into it, and `guest/cards.la` allows it inside a card only (counting at most 4, none nested). A screen with one is refused in `tabs.ran`, as a slot from an app without `show_widgets` is.
- **Messages:** `render.Msg.OpenWidget(i)` becomes `OpenWidget(i, Option<i64>)`, the card and the place within it. `widgetchoices.opened` reads the place's fields from the kept card's tree, in reading order.
- **`ui/render.la`:** `screen_with` draws a `Place` as nothing special (its box) unless the caller gives a `place` drawer. `ui/widgets.la` gives one on the new tab: a button sending `OpenWidget(i, Some(k))`.
- **SDK:** `View::open` writes `open` as a map of `FieldValue`s, reusing `notify`'s. A test checks the encoding.

## TESTS

- [ ] `test/cards.la`:
  - [ ] a card with four places is read;
  - [ ] five, or a place inside a place, stop it;
  - [ ] a place with bad fields stops it;
  - [ ] a screen (not a card) with `open` is refused.
- [ ] `test/widgets.la`: clicking a card's second place opens the app at its fields; an index past its places opens nothing; the card's own click still opens the card's place.
- [ ] `test/widgetsview.la`: the new tab draws each place as a button (found by its tooltip "Open"), and a preview draws none.
- [ ] rapidkoi-rust: `View::open` encodes its fields, and an empty map for the start.

## ACCEPTANCE

- [ ] A card with places opens the app at the place clicked, and at the card's own place from its top line.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main`, and rapidkoi-rust's and guest/rust's tests pass.
- [ ] For the owner, by eye: a counter card with "+1 here" and "Reset here" places (`?count=N` each). Clicking each opens the counter at that count, and Tab reaches them.
