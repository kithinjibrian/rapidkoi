# DECISIONS.md — rapidkoi

Tracks architectural and design questions that are open, deferred, or resolved.

Rules:
- Every open decision blocks implementation of the code it affects.
- The AI must not implement anything that depends on an open decision.
- When a decision is resolved, move it to the RESOLVED section and record the outcome.
- Once resolved, copy the outcome to MEMORY.md as an architectural decision.

---

## OPEN — Requires human input before implementation

(none)

---

## DEFERRED — Acknowledged, not yet needed

### DECISION-038 — Shipping Rapidkoi's own apps inside the binary needs Lugha's include_bytes

**Status:** deferred (requested from Lugha: `docs/specs/lugha-include-bytes.md`)
**Raised:** 2026-10-22 — Session 157 (the owner chose to embed the bundled new tab in `./main`, over a resources folder beside it)

**Question:** The new tab is a Rust app packed into a signed bundle. Lugha can't read a file into a program at compile time, so the bundle can't be embedded without pasting its bytes into source.

**Meanwhile:** the new-tab app, its packing and signing, and the build script can be made; Rapidkoi doesn't run it until Lugha has `include_bytes`. Not a silent workaround.

**Revisit when:** Lugha ships `include_bytes`/`include_str`; then embed `newtab.rkb` (and move `ui/icons.la` to `include_str`).

---

### DECISION-037 — Chrome's + and scrolling an app into view need kivuli

**Status:** resolved — Session 161: kivuli 0.53 `scroll_fit` and 0.54 `Cmd.scroll_to`; + follows the last tab and the current tab scrolls into view (PRP-028).
**Was:** deferred (requested from kivuli: `docs/specs/kivuli-scroll-fit-and-scroll-to.md`)
**Raised:** 2026-10-22 — Session 148 (the owner: "i want chrome's style and also scroll into view")

**Question:** The sidebar's + should sit right after the last app until the apps overflow, then stay at the bottom, and opening or selecting an app should scroll the sidebar to it. kivuli 0.51's `scroll` always takes all its room (no natural height, GTK's `propagate-natural-height`) and has no scroll-to command (GTK's `gtk_viewport_scroll_to`); `Cmd.focus` scrolls but steals focus.

**Meanwhile:** the apps scroll, + fixed at the bottom of the strip; nothing scrolls an app into view. Not a silent workaround.

**Revisit when:** kivuli ships a fitting scroll and `Cmd.scroll_to`; then + follows the last app and select/open scroll to it.

---

### DECISION-036 — A × on any hovered app needs a parent hover (kivuli)

**Status:** resolved — Session 161: kivuli 0.52 group states; every open tab shows its × while hovered (PRP-028).
**Was:** deferred (requested from kivuli: `docs/specs/kivuli-group-hover.md`)
**Raised:** 2026-10-22 — Session 145 (the owner: "add a x when one hovers on any app")

**Question:** kivuli 0.51 styles a node only by its own state, so a × inside an app's sidebar button can't appear while the button is hovered. kivuli needs a group hover (`.group()` and `State.GroupHover`, as Tailwind's `group-hover`; GTK's CSS has `.parent:hover .child`).

**Meanwhile:** the × shows on the active app only; right-click Close and Ctrl+W close any (PRP-023). Not a silent workaround: once kivuli has group hover, the × goes on every app, shown on hover.

**Revisit when:** kivuli ships group hover; bump and move the ×.

---

### DECISION-034 — A picture's `.size(n)` doesn't cap it (kivuli)

