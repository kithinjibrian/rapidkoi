## FEATURE: Right-click on an app's box (`on_context`), and All apps' details on a right-click

**Status:** approved (the owner, 2026-11-04: "yes"). Uses kivuli 0.68.0 (`.on_context`, kivuli's PRP-073).

## OBJECTIVE

- **An app can answer a right-click on a box**: a right-click, a long press, or the Menu key (Shift+F10) while it has the focus sends the app the box's `on_context` message, as `on_press` sends its own. Rapidkoi opens nothing: what follows is the app's to draw, such as a card of its own (the owner, 2026-11-03: "the app can design the menu however they see fit", `on_context` over a menu Rapidkoi draws).
- **All apps' details open on a right-click on a tile**, over the dimmed page, as its design's details card. Nothing else in All apps changes.

## CONTEXT

- **What exists:** a box's `on_press` (`guest/tree.la` reads it, `ui/render.la` draws a pressable box as a button sending `Msg.Event(message, None)`); kivuli 0.68.0's `.on_context(msg)` and `kivuli.testing.context(view, label)`; All apps' details card (`guest/rust/allapps`), its `Msg::Details(i)` reached by nothing since the ⋯ went.
- **kivuli:** `lugha.toml` pins 0.67; it moves to 0.68 (`lughac update` rewrites `lugha.lock`). In kivuli, the innermost node answers a right-click, and a node can't have both `.menu` and `.on_context` (kivuli stops): Rapidkoi never puts `.menu` on an app's box, so an app can't trip it.
- **The contract:** an addition to contract 11, with no new version, as PRP-062, 065 and 067 were. A box without `on_context` is unchanged, so every app published today runs as it does.
- **Decisions:** none recorded for this; none open touch it.
- **Changes:**
  - **code:** `src/guest/tree.la` and `src/guest/nodes.la` (a box's `on_context`, checked as `on_press` is), `src/ui/render.la` (drawn with kivuli's `.on_context`), `lugha.toml`;
  - **the SDK** (rapidkoi-rust): `View::on_context(msg)`, released as `v0.5.0-pre.6` (the owner's ask before tagging);
  - **All apps:** its tiles `.on_context(Msg::Details(i))`; the built-ins move to the tag and All apps is repacked;
  - **docs:** `docs/specs/contract-11.md`, CHANGELOG.md, CLAUDE.md (kivuli 0.68, the SDK tag).

## REQUIREMENTS

### Must Do

- **`on_context` on a box:** a message, checked as `on_press`'s is (its bytes, its size cap); a bad one stops the app with its sentence. A box may have both: a click sends `on_press`, a right-click `on_context`. Nested boxes: the innermost with `on_context` answers (kivuli's rule).
- **Drawn with kivuli's `.on_context`**, sending `Msg.Event(message, None)` as a press does, so the app hears it as any message. A disabled box sends nothing.
- **Accessibility:** the box's `label` names it for the right-click as for a press.
- **The SDK:** `.on_context(msg)` on a box, as `.on_press(msg)`; its test writes the prop.
- **All apps:** a right-click on a tile opens that app's details; again on the same tile, or on the dimmed backdrop, closes them.

### Must NOT Do

- **A menu Rapidkoi draws for apps**, or the pointer's position: the app draws what follows; kivuli gives no position (0.68).
- **`on_context` on anything but a box** (texts, typing boxes): as `on_press`.

## DESIGN

- **`Node.Boxed`** gains the `on_context` message beside `on_press` (`Option<Value>`).
- **Render:** a box with `on_context` gets `.on_context(Msg.Event(m, None))`; with `on_press` too, it's the same button with both.
- **Security:** no new capability. A right-click sends the app only its own message, as a press does; nothing about the pointer, the window or other apps.

## TESTS

- [ ] `test/tree.la`: a box's `on_context` read; a bad one (not bytes, over the cap) stops the app with its sentence.
- [ ] `test/render.la`: `kivuli.testing.context(view, label)` on a rendered box gives its event; a box with both sends `on_press` on a click and `on_context` on a right-click; a disabled box sends nothing.
- [ ] rapidkoi-rust: `.on_context` writes the prop.
- [ ] `guest/rust/allapps`: a tile's right-click sends `Msg::Details(i)`; `test/allapps.la`: a right-click on a tile shows its details ("Open", "Permissions").

## ACCEPTANCE

- [ ] An app's box answers a right-click with its message; All apps' details open on a right-click on a tile.
- [ ] `lughac test test/` (the owner runs it), `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main`, and the built-ins' `cargo test` pass.
- [ ] For the owner, by eye: open All apps, right-click a tile: its details card over the dimmed page; right-click again or click the backdrop: it closes. Long-press on a touch screen does the same.
