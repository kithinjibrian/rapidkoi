## FEATURE: A box's right-click menu (`menu`), and All apps' tiles on it

**Status:** approved (the owner, 2026-11-04). Uses kivuli 0.68's `.menu` (already used by the dock's app buttons). Follows PRP-070.

## OBJECTIVE

- **An app can give a box a right-click menu:** a list of items, each a label and a message. Rapidkoi draws it as the dock's menus are drawn, a native menu at the pointer. A right-click, a long press, or the Menu key (Shift+F10) opens it, and picking an item sends the app that item's message (the owner, 2026-11-04: "dropdown", over the details dialog).
- **All apps' tiles use it:** Open, Pin to dock or Unpin from dock, Permissions, and Forget this app…. The details dialog is no longer opened by a right-click.

## CONTEXT

- **What exists:**
  - PRP-070's `on_context`, built but not yet released;
  - kivuli's `.menu([item(label, msg), menu_separator(), …])`, with `item(…).disabled(true)` and `kivuli.testing.pick(view, owner, item)`;
  - the dock's app buttons (`ui/sidebar.la`) and the strip, which already use `.menu`;
  - All apps' `Msg::Open`, `Pin`, `Unpin`, `Permissions` and `Forget`. Forget goes through Rapidkoi's `forget_app` ask, which confirms.
- **kivuli's limits:**
  - **No icons:** GTK draws none beside menu items.
  - **Not with `on_context`:** kivuli stops when one node has both `.menu` and `.on_context`. A stop would take down the whole shell, so Rapidkoi refuses that tree first.
- **The contract:** an addition to contract 11 with no version bump, as PRP-070 was. A box without `menu` is unchanged.
- **The SDK:** PRP-070's `v0.5.0-pre.6` isn't tagged yet. It waits, so one tag carries both `on_context` and `menu`.
- **Decisions:** none recorded or open.
- **Changes:**
  - **code:** `src/guest/nodes.la` and `tree.la` (read a box's `menu` and check it), `src/guest/limits.la` (caps), `src/ui/render.la` (draw it with `.menu`);
  - **the SDK:** `.menu(items)` with `item(label, msg)`, `.disabled(true)` and `separator()`;
  - **All apps:** its tiles take `.menu(…)` in place of `.on_context(Details)`; it moves to the tag and is repacked;
  - **docs:** `docs/specs/contract-11.md`, CHANGELOG.md, CLAUDE.md (the SDK tag).

## REQUIREMENTS

### Must Do

- **`menu` on a box:** an array whose entries are either an item `{ label, on_pick, disabled? }` or a separator `{ separator: true }`.
  - `label` is a string from 1 to `MAX_LABEL` (200) bytes;
  - `on_pick` is a message, checked as `on_press` is;
  - at most `MAX_MENU_ITEMS` (32) entries.
- **What breaks the rules:** anything outside the rules above, an empty menu, or a box with both `menu` and `on_context` is a decode error. It stops that app with its sentence and never reaches kivuli.
- **Drawn with kivuli's `.menu`:**
  - each item sends `Msg.Event(on_pick, None)`, as a press does;
  - a disabled item is greyed out and sends nothing;
  - a disabled box gets no menu.
- **A box may have `on_press` and `menu`:** a click sends the press, a right-click opens the menu. Nested boxes follow kivuli's rule: the innermost box with a menu answers.
- **The SDK:** `.menu(vec![item("Open", Msg::Open(i)), separator(), item("Forget", …).disabled(true)])` on a box.
- **All apps:** a right-click on a tile opens Open, Pin to dock or Unpin from dock, Permissions, a separator, and Forget this app…, each sending the message its details card sends. The details card stays in the code and opens from nothing for now. Removing it is a later chore if you want.

### Must NOT Do

- **Submenus, check items, radio items, shortcut hints, icons or styling:** items are plain text.
- **A menu on anything but a box.**
- **Telling the app where the pointer was.**

## DESIGN

- **`Node.Boxed`** gains `menu: MenuEntry[]`, with `enum MenuEntry { Item(string, Value, bool), Separator }`. An empty array means no menu.
- **Render:** `menu` empty → as today; otherwise `.menu(entries)` on the box's view, with `item(label, Msg.Event(v, None)).disabled(d)` or `menu_separator()`. The look is the same as the dock's menus.
- **Security:** no new capability. Picking an item sends the app only its own message. The labels are the app's own text, capped, in a menu Rapidkoi draws over the app's own box. They can't sit over the frame, the address bar or another app.

## TESTS

- [ ] `test/tree.la`:
  - a box's menu is read with its items, separators and disabled items;
  - each of these stops the app: an empty label, a label over the cap, a message that isn't bytes, more than 32 entries, an empty array, an entry that is neither an item nor a separator, and `menu` with `on_context`.
- [ ] `test/render.la`:
  - `pick(view, label, item)` gives the item's event;
  - a disabled item is `Silent`;
  - a box with `on_press` and `menu` sends the press on a click and the item on a pick;
  - a disabled box opens no menu.
- [ ] rapidkoi-rust: `.menu` writes the prop with items, a separator and a disabled item.
- [ ] `guest/rust/allapps`: a tile's menu holds Open, the pin item for its state, Permissions and Forget, with their messages.
- [ ] `test/allapps.la`: picking Open on a tile opens the app; picking Pin to dock pins it.

## ACCEPTANCE

- [ ] An app's box opens its menu on a right-click and sends the picked item's message. All apps' tiles show Open, Pin or Unpin, Permissions and Forget.
- [ ] These pass:
  - `lughac test test/` (you run it);
  - `lughac fmt --check src test`;
  - `lughac build -O 2 src/main.la -o main`;
  - the built-ins' `cargo test`.
- [ ] By eye (you): open All apps and right-click a tile. A native menu opens at the pointer, like the dock's. Pin to dock pins it and the item reads Unpin from dock next time. Long-press on a touch screen does the same.
