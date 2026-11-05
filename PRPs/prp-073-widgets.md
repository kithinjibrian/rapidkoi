## FEATURE: Widgets: an app's small cards, which you put on the new tab yourself, four at most

**Status:** approved (the owner, 2026-11-04: "approved"). Designed with the owner on 2026-11-04, on the canvas "Widgets on the new tab", boards 4 to 7 (https://claude.ai/artifact/6rFHa7jz4iJpjAdwr3BSGL).

## OBJECTIVE

- **An app declares its widgets** in its manifest: for each, a name and a label ("Last edited", "Note count", "Pinned note"). It can declare up to 16.
- **While it runs, the app fills them.** It sends `widget` with a widget's name and a small view, and Rapidkoi keeps the newest card for each widget, across restarts.
- **You choose which ones go on the new tab.** Nothing goes there unless you add it, whether from an app or from Rapidkoi.
  - **App info › Widgets** (board 5): App settings gets a **Widgets ›** row ("2 of 3 on new tab"). Its page has a row per declared widget: a preview, its label and a switch.
  - **⚙ › Widgets** (board 7): the same row and page in the site settings card on the address bar.
- **The new tab** (board 4) shows a WIDGETS row of up to four equal cards above Recent, in the order you added them.
  - Rapidkoi draws each card's frame and its top line (the app's icon, its name, "· 20 minutes ago").
  - The app's view goes under that line.
  - Clicking a card opens its app at the card's place.
  - Right-clicking a card gives "Remove from new tab".
  - With no cards, the row isn't there.
- **Turning on a fifth widget** asks "Replace a widget?" (board 6). It shows the four current cards to pick from, with Cancel and Replace.
- **Forget** removes the app's cards and takes its widgets off the new tab.
- **A widget the app hasn't sent yet** previews as just its label (board 5, "Pinned note"). On the new tab, it shows its frame and top line with nothing under them until the app sends it.

## CONTEXT

- **Where it comes from:** the owner, 2026-11-04.
  - "an app can declare as many widgets as they want. then on app info we have a widget page the user goes through and can decide to add a widget from the app to new tab. site settings on the address bar should also have widgets"
  - "looking at screen real estate, i think we can have 4 cards unless we do scrolling"
  - Adding a widget is always the person's explicit decision.
  - The four boards were pasted back as the target ("so you don't implement something different").
- **Settled in that conversation:**
  - **Refresh:** the app updates its cards only while it runs (option A). Widgets are output only (DECISION-097, the owner, 2026-11-04): Rapidkoi never wakes an app for a card, so option B is ruled out.
  - **One look per card:** a single card size. Previews in App info and in ⚙ both carry the top line, as on the new tab.
- **What exists:**
  - **Notifications (PRP-065)** has the same shape: an app sends something, Rapidkoi keeps it in a data-folder file and the new tab shows it. This PRP follows that pattern: `guest/notifyasks.la`, `data/notificationsfile.la`, `window/notifications.la`, `guest/home.la`.
  - **App info (PRP-072):** `ui/appinfo.la`, with its pages in `window/info.la` (`InfoPage`).
  - **⚙'s pages:** `ui/settings.la`, `PanelPage` in `window/actions.la`.
  - **Questions over the app's area:** `ui/questions.la`.
  - **A right-click menu drawn by Rapidkoi:** kivuli's `.menu` with `Menu.look()` (PRP-071).
  - **Parts** (`guest/placeholders.la`) fill names and times at draw time, so "2 hours ago" stays correct without the app running.
  - **The manifest:** `bundles/bundle.la` (`Manifest`) and `bundles/project.la` (`rapidkoi.toml`), extended the way PRP-063 added `colors`.
- **Files near the limit:**
  - `guest/effects.la` is at 496 lines: the asks are answered from a new `guest/widgetasks.la`, and `effects` gains one call.
  - `window/window.la` is at 473 lines: the model gains one field.
  - `ui/settings.la` is at 434 lines: the Widgets page is drawn in a new `ui/widgets.la`, which both App info and ⚙ use.
- **The contract:** additions to contract 11 with no version bump, as PRP-065 and PRP-070 were:
  - the manifest's `widgets`;
  - the `widget` ask;
  - a built-in-only box prop, `slot: "widgets"`.

  An app without them is unchanged.
- **The SDK:** rapidkoi-rust `v0.5.0-pre.7` adds:
  - `Ask::widget(name, view)`;
  - `[[widgets]]` in `rapidkoi.toml`;
  - ready-made cards in `rapidkoi::widget`: `headline(title, text)`, `number(value, label)`, `progress(done, of, label)` and `list(title, lines)`, each built from boxes and texts;
  - a `count` widget in the counter example, for testing.

  I'll ask before tagging. The built-ins then move to the tag.
