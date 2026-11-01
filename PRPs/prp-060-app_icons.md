## FEATURE: Every app has an icon, and Rapidkoi's own apps can show any app's icon (contract 11), starting with Restore's apps

**Status:** approved (the owner, 2026-11-01: "yes for the two": Q1 a bare `.wasm` still runs, Q2 the three icons drawn now). Decision: DECISION-092.

## OBJECTIVE

- **Every app has an icon.**
  - `./main pack` and `./main publish` refuse an app without one, saying how to add it.
  - Rapidkoi refuses a bundle without one.
  - Wherever an app is shown (the dock, Restore, Pinned), its own icon is there, never a letter standing in for it.
- **The new tab's Restore pill shows the real apps of your last session:** at most three icons, then "+N". Pinned can show real icons the same way once the new tab moves to real data.

The new tab, as a built-in app, names a picture Rapidkoi hands it, and Rapidkoi draws it:

```
// Rapidkoi's session answer: each app once, its icon as a source to show.
{ "count": 5, "apps": [ { "name": "Notes", "place": "notes.dafifi.net", "icon": "icon:notes.dafifi.net" }, … ] }

// The new tab's view.
{ "kind": "image", "props": { "src": "icon:notes.dafifi.net", "style": { "width": 26, "height": 26 } } }
```

## CONTEXT

- **Where it comes from:**
  - the owner, 2026-11-01: "let's build it. also we need to enforce that each app have an icon";
  - `docs/designs/newtab-target.md`: Restore, and "What it needs": the shell's item 3 and the contract's items 6–7.
