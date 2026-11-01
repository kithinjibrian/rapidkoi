# rapidkoi-rust request: contract 11, icons (`v0.5.0-pre.1`)

From Rapidkoi, PRP-060 (DECISION-092). Built in `../rapidkoi-rust` at the owner's word (2026-11-01: "yes for the two", as for contract 10). Rapidkoi's side is on its branch `prp-060-app_icons`; it waits on this tag to move its built-in apps and merge. Notes and the counter on the VM need rebuilding on it too, and Notes needs an icon, or they stop opening once Rapidkoi moves.

## What changed in the contract

Contract 11 is contract 10 with icons, and Rapidkoi runs **only** contract 11 (`rapidkoi_version` 11; 10 and older are refused with "This app needs to be rebuilt for this Rapidkoi"). The full text: `docs/specs/contract-11.md` in Rapidkoi.

1. **Every app has an icon:** a square PNG of 128 to 1,024 px, or an SVG, named in the manifest. Rapidkoi refuses a bundle without one, and `pack` and `publish` refuse to make one.
2. **`icon:PLACE`**, a picture source only Rapidkoi's own apps may use: the icon of the app at a place, drawn by Rapidkoi.
3. **Answers name icons:**
   - a place in `pins` and `left_off` is `{name, place, icon}`, `icon` being `"icon:PLACE"` (never bytes) and always present;
   - `session` is `{count, apps: [place]}`, each app once.

## What the SDK needs

1. **`__CONTRACT_VERSION` = 11.**
2. **`Place.icon`** becomes a `String`, required: a source to pass to `image`.
3. **`Session.apps`** becomes `Vec<Place>`.
4. **Docs:** contract 11, and every app's icon in `rapidkoi.toml`.
5. **Tests:** places and sessions read with their icons; a place without an icon is `None`.
6. **Tag** `v0.5.0-pre.1` (a breaking change: every app rebuilds), asking the owner before pushing it.

## Done

Tagged `v0.5.0-pre.1` and pushed (2026-11-01); `guest/rust` moved to it, the built-ins repacked and `./main` rebuilt. Notes needs its icon, then Notes and the counter are republished, asked for separately.