- **Decisions:** none open touch this. Once approved, a new entry, DECISION-097 "Widgets", records three choices:
  - you add every card yourself;
  - an app updates its cards only while it runs;
  - Rapidkoi draws widgets into the new tab through a slot.
- **Security:** `widget` is a new host function any app can call, so under CLAUDE.md it needs **the owner's review**. Its checks are under DESIGN.

## REQUIREMENTS

### Must Do

- **The manifest's `widgets`:**
  - a list of `{ name, label }`, at most 16;
  - `name` is 1–32 bytes of `a–z 0–9 _ -`, unique within the app;
  - `label` is 1–40 bytes.
  - It's set by `[[widgets]] name = "…" label = "…"` in `rapidkoi.toml`, or `pack --widget name=Label`. A bad entry is refused before anything is written.
  - It's read and checked like `colors`. A bundle without it declares no widgets.
- **The `widget` ask:**
  - `widget` + `name` (one the manifest declares) + `view` (a card tree) + `fields`? (its place, as `notify`'s are). Not answered.
  - It's allowed from any running app, with no permission: the card is shown only where you've put it, and its preview only on that app's own pages.
- **The card tree, checked as hostile:**
  - **Allowed:** boxes (column, row, layers), texts (with `parts`) and styles, as on the screen.
  - **Refused:**
    - typing boxes, pictures, scrolling, links, `on_` events, `menu` and roles;
    - more than 64 nodes, depth over 8, or over 16 KB encoded;
    - an undeclared name.
  - A bad `widget` ask stops the app with its sentence, as other bad asks do.
- **Rate:** at most 30 `widget` asks a minute per app. Over that, the extra ones are dropped and the app keeps running. A new card for the same name replaces the old one.
- **Keeping:**
  - a `widgets` file in the data folder holds each app's newest card per declared name (as checked bytes, re-checked when read), its time, its place, and the list of what's on the new tab (at most 4, in order);
  - a file that can't be read starts empty;
  - Forget removes the app's cards and its slots;
  - a card whose app no longer declares its name is dropped when that app next starts.
- **The new tab:**
  - the built-in new tab app places a box with `slot: "widgets"` above Recent (board 4);
  - Rapidkoi fills it with the WIDGETS heading and up to four cards in equal columns, 16 apart, or with nothing when none are added;
  - placing `slot` needs the built-in permission `show_widgets` ("Showing your widgets"), which the new tab declares (`scripts/newtab.sh`); any other app placing it is stopped, and blocking it in the new tab's ⚙ leaves the slot empty (the owner, 2026-11-04);
  - **each card** is a frame (the new tab's card frame), a top line drawn by Rapidkoi (the app's icon, its name, "· when" as a part), then the app's view;
  - **clicking a card** opens its place (`Msg.OpenPlace`), and right-clicking it gives a menu with "Remove from new tab".
- **The Widgets pages (App info and ⚙):**
  - a **Widgets ›** row in App settings, only when the app declares widgets, with the summary "N of M on new tab";
  - the page (`InfoPage.Widgets`, `PanelPage.Widgets`) has one row per declared widget: a preview at 220 × 120 (App info) or 150 × 84 (⚙, so it fits the card), the label, and a switch;
  - turning a switch on adds that widget at the end of the new tab's row; turning it off removes it.
- **A fifth widget:**
  - turning a switch on with four already added asks "Replace a widget?" over the app's area (board 6), with the four cards as choices (the first preselected), Cancel and Replace;
  - Replace puts the new card in the chosen one's place, and Cancel leaves the switch off.
- **The spec:** `docs/specs/contract-11.md` gains the manifest's `widgets`, the ask, the card tree's limits and `slot`.

### Must NOT Do

- **Waking an app for a card,** to refresh it (option B) or to act on a press: ruled out, since widgets are output only (DECISION-097).
- **Buttons, typing, pictures or scrolling inside a card:** a card does one thing, opening its app. Pictures can follow with their own limits.
- **Sizes other than one,** reordering by dragging, or more than four cards.
- **An app adding, removing or reordering cards on the new tab,** or seeing which of its widgets are shown.
- **Changing Notes:** it's its own repository, and declares widgets in its own release.
- **A new contract version.**

## DESIGN

**Pure state** (`window/widgets.la`, like `window/notifications.la`):

```
Card { path, origin: Option<u8[]>, name: string, view: u8[], tree: Node, address: string, sent: i64 }
Slot { path, origin: Option<u8[]>, name: string }
Widgets { cards: Card[], shown: Slot[], sent_lately: (key, ms)[] }
```

- **Functions:**
  - `keep(w, page, name, view, tree, fields, now)`, which replaces the card with the same name and returns `w` unchanged when over the rate;
  - `add(w, slot)`, which fails when the slot is already shown or four are already shown;
  - `replace(w, i, slot)`, `remove(w, slot)`, `forget_app(w, path, origin)`, `prune(w, path, origin, declared)`;
  - `card_of(w, slot)`, `shown_of(w, path, origin)`.
- **The model:** gets `widgets: Widgets` and `replacing: Option<Slot>`, the fifth widget waiting for an answer.
- **Messages:** `WidgetOn(Slot)`, `WidgetOff(Slot)`, `ReplaceWidget(i64)`, `CancelReplace`. Opening a card reuses `OpenPlace`.
- **Asks:** `guest/widgetasks.la` decodes the ask (`Effect.Widget`) and checks the tree with `tree.la`'s reader in a card mode that refuses what's listed above. It uses limits `MAX_WIDGETS` 16, `MAX_WIDGET_NODES` 64, `MAX_WIDGET_DEPTH` 8 and `MAX_WIDGET_BYTES` 16 KB in `guest/limits.la`.
- **The file:** `data/widgetsfile.la`, read at Restore and saved by Settle's save pass when the widgets change, as notifications are.
- **The view:** `ui/widgets.la` draws:
  - the card (frame, top line, the app's tree drawn by `render.la` with its parts filled);
  - the new tab's row, for the slot;
  - the Widgets page's rows;
  - the replace question.

  `ui/appinfo.la` and `ui/settings.la` each gain the Widgets › row and page from it.

**The slot:**
- A box from a built-in app with `slot: "widgets"` is read as `Node.Slot("widgets")`.
- When Rapidkoi draws the new tab, it replaces that node with the row.
- The new tab keeps its own layout and scrolling, and never receives the cards, so it can't read them.

**Security, for the owner's review:**
- **What an app can't choose:** a card can't name another app, a time, an icon or an action. Rapidkoi fills each of those in from the page that sent it, and a card's only action is opening its own app's place.
- **The card tree is capped and checked** before it's kept, and checked again when the file is read. A bad card from the file is dropped, never drawn.
- **Who sees what:** an app never learns whether its widgets are shown. Only built-ins can place `slot`.
- **Size:** the file holds at most 16 cards per app (the manifest's cap), each at most 16 KB.
- **Privacy:** what a card shows is visible on screen, like a notification. Showing one is your choice, and Forget deletes it.

## TESTS

- [ ] `test/bundle.la`, `test/project.la`: `widgets` is read and checked (17 widgets, a bad name, a repeated name and a 41-byte label are each refused), `[[widgets]]` and `--widget` are read, and a manifest without widgets declares none.
- [ ] `test/widgetasks.la`:
  - [ ] a card decodes;
  - [ ] an undeclared name stops the app;
  - [ ] each refused piece (an input, a picture, `on_press`, scrolling) stops the app;
  - [ ] 65 nodes, depth 9 or over 16 KB stops it.
- [ ] `test/widgets.la`:
  - [ ] keep replaces by name;
  - [ ] the 31st in a minute is dropped;
  - [ ] add, then a fifth add fails;
  - [ ] replace swaps in place;
  - [ ] remove, forget_app and prune each work.
- [ ] `test/widgetsfile.la`: a round trip; a missing or broken file is empty; a card that fails its checks is dropped.
- [ ] `test/widgetsview.la` (through `kivuli.testing`):
  - [ ] the new tab's slot shows four cards, each with its app's name;
  - [ ] clicking one opens its place;
  - [ ] its menu's "Remove from new tab" removes it;
  - [ ] no cards means no heading.
- [ ] `test/appinfo.la`:
  - [ ] Widgets › shows "1 of 3 on new tab";
  - [ ] its page lists three, an unsent one shows its label;
  - [ ] flipping a switch adds it;
  - [ ] a fifth flip asks "Replace a widget?", and Replace swaps it in.
- [ ] `test/settings.la`: ⚙'s Widgets row and page, the same as App info's.
- [ ] `test/tree.la`: `slot` from a built-in is read, and from any other app it's refused.
- [ ] Forget: an app's cards and slots go.
- [ ] guest/rust: the new tab places its slot above Recent; the counter example declares and sends `count`.

## ACCEPTANCE

- [ ] An app's declared widgets show in App info and in ⚙, and turning one on puts it on the new tab with the app's view.
- [ ] Using the app updates its card, and the card survives a restart.
- [ ] A fifth asks which card to replace; Forget removes the app's cards.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main` and guest/rust's tests pass.
- [ ] For the owner, by eye: open the counter example and press + a few times. In All apps › Counter › App info › Widgets, turn on "Count". A new tab shows the Counter card with the count and "· just now". Clicking it opens the counter. Press + again and open a new tab: the card shows the new count.