**Status:** deferred (reported; Rapidkoi uses kivuli's documented exact `Sizing` meanwhile)
**Raised:** 2026-10-21 — Session 104 (the owner's screenshot: the header's icons far bigger than `HEADER_ICON`)

**Question:** kivuli 0.48's README says `.size(n)` on a picture is "an n × n box", but `render/styling.la` applies it as GTK's size request, a minimum, so an SVG (the Iconsax icons) draws at its own larger size. Should kivuli make `.size(n)` a box (a clamp, as an exact `Sizing` gets)?

**Meanwhile:** Rapidkoi sizes its icons with `tokens.square(n)`, an exact `Sizing` with a maximum, which kivuli documents as clamping. Not a silent workaround: once kivuli fixes `.size`, `square` can go.

**Revisit when:** kivuli's owner looks at it, in `../kivuli`.

---

### DECISION-002 — Publishing

**Status:** deferred
**Raised:** 2026-10-17 — Session 1
**Revisit when:** The user wants a remote, or a first release.

**Question:** Where is the repository hosted (e.g. `kithinjibrian/rapidkoi` on GitHub), public or private, and under what license?

**Notes:** Safe to defer: work is local-only until then, and no push happens without asking.

---

### DECISION-007 — Platforms beyond GTK desktop

**Status:** deferred
**Raised:** 2026-10-17 — Session 2
**Revisit when:** Work on Android, macOS or Windows starts.

**Question:** kivuli renders to GTK 4 only. How do Android (and native macOS/Windows) get a renderer?

**Notes:** Safe to defer: the vision's build path is desktop first, and Linux is the development machine. Keeping the guest contract (DECISION-005) independent of GTK keeps this open.

---

### DECISION-008 — Process isolation for apps

**Status:** deferred
**Raised:** 2026-10-17 — Session 2
**Revisit when:** Before the shell runs any bundle it didn't load from a local file the user chose (build path step 2).

**Question:** The vision says each app runs separately from Rapidkoi itself. In-process sandbox only, or a separate process per app?

**Notes:** In step 1 the person only opens local bundles they picked, and the runtime's sandbox plus fuel and memory limits contain a faulty one.

---

### DECISION-009 — Manifest, bundle format and signing

**Status:** deferred: design settled (2026-10-18), building waits on `std.crypto`, `std.toml` and DECISION-021
**Raised:** 2026-10-17 — Session 2
**Revisit when:** `std.crypto` and `std.toml` land in Lugha, and background loading exists.

**Question:** How bundles are published, found, verified and updated.

**Settled** (`docs/source/meetings/2026-10-18-packaging.md`, `docs/source/meetings/2026-10-18-signing-and-panel.md`):
- one app per host; a key file over HTTPS, remembered on first use; fingerprints in shared links later
- a signed manifest per build, and a signed index with a rising sequence
- static files only: `key`, `index`, `builds/<sha256>.rkb`
- signed rotations; an unapproved change makes a separate app (identity = host + key)
- sign bytes, not shapes; Ed25519 and SHA-256 via OpenSSL (`std.crypto`)
- `rapidkoi.toml` (`std.toml`) with keygen, pack, publish, serve and rotate
- markers, refusal without bypass, automatic updates with Undo

**Later:** index expiry (needs a clock), revocation, DNS as an optional extra.

---

### DECISION-011 — How synced app data resolves conflicts

**Status:** deferred
**Raised:** 2026-10-17 — Session 3
**Revisit when:** Build path step 4 (storage and sync).

**Question:** When two devices or two people edit the same data, what wins: last write per row, a change log with merge rules per table, or CRDTs on SQLite (e.g. cr-sqlite)?

**Notes:** Safe to defer: Rapidkoi owns each app's database (DECISION-004), so any of the three stays possible. See `docs/source/meetings/2026-10-17-architecture.md`.

---

### DECISION-014 — Running two apps at once (kivuli single-instance)

**Status:** deferred
**Raised:** 2026-10-17 — Session 5
**Revisit when:** The shell runs more than one app, or a second launch must open a second app.

**Question:** kivuli starts GTK with `gtk_application_new(id, 0)` (`../kivuli/src/live.la:74`), so GTK makes the program single-instance: a second `./main other.wasm` raises the first window and drops the new app. Should kivuli accept application flags (e.g. `G_APPLICATION_NON_UNIQUE`, or `HANDLES_OPEN` for links later)?

**Notes:** A kivuli gap, fixed upstream when needed. Step 1 runs one app, so it's a known limitation. Build path step 2 (links) will need `HANDLES_OPEN`-style behaviour anyway.

---

### DECISION-015 — A file-size query in std.fs (lughac)

**Status:** deferred
**Raised:** 2026-10-17 — Session 5
**Revisit when:** Opening apps that aren't local files the person chose (step 2), or the next lughac std work.

**Question:** `std.fs` has no size query, so the shell's 32 MB module cap is checked after `read_bytes` has read the whole file. Add `fs.size(path)` (or `metadata`) to lughac's std?

**Notes:** A lughac gap, reported upstream. Acceptable for step 1: the file is local and chosen by the person.

---

### DECISION-017 — Listing a module's exports (lugha-wasmtime)

**Status:** deferred
**Raised:** 2026-10-17 — Session 6
**Revisit when:** The next lugha-wasmtime PRP, or before build path step 2.

**Question:** lugha-wasmtime can't list a module's or instance's exports and their types, so the shell can't check `alloc`, `init`, `update` and `view` before calling them (PRP-001 asked for that). Add export listing (`wasmtime_module_exports` / `wasmtime_instance_export_nth`) upstream?

**Notes:** Safe meanwhile: a missing or mistyped export fails its first call with `MissingExport`/`WrongExport` and the error screen. `memory` and `rapidkoi_version` are still checked at load. Just later than ideal: a missing `update` shows on the first click.

---

### DECISION-019 — Restoring an app's exact state on Continue

**Status:** deferred
**Raised:** 2026-10-17 — Session 8
**Revisit when:** PRP-003 (remembering).

**Question:** Rapidkoi holds each app's state, so "Continue" could reopen an app exactly as it was. Within a session that's free: the bytes are in memory. Across restarts, the state is saved to disk. How big may it be, is it encrypted, and how is it forgotten?

**Notes:** PRP-002 restores state within a session only. See `docs/source/meetings/2026-10-17-ui-design.md`.

---

### DECISION-020 — What kivuli needs for Rapidkoi's frame

**Status:** deferred (part done; Rapidkoi uses placeholders for the rest, DECISION-018)
**Raised:** 2026-10-17 — Session 10
**Revisit when:** Each kivuli release. Swap the matching placeholder out.

**Question:** Which kivuli additions replace PRP-002's placeholders?

**Notes:**
- **Done in kivuli 0.2.0, used since session 11:**
  - typed looks and themes (`.style`, `.theme`), which replaced the stylesheet idea
  - `.maximized()`
  - `.header(start, title, end)`: the bar lives in the title bar
  - kivuli 0.3.0: `.header_style([…])` and `Surface.flat`. The header bar takes the flat frame look, with no shadow (session 13)
  - kivuli 0.4.0: per-corner radius. The page curves only at its top right (session 16)
  - kivuli 0.4.1: button content centered (a bug found in session 17). Tabs and tiles center their letter (session 18)
  - kivuli 0.5.0: images from bytes. A bundle's icon shows on its tab and home tile (session 21)
- **Not needed:** forcing dark (kivuli's DECISION-017). kivuli apps follow the desktop's dark preference already.
  - kivuli 0.7–0.10: a wrapping grid, an overlay, context menus and `button_with`. Home's grid, the panel over the app, tab and tile menus, and clickable panel rows (PRP-010, session 48)
- **Still to come:**
  1. a file dialog
  4. a wrapping grid, for home's tiles

---

### DECISION-022 — Accounts: per-app identity and sign-in

**Status:** deferred (keys, auto sign-in and names built in PRP-011; recovery, sync and apps reaching their server still to come)
**Raised:** 2026-10-18 — Session 26
**Revisit when:** The owner wants sign-in (the vision's passkeys held by Rapidkoi).

**Question:** How Rapidkoi gives each app its own user ID for the person (so apps can't link them across apps), a name the person chooses per app, and sign-in and out. Where keys live, how IDs are derived, and what an app's server can verify.

**Notes:** Authentication: a design talk of its own and the owner's security review (SECURITY RULE). Until then the panel's Account row says "Not signed in".

---

### DECISION-023 — Reading a file's mode back (std.fs)

**Status:** deferred
**Raised:** 2026-10-18 — Session 35
**Revisit when:** std.fs gains a way to read a mode (a `stat`), or a check of a key file's mode is needed at run time.

**Question:** keygen now writes the secret key at 0600 (`fs.set_mode`, m115), but no Lugha test can read the mode back: std.fs has `set_mode` and no getter, and running `stat` would be process execution. Should std.fs get `mode(path)`, and should `publish` then refuse a key file that others can read (as ssh does)?

**Notes:** Covered for now by lugha's own m115 test (it reads 0600 back) and by hand (`stat` showed 600, session 35). A small std addition when the owner wants it.

---

## RESOLVED

### DECISION-035 — The window controls (kivuli)

**Status:** resolved
**Raised:** 2026-10-21 — Session 105 (the owner: "the windows controls are fucking ugly. improve them")
**Resolved:** 2026-10-21 — Session 109 (kivuli 0.49 shipped it)

**Question:** GTK draws − □ × at the header bar's end, small and in the desktop's look. kivuli 0.48 keeps them GTK's: nothing styles them, nothing hides them, and there's no command to minimise or maximise, so Rapidkoi can't draw its own. How does kivuli gain it?

**Options:**
- A) Style GTK's own controls: `Inside(Part.WindowButton, [...])` (and their icons) in `header_bar(…).style([...])`, so Rapidkoi sets their size, colours and hover. Keeps GTK's behaviour (the desktop's button layout, double-click, snapping). Smallest change.
- B) Hide GTK's controls (`header_bar(…).controls(false)`) and add `Cmd.minimize()`, `Cmd.toggle_maximized()` (`Cmd.quit()` exists), so Rapidkoi draws its own with Iconsax icons, like Chrome. Full control; Rapidkoi then owns their placement on every desktop.

**Until then:** GTK's controls as they are (CLAUDE.md known issues). The fix is a kivuli change in `../kivuli` under its own CLAUDE.md, then a kivuli bump here.

**Outcome:** A. kivuli 0.49 styles GTK's own controls (`Part.WindowButton`, `Part.WindowClose`, `Part.WindowIcon`); Rapidkoi's header bar takes `kivuli.ui`'s `WindowControls.look()`: ghost buttons in the palette, muted until hovered, close red on hover. GTK keeps which show, their order and what they do.

**Copied to MEMORY.md:** no (a look)

---

### DECISION-033 — Tests can't sit in folders

**Status:** resolved
**Raised:** 2026-10-20 — Session 92 (PRP-018)
**Resolved:** 2026-10-20 — Session 93 (the owner approved PRP-018 with A)

**Question:** A test file's imports start from its own folder, so `test/ui/layout.la` can't import `test/support.la` (E0312). Do tests mirror `src/`'s new folders?

**Options:**
- A) `test/` stays flat, each file named for the module it tests. Recommended.
- B) A copy of `support.la` in each test folder.
- C) A Lugha change letting a test import from `test/`'s top (in `../lugha`).

