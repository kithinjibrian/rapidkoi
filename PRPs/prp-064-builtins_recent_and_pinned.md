## FEATURE: Rapidkoi's own apps go into Recent (all but the new tab), and Settings comes pinned

**Status:** approved (the owner, 2026-11-02: "yes approved").

## OBJECTIVE

- **Recent records Rapidkoi's own apps too:** Settings now, and any built-in to come. The new tab is the exception: it's the page Recent lives on.
- **Settings comes pinned:** on a first run, with no pins yet, your pins start with Settings. It shows in the dock and in the new tab's Pinned with its icon. Unpin it and it stays unpinned.
- **Opening Settings from a pin, a recent card or its path** (`builtin:settings`) opens it as ☰'s Settings does: its tab if open, else a new one.

## CONTEXT

- **Where it comes from:** the owner, 2026-11-02: "i feel like only new tab should not be in recents the other builtins can appear plus some builtins should come pinned like settings".
- **What exists:**
  - `window.recent.shown` skips every `builtin:` path (PRP-062).
  - Settings opens only from ☰ (`navigate.open_settings`, its tab or a new one, path `builtin:settings`).
  - A pin or a recent place opens by path (`open_pin`, `open_place`, `open_file`), which doesn't know `builtin:` paths.
  - Pins start empty; the pins file is read at launch (`data.saving.restore`), `Fresh` when there's none.
  - The ⚙ card's "Show in Recent" row is skipped for built-ins, and Settings' manifest shows only its permissions row (`--control settings=app_settings.permissions`).
- **No contract change, no SDK release, nothing republished.** It resolves the design's open "first run: an empty page, or pins Rapidkoi ships with" (`docs/designs/newtab-target.md`, "Not decided").
- **Decisions:** none open touch it.
- **Changes:**
  - code: `src/window/recent.la`, `src/window/navigate.la`, `src/data/saving.la`, `src/window/settings_app.la` (its pin), `src/ui/settings.la`, `src/bundles/bundle.la` (a control name);
  - the built-ins: `scripts/settings.sh` (the row);
  - tests, the design doc and CHANGELOG.md.

## REQUIREMENTS

### Must Do

- **Recording:** a page's place is kept unless its path is the new tab's (`builtin:newtab`), which is never a window anyway. Settings' place is `builtin:settings` with its name, icon and primary colour (`#71717a`).
- **Opening `builtin:settings`** by any path (`open_file`, so pins, recent places and the address box) is `open_settings`. Any other `builtin:` path, which only Rapidkoi writes, opens nothing new: "No app or file called …", as today.
- **Default pins:**
  - when the pins file is `Fresh` at launch (none yet), the pins are `[Settings]`: its name, `builtin:settings`, and its icon from its bundle;
  - saved like any pin change;
  - a pins file that exists, even empty, is never added to, so unpinning sticks;
  - with no data folder (`start(…, None)`), pins stay empty as today.
- **The ⚙ card:**
  - "Show in Recent" shows for any app but the new tab, under its own control name `app_settings.recent` (shown by default, as other rows are);
  - Settings declares it (`--control settings=app_settings.recent`), so its card offers it;
  - `app_settings.recent` joins the known rows.
- **The design doc:** its "Not decided: the first run" is answered (Settings pinned).

### Must NOT Do

- **The new tab in Recent, or pinnable:** it's no window.
- **More default pins:** only Settings is a built-in besides the new tab. Notes, a site, isn't pinned for you.
- **Re-pinning Settings after you unpin it**, or after Forget.

## TESTS

- [ ] `test/recent.la` / `test/remembering.la`: Settings' window is recorded; a `builtin:newtab` page isn't.
- [ ] `test/settings_app.la`: a pin or a recent place at `builtin:settings` opens Settings (its open tab, else a new one).
- [ ] `test/saving.la`: a fresh data folder starts with Settings pinned, with its icon; unpinned, a restart keeps it unpinned; no folder, no pins.
- [ ] `test/settings.la`: Settings' ⚙ card offers Show in Recent.
- [ ] `test/bundle.la`: `app_settings.recent` is a known row.
- [ ] Every test that counted pins from a fresh folder is checked.

## ACCEPTANCE

- [ ] Settings appears in Recent after you use it, and opens from its card.
- [ ] A first run shows Settings pinned in the dock and on the new tab; unpinning sticks.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: with a fresh data folder (`XDG_DATA_HOME=/tmp/rk ./main`), Settings is pinned; open it, change something, open a new tab: it's in Recent, grey wash, and its card opens it.
