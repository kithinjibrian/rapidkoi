## FEATURE: Recent places replace "where you left off": the places you were, newest first, on the new tab and in the address box, each opening a window at its link

**Status:** approved (the owner, 2026-11-01: "approved, go ahead"). Resolves DECISION-044 (the owner, 2026-11-01: "yes for the two").

## OBJECTIVE

- **The new tab's Recent is real.**
  - The newest place is the large card; the next four are the small cards.
  - Each card shows:
    - its picture area: the app's icon on a wash, until apps give a picture (a later PRP, with the app's summary);
    - the place's title (the screen's title, else the app's name);
    - its app, its address on the large card, and when you were last there ("2 min ago", kept current by Rapidkoi).
  - Pressing a card opens a new window at that place's link, as following the link would. With no places, there's no Recent.
- **The address box suggests recent places as you type,** where it suggested "where you left off".
- **"Show in Recent"**, a switch in each app's ⚙ card (on by default). Off, the app's places go and none are kept until it's on again. Forgetting an app removes its places.
- **"Where you left off" goes:** the left-off record and its file, and Continue reopening a closed window with its state. Closing a window still deletes its state; nothing is revived.

## CONTEXT

- **Decisions:**
  - DECISION-044 (open → resolved here): Recent replaces "Continue where you left off". A place is a link; opening it opens a fresh window at that link.
  - DECISION-056 and the precedent of PRP-055: additions to the current contract that change nothing else (asks only Rapidkoi's own apps use) go into contract 11 without a new version, under a new SDK `pre` tag. Published apps keep working, so nothing is republished.
- **Where it comes from:** `docs/designs/newtab-target.md`: Recent; the shell's items 1 and 2.
- **What exists:**
  - `window/leftoff.la` (`LeftOff`, `leave`), `data/pinfiles.la` (the `left-off` file), `Msg.Continue`, `window.card`;
  - the address box's suggestions (`ui/suggestions.la`);
  - the `left_off` and `continue` asks (`guest/home.la`), under the `read_left_off` and `open_apps` permissions;
  - a window's address (`ui/frame.la` `address_of`: a site's link with its fields, or a file's name);
  - the minute tick (`m.now`, PRP-053);
  - per-app switches (`people.looks_of`, `Msg.OwnLooks`, the ⚙ card's App settings);
  - Forget (PRP-043, PRP-052).

  Sixteen files mention where you left off; all of them change.
- **Files:**
  - **new:** `src/data/recent.la` (the file), `src/window/recent.la` (recording, pure), `test/recent.la`, `test/recent_file.la`;
  - **removed:** `src/window/leftoff.la`, after its parts move;
  - **changed:**
    - window and data: `window/window.la`, `window/navigate.la`, `window/pins.la`, `window/restarts.la`, `window/choosing.la`, `data/saving.la`, `data/pinfiles.la`;
    - net and guest: `net/loads.la`, `guest/home.la`, `guest/effects.la`, `guest/contract.la`, `guest/nodes.la`;
    - ui and people: `ui/suggestions.la`, `ui/settings.la`, `ui/appicons.la`, `people/people.la`;
    - `bundles/permissions.la` (wording);
    - the new tab;
    - the spec, DECISIONS.md and CHANGELOG.md.
- **Then:** rapidkoi-rust `v0.5.0-pre.2` (additions only), asking before the tag.

## REQUIREMENTS

### Must Do

- **A place** is:
  - its address, what the address box shows for the window: a site's link with its fields, or a local file's path;
  - its app's path and origin, name and icon;
  - its title (the screen's, if it gave one);
  - when it was last open (`seen`, in ms).

  Two places are the same place when their addresses are.
- **Recording:** when a window's address or title changes, when it leaves the front, and when it closes, its place's `seen` becomes now and its title the latest, the place moving to the top. Not recorded:
  - the new tab and Settings (built-in);
  - an app with "Show in Recent" off;
  - a window that never ran (refused or broken).
- **Kept:** the newest 100 places, in a `recent` file in the data folder. Each app's name and icon are kept once with them, as the session file keeps its apps' (PRP-060). The cap is 32 MB, read as every file is: hostile, every field checked, broken set aside with a notice.
- **Asks** (contract 11 additions, only for an app holding `read_left_off` / `open_apps`):
  - `recent` answers `{places: [{name, place, icon, address, title?, seen}]}`: newest first, at most 20; `icon` is `icon:PLACE`; the address is shown, never a path beyond a file's name.
  - `open_recent` + `index` opens that place as the list is now: a new window at its address.
  - `left_off` answers the newest place, and `continue` opens it, so a contract 11 app that asks them still gets an answer. Both are marked for removal at the next contract version.
- **The new tab:**
  - It asks for `recent` with the session and pins: when shown, and on change.
  - It shows the newest place large and the next four small, as the replica does, and presses open them (`open_recent`).
  - The large card's picture area is the app's icon (88 px) on the wash; a small card's is its icon (44 px) on the wash.
  - Times are `{time: seen, show: "relative"}` parts, so Rapidkoi keeps them current and the app never learns the zone.
  - With no places: no Recent section.
- **The address box:** suggestions are your recent places matching what's typed (by title, app name or address), newest first, replacing the left-off card. An open window's place selects that window, as the left-off card did for an open tab.
- **"Show in Recent":**
  - a switch row in the ⚙ card's App settings for every app but the built-ins, stored per app with its looks (`people`);
  - turning it off removes the app's places at once;
  - Forget removes them too.
- **Removed:**
  - `LeftOff` and the `left-off` file (read once at the first launch after this, into a first recent place, then deleted);
  - `Msg.Continue` and `window.card`;
  - keeping a closed window's state or compiled module for Continue.
- **The permission's words:** `read_left_off` reads "reading your recent places" (the name in manifests is unchanged).
- **The spec** (`contract-11.md`): the `recent` and `open_recent` asks, `left_off` and `continue` deprecated, and Recent places described.

### Must NOT Do

- **An app's own summary or picture for a place:** the next PRP, which needs a contract change.
- **Reviving a closed window's state:** a place opens fresh, at its link.
- **Recording built-in apps, or any app with Show in Recent off.**
- **A history page, or clearing all places at once:** not asked for. Show in Recent and Forget cover it per app.
- **Notifications** on the new tab.

## DESIGN

- **`window/recent.la`** (pure):
  - `record(places, page, now): Place[]` updates or adds a place, moves it to the front and caps the list at 100;
  - `forget_app(places, path, origin)` removes an app's places;
  - `matching(places, query)` returns places for suggestions.

  The model gains `recent: Place[]` and loses `left_off`.
- **Where recording happens** (`update`, through the existing paths): navigation and title changes (`navigate`, the result of `handle`), selecting another tab, and closing one. Each is a pure model change, saved with the model's other small files (`data/saving.la`, CLAUDE.md's exception).
- **`data/recent.la`** reads and writes the `recent` file, format 1: places, plus each app once with its name and icon. Checks:
  - addresses at most 4,096 bytes;
  - titles at most 200 bytes;
  - names 1–64 characters;
  - `seen` from 0 to year 9999;
  - icons by the icon rules.
- **Answers** (`guest/home.la`): `recent_of(m)`, `left_off_of` from the newest place, `recent_msg(m, i)` and `continue_msg` opening a place by `Msg.OpenAddress(address)`, the address box's own way to open.
- **The ⚙ card:** `Msg.ShowInRecent(key, on)`, stored with the looks in `people`, and the row shown under `app_settings.looks`'s control name.
- **Icons:** `ui/appicons.icon_for` searches recent places' apps instead of left off.
- **SDK** (`v0.5.0-pre.2`): `Ask::recent(msg)`, `Ask::open_recent(msg, index)`, and `Recent::read(cbor)` giving `Vec<RecentPlace { name, place, icon, address, title, seen }>`.
- **Security:**
  - No new host function beyond two asks gated by existing built-in permissions.
  - Answers carry an address as the address box shows it, never a path's folders.
  - The file is read as hostile.
  - Nothing an app says beyond its screen's title (already shown in the frame) is kept.

## WHAT CHANGED WHILE BUILDING

- **The address box shows no suggestions today:** its suggestions card was removed until its redesign (session 108), and `ui.suggestions.suggest` has no caller. `suggest` now offers recent places in place of the left-off card, ready for when the card returns. Nothing new is shown.
- **A place is its address and its app's origin,** not its address alone. Two apps at one address (PRP-049) are two places, and Forget or Show in Recent removes only that app's.
- **Opening a place carries its origin** (`Msg.OpenPlace(address, origin)`). Without fields it opens as a pin does; with fields, a new window for that exact app at them (`net.loads.begin_place`), never asking which.
- **Where recording happens:** closing a window, replacing its app, and choosing another app record its place where they close the page (`window.recent.remember`). Moves while open (first running, address or title changes, leaving the front) are recorded by `window.remembering.noted`, called from `data.saving.persist` before the data folder check, so places are kept with or without a folder. That keeps Forget from re-recording what it closes.
- **The picture area** is the app's icon on a wash of the theme's own colours, since an app's colour isn't known (PRP-060). An app's own picture comes with the next PRP.
- **The new tab's placeholder pictures** for Recent (the beach, counter and settings icons) are gone.

## TESTS

- [ ] `test/recent.la`: recording adds a place at the top; the same address updates it and moves it up; 101 places keep the newest 100; built-ins, stopped windows and apps with Show in Recent off aren't recorded; forgetting an app removes its places; suggestions match title, name or address.
- [ ] `test/recent_file.la`: the file round-trips; hostile fields (a long address, a bad time, a bad icon, a duplicate) are refused or dropped as specified; a left-off file becomes a first place, then goes.
- [ ] `test/home.la`: `recent` answers newest first, at most 20, with `icon:` and no paths; `open_recent(i)` opens that place; `left_off` and `continue` answer from the newest place.
- [ ] `test/suggestions.la`: typing suggests recent places; an open one selects its window.
- [ ] `test/settings.la`: the ⚙ card's Show in Recent switch turns recording off and removes the app's places; built-ins have no such row.
- [ ] End to end: open an app, close it, open a new tab: Recent shows it, and pressing it opens a new window at its link.
- [ ] guest/rust: the new tab asks `recent`, draws the large card and up to four small ones with relative times, and a press asks `open_recent(i)`; no places, no Recent.
- [ ] Every test that used where you left off is moved to recent places or removed with its feature.

## ACCEPTANCE

- [ ] Recent on the new tab and the address box's suggestions show your real places, newest first, and open them.
- [ ] Show in Recent works per app; Forget removes an app's places.
- [ ] Nothing of "where you left off" is left in `src/`; DECISION-044 is resolved; the spec and CHANGELOG.md say it.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main` and guest/rust's tests pass.
- [ ] For the owner, by eye:
  - open three apps and move around in them;
  - open a new tab: Recent shows them, newest large, with times;
  - press one, and it opens at that place;
  - type in the address box: they're suggested;
  - turn Show in Recent off in one app's ⚙: it leaves Recent.
