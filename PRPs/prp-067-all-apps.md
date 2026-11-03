## FEATURE: All apps: every app you've opened, in a built-in app opened from the dock; Pinned leaves the new tab

**Status:** approved (the owner, 2026-11-03: "build"). DECISION-094. Builds on PRP-066 (its branch, not yet merged).

## OBJECTIVE

- **A button at the dock's end** opens **All apps**, a built-in app in a tab of its own, as Settings is.
- **All apps lists every app you've opened** until you Forget it, Rapidkoi's own Settings among them (not the new tab, not All apps itself):
  - "All apps", then a search box, then a grid: each app's icon and name, A–Z, with a switch to **Last used**;
  - a pin on the ones on the dock;
  - when two apps share a name, each also shows where it's from (its site, or its file's name);
  - typing filters by name; Enter opens the first match; pressing a tile opens it.
- **Each tile's ⋯** shows its details: its address, its publisher (signed or not), when you last opened it. Its actions:
  - **Open**;
  - **Pin to dock** or **Unpin from dock**;
  - **Permissions**: opens the app with Rapidkoi's ⚙ panel on its permissions;
  - **Forget this app…**: opens the app with Rapidkoi's own "Forget this app?" question, as the dock's menu does. All apps never forgets anything itself.
- **The new tab no longer shows Pinned** or "See all": Notifications, then Recent, as on the owner's canvas.
- **The design:** the canvas "All apps" (https://claude.ai/artifact/1GBB2pFkSh1EfYMkA3vRKR), its three boards.

## CONTEXT

- **What exists:**
  - built-in apps: the new tab and Settings (`window/newtab.la`, `window/settings_app.la`, `scripts/*.sh`, built-in permissions, `Tier.BuiltIn`);
  - Recent (`window/recent.la`): places and their apps' faces, at most 100 places, so an app drops out with its last place;
  - pins (`window/pins.la`, `data/pinfiles.la`), the dock (`ui/sidebar.la`) and its menu (`PinAction`: Open, Unpin, Forget);
  - Forget (PRP-052) and the ⚙ panel's pages (`PanelPage`);
  - the new tab's asks by index (`open_pin`, `open_recent`), answered in `guest/home.la`.
- **Files at the limit:**
  - `src/guest/effects.la` is at 500 lines and must grow (the new asks' permissions). **Proposed split:** `needed` (which permission each ask needs) moves to a new `src/guest/needs.la`;
  - `src/data/remembered.la` is at 503 lines (noted after PRP-065): this PRP adds nothing to it; the apps file is its own module.
- **Contract:** additions to contract 11 for built-in apps only, with no new version, as PRP-062 and PRP-065: `apps`, `open_app`, `pin_app`, `unpin_app`, `app_permissions`, `forget_app`. Released in rapidkoi-rust as a new tag (`v0.5.0-pre.5`), which needs the owner's ask.
- **Decisions:** DECISION-094 (resolved). None open touch it.
- **Changes:**
  - **code:**
    - new: `src/window/yourapps.la` (your apps, pure; `window/apps.la` is taken), `src/data/yourappsfile.la` (its file), `src/window/allapps.la` (the built-in app, as `settings_app.la`), `src/guest/appasks.la` (the asks, decoded), `src/guest/appanswers.la` (their answers), `src/guest/needs.la` (the split), `guest/rust/allapps/` (the app), `scripts/allapps.sh`, `assets/apps/allapps.rkb`;
    - changed: `src/bundles/permissions.la` (`ReadApps`, `ManagePins`), `src/guest/nodes.la`, `contract.la`, `effects.la`, `home.la`, `src/window/window.la`, `actions.la`, `navigate.la`, `remembering` (where an opened app is recorded), `pins.la` (Forget removes it), `src/data/parts.la`, `saving.la`, `src/ui/sidebar.la`, `src/ui/icons.la`;
    - the new tab: Pinned and `read_pins` removed (`guest/rust/newtab`, `scripts/newtab.sh`), repacked;
  - **tests and docs:** tests; `docs/specs/contract-11.md`, `docs/DESIGN.md`, CHANGELOG.md, CLAUDE.md (the SDK tag, the built-in apps).
- **From the owner:** the dock button's icon, `assets/icons/layout-grid-circles.svg`.

## REQUIREMENTS

### Must Do

- **Your apps (`window/yourapps.la`):** one entry per app (its path and first key, as Recent and pins name an app):
  - its name, icon and colour (a `Face`), when it was first and last opened;
  - recorded each time an app opens in a window, where Recent records a place;
  - removed only by Forget (PRP-052), with its pin, data and places;
  - kept in an `apps` file (`data/yourappsfile.la`, read as hostile as every file); on the first run with no file, seeded from Recent's apps and your pins (notifications keep an app by path alone, with no first key, so theirs wait until opened);
  - Settings always listed; the new tab and All apps never.
- **The asks** (built-in apps only; anything else is refused as built-in permissions are):
  - `apps` (`read_apps`): answers `{ items: [{ name, icon, from, address, publisher, signed, pinned, opened }] }`, A–Z. `icon` as the new tab's are (`icon:PLACE`); `from` is the host, or the file's name; `opened` in milliseconds since 1970;
  - `open_app(i)` (`open_apps`): opens app `i` as the dock opens a pin;
  - `app_permissions(i)` and `forget_app(i)` (`open_apps`): open app `i` with the ⚙ panel on its permissions, or on "Forget this app?", as the dock's Forget does;
  - `pin_app(i)` and `unpin_app(i)` (`manage_pins`): pins it at the dock's end, or unpins it;
  - an index past the end does nothing; `i` is as last listed, as `open_pin` is.
- **All apps is told when your apps change** (`on_change`), as the new tab is told of pins.
- **The dock:** its last button, "All apps", opens All apps in its tab, or goes to it when it's open; the button is current while it's in front.
- **All apps (Rust, `guest/rust/allapps`):** the grid, search, A–Z and Last used, "from" when names clash, the ⋯ details and their actions; Enter in the search box opens the first match (`on_submit`). Its own tests in Rust, as the new tab's.
- **The new tab:** Pinned and See all go; it no longer declares `read_pins`.
- **The look:** the canvas: surfaces at `PAGE_RADIUS`, every field and control round.

### Must NOT Do

- **Size on this device** in the details: it needs reading file sizes in a task and a further answer. A later PRP, with cleaning up.
- **Focus in the search box when it opens, and arrow keys between tiles:** the contract can't focus a typing box or take arrow keys. Click the box; Tab moves between tiles. Recorded as a deferred contract gap.
- **Categories, folders, suggestions, manual order:** DECISION-094 and the canvas.
- **"Add widget":** the widgets PRP.
- **Opening All apps in a bottom sheet:** a tab first; a sheet later if it feels heavy.
- **All apps forgetting or changing permissions itself:** only Rapidkoi's own panel, where the person decides.

## DESIGN

- **Model:** `apps: Apps` in `window.Model` (`Apps { items: App[] }`; `App { path, origin, face: Face, first: i64, last: i64 }`).
- **Messages:** `Msg.OpenAllApps` (the dock's button). The asks are answered as the new tab's are, in `guest/home.la` and a new `guest/appasks.la`.
- **Permissions:** `ReadApps` ("read_apps") and `ManagePins` ("manage_pins"), both `Tier.BuiltIn`, with words and labels in Settings.
- **Security (for the owner's review):** two new built-in capabilities and six asks. None reaches a file, a path or another app's data; `publisher` and `signed` are what the shield already shows. Forget and permissions only open Rapidkoi's own panel. An unsigned bundle can never hold `read_apps` or `manage_pins` (built-in permissions are for bundles built into `./main`, PRP-030).
- **Errors:** a broken `apps` file is set aside, as other files are, and your apps start again from Recent and pins.

## TESTS

- [x] `test/yourapps.la`: opening an app records it (first and last); opening again moves only `last`; Forget removes it; Settings listed, the new tab and All apps never; seeding from Recent and pins.
- [x] `test/yourappsfile.la`: it round-trips; hostile fields refused; a broken one set aside.
- [x] `test/appasks.la`: each ask decoded; a bad index or shape refused with its sentence.
- [x] `test/appanswers.la` (or `home.la`): `apps` answers A–Z with each field; `pin_app` and `unpin_app` change pins; `open_app`, `app_permissions`, `forget_app` open the app with the right panel; past the end nothing changes; without the permission, refused.
- [x] `test/permissions.la`: the two new permissions, built-in only.
- [x] `test/sidebar.la`: the dock's last button opens All apps, and is current while it's in front.
- [x] `test/newtab.la`: no Pinned, four permissions.
- [x] `guest/rust/allapps`: search, A–Z and Last used, "from" when names clash, Enter opens the first match, each ⋯ action sends its ask.

## ACCEPTANCE

- [x] All apps lists every app you've opened, opens, pins, unpins, and sends you to Rapidkoi's own Forget and permissions; the new tab has no Pinned.
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass; `cargo test` for the built-in apps passes.
- [ ] For the owner, by eye: open a few apps, then the dock's All apps. Compare with the canvas's three boards; search, open one, pin and unpin one, try Forget from its ⋯. The new tab: Notifications and Recent only.

## AS BUILT (after the owner's review, 2026-11-03)

- **The dock's button opens and closes All apps:** pressed while it's in front, it closes and you're back on the tab you were on (`before_all_apps`); leaving it for any other tab closes it (`ownapps.left_all_apps`, first in `update` before Settle). It opens fresh each time, cheaply: its module is kept for the run (PRP-068).
- **⚙ shows its permissions** (`--control settings=app_settings.permissions`).
- **Rapidkoi's own apps** show a "Rapidkoi" chip in the address bar, then their address, and no shield; the address text is a step larger (`TEXT_ADDRESS`).
- **A pin replaces ☆** in the address bar (the owner's `pin.svg`, filled in the accent while pinned); All apps marks pinned apps with the same pin on the icon's corner; All apps itself is never pinned.
- **No ⋯ on tiles:** an app's details open on a right-click once the contract has `on_context` (a later PRP; kivuli first).
- **Opening Settings and All apps** moved from `window.navigate` to `window.ownapps` (its size).
