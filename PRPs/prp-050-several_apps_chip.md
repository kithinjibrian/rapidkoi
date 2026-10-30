## FEATURE: Several apps at one address, told apart: a chip and its list in the address box, a colour per app from its key, a default, renaming, and a dock entry per app

## OBJECTIVE
When an address has more than one app (DECISION-064), you can always tell which one you're in and switch between them. A chip at the start of the address box shows the app's colour and name; clicking it lists every app at that address (colour, name, key fingerprint), with a default and Rename. Typing the address opens the default, or asks "Which app?" when there's none. Each app has its own entry in the dock, marked with its colour. The colour is shown from the moment you name a new app.

This is the rest of the journey (https://claude.ai/artifact/Nx55CYNApso8E7GjWYGyvN): steps 6 (the colour while naming), 7–9 (chip, list, default, dock marks) and 10 (renaming).

## CONTEXT
- From chat with the owner (2026-10-30), with their sketch; DECISION-064 records it. Builds on PRP-049 (merged): `window/apps.la`, `trust/publishers.la`, `window/choosing.la`, `ui/asking.la`.
- Starting state (read on `main`):
  - **The dock groups apps by address** (`ui/sidebar.la`: `open_tab_of(m, path)`, pins matched by `p.path`), and `instances.instances_of` counts windows by address. With two apps at one address open, the dock shows one of them and counts both's windows together: a gap PRP-049 left, fixed here.
  - The address box (`ui/addressbox.la`, 236 lines) is one group: upgrade arrow, shield, ⚙, the field, share, ☆. The shield's card is a `kivuli.ui` popover (`m.shield`).
  - GTK menus can't draw coloured dots ("GTK draws no icons beside menu items", kivuli README), so the chip's list is a popover with its own rows, like the shield's card.
  - Colours come only from `ui/themes.la` and `ui/schemes.la` (DECISION-031). Instances already have a palette (`Theme.instances`, by instance key, on the strip's chips).
  - Each identity has a name (PRP-049: the manifest's at first use, or the person's), saved in the trust file (format 4).
- Files changed: `src/ui/addressbox.la` (the chip), new `src/ui/several.la` (the chip's list, its rows, the dot), `src/ui/asking.la` (the naming dot; "Which app?" rows), `src/ui/sidebar.la` and `src/window/instances.la` (one entry per app), `src/ui/schemes.la` and `src/ui/themes.la` (the palette), `src/trust/verify.la` and `src/data/remembered.la` (the default, trust format 5), `src/window/choosing.la` (default and renaming), `src/window/actions.la`, `src/window/window.la`, `src/shell.la` (messages, wiring). Tests: `test/several.la`, and the files of the modules changed.
- Decisions: DECISION-064 (resolved), DECISION-031 (colours from the theme). None open.

## REQUIREMENTS

### Must Do
- **A colour per app**, from its first key: a palette of publisher colours in each scheme, with no danger red and no brand orange; the key picks one, so an app's colour never changes.
- **The chip**, only on an address with more than one app known: at the start of the address box, before the upgrade arrow and the shield, the app's colour dot and its name. Clicking it opens the list; a click outside or Escape closes it.
- **The list:** one row per app at the address, in the order they were met: its colour dot, its name, its key's short fingerprint, a **Default** check box, and **Rename**. The current app's row is marked. Clicking a row's name goes to that app's tab, opening it if needed (as "Which app?" does).
- **The default:** at most one per address. Ticking one unticks any other; unticking leaves none. Typing the address, a link, a pin or a session saved before PRP-049 open the default directly; with none, "Which app?" as now. Saved with the identity (trust file format 5; 4 reads with no default).
- **Renaming:** Rename opens the naming dialog, titled "Rename app", the field holding the current name; the same rules (not empty, at most 64 characters, not another app's name at this address). The name shows everywhere at once: chip, list, tab, dock, pins.
- **The colour while naming:** the naming dialog shows the new app's colour dot beside the field, and its key's fingerprint under it.
- **"Which app?"** rows get the colour dot and the Default check box too, the same rows as the list.
- **The dock:** one entry per app (identity), not per address, each signed app at a several-app address marked with its colour dot on its icon's corner. Pins match their app by key. An app's window count counts only its own windows.

### Must NOT Do
- No colour or chip at an address with one app: nothing changes there.
- No words beyond the owner's: "Default", "Rename", "Rename app" (proposed, Q1).
- No moving data between apps (DECISION-064).

## DESIGN
- **Palette:** `Theme.publishers: string[]` (beside `Theme.instances`); `themes.publisher_colour(t, origin)` picks by the first key's first byte modulo the palette's length. Proposed (Q2): dark `#2bb3a3`, `#9a7cf8`, `#5fa8f5`, `#4cc38a`, `#e8c547`; light `#1a8577`, `#6f4fd1`, `#2c6fd1`, `#1f8a5b`, `#a87b00`.
- **`Identity.default: bool`** (`trust/verify.la`), carried through rotations like the name; `remembered.la` trust format 5. `verify.set_default(ring, host, origin, on)` and `verify.rename(ring, host, origin, name)` update the ring (its change count moves, so `saving` saves it).
- **Model:** `chip: bool` (the list open). `naming` gains what it's for: a new separate app (the key change) or a rename (the app's host and first key).
- **Messages:** `Chip(bool)`, `SetDefault(u8[], bool)`, `Rename(u8[])`; renaming reuses `NameSeparate`, `ConfirmSeparate`, `CancelSeparate`.
- **`choosing.asks_which`** opens the default when there is one; `the_app` (loads) does too, so a pin or session with no key opens it.
- **The dot:** `Icon.Dot` recoloured, as the update dot is.
- **Security:** no new input beyond a name (as PRP-049's, checked the same way); the default and the name are saved in the trust file, read as hostile (format 5: `default` a boolean). Trust code changes again (`verify.la`): owner review.

## TESTS
- [x] An app's colour comes from its key: the same key the same colour; never danger or the brand colour; both schemes
- [x] No chip at an address with one app; with two, the chip shows the app's colour and name, first in the box
- [x] The chip opens the list: each app's row with its name, fingerprint, Default and Rename; the current one marked; Escape or a click outside closes it
- [x] A row's name goes to that app's tab, or opens it
- [x] Default: ticking one unticks the other; typing the address then opens it; unticking brings "Which app?" back; it survives a restart
- [x] Rename: the dialog with the current name; the rules; the new name in the chip, tab, dock and pin; survives a restart
- [x] The naming dialog shows the new app's colour dot and fingerprint
- [x] "Which app?" rows show the colour dot and Default
- [x] The dock: two apps at one address, two entries, each with its colour dot and its own window count; a pin matches its own app
- [x] The trust file round-trips the default; format 4 reads with none; a bad one is broken, naming the field

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] For the owner, by eye: with two apps at one address (PRP-049's check), the chip's colour and name, its list, ticking Default then typing the address, renaming one, and the dock's two marked entries, in the light and dark themes

## APPROVAL
- The owner (2026-10-30): the words ("Default", "Rename", "Rename app"); the palette as proposed; Rename on each row of the chip's list.

## AMENDMENTS

- **Test-first slipped:** the window-level tests in `test/several.la` were written after the code they cover (the palette's, the trust file's and `trust/publishers.la`'s came first). All pass; the next PRP goes back to failing tests first.
- **High-contrast themes:** their `publishers` take the same five hues from each theme's own instance colours (`#4fd1c4`, `#c49bff`, `#6fb0ff`, `#4fd19a`, `#ffd24d`; light `#0f6b63`, `#5c2fa0`, `#0b4fa8`, `#0f6b44`, `#6b5000`). The owner approved the dark and light ones.
- **Default and renaming** live in `trust/publishers.la` (`default_at`, `set_default`, `renamed`), not `verify.la`, for its size.
- **A name kept with an app** shows for every app at an address with several, not only a separate one, so a renamed first app keeps its name when reopened.
- **The dot's token:** `APP_DOT` (10) in `ui/tokens.la`, the look `app_dot` in `ui/frame_looks.la`; `docs/DESIGN.md` records them.
- **Test helpers:** the PRP-049 journey's helpers moved from `test/choosing.la` to `test/twoapps.la`, shared with `test/several.la`.
