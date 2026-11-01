## FEATURE: An app may declare its primary colour; Recent's cards wash in from the top-left in it, the icon there

**Status:** approved (the owner, 2026-11-02: "approved, go ahead"). Builds on PRP-062 (its branch, not yet merged).

## OBJECTIVE

- **An app may name its primary colour** under `[colors]` in `rapidkoi.toml` (`primary = "#14b8a6"`, the owner, 2026-11-02: "in rapidkoi.toml have the colors under [colors]"), or `--color primary=#14b8a6` for `pack`. It's its main colour, the one its icon is in.
- **The `[colors]` family:** further colours (a `background` for a loading screen or a landing page, say) are more keys in the same table, each added when Rapidkoi first draws it, so no field sits in manifests unused (the owner, 2026-11-02: "update it to primary_color").
- **The new tab's Recent cards wash in that colour from the top-left,** with the app's icon at the top-left of the picture area: the large card as the small ones, as board A5 draws them.
- **An app that names none** gets its colour from Rapidkoi's palette: the one the dock gives it, steady for that app.
- Rapidkoi's own apps and Notes declare theirs.

## CONTEXT

- **Where it comes from:** the owner, 2026-11-02, comparing A5 with the build: "why don't the cards have with icon at the top left and the icon main color as a gradient"; option 1 chosen: "go with 1".
- **Why there's none today:** PRP-060 dropped the design's "an app's colour in answers", since `icon:` draws an initial for apps without an icon, and Rapidkoi reads only pictures' headers, never their pixels.
- **Additive:** a manifest field, plus a field in the `recent` answer, both optional to older readers.
  - It's no new contract version (as PRP-055 and PRP-062 weren't), and nothing is republished for it.
  - Notes and the counter keep working without one; they get their colour from the palette until they're next released.
- **Decisions:** DECISION-056 (one contract; additions without a version). None open touch it.
- **Changes:**
  - **code:**
    - bundles: `src/bundles/bundle.la` (the field, read and checked), `src/bundles/project.la` (the `[colors]` table);
    - commands: `src/commands/pack.la` (`--color NAME=#rrggbb`), `src/commands/publish.la`;
    - data: `src/data/remembered.la` (`Face.colour`), `src/data/recentfile.la`;
    - window and guest: `src/window/recent.la`, `src/guest/home.la` (the answer's `colors`);
  - **tests:** every Manifest literal in `test/` (a new field), and the files covering the change;
  - **the built-ins:** the new tab (its cards), `scripts/newtab.sh` and `scripts/settings.sh` (`--color primary=…`);
  - **docs:** the spec and CHANGELOG.md;
  - **then:** rapidkoi-rust `v0.5.0-pre.3` (`RecentPlace.primary_color`), asked first; Notes' `[colors] primary` with its next release.

## REQUIREMENTS

### Must Do

- **The manifest's `colors`:** a map of names to colours; only `primary` is known today (an unknown name is ignored, so later colours don't break older readers).
  - `primary` is optional, `#rrggbb` (six hex digits, no alpha: a wash fades it itself);
  - anything else refuses the bundle with "This app's primary colour must be #rrggbb";
  - carried in format 1 and signed in format 2 with the rest of the manifest.
- **`rapidkoi.toml`** `[colors] primary = "#rrggbb"` and **`pack --color primary=#rrggbb`** set it, checked before anything is written; an unknown key in `[colors]` is refused, as other tables' are.
- **Kept with its app:** a recent place's app keeps its primary colour beside its name and icon (`Face.colour`, in the recent file, and the session's for later use). It's optional, so old files read as none.
- **The `recent` answer's places** each carry `colors: { primary }`: the app's own, else `themes.publisher_colour` from its first key (or its name, unsigned), as `#rrggbb`.
- **The new tab's cards:**
  - **the picture area:** the background `#1b1b1b`, under an ellipse from the top-left (`[0, 0, 1.1, 1.5]`) in the colour at 50 %, 12 % and 0 % (stops 0, 0.6 and 0.9), as A5's washes;
  - **the icon:** 88 px on the large card, 44 px on a small one, at the top-left with 28 px and 12 px of room.
- **Primary colours declared:** the new tab `#ff7700`, Settings `#71717a`, and Notes `#14b8a6` in its repository, with its next release. The counter keeps its palette colour until someone gives it one.
- **The spec:** `colors` in the manifest's fields and in `recent`'s places, and the `[colors]` family's rule.

### Must NOT Do

- **Reading an icon's pixels** for its colour (option 2): it needs a picture decoder and guesses.
- **A colour anywhere but Recent's cards** for now (the dock, Pinned, Restore keep their looks).
- **Other colours** (`background`, `accent`…) before Rapidkoi draws them. `accent` is Rapidkoi's own colour name in app styles, so a later field avoids it.
- **Republishing Notes or the counter** for this alone.
- **A new contract version.**

## TESTS

- [ ] `test/bundle.la` or `test/bundle_icons.la`: `colors.primary` `#14b8a6` round-trips; an unknown colour name is ignored; `teal`, `#14b8a`, `#14b8a6ff`, `14b8a6` are refused with the sentence; no colour reads as none.
- [ ] `test/pack.la`: `--color primary=` sets it; a bad one or an unknown name is refused before writing. `test/project.la`: `[colors] primary` is read and checked, an unknown key refused.
- [ ] `test/recentfile.la`: an app's colour round-trips, and a file without one reads as none.
- [ ] `test/home.la`: a place's `colors.primary` is the app's own, or the palette's by its key or name, `#rrggbb`.
- [ ] guest/rust: a recent place's colour reaches its card; the large card's icon is at the top-left.

## ACCEPTANCE

- [ ] Recent's cards wash in each app's colour from the top-left, icon at the top-left, the large as the small.
- [ ] An app with no colour gets the dock's.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main` and guest/rust's tests pass.
- [ ] For the owner, by eye: Recent with Notes (teal) and the counter (its palette colour, until it names one), each card's icon at the top-left over its wash, as A5.
