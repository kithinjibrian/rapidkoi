## FEATURE: Settings, redesigned: categories down the side, Find, and only what's Rapidkoi-wide

**Status:** approved (the owner, 2026-11-06, with the security notes under DESIGN). Design: the Rapidkoi Settings canvas (https://claude.ai/artifact/GYQFzuoDVE4xvKx6bsZGQv, 2026-11-06). Decision: DECISION-100 (Settings stays a built-in app).

**As built (2026-11-06), where it differs:**
- Text sizes are now 80–200 in steps of 5, not 10: the slider's 125 and 175 weren't valid sizes.
- An app has no record of when you signed in to it, so You's details say First opened (from Your apps), not Signed in since.
- `you` reads the master secret in `update`, as sign-in always has (the data folder's own small file), not in a task.
- Find can't be cleared with Escape: an app can't take keys (DECISION-095). Picking a category clears it.
- The app has five files, not four: `lists.rs` (You, Publishers, the details) keeps `pages.rs` under 500 lines.

## OBJECTIVE

Settings (☰ › Settings, its own tab) becomes a two-column page, as on the canvas:

- **Left:** the title Settings, then the categories, each an icon and a name, the current one on `surface` with its icon in `accent`: Appearance, Startup | You, Publishers | About Rapidkoi (lines between the three groups).
- **Right:** a Find box, the category's icon and name as the page title, then its sections: each a dim heading over one rounded group of rows.
  - **Appearance:** Theme as two picture cards (Dark, Light), the chosen one ringed in `accent`. Reading: More contrast (a switch) and Text size (a slider with eight stops: 80, 90, 100, 110, 125, 150, 175, 200; the value at the row's end; a small and a large "A" at its ends; each stop's number under it, the current one bold).
  - **Startup:** When Rapidkoi opens: Open a new tab, or Restore your last session.
  - **You:** your initial and name; Name apps see (opens a page with first and last name); Signed in to: each app you've signed in to, with when.
  - **Publishers:** Keys you trust: each host, its short fingerprint, and when you first trusted it.
  - **About Rapidkoi:** the version, and the apps' contract.
- **A details column** (300 wide) beside the list on You and Publishers. Picking a row shows its facts there; the first is picked when the page opens:
  - an app you're signed in to: its name and address, the name it sees, your key for it (mono), signed in since;
  - a key: its host, the full fingerprint (mono), the apps it signs, known since, how many times it has rotated.
- **Find** narrows the categories to those whose name or words match; the first match opens; none: "No results". Picking a category clears Find.
- Everything applies at once, as today. Per-app choices stay in ⚙ and App info; Settings shows none of them.

## CONTEXT

- **Read first:** PRP-038 (the person's controls), PRP-067 and PRP-072 (All apps' asks, App info), PRP-035 (sessions), DECISION-027, DECISION-093, DECISION-100; `docs/specs/contract-11.md` (Asks, The person's choices, Roles); `docs/DESIGN.md`.
- **Changes, the app:** `guest/rust/settings/src/` (rewritten; split into modules under 500 lines: `lib.rs`, `pages.rs`, `find.rs`, `tests.rs`), `guest/rust/settings/assets/` (icons), `scripts/settings.sh` (the new permissions), `assets/apps/settings.rkb` (repacked).
- **Changes, the shell:**
  - `src/bundles/permissions.la`: `ReadYou`, `ReadPublishers`, both `Tier.BuiltIn`.
  - `src/guest/nodes.la`, `src/guest/tree.la`, `src/guest/props.la`: the `slider` node.
  - `src/ui/render.la`: draws it with `kivuli.ui`'s `slider` (kivuli 0.68 has it: `../kivuli/src/ui/sliders.la`, since v0.46).
  - A new `src/guest/settingsasks.la`: `you`, `publishers`, `set_name`, read and held to their shapes (as `appasks.la`).
  - `src/guest/effects.la`: answers them.
  - `src/data/preferences.la`: `start` in the preferences file and in `preferences` / `set_preference`.
  - `src/boot/boot.la`: restores the previous session at start when `start` is `restore`.
  - `docs/specs/contract-11.md`, `docs/DESIGN.md`, `CHANGELOG.md`.
- **Data it reads, all existing:** the preferences file; `People.first`/`last` (the name every app shows, `NameScope.Everywhere`), `People.apps` (`signs_in`, `first`), each app's own name (`name_in`); public keys from `people/identity.la` (`public_key`, which needs the master secret, `m.master`); the trust ring (`trust/verify.la`: key, `since`; `trust/index.la`: `rotations`); Your apps for names and icons.
- **rapidkoi-rust first:** the slider, the three asks and `start` are added there and tagged (`v0.5.0-pre.9`); the built-ins move to the tag and are repacked.
- **Decisions:** DECISION-100 (resolved). None open.

## REQUIREMENTS

### Must Do

- **The page, as on the canvas,** in Rapidkoi's names only (`surface`, `text_dim`, `accent`, `on_accent`, `border`; `sm`–`xl`; `medium` and `large` corners): no raw colours or sizes, so it follows the theme, contrast and text size.
  - Content capped at 680; the details column 300, `lg` from it. When the window is too narrow for both, the details go under the list (a wrapping row).
  - Theme cards: two wide, each a small drawing of the frame, page, a text line, a control and the accent, in that theme's own colours (the app's `#rrggbb`: the only raw colours it draws, and they show the other theme, not this one).
  - The switch and the theme and startup choices take roles (`switch` with `checked`; `tab_list` and `tab` with `selected`), so the keyboard and screen readers work them.
- **Icons:** Iconsax, as SVG assets in the bundle, coloured with `color`: `brush-2`, `user`, `info-circle` (new; taken from Iconsax's Outline set), and Rapidkoi's own `refresh-arrow`, `shield-tick`, `arrow-right4`, `search` (copied from `assets/icons/`).
- **Contract 11 additions, no new version (DECISION-100):**
  - `slider`: `min`, `max` (whole numbers, `min < max`, both within ±1,000,000), `value` (clamped), `step` (≥ 1), `label` (1–200 bytes, required), `on_change` (none: disabled), `style`. Moving it sends `on_message` with `on_change` and the new number as `value`. No children.
  - `you` (needs `read_you`): `{ "first", "last", "apps": [{ "name", "place", "icon", "first", "last", "since", "key" }] }`: the name every app shows; each app you're signed in to, oldest first, at most 256, with the name that app sees, when you first signed in (ms), and your public key for it as hex. Answered after the master secret is read, in a task.
  - `publishers` (needs `read_publishers`): `{ "items": [{ "host", "key", "since", "signs": [names], "rotations" }] }`: each trusted key as hex, at most 256, newest first.
  - `set_name` + `first`, `last` (needs `change_settings`): the name every app shows, each part at most 64 characters (the ⚙ Name page's limit); not answered; `on_change` follows, as for preferences.
  - `preferences` gains `"start": "new_tab" | "restore"`; `set_preference` takes `start`.
- **Startup:** `start` is kept in the preferences file (missing: `new_tab`, so today's files read unchanged). With `restore`, a start with a previous session restores it, as the new tab's Restore does; nothing to restore opens the new tab.
- **About:** the version is built into the app: `scripts/settings.sh` reads `version` from `lugha.toml`, builds with it as `RAPIDKOI_VERSION` (read by `env!`), and packs with the same `--version`; the contract number is the SDK's. No ask.
- **Text size:** a saved size between stops (120, 130, …) shows at the nearest stop below and keeps its number until the slider moves; moving it saves the stop.
- **Find:** case-insensitive, on each category's name and its words (theme, dark, light, contrast, text size; open, restore, session; name, sign in, key; keys, trust, signed; version). Escape clears it.
- **Every function used outside its module tested** (CLAUDE.md), the asks' every error included.

### Must NOT Do

- **No per-app settings in Settings** (permissions, servers, looks, storage): they stay in ⚙ and App info, one place each (the owner, 2026-11-06).
- **No "match the system" theme:** Rapidkoi ignores the desktop's (DECISION-031).
- **No actions on You or Publishers** beyond the name: no sign out, no forgetting a key, no copying a key. Each would be its own PRP and security review.
- **No secrets to the guest:** `you` gives public keys only; the master secret never leaves the shell.
- **No new contract version,** and no slider marks: kivuli's slider has none, and the stops' numbers are the app's own texts under it.
- **Nothing for apps that aren't built in:** `read_you`, `read_publishers` and `set_name` refuse any other app, as `change_settings` does.

## DESIGN

- **Why an app:** DECISION-100. The shell's part is answering asks and drawing one new node.
- **Messages (the app):** `Go(Page)`, `Find(String)`, `ClearFind`, `Theme(bool)`, `Contrast`, `TextSize(i64 stop index)`, `Start(Start)`, `PickSignedIn(usize)`, `PickKey(usize)`, `EditName`, `First(String)`, `Last(String)`, `Back`. State: the page, Find's text, the preferences, the `you` and `publishers` answers (re-asked on `on_open`, `on_resume`, `on_change`), the two picks, and whether the name page is open.
- **The shell's new errors** (`ContractError`, each stopping the app with its sentence, as other bad asks and nodes do):
  - a slider with no `label`, `min ≥ max`, a number out of range, or `step < 1`: "This app drew a slider Rapidkoi can't show";
  - `set_name` with a part over 64 characters or not text: "This app sent a name Rapidkoi can't keep";
  - `you`, `publishers` or `set_name` without its permission: as today's built-in asks.
- **Reading keys:** `public_key` needs the master secret. `you` reads it in a `Cmd.task` (as a sign-in does), then answers; if it can't be read, the apps come back without `key` and the details column shows Your key for it as "Not available".
- **Security, for the owner's review before building:** three new host calls a guest can make (`you`, `publishers`, `set_name`), each built-in only; the master secret read for `you`; names and keys handed to one built-in guest. Every answer is capped (256 items, the name's 64 characters), and nothing from the guest becomes a path, SQL or command.

## TESTS

- [ ] The app (`cargo test`): each category opens its page; the current one is marked; Find narrows (case-insensitive, by words), opens the first match, shows "No results", and clears on a pick and on Escape.
- [ ] The app: the theme cards send `set_preference` `theme`; the switch `more_contrast`; the slider each stop's size; a saved 120 shows at 110 and still reads 120%; the startup choice sends `start`.
- [ ] The app: You lists the `you` answer's apps, the first picked, its details shown; picking another shows its; no apps: the section isn't drawn and nor is the details column. The same for Publishers.
- [ ] The app: the name page sends `set_name` with both parts as typed.
- [ ] `test/slider.la`: a good slider draws and sends its number; each bad prop stops the app with its sentence; a value outside the range is clamped; no `on_change` draws it disabled (`kivuli.testing`).
- [ ] `test/settingsasks.la`: each ask's shape, and each error: wrong types, a 65-character name part, missing permission.
- [ ] `test/preferences.la`: `start` saved and read; a file without it reads as `new_tab`; a bad value is refused as other fields are.
- [ ] `test/boot.la`: `restore` with a previous session restores it; with none, the new tab; `new_tab` never restores.
- [ ] `test/effects.la`: `you` answers names, first sign-in and keys for signed-in apps only, oldest first, capped at 256; `publishers` answers the ring with `since`, apps and rotations.

## ACCEPTANCE

- [ ] Settings looks and works as the canvas: five categories, Find, the details column on You and Publishers.
- [ ] Theme, contrast, text size and startup apply at once and survive a restart.
- [ ] No other app can use `read_you`, `read_publishers` or `set_name`.
- [ ] rapidkoi-rust tagged with the additions; `cargo test --manifest-path guest/rust/Cargo.toml` passes; the built-ins repacked.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: open ☰ › Settings in Dark and Light, with and without More contrast; drag the slider through every stop; type "key" in Find; pick each app under You and each key under Publishers, then narrow the window until the details drop under the list; set Startup to Restore, quit, start again.