- **What exists:**
  - a manifest's `icon` is optional (`src/bundles/bundle.la:244`), checked only as an asset of at most 1,024 px a side;
  - `pack --icon` and `rapidkoi.toml`'s `icon` add it;
  - the `pins` and `left_off` answers carry an icon's PNG bytes (`src/guest/home.la`, `MAX_PLACE_ICON` 64 KB), which no `image` can show;
  - the `session` answer is `{count, apps: [place]}`;
  - the dock draws an app without an icon as its initial (`ui/frame.la`'s `initial`).
- **Apps without an icon today:**
  - the built-in new tab and settings (packed with no `--icon`);
  - Notes (`../notes/rapidkoi.toml` has no `icon`).

  The counter has one.
- **Decisions:**
  - DECISION-056: one contract; older apps are refused.
  - DECISION-091: contract 10.
  - DECISION-044 (open) isn't touched: Recent places stays out.
- **Files to read first:**
  - `src/bundles/bundle.la`, `src/bundles/rules.la`, `src/bundles/loader.la`;
  - `src/commands/pack.la`, `src/commands/publish.la`;
  - `src/guest/home.la`, `src/guest/pictures.la`, `src/ui/pictured.la`;
  - `src/guest/limits.la`, `src/ui/frame.la`, `src/ui/sidebar.la`;
  - `test/bundle.la`, `test/pack.la`, `test/publish.la`, `test/home.la`, `test/pictures.la`.
- **Changes:**
  - **code:** the files above; `test/support.la` (stand-ins get an icon);
  - **docs:** `docs/specs/contract-10.md` → `contract-11.md`, DECISIONS.md, CHANGELOG.md, CLAUDE.md;
  - **the built-ins:**
    - `scripts/newtab.sh` and `scripts/settings.sh` (`--icon`);
    - `guest/rust` (the SDK tag);
    - `assets/apps/*.rkb`;
    - their icon files, new.
- **Then, outside this repository:**
  - rapidkoi-rust: a request and tag `v0.5.0-pre.1`;
  - Notes: an icon, then a release;
  - the counter: a release.

## REQUIREMENTS

### Must Do

- **Contract 11.** `CONTRACT_VERSION` 11; contract 10 apps are refused with the rebuild sentence.
- **An icon is required in every bundle:**
  - The manifest's `icon` must name an asset that is a square PNG of 128 to 1,024 px a side, or an SVG.
  - No icon: "This app has no icon". A PNG that isn't square: "This app's icon isn't square". One under 128 px: "This app's icon is smaller than 128 px".
  - Checked where bundles are read (`bundle.read`), so it holds for files, sites, kept copies and the built-ins alike.
- **`pack` and `publish` refuse an app without an icon** before writing anything, with how to fix it:
  - `pack`: "Give the app an icon: --icon icon.png (a square PNG of at least 128 px, or an SVG)";
  - `publish`: the same, naming `icon = "…"` in `rapidkoi.toml`.
- **A bare `.wasm`** is the development path (`./main app.wasm`, no manifest), so it keeps running with no icon (Q1). It's drawn with Rapidkoi's mark for a bare file: its initial on its colour, as today.
- **`icon:PLACE`, a picture source for Rapidkoi's own apps:**
  - It's that app's icon, found by its place (a host, or a file's name) among your pins, your open windows, your kept apps and your last session.
  - An app Rapidkoi can't find, or a bare `.wasm`, is drawn as its initial on its colour, as the dock draws it. Rapidkoi makes that picture.
  - Only a bundle built into `./main` may use it. Any other app naming one stops with "Only Rapidkoi's own apps can show other apps' icons".
  - It's shown at once, never fetched, and has no `color`. `on_load` and `on_fail` are never sent for it, so no app learns which apps you have.
- **The answers name icons, not bytes:**
  - `pins`, `left_off`: `icon` becomes `"icon:PLACE"`, always present.
  - `session`: `{count, apps: [{name, place, icon}]}`, each app once, in the order of its first window.
  - `MAX_PLACE_ICON` goes, and no icon bytes are sent to an app.
- **The built-ins get icons:**
  - Settings: a gear on the settings grey, like the replica's placeholder.
  - The new tab: Rapidkoi's mark, an R on the brand gradient.

  These are SVGs in `guest/rust/<app>/icon.svg`, packed with `--icon` (Q2).
- **The new tab's Restore pill uses the real answer:**
  - it asks for `session` on open, and shows the pill only while there's a session;
  - at most three `icon:` pictures overlapping, then "+N", where N is the apps past three;
  - pressing Restore asks `restore_session`.

  The rest of the replica stays as it is: real pins and the rest come with the new tab on real data, a later PRP.
- **No new permission:** the `session` ask is already covered by `restore_session` (`src/guest/effects.la:107`), which the new tab declares.
- **rapidkoi-rust, tagged first:**
  - version 11;
  - `Place.icon` becomes a `String` source;
  - `Session.apps` becomes a list of `{name, place, icon}`;
  - `image(&p.icon)` works as it is.

### Must NOT Do

- **Recent places, or the new tab on real data beyond Restore:** DECISION-044, and a later PRP.
- **A colour in answers** (the design's contract item 7): `icon:` draws the fallback, so apps never need one.
- **Icon bytes to any app**, or `icon:` for an app not built into `./main`.
- **Refusing a bare `.wasm`:** it's how apps are developed (Q1).
- **Generating an icon for an app that has none:** the rule is that the developer gives one.

## DESIGN

- **Bundle check** (`bundle.la` `check`): `m.icon` is `None` gives `BundleError.NoIcon`. The size and squareness come from `lugha-imagesize` (PRP-047) through `rules.size_of`, with SVG exempt from both. The new `BundleError` variants get their sentences in `loader.why`.
- **Pack and publish:** each checks its icon option before building the bundle, through a new `PackError`/`PublishError` variant with the sentence above.
- **Finding an icon** (`src/guest/icons.la`, new, pure):
  - `icon_for(m: Model, place: string): Option<(string, u8[])>` returns the bytes and a key for kivuli's cache;
  - it searches pinned, open tabs, `m.left_off`, the kept apps' manifests already in the model, and the previous session's tabs;
  - with nothing found, `fallback_svg(place, colour, letter)` makes a 64 px SVG: the initial on the app's publisher colour (`themes.colour_for`), as `frame.initial` does.
- **Drawing:**
  - `guest.pictures` reads `icon:` as a new `Source.Icon(place)`, refused for non-built-ins (`Origin` is known on the page);
  - `ui.pictured` draws it from the bytes the shell resolves when it decodes the screen (in `update`, never `view`), so it's `Ready` at once, as `asset:` is.
- **Answers** (`home.la`): `fields(…)` writes `icon: "icon:{place}"`. `session_of` builds a map per app, with its name from the tab's saved label or its manifest name if kept, else its place.
- **Limits:** `icon:` pictures count against the 256 pictures a screen like any other. A place is at most 253 bytes, as a host is.
- **Security:**
  - No new host function: `icon:` is a picture source, read in the existing picture path.
  - It's limited to built-in bundles, so no outside app can probe which apps you have (no `on_load`/`on_fail`, and refused anyway).
  - The fallback SVG is made from a single letter and a theme colour, never from app text beyond one character, which is escaped.
- **What the person notices:**
  - Apps without an icon stop opening until their developers add one: Notes, until it's republished with its icon.
  - All contract 10 apps need rebuilding (counter.dafifi.net and notes.dafifi.net), asked for separately.

## QUESTIONS FOR THE OWNER

- **Q1. Does a bare `.wasm` (development, no manifest) still run without an icon?** *Recommended:* yes. It's how apps are written and tested (`./main app.wasm`); the rule binds bundles, which is everything published or shared. *Or:* refuse it too, and every developer packs before each run.
- **Q2. The built-ins' and Notes' icons.** *Recommended:*
  - Settings: a gear on grey (the replica's placeholder);
  - the new tab: an R on the brand gradient;
  - Notes: the teal document from the replica's placeholders.

  All three are SVGs I draw now, for you to replace whenever you like.

## WHAT CHANGED WHILE BUILDING

- **The session file keeps its apps' names and icons** (`Open.apps`, each a `Face { path, name, icon }`, once per app), as the pins file keeps pins'. Without that, the last session's apps, none of them open, have no icon or name for Rapidkoi to hand over. The file's cap rises from 1 MB to 32 MB, as the pins file's. An icon that breaks the icon rules is dropped on reading, as a pin's is.
- **`icon:` is resolved while drawing, as `asset:` is** (`ui/appicons.la`, called from `ui.content`). It's a pure lookup over what the model holds: no I/O, so it isn't moved into `update`.
- **Kept apps' manifests aren't searched:** the model doesn't hold them, and pins, open windows, where you left off and the session cover what the new tab shows.
- **The refusal reads** "This app's screen names a picture it can't have: Only Rapidkoi's own apps can show other apps' icons" (the picture check's own opening).
- **The icon asset is named after its file** (`icon.svg` for an SVG), where `pack` and `publish` used to call every icon `icon.png`.
- **The new tab's Restore** pill sizes its row of icons to their number (26 px, then 18 more each).

## TESTS

- [ ] `test/bundle.la`: a bundle without an icon, with a non-square PNG, with one under 128 px, and with one over 1,024 px is refused with its sentence; a square 128 px PNG and an SVG pass.
- [ ] `test/pack.la` and `test/publish.la`: no icon is refused before anything is written, with how to fix it.
- [ ] `test/icons.la`: a place found through each source (pins, open windows, left off, kept apps, the last session), and the fallback for one that isn't found; the fallback escapes its letter.
- [ ] `test/pictures.la`: `icon:` from a built-in is read and drawn at once, with no `on_load`; from any other app it stops the app with its sentence.
- [ ] `test/home.la`: `pins`, `left_off` and `session` carry `icon: "icon:PLACE"` and no bytes; the session lists each app once with its name.
- [ ] `test/guest.la` and `test/tabs.la`: contract 10 is refused.
- [ ] The new tab stand-in: Restore shows only while there's a session (via `start_with`).
- [ ] guest/rust: the new tab asks `session` and draws the apps' icons, at most three, then "+N".

## ACCEPTANCE

- [ ] No bundle without a proper icon opens, and pack and publish say how to add one.
- [ ] The new tab's Restore shows your last session's real app icons, and its button restores.
- [ ] `docs/specs/contract-11.md` describes contract 11; DECISION-092 is resolved; CHANGELOG.md says it.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: open a few apps, quit, start again. Restore shows their icons (three, then "+N"), the dock shows every app's own icon, and Settings and the new tab have theirs.