**Outcome:** A. `test/` stays flat; each test file is named for the module it tests.

**Copied to MEMORY.md:** no (CLAUDE.md's TESTING RULE says it)

---

### DECISION-029 — Clicking away from the address box doesn't take focus away (kivuli)

**Status:** resolved
**Raised:** 2026-10-19 — Session 81 (the owner, trying PRP-015)
**Resolved:** 2026-10-20 — Session 89 (the owner, on kivuli 0.18: "if i click somewhere else the focus does go away")

**Question:** Clicking the page or the frame leaves the cursor in the address box. GTK only moves focus to something focusable, and kivuli 0.10 has no way to clear the window's focus, no event when a field loses focus, and no click on empty space. Only while the suggestions card is open does a click outside reach Rapidkoi (the clear layer). How does kivuli gain it?

**Options:**
- A) kivuli adds two general pieces: `.on_blur(msg)` on text fields (the message sent when focus leaves the field), and a click on space no widget takes clearing the window's focus (the way browsers and most GTK apps behave). Rapidkoi then sends `Dismiss` on blur: the box empties, the card closes, an app page shows its address again, and the clear layer can go. Recommended: matches every browser.
- B) kivuli adds only `Cmd.unfocus()` (clear the window's focus); Rapidkoi calls it on `Dismiss`. Fixes clicks while the card is open, but not a click on the page with an empty box, since nothing reports that click.

**Until then:** focus stays in the box until something else focusable is clicked or Tab moves it. The fix is a kivuli change in `../kivuli` under its own CLAUDE.md, then a kivuli bump here.

**Outcome:** Doesn't happen on kivuli 0.18 with Rapidkoi's own styles (PRP-017): clicking elsewhere takes focus from the box. No kivuli change; reopen if it comes back.

---

### DECISION-031 — Colours: Rapidkoi's theme for everything, no desktop theme

**Status:** resolved
**Raised:** 2026-10-20 — Session 84
**Resolved:** 2026-10-20 — Sessions 86 and 87 (the owner, in chat: "buttons should just be styled like the rest not following desktop themes"; "everything follows rapidkoi throw away the notion of desktop")

**Question:** kivuli 0.17 has no button looks (and won't). Where do colours come from?

**Outcome:**
- Everything Rapidkoi shows follows Rapidkoi's theme (`src/themes.la`): the frame, text, fields, buttons, and apps' content. The desktop's GTK theme is ignored, light or dark.
- Buttons are styled through `.style` like everything else. Contract 2's `Normal`, `Primary`, `Danger` and `Subtle` stay: the app names the kind, Rapidkoi's theme decides the look. Apps still never set colours.
- The header bar takes the frame's colour through `header_bar(…).style([...])` (kivuli 0.17).

**Rationale:** One look Rapidkoi controls: the page is always Rapidkoi's dark, so its text and controls must be too.

**Copied to MEMORY.md:** yes (decision 20)

---

### DECISION-030 — Moving to kivuli 0.17: port, or rewrite the UI

**Status:** resolved
**Raised:** 2026-10-20 — Session 84 (the owner: "kivuli has pushed very breaking changes. i need us to rewrite")
**Resolved:** 2026-10-20 — Session 85 (the owner, in chat)

**Question:** kivuli 0.11–0.17 replaced every styling modifier with typed values in `.style([...])`: `.bold`, `.dim`, `.size` on text, `.padding`, `.spacing`, `.width`, `.height`, `.button_style`/`ButtonStyle`, named looks (`Surface`, `theme`, `.theme`, `.header_style`). Rapidkoi uses them in 15 of its modules, `render.la` (guests' screens) included. What does the rewrite cover?

**Options:**
- A) Rewrite the UI for the redesign only. Port what the new frame draws (`layout`, `addressbox`, `content`, `render`, `theme`/`themes` as style functions), and delete the old view (`legacy`, `frame`'s bar and rail, `home`, `panel`'s, `account`'s and `access`'s views) with the tests that drive it. The model, `update` and everything under it stay; the behaviour those tests guarded gets tests that drive `update` directly. Recommended: the redesign replaces the old view anyway, and porting it is work thrown away.
- B) Port everything as it is, old view included: the most coverage kept, the most throwaway work.
- C) Rewrite all of Rapidkoi, not just its UI.

