## FEATURE: App info, a Rapidkoi screen in All apps with everything about one app

**Status:** approved (the owner, 2026-11-04). Follows PRP-071. Design: the All apps canvas, artboards 4 and 5 (2026-11-04).

## OBJECTIVE

- **All apps' tile menu:** Open, Pin to dock or Unpin from dock, App info, a separator, Forget this app…. Permissions is gone from the menu.
- **App info is a screen in the All apps tab**, drawn by Rapidkoi, in two columns, with ← All apps back to the grid.
  - **Left, who it is:**
    - its icon, name and address;
    - a verified publisher badge when signed;
    - Open and Pin or Unpin;
    - **Publisher:** key, known since, version, updates;
    - **Used:** first and last opened;
    - Forget this app… at the bottom.
  - **Right, what it can do and what it keeps:**
    - **Permissions:** each permission it declares, Sign in, and your name in it;
    - **Servers it talks to:** each server it reaches;
    - **Looks:** your colours, your text size, Show in Recent;
    - **Storage:** its total on this device.
- **It works for an app that isn't open.** Opening App info doesn't start the app. Every change takes effect as in the ⚙ panel, saved the same way.
- **Forget, from the menu or App info,** asks in Rapidkoi's own confirmation over All apps. It forgets the app without opening it, which the dock's Forget today does by opening it first.

## CONTEXT

- **What exists:**
  - **The shield card** (`ui/shield.la`, the publisher facts) and the **⚙ panel** (`ui/settings.la`: account, servers, permissions, looks, Recent, data, Forget). Both read a running tab's `AppPage`.
  - **The model's own settings**, keyed by `key_of` (`people/access.la`: the host when signed, else the path):
    - `People.apps` (`Person`: sign in, your name in the app, servers off);
    - `People.grants`, `People.looks`, `People.unlisted`.
  - **The trust ring:** a publisher's key and when it was first known.
  - **Your apps** (`window/yourapps.la`): its path, host, first key, face, first and last opened.
  - **A signed app's kept copy** (`data/kept.la`, PRP-043): its build, so its manifest (version, declared permissions and servers), and when it was last checked. It can be evicted.
  - **All apps' `app_permissions` ask:** it opens the app, then its ⚙ on Permissions (`window/navigate.la`).
- **The data sources a closed app needs:**
  - **The manifest:**
    - for a signed app, from its kept copy;
    - for a local file, from the file itself;
    - for a built-in, from its bundle.

    All are read in a task, checked as at opening.
  - **Sizes:** read in a task.
  - **Everything else:** already in the model.
  - **Left out:** the connection row, known only while the app runs.
- **Decisions:** DECISION-094 (All apps), DECISION-030 (settings drawn natively). None are open.
- **Changes:**
  - **new:**
    - `src/window/appinfo.la`: App info's model, its messages, and what it reads for a closed app;
    - `src/ui/appinfo.la`: the screen;
  - **changed:**
    - `src/ui/settings.la`, `src/ui/grants.la`, `src/people/access.la`, `src/ui/shield.la`: their rows and changes are keyed by an app, not only by the current tab, so App info and the ⚙ share them;
    - `src/guest/appasks.la`, `src/guest/appanswers.la`: `app_info`;
    - `src/ui/layout.la`: App info drawn in All apps' place;
    - `src/window/navigate.la`: Forget for a closed app;
    - `guest/rust/allapps`: the menu and the ask; its details card removed;
    - **docs:** `docs/specs/contract-11.md`, CHANGELOG.md.

## REQUIREMENTS

### Must Do

- **`app_info` + `index`** replaces `app_permissions`, under the same permission (`open_apps`). It's a built-in-only ask, used only by All apps, which is repacked in the same change, so nothing published breaks. Rapidkoi answers nothing; it opens App info on that app.
- **Drawn by Rapidkoi** in the All apps tab's content area, while All apps keeps running underneath with its state.
  - ← All apps closes it, as does Escape.
  - Leaving All apps closes it too.
  - The address bar stays `builtin:allapps`.
