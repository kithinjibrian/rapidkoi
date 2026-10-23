## FEATURE: The person's controls: ☰ opens Settings, About and Quit; Settings, a built-in app, sets light or dark, more contrast and text size for Rapidkoi and every app; apps can read those three; each app's ⚙ card can force the person's colours and text size on it.

## OBJECTIVE
From chat with the owner (sessions 197–200), part 2 of contract 5 (DECISION-049): "follow by default, force as a last resort", for the controls that have something to act on today.

Done looks like this:
- ☰ opens Settings, About Rapidkoi and Quit.
- Settings opens in its own tab, with Theme `Dark · Light`, More contrast on/off, and Text size `[ − ] 100% [ + ]` from 80% to 200%.
- Each change applies at once to Rapidkoi's pages and to every app built with Rapidkoi's names.
- An app that hard-codes its colours or point sizes can be made readable from its ⚙ card: Use my colours, Use my text size.
- Every choice is remembered across launches.

## WHY (the owner's decisions, recorded)
- **Settings is a built-in app** (MEMORY decision 19: harmless preferences are bundled apps), with a new built-in permission to change settings.
- **More contrast is a switch** on top of light or dark, not a third theme, so there are four colour sets.
- **Text size sets the base size;** Rapidkoi's spacing names follow it. It is not zoom: raw pixels stay, unless the app's switch forces them.
- **80% to 200%** in 10% steps: 200% meets WCAG's bar without breaking Rapidkoi's frame.
- **Apps can read theme, more contrast and text size without permission,** as the web does. They never read colours or anything else.
- **The per-app switch:** Use my colours and Use my text size, off by default, remembered per app.
- **☰ holds Settings, About Rapidkoi and Quit.**
- **Motion is out:** nothing animates yet.

## CONTEXT

- Starting state:
  - One theme, `DARK` in `src/ui/themes.la` (412 lines).
  - ☰ is disabled ("Menu (coming soon)", `src/ui/header.la`).
  - Text sizes are fixed tokens.
  - Contract 5 part 1 (PRP-037) resolves Rapidkoi's names in `src/ui/paint.la`.
  - Built-in apps work like the new tab: `src/window/newtab.la`, `include_bytes`, `Origin.BuiltIn`, built-in permissions in `src/bundles/permissions.la`.
  - Per-app permission settings live in the people file (`src/people/people.la`, format 3).