**Outcome:** A. Rewrite the UI for the redesign on kivuli 0.17: port what the new frame draws, delete the old view and the tests that drive it, and give the behaviour they guarded tests that drive `update` directly. The model and logic stay.

**Rationale:** The redesign replaces the old view; porting it would be thrown away.

**Copied to MEMORY.md:** yes (decision 19, amended)

---

### DECISION-028 — The address box's focus ring on the whole pill (kivuli)

**Status:** resolved
**Raised:** 2026-10-19 — Session 79
**Resolved:** 2026-10-19 — Session 80 (the owner, in chat)

**Question:** Focus showed on the field inside the pill, and kivuli can't style focus (an outline, `:focus-within`).

**Outcome:** No kivuli change. The pill is scrapped: the address box is the field itself, rounded (`Look.Address`: radius half its height), with no box around it and no → button (Enter goes). GTK's focus ring then goes round the whole box.

**Rationale:** The owner's call: the simplest shape that gets the focus right.

**Copied to MEMORY.md:** no (a look, not architecture)

---

### DECISION-027 — The shell draws the frame; its pages become bundled apps

**Status:** resolved
**Raised:** 2026-10-19 — Session 70
**Resolved:** 2026-10-19 — Session 71 (the owner, in chat)

**Question:** Should the shell's own pages (new tab, search, notifications) be drawn by the shell, or be apps bundled with it?