- **The same rows as the ⚙, not copies:**
  - permissions use today's three-way choice (Allow, Ask, Block);
  - servers use the ⚙'s switches, on a signed app only;
  - looks and Show in Recent use its switches;
  - your name uses its inline fields.
- **Built-in apps** show only the rows their `controls` name, as the ⚙ does.
- **What a closed app's info can't show:**
  - **Manifest:** if the kept copy was evicted or the file is gone, the manifest-based rows aren't shown. No error is shown for it, and the settings Rapidkoi keeps still show.
  - **Size:** if it can't be read, Storage isn't shown.
- **Open** opens the app and closes All apps, as a tile does. **Pin or Unpin** works as the menu's item does.
- **Forget:** Rapidkoi's own confirmation, over All apps. Confirming forgets the app exactly as the ⚙'s Forget does (PRP-043, PRP-052): its settings, data, kept copy and pin, and its place in your apps. App info then closes back to the grid.
- **The tile menu (PRP-071):** App info in place of Permissions, as in OBJECTIVE. The details card and `Msg::Details` go.

### Must NOT Do

- **All apps drawing App info, or being given the publisher facts or any way to change a setting:** Rapidkoi draws it, and no app gets new power.
- **Starting the app to show its info.**
- **New settings:** no Clear data, and no storage split (the owner, 2026-11-04: the total only).
- **Changing the address bar's shield or ⚙:** after this is seen on screen.

## DESIGN

- **Model:** `info: Option<Info>`, where `Info` holds:
  - the app (from your apps);
  - what was read for it: `Option<Manifest>`, `Option<i64>` bytes, `Loading` until the task returns.
- **Messages:** `Msg.AppInfo(path, origin)`, `Msg.InfoRead(path, Result<…>)`, `Msg.CloseInfo`. The ⚙'s setting messages gain the app they're for, so one message changes the current tab's app or App info's.
- **The read:**
  - one task per open, checked as an opening checks a manifest;
  - an answer for an app that's no longer shown is dropped.
- **Security:**
  - no new host function or capability, and `app_permissions` becomes `app_info` with nothing answered;
  - the manifest read for a closed app goes through the same checks as opening it, with the same caps;
  - All apps learns nothing it didn't know.
- **Files at or under 500 lines:** `settings.la` is split if the keying pushes it past.

## TESTS

- [ ] `test/appinfo.la` (new):
  - [ ] App info for a signed app that isn't open shows its publisher, versions, permissions, servers, looks and storage, and doesn't start it;
  - [ ] a local file's info shows its manifest rows;
  - [ ] with no kept copy, the manifest rows are absent and the rest shows;
  - [ ] a change made there (a permission, a server, a look, Show in Recent, your name) is kept and seen by the ⚙ when the app opens;
  - [ ] ← and leaving All apps close it;
  - [ ] Open opens the app, and Pin pins it;
  - [ ] Forget asks, then forgets without opening the app;
  - [ ] a built-in shows only its `controls` rows;
  - [ ] a stale read answer is dropped.
- [ ] `test/appasks.la`: `app_info` read with `open_apps`, refused without it; `app_permissions` is no longer read.
- [ ] `test/allapps.la`: picking App info on a tile shows App info in the All apps tab.
- [ ] `test/settings.la`, `test/grants.la`, `test/access.la`: still pass with settings keyed by app.
- [ ] `guest/rust/allapps`: the menu holds App info, and its message asks `app_info`.

## ACCEPTANCE

- [ ] App info shows everything in OBJECTIVE for an app that isn't open, and its changes are kept.
- [ ] These pass:
  - `lughac test test/` (the owner runs it);
  - `lughac fmt --check src test`;
  - `lughac build -O 2 src/main.la -o main`;
  - the built-ins' `cargo test`.
- [ ] For the owner, by eye: open All apps and right-click a tile, then App info.
  - The two-column screen appears as on the canvas.
  - Switch a look, go back, open the app: the look holds.
  - Forget an app from App info: it asks, and it's gone from the grid and the dock.