- Ending state:
  - **`src/ui/schemes.la` (new; a split of `themes.la`, which would pass 500 lines; to approve with this PRP):** the `Theme` struct and its four colour sets, `DARK` (moved unchanged), `LIGHT`, `DARK_CONTRAST`, `LIGHT_CONTRAST`; `theme_for(prefs)`; `contrast(a, b)`, the WCAG ratio. `themes.la` keeps the looks and imports `Theme` from there.
  - **`src/data/preferences.la` (new):** `Prefs { light: bool, more_contrast: bool, text_size: i64 }`, the `preferences` file in the data folder (read as hostile input: each field checked, a broken one set aside with a notice, as the other files), defaults Dark, off, 100.
  - **`Model.prefs`** (`src/window/window.la`), restored at launch and saved when it changes (`src/data/saving.la`). `Model.theme` becomes `theme_for(prefs)`.
  - **Text size:**
    - `Theme.scale` (`text_size / 100`);
    - `paint.resolve` scales Rapidkoi's sizes (`title`, `heading`, the `xs`–`xl` spacing) by it;
    - the page's root sets the body size, so `body`, `small` and `rem` follow;
    - Rapidkoi's own content pages (error screens, loading) use the scaled tokens;
    - the frame (header, address box, strip, dock) keeps its sizes.
  - **The per-app switch:**
    - The people file goes to format 4: `own_colours` and `own_text_size` per app, both off by default.
    - `paint.resolve` takes them:
      - Use my colours turns raw and mixed colours into Rapidkoi's (text → `text`, border and outline → `border`, a pressable box's background → `surface`, any other background → none, shadow off). Named colours stay.
      - Use my text size scales raw `pt` sizes and raw pixel spacing by the person's text size too.
    - The ⚙ card (`src/ui/settings.la`) shows both as switches under Permissions.
  - **Apps' reads (contract 5, open until PRP-039):**
    - A `preferences` ask, open to every app, answered with `{theme: "dark"|"light", more_contrast: bool, text_size: 80–200}`.
    - When one changes, apps get `on_change`, as for pins (PRP-031).
    - A built-in permission `change_settings` (tier BuiltIn) gives a `set_preference` ask, `{name, value}`, unanswered: Rapidkoi saves the change, and `on_change` follows.
  - **The Settings app:** `guest/rust/settings` (contract 5: `tabs`, a switch built from the library's checkbox, `−`/`+` buttons), packed by `scripts/settings.sh` into `assets/apps/settings.rkb`, built in by `src/window/settings_app.la` (key `builtin:settings`) like the new tab.
  - **The Rust library:** `Ask::preferences`, `Prefs::read`, `Ask::set_preference`.
  - **☰:** a `kivuli.ui` popover with Settings (opens the Settings app in a tab, or switches to it), About Rapidkoi (Rapidkoi's dialog: "Rapidkoi" and its version, `RAPIDKOI_VERSION` 0.1.0, as `lugha.toml` says), and Quit (`Cmd.quit()`). New `Msg`s below.
  - **Docs:**
    - `docs/specs/contract-5.md` (the reads);
    - `docs/specs/app-events.md`;
    - DESIGN.md: the four colour sets, text size, the menu, the switch;
    - CLAUDE.md: tree, commands (`scripts/settings.sh`);
    - MEMORY.md (decision 25 amended);
    - CHANGELOG.md.
- Related existing code: the files above; `src/ui/header.la`, `src/ui/addressbox.la` (⚙'s popover), `src/guest/effects.la` (asks behind permissions), `src/window/newtab.la`, `scripts/newtab.sh`, `guest/rust/newtab`; `../kivuli/src/ui/` (popover, menu items), `core/effects.la` (`Cmd.quit`).
- Lugha spec sections relied on: `include_bytes`, structs, `f64` arithmetic. Confirmed while building.
- kivuli API used: `ui.popover`, buttons, `Cmd.quit()`, `Font { size: Size.Scale(…) }` on the page root. All in 0.54.
- Open decisions that must be resolved first: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **Every colour set meets WCAG contrast:**
  - Dark and Light: text on background at least 4.5:1, `text_dim` at least 4.5:1, `on_accent` on accent at least 4.5:1, borders at least 3:1.
  - The contrast sets: at least 7:1 for text and 3:1 or more for borders, with focus rings unmistakable.
  - Tested with `contrast`.
- **A change applies at once,** to every tab, with no reload: `Model.theme` is derived, and every running contract 5 app gets `on_change`.
- **The `preferences` file and the people file's new fields are hostile input:** text size is 80–200 in steps of 10 (anything else is the default); unknown fields are ignored; a broken file is set aside with a notice.
- **`set_preference` only from a page holding `change_settings`,** which only bytes built into `./main` can hold (PRP-030's rule). Bad names or values are refused (the app is stopped, as for any bad ask).
- **The frame keeps its sizes at 200%:** header, address box, strip and dock don't grow.

### Must NOT Do
- Motion or reduced motion (nothing animates).
- Zoom (scaling everything).
- Any read beyond theme, more contrast and text size.
- Following the desktop's light/dark or GTK theme (DECISION-031 stands).
- Link paths, nested state, typed messages (PRP-039).
- Merging into `redesign` or `main`.

## MODEL AND MESSAGES
- `Model.prefs: Prefs`. `Model.theme` is computed from it.
- `Msg.Menu(bool)`: ☰'s card open or closed.
- `Msg.OpenSettings`: open the Settings app in a tab, or switch to it.
- `Msg.About(bool)`: the About dialog shown or closed.
- `Msg.Quit`.
- `Msg.SetPreference(string, Value)`: from the Settings app's ask, by `Cmd.after(0, …)` as `open_pin` does.
- `Msg.OwnLooks(string, bool, bool)`: an app's key, Use my colours, Use my text size, from its ⚙ card.

## ERROR HANDLING REQUIREMENTS
- `PrefsError` (the file): `Broken(why)`, `Newer(format)`, each a notice. Rapidkoi falls back to the defaults and never crashes.
- A bad `set_preference` is a stopped Settings app with its sentence, as any refused ask.

## SECURITY CONSIDERATIONS
- **New built-in permission and asks exposed to guests: owner review (SECURITY RULE).**
  - `preferences` is open to every app. It reveals three coarse values (dark or light, contrast on or off, one of 13 text sizes): a few bits, as the web exposes. Nothing else from settings.
  - `change_settings` (BuiltIn tier) lets only Rapidkoi's own Settings app change those three values. An external app declaring it doesn't run.
- The per-app switch is the person's, set only in Rapidkoi's own ⚙ card (shell-drawn). An app can't read or change it.
- No new dependencies.

## TESTS TO WRITE

`test/schemes.la` (new):
- [ ] Each of the four sets meets its contrast bar for text, dim text, text on accent and borders.
- [ ] `theme_for` picks the right set and scale.

`test/preferences.la` (new):
- [ ] The file round-trips.
- [ ] A text size outside 80–200 or off the 10% steps is the default.
- [ ] A broken file is set aside with a notice.
- [ ] A newer one is refused.

`test/paint.la`:
- [ ] At 150%: `title`, `heading` and `md` scale.
- [ ] Raw `pt` and px don't, unless Use my text size is on.
- [ ] Use my colours maps raw and mixed colours as specified, and leaves named ones.

`test/effects.la`, `test/permissions.la`:
- [ ] `preferences` is answered for any app.
- [ ] `set_preference` needs `change_settings`, and a bad name or value is refused.
- [ ] `change_settings` is BuiltIn.

`test/settings_app.la` (new):
- [ ] The built-in Settings bundle declares `change_settings`.
- [ ] ☰ → Settings opens it once (a second time switches to it).
- [ ] Its tabs and buttons change `Model.prefs`, and running contract 5 apps get `on_change`.

`test/header.la`, `test/menus.la`:
- [ ] ☰ shows Settings, About Rapidkoi and Quit.
- [ ] Quit quits (`Cmd.quit`).
- [ ] About shows the version.

`test/people.la`:
- [ ] Format 4 round-trips the two switches.
- [ ] Format 3 reads with both off.

`test/settings.la` (the ⚙ card):
- [ ] The two switches show and send `OwnLooks`.

`test/saving.la`:
- [ ] Prefs are restored at launch and saved on change.

`guest/rust` (`cargo test`):
- [ ] The Settings app's view (tabs, switch, `−`/`+` limits at 80 and 200) and its asks.
- [ ] `Prefs::read` decodes an answer.

## ROLLBACK PLAN
- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits. This returns to one dark theme, a disabled ☰, and no reads.

## ACCEPTANCE CRITERIA
- [ ] ☰ → Settings: switching Light, More contrast and text size changes Rapidkoi and the styled example at once, and is still so after a restart.
- [ ] At 200% the frame holds its shape.
- [ ] An app with raw colours, with Use my colours on, shows in Rapidkoi's colours.
- [ ] About shows the version; Quit quits.
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines.
- [ ] Docs and CHANGELOG.md updated.

## VALIDATION
- `lughac fmt --check src test`, `lughac test test/`, `lughac build src/main.la -o main`, `cargo test --manifest-path guest/rust/Cargo.toml`
- For the user:
  1. `./main`, ☰ → Settings. Pick Light, then More contrast, then text size 150%: Rapidkoi, the new tab and an open app follow.
  2. Open the styled example, then its ⚙ → Use my colours: its green turns Rapidkoi's.
  3. Restart: everything as you left it.