**Outcome:**
- The shell draws the frame: the bar, the tab rail, the address box and the error screen.
- Screens that grant, approve or verify stay native in the frame: app info (ⓘ), key-change approval, sign-in approval, Access, Account. Test: if a hostile app drew a perfect copy, could it trick the person into giving something away or believing something false?
- Content pages (new tab/home, search, notifications, harmless preferences) become apps bundled with the shell, the new tab first. Data is undecided (deleting needs a native confirmation).
- The redesign lives on the long-lived branch `redesign`; each PRP's branch merges into it, and `redesign` merges into `main` only when the whole redesign is built. Regressions on `redesign` meanwhile are expected.

**Rationale:** Bundled apps use the same platform outside developers get, and keep the trusted core small. Anything any app can draw can be faked, so trust screens stay where guests can't paint.

**Copied to MEMORY.md:** yes (decision 19)

---

### DECISION-026 — A declared server name that resolves to this computer or the local network (std.http)

**Status:** resolved (the design; building it waits for std: PRP-013)
**Raised:** 2026-10-19 — Session 63 (building PRP-012)
**Resolved:** 2026-10-19 — Session 66 (the owner, in chat: `docs/source/meetings/2026-10-19-local-addresses.md`)

**Question:** std looks a name up and connects wherever it points, so a signed app declaring `evil.example` pointed at `127.0.0.1` or a router had Rapidkoi connect there, guarded only by the certificate check.

