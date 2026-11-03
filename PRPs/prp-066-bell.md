## FEATURE: The bell: every notification in a card from the header

**Status:** approved (the owner, 2026-11-03, on the design canvas: "cool build"). Builds on PRP-065 (its branch, not yet merged). The second of PRP-065's three; the dock's dot and the See all page come after.

## OBJECTIVE

- **A bell in the header,** before 👤 and ☰ (`assets/icons/notification.svg`), with an accent dot while anything is unread. Its tooltip says "Notifications", or "Notifications, 3 unread".
- **Pressing it opens a card** hanging from it, as ☰'s does:
  - "Notifications" with the unread count in the accent;
  - every notification: unread first, then "Earlier" and the read ones, newest first within each;
  - each row is the app's icon, its sentence (bold while unread), and its app and when, in dim;
  - pressing a row opens its place, marks it read and closes the card; × removes it;
  - with none, the bell drawn faint and "No notifications".
- **Opening the card marks nothing read:** only opening a row does, as on the new tab.
- **The design:** the canvas "Notifications dropdown" (https://claude.ai/artifact/1NuQYNe4mEesfdyFqoXqKt), its three boards.

## CONTEXT

- **What exists:**
  - PRP-065's notifications (`window.notifications`: `listed`, `unread`, `open_at`, `dismiss`; `Model.notifications`);
  - ☰'s card (`ui.header`: `ui_popover`, `Msg.Menu`);
  - apps' icons by place (`ui.appicons.icon_for`), and the address box's upgrade dot (an overlay of `Icon.Dot`);
  - times in words (`guest.times.words`, `Show.Relative`), with zone offsets that `window.zones` finds for moments in front.
- **Files at the limit:** `src/shell.la` is at 500 lines, so the bell is one message (`Msg.Bell(BellAction)`), answered in `window.navigate`; a comment there makes room for it.
- **No contract change, no SDK release, nothing republished.**
- **Decisions:** DECISION-093. None open touch it.
- **Changes:**
  - **code:**
    - `src/ui/bell.la`: the bell and its card (new);
    - `src/ui/header.la`, `src/ui/icons.la` (`Icon.Bell`), `src/ui/tokens.la`;
    - `src/window/actions.la`, `src/window/window.la` (`bell: bool`), `src/window/navigate.la`, `src/window/zones.la` (offsets for the card's times), `src/shell.la`;
  - **tests and docs:** tests, and CHANGELOG.md.

## REQUIREMENTS

### Must Do

- **The messages:** `Msg.Bell(BellAction)`, with these actions:
  - `Show(bool)`: opens or closes the card; opening it closes ☰'s;
  - `Open(i)`: opens notification `i` as they're listed (`open_at`), then its place (`navigate.open_place`), and closes the card;
  - `Dismiss(i)`: removes it.
  - An index past the end changes nothing.
- **The card:** at most `CARD_WIDTH` wide; its list scrolls inside it when it's taller than the window allows.
  - **The order:** `listed`. "Earlier" goes before the first read row, only when unread rows come before it.
  - **Each row's time:** relative, in the person's zone (`times.words`). `window.zones` looks up offsets for the notifications' times while the card is open.
- **The rows' icons:** `icon_for(m, place)`, as built-in apps' `icon:PLACE` draws them: the app's icon, or its initial.
- **Escape or a click outside** closes the card, as ☰'s does.

### Must NOT Do

- **Mark all read, Clear all,** or any other copy beyond the design.
- **The dock's dot, the See all page, desktop alerts:** each is its own PRP.
- **A shell message per action:** `shell.la` is at its limit.

## TESTS

- [ ] `test/bell.la`:
  - [ ] the bell's tooltip, with and without unread;
  - [ ] the card lists unread first, then "Earlier" and the read ones; with no unread there's no "Earlier"; with none, "No notifications";
  - [ ] pressing a row opens its place, marks it read and closes the card;
  - [ ] × removes one;
  - [ ] opening the card closes ☰'s, and ☰'s card is untouched otherwise;
  - [ ] an index past the end changes nothing.
- [ ] `test/icons.la`: the bell is built in.

## ACCEPTANCE

- [ ] The bell shows its dot while something is unread, and its card lists every notification and opens or removes each.
- [ ] `lughac fmt --check src test`, the tests above, and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: with the counter's notifications from PRP-065, press the bell. The card should be as board 1; open one and it opens the counter; dismiss one; with none, board 2; the bell with and without its dot, board 3.