**Outcome:** Option B, as A of the session 66 talk:
- **std gains** `net.resolve(name)` and an option to connect TLS/HTTP to one given address while the name is still used for `Host`, SNI and the certificate.
- **Rapidkoi** resolves, refuses the name whole when any address is local (this computer, private, link-local, CGNAT, multicast, broadcast, reserved; IPv4 inside IPv6 as IPv4), and connects to exactly the addresses it checked.
- **Where:** app fetches and opening apps at an address.
- **Until std has it:** the certificate check stays the only guard (CLAUDE.md known issues).

**Copied to MEMORY.md:** yes (decision 18)

---

### DECISION-025 — UI contract 2: one entry point, effects and name placeholders

**Status:** resolved
**Raised:** 2026-10-18 — Session 55 (designing identity)
**Resolved:** 2026-10-18 — Session 56 (PRP-011, the owner approved)

**Question:** How does an app ask Rapidkoi for things (its key, a sign-in signature), and show the person's name without learning it, without host functions?

**Outcome:** Contract 2, replacing v1 (amendment 1: the owner, "you can do breaking changes nothing has shipped"): one export `handle(state, event)` for every event (`init`, `opened`, `message`, `answer`) and `view`; the `identity` and `sign` effects from any result, answered as answer events; `{FIRST_NAME}`/`{LAST_NAME}` filled while drawing. Detail: the contract 2 appendix of `docs/source/meetings/2026-10-17-ui-contract.md`.

**Copied to MEMORY.md:** yes (decisions 10 and 17)

---

### DECISION-024 — One HTTPS request with both limits and an extra CA (std.http)

**Status:** resolved
**Raised:** 2026-10-18 — Session 43 (writing PRP-009)
**Resolved:** 2026-10-18 — Session 44 (Lugha milestone 119 added `request_tls_with`)

**Outcome:** Option A. The app and the tests make the same call, `request_tls_with(…, limits, opts)`; the app's `opts` never carries a CA file.

**Question:** Remote apps fetch with `std.http.request_with(…, limits)` (32 MB builds, 30 s, the system's roots). The tests must add a test CA, which only `request_tls(…, opts)` takes, and it has no `Limits` (a 16 MB default cap). The tests would then exercise a different call from the app's.

**Options:**
- A) A small std addition, `request_tls_with(method, url, headers, body, limits, opts)`: both through one call. Recommended; a prompt for the Lugha session was given to the owner.
- B) Build PRP-009 with two calls (the app's `request_with`, the tests' `request_tls`), noting that the tests don't cover the app's exact call and that test bundles must stay under 16 MB.

**Notes:** Not worked around: PRP-009 waits on A or the owner's B.

---

### DECISION-021 — Loading in the background

**Status:** resolved
**Raised:** 2026-10-18 — Session 22
**Resolved:** 2026-10-18 — Session 41 (PRP-008, the owner approved)

**Outcome:** Option A: kivuli 0.6's `Cmd.task`. A load's task fetches and checks against a snapshot of trust; the window records trust and compiles on arrival. A DNS lookup, TLS handshake and compiling still briefly hold the thread (a Lugha runtime matter, kivuli's DECISION-022).

**Question:** kivuli runs GTK's main loop, and a `std.http` request made from `update` waits on that thread, so the window pauses for the download. How does Rapidkoi load in the background and get a message when it's done?

**Options (to weigh then):**
- A) A kivuli command that runs a function off the main loop (a green thread or an OS thread) and sends its result as a message.
- B) An integration between Lugha's scheduler and GLib's main loop, so green threads run while GTK waits.

**Notes:** PRP-004 fetches synchronously from localhost only, bounded by `MAX_BUNDLE` and a 3-second timeout.

---

### DECISION-018 — Placeholders while kivuli catches up

**Status:** resolved
**Raised:** 2026-10-17 — Session 8
**Resolved:** 2026-10-17 — Session 8 (the owner: "placeholders where you can, don't get blocked")

**Question:** PRP-002 needs kivuli features that don't exist yet: a file dialog, an overlay, right-click menus, a wrapping grid, a disabled text field. Wait, or build around them?

**Outcome:** Build with **explicit placeholders**. Each one is recorded in PRP-002's placeholder table and in KNOWN ISSUES, and swapped out when kivuli ships the feature:
- an "Open a file" path field instead of a file dialog
- the info panel as a column beside the app instead of over it
- tab actions in the info panel only, without right-click
- fixed rows of tiles instead of a wrapping grid
- inputs without `on_change` still refused (DECISION-016)

**Rationale:** It keeps momentum without silent workarounds: every placeholder is visible, named and temporary. The real features are fixed upstream (MEMORY.md decision 9).

**Copied to MEMORY.md:** yes

---

### DECISION-016 — A read-only text input (kivuli)

**Status:** resolved
**Raised:** 2026-10-17 — Session 6
**Resolved:** 2026-10-17 — Session 8 (the owner chose A)

**Question:** How should the shell show a contract `input` with no `on_change`?

**Outcome:** A: kivuli gains `.disabled(true)` for text fields (its PRP-006, prompt given to the owner). Until kivuli 0.1.1 is tagged, `contract.read_tree` keeps refusing such inputs with a visible error.

**Rationale:** It matches "no handler means disabled" everywhere, with no contract change.

**Shipped:** kivuli 0.1.1; Rapidkoi renders such inputs disabled since session 11.

**Copied to MEMORY.md:** yes

---

### DECISION-012 — UI contract v1

**Status:** resolved
**Raised:** 2026-10-17 — Session 4
**Resolved:** 2026-10-17 — Session 4

**Question:** What exactly passes between a guest and the shell?

**Outcome:** CBOR. Nodes are `{kind, props, children?, key?}`; step 1 kinds are text, button, input, column, row. Props are semantic only (no pixels or colours). Unknown props are ignored; unknown kinds render a placeholder. Messages are opaque app-chosen CBOR values, returned in an envelope `{msg, value?}`. No handler means disabled. The guest exports `rapidkoi_version() → 1`, `init`, `update(state, event) → {state, effects: []}` and `view(state)`; the shell holds the state bytes. Limits: fuel ≈100 ms per call, 256 MB memory, view 4 MB / 10,000 nodes / depth 64, strings 64 KB, state 16 MB. A failing app stops alone, behind a Restart screen. Full detail: `docs/source/meetings/2026-10-17-ui-contract.md`.

**Rationale:** Language-neutral, inspectable, one crossing per call; semantic props let the person's settings apply to every app; reserved `key` and `effects` avoid breaking changes later.

**Copied to MEMORY.md:** yes

---

### DECISION-013 — lugha-cbor

**Status:** resolved
**Raised:** 2026-10-17 — Session 4
**Resolved:** 2026-10-17 — Session 4

**Question:** Where does the CBOR code live?

**Outcome:** A standalone, pure-Lugha package `../lugha-cbor` (package `cbor`), built before the shell. Its decoder is safe on hostile input.

**Rationale:** MEMORY.md decision 9: generic pieces are their own packages.

**Copied to MEMORY.md:** yes

---

### DECISION-003 — Which WebAssembly runtime the shell embeds

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** Which C-API WebAssembly runtime does the shell embed?

**Outcome:** wasmtime, through its C API, wrapped by the separate Lugha package `lugha-wasmtime` (DECISION-010).

**Rationale:** The most mature and hardened option, with fuel/epoch and memory limits for untrusted code. WAMR is the fallback to reconsider for Android (DECISION-007).

**Copied to MEMORY.md:** yes

---

### DECISION-004 — Where an app's state lives

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** Does the guest own its state, or does the platform hold it?

**Outcome:** Two kinds of state. Screen state (route, filters, drafts, selection) is a serialisable model Rapidkoi holds; apps are Elm-shaped (`init`, `update`, `view`). App data lives in a per-app SQLite database that Rapidkoi owns and provides (from build path step 3 or 4); queries and writes are effects, with live-query subscriptions. Apps don't bundle their own database. Conflict handling is DECISION-011.

**Rationale:** Platform-held screen state gives links, history and live sessions to every app. A real database is needed for offline apps (the owner's point), and Rapidkoi owning it keeps sync, export, inspection and removal possible. SQLite fits many small reads and writes; DuckDB is for analytics.

**Copied to MEMORY.md:** yes

---

### DECISION-005 — The guest↔host UI contract

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** How does a bundle hand its view tree to the shell, and how do events get back?

**Outcome:** The guest writes its whole view tree into its memory as one compact, versioned encoding per update; the shell decodes it into kivuli `View`s, and kivuli diffs. Events go back to the guest as its own encoded messages. Step 1 components: text, button, text input, column, row. The exact encoding is designed in the step 1 PRP and stays language-neutral.

**Rationale:** One crossing per update, simple, inspectable, and independent of any guest language.

**Copied to MEMORY.md:** yes

---

### DECISION-006 — The first guest language and test bundles

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** What language is the first guest UI library and demo bundle written in?

**Outcome:** Rust (`wasm32-unknown-unknown`), in this repo under `guest/rust/`. The shell's own tests use hand-written `.wat` fixtures, so `lughac test` never needs Rust. A lughac WebAssembly target and a Lugha guest library are a later milestone in the lugha repository.

**Rationale:** Rust is the best-supported WebAssembly language and already installed; lughac emits native code only today.

**Copied to MEMORY.md:** yes

---

### DECISION-010 — Rapidkoi grows Lugha's ecosystem

**Status:** resolved
**Raised:** 2026-10-17 — Session 3
**Resolved:** 2026-10-17 — Session 3

**Question:** Where do general-purpose pieces built for Rapidkoi live?

**Outcome:** A second project goal is improving Lugha's ecosystem. Generic pieces (runtime bindings, database bindings) are standalone Lugha packages in their own repositories, starting with `lugha-wasmtime` (package `wasmtime`) from day one. Rapidkoi-specific pieces (link parser, UI encoding) start in this repo and split out once their shape settles. kivuli and lughac gaps are fixed upstream.

**Rationale:** The owner's direction (`docs/source/meetings/2026-10-17-architecture.md`).

**Copied to MEMORY.md:** yes

---

### DECISION-001 — What rapidkoi is for

**Status:** resolved
**Raised:** 2026-10-17 — Session 1
**Resolved:** 2026-10-17 — Session 2

**Question:** What does rapidkoi do, and for whom?

**Outcome:** A native app for running apps: each app lives at an address, every screen has a link carrying route and state, and apps are sandboxed, signed WebAssembly bundles that describe their UI as a component tree Rapidkoi renders. Build path: shell → links → a real app → storage and sync → live sessions → history.

**Rationale:** The project owner's vision, recorded in `docs/source/stakeholder/vision-2026-10-17.md`.

**Copied to MEMORY.md:** yes
