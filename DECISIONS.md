# DECISIONS.md — rapidkoi

Tracks architectural and design questions that are open, deferred, or resolved.

Rules:
- Every open decision blocks implementation of the code it affects.
- The AI must not implement anything that depends on an open decision.
- When a decision is resolved, move it to the RESOLVED section and record the outcome.
- Once resolved, copy the outcome to MEMORY.md as an architectural decision.

---

## OPEN — Requires human input before implementation

### DECISION-044 — What "Continue where you left off" means now that a window's state goes with it

**Status:** open (parked by the owner, session 189)
**Raised:** 2026-10-23 — Session 189

**Question:** State lives with its window (the owner, session 189): closing a window deletes its state, and an app opened anew starts fresh. Continue used to reopen the last closed app with its state. Should it become a browser's "reopen closed tab" (the last closed window with its state, this run only, held in memory), or go?

**Meanwhile:** Continue works as it did within a run (a window closed this run is held in memory with its state). After a restart it reopens the app fresh, since its state file is gone. Not redesigned until the owner decides.


---

## DEFERRED — Acknowledged, not yet needed

### DECISION-050 — Screen readers need accessible roles from kivuli

**Status:** deferred (requested: `docs/specs/kivuli-accessible-roles.md`)
**Raised:** 2026-10-23 — Session 198 (PRP-037: roles in contract 5)

**Question:** Contract 5 lets any node say what it is (`role`, `checked`, `selected`, `label`), so a library-built checkbox can read as "Buy milk, checkbox, ticked". kivuli 0.54 can't set a GTK accessible role, state or label on a node.

**Meanwhile:** roles drive what Rapidkoi does itself: focus, pressing by keyboard, the `Checked` look. A screen reader hears what GTK infers from the widget. No workaround through visible text.

**Revisit when:** kivuli sets accessible roles and states; then they're passed through with no contract change.

---

### DECISION-047 — Share needs a clipboard command (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-clipboard.md`)
**Raised:** 2026-10-23 — Session 193 (PRP-036: links)

**Question:** Share should copy the window's link. kivuli 0.54 has no command that writes to the clipboard, and rapidkoi never touches GTK itself (architecture rule 2).

**Meanwhile:** share stays disabled. The address box shows the full link, and its text can be selected and copied. No workaround.

**Revisit when:** kivuli offers a clipboard command; then share copies `address_of(page)`.

---


### DECISION-045 — Apps keep nothing between calls: by convention, checked in development

**Status:** deferred (to settle before live sessions or developer tools, the owner, session 190)
**Raised:** 2026-10-23 — Session 190 (chat with the owner)

**Question:** A Rapidkoi app can't cause side effects (nothing is plugged in; it hands back requests), but its memory stays alive while its window is open, so it can keep hidden variables between calls. Restore, Reload, history and live sessions rebuild an app from its state, and hidden variables make them misbehave. Enforce "nothing kept between calls"?

**Direction agreed:** by convention, as React does with StrictMode.
- The Rust library keeps everything in the app's `State`, and the docs say so.
- A development check: for apps run locally (`./main serve`, a localhost address), each call runs twice on a fresh copy, with a clear warning when the answers differ.
- Published apps run as now, at no cost.
- Full enforcement (a fresh copy per call, or resetting memory) was weighed and not chosen: it costs speed, though it would also free an idle window's memory.

**Revisit when:** live sessions (step 5) or developer tools.

---

### DECISION-043 — The instance strip can't adapt to the window's width (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-fit-or-window-width.md`)
**Raised:** 2026-10-23 — Session 177 (the owner's screenshot: a narrower window shifts the address box and won't shrink further; "keep taking off instances and putting them in the more menu")

**Question:** For the strip to move pills into ⋯ as the window narrows, Rapidkoi needs the window's width, or a kivuli row that shows what fits and reports the rest. kivuli 0.54 has neither.

**Meanwhile:** the strip is a fixed `STRIP_WIDTH` (520) with at most three pills. A narrower window shifts the address box and can't shrink past it (CLAUDE.md KNOWN ISSUES). No workaround.

**Revisit when:** kivuli offers either; then the strip shows as many pills as fit.

---

### DECISION-042 — Several Rapidkoi windows (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-several-windows.md`)
**Raised:** 2026-10-23 — Session 177 (PRP-033; the owner: people who want two instances side by side "can open multiple windows")

**Question:** A second `./main` only raises the first window (DECISION-014): kivuli makes the GTK application single-instance. Should kivuli support several windows in one program (one model, a view per window), or let a second launch run separately?

**Meanwhile:** one window; instances are switched in the header's strip. Panes were weighed and deferred by the owner (session 176).

**Revisit when:** kivuli offers several windows; then a "New window" for an instance.

---

### DECISION-040 — A drop-down inside a popover breaks the popover's dismissal (kivuli)

**Status:** deferred (reported: `docs/specs/kivuli-dropdown-in-popover.md`)
**Raised:** 2026-10-22 — Session 172 (the owner: after changing a permission in ⚙'s card, it couldn't be dismissed; the owner's run of a kivuli-only repro: "it's stuck here too")

**Question:** After a pick in a `ui.dropdown` inside a `popover`, a click inside the window and Escape no longer close the popover. Only a click on another surface does, and kivuli's model and the widget drift apart. kivuli 0.54, no Rapidkoi code in the repro.

**Meanwhile:** no workaround. The card still closes with ⚙ or a click outside the window (CLAUDE.md KNOWN ISSUES). Session 171's `ClosePanel` stays: it's how a popover's on_close should read anyway (the shield's does), not a fix for this. Session 171's "approvals wait while a card is open" stays as a rule of its own (the owner left it to me, session 173: no dialog over the settings you're changing), not as a fix for this.

**Revisit when:** kivuli fixes it; then check ⚙'s Permissions page closes normally after a pick.

---

### DECISION-038 — Shipping Rapidkoi's own apps inside the binary needs Lugha's include_bytes

**Status:** resolved — Session 162: Lugha milestone 128 (`include_bytes`/`include_str`, lughac beccfd8); `src/window/newtab.la` embeds `assets/apps/newtab.rkb` (PRP-027 Part 2). The bundle shipped unsigned (the owner, session 158). `ui/icons.la` to `include_str` is its own PRP.
**Was:** deferred (requested from Lugha: `docs/specs/lugha-include-bytes.md`)
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

### DECISION-046 — Persistent storage: SQL, a database per app

**Status:** resolved
**Raised:** 2026-10-23 — Session 190 (chat with the owner)
**Resolved:** 2026-10-24 — Session 207, from chat after PRP-040 (the owner: "yes go sql-first", "all six as they are", "i'm good with that", "package first then storage prp")

**Question:** What do apps use to keep data for good, now that a window's state goes with it (session 189)? Session 190 leaned to localStorage-style keys with a log underneath.

**Outcome:**
- **SQL-first** (the owner: a real database is a selling point for offline apps). Each app gets its own SQLite database on this device.
  - Its own SQL; values only as parameters.
  - A read is one statement. A write is a batch run as one transaction, with nothing held open between events.
  - Answers come back as events, run in the background, capped at 10,000 rows or 4 MB.
  - Other windows of the app get `on_change` with the names of the changed tables.
  - Values are SQLite's own types, carried as CBOR.
- **The split** stays as agreed: window state is what you're looking at (per window, in the link); the database is what the app knows (per app, shared, kept). A window copies the rows it asked for into its state.
- **Who and how long:**
  - signed apps: one database each, by host and origin key, until Forget or Clear;
  - unsigned local development apps: one per address or file, kept;
  - built-in apps: none yet.
- **Limits:** 100 MB, 1 s per query or batch. No permission dialog.
- **Guardrails, all SQLite's own:** an authorizer (no `ATTACH`, no extensions, no dangerous `PRAGMA`s), a progress handler (time), `SQLITE_DBCONFIG_DEFENSIVE`, `sqlite3_limit`, `sqlite3_hard_heap_limit64` (memory), and `max_page_count` (size).
- **Later:** the change log (history, undo, sync), from SQLite's session extension. Apps never see it, so adding it changes nothing for them.

**Rationale:**
- WebSQL was dropped for lack of independent implementations, a web standards rule that doesn't bind a single Rapidkoi ([Wikipedia](https://en.wikipedia.org/wiki/Web_SQL_Database)).
- SQLite promises its SQL, C API and file format through 2050 ([sqlite.org/lts](https://sqlite.org/lts.html)).
- The guardrails are SQLite's own security guidance.
- Weak spots, noted: the update hook names only a table and a rowid; sync between devices (changesets, cr-sqlite) is real work.

**Copied to MEMORY.md:** yes (decision 26)

---

### DECISION-052 — Contract 6: storage

**Status:** resolved
**Raised:** 2026-10-24 — Session 208
**Resolved:** 2026-10-24 — Session 208 (the owner approved PRP-041: "yes approved, build it")

**Question:** How does an app reach its database (DECISION-046) through the guest contract?

**Outcome:** Contract 6 is contract 5 plus two asks and a field (`docs/specs/contract-6.md`):
- `sql` `{msg, sql, params, max_rows?}`, answered `on_answer {columns, rows, cut}` or `on_fail {error}`;
- `sql_batch` `{msg, steps: [{sql, params}]}`, one transaction, answered `on_answer {changes, last_rowid}` or `on_fail`;
- `on_change` gains `tables` when another window of the same app changed its data.
- Bounds: SQL 100 KB, 999 parameters, a value 4 MB, 10,000 rows or 4 MB an answer (cut past it), 100 steps; 100 MB a database, 1 s a call, 256 MB of SQLite memory for all.
- Every call runs in a `Cmd.task`, guarded by lugha-sqlite's `Guard.untrusted()`. Contracts 2–5 run unchanged, without storage. Built-in apps get no database.

**Rationale:** PRP-041. The asks follow `fetch`'s shape (a msg, answered later), so apps learn one pattern.

**Copied to MEMORY.md:** yes (decision 26, amended)

---

### DECISION-051 — SQLite comes through a new package, lugha-sqlite

**Status:** resolved
**Raised:** 2026-10-24 — Session 207
**Resolved:** 2026-10-24 — Session 207 (the owner: "package first then storage prp")

**Question:** Where do the SQLite bindings live?

**Outcome:** Its own package, `lugha-sqlite` (package `sqlite`, `../lugha-sqlite`, later `github.com/kithinjibrian/lugha-sqlite`), generic like lugha-wasmtime and lugha-cbor (CLAUDE.md: bindings to a C library are their own package). It's built first, under its own rules, then Rapidkoi's storage PRP. It links the system's `libsqlite3`, whose session extension, authorizer, progress handler and heap limit are all present here; building it needs `libsqlite3-dev` (the owner installs it). Publishing the repository is asked first.

**Copied to MEMORY.md:** yes (decision 26)

---

### DECISION-049 — Apps style themselves; contract 5 carries primitives, styles and roles

**Status:** resolved (the direction; PRP-037 to PRP-039 carry it out on approval)
**Raised:** 2026-10-23 — Session 197 (chat with the owner: a real app needs more than five node kinds)
**Resolved:** 2026-10-23 — Sessions 197–198 (the owner, step by step: "let people style their apps however they want"; the manifest theme "is an overreach"; apps ask for Rapidkoi's colours by name)

**Question:** How do apps describe and style their screens, now that a real app is next (step 3), and what stays in the person's hands?

**Outcome:**
- **The person always controls five things:** bigger text; light or dark and contrast; less motion; knowing which app they're in; using any app without seeing it or without a mouse. Apps follow by default (Rapidkoi's names, roles); a per-app emergency switch forces it as a last resort.
- **Apps style themselves freely,** through one door (`style`, kivuli's categories), within what Rapidkoi can draw. This replaces DECISION-031's "apps never set colours". The desktop's GTK theme stays ignored, and the frame stays Rapidkoi's.
- **Rapidkoi's names** (colours, text sizes, fonts, spacing, corners) are used without the app learning their values. Raw values are allowed and don't follow the person. Apps may read preferences they need to decide something.
- **Nothing about looks goes in the manifest.**
- **Rapidkoi is the engine, the Rust library is the framework:** the contract has boxes, text, styles, roles from AccessKit's list, and what an app can't build without painting (typing, scrolling and long lists, things on top, pictures, pickers). Widgets live in the library.
- **State is a tree with one owner per piece; other parts hold IDs.** Link fields become paths into it (`editor.task=42`); pages stay the app's own.
- **This is contract 5,** built in PRP-037 to PRP-039, and closed with PRP-039 (session 204). That settles DECISION-048's step B question: B's drawing goes into 5 as real apps need it. Contract 4 stays as it is.

**Rationale:** goals first (session 198): the developer wants an app that looks and behaves their way; the person wants apps that work for them. The research (the Extensible Web Manifesto, Flutter's layers, the first rule of ARIA, AccessKit) shaped the engine-and-framework split.

**Copied to MEMORY.md:** yes (decision 25)

---

### DECISION-048 — Contract 4: a state of named fields, and links

**Status:** resolved
**Raised:** 2026-10-23 — Session 191 (PRP-036: links, chat with the owner)
**Resolved:** 2026-10-23 — Session 193 (the owner approved PRP-036, the contract 4 review included: "yes approved, build it.")

**Question:** For a link to carry part of a window's state, Rapidkoi has to read it. Contract 3's state is sealed bytes. Do links read named fields (a new contract), or does the app hand Rapidkoi the link's values itself?

**Outcome:** contract 4 (the owner: "yes named fields, contract 4"), `docs/specs/contract-4.md`.
- A window's state is a CBOR map with text keys at the top level.
- `handle`'s result may carry `link`: a list of top-level field names, or null to clear them. Absent keeps them. The values are always read from the state, never given twice.
- Only simple values go in an address (numbers, text, true/false; null leaves a field out). Anything else stops the app.
- Opening a link sets its fields in the fresh state after `on_init`, only where that name holds the same kind of value, then sends `on_open`.
- A link is `https://host/@version?name=value`: the app's own domain, the version in the path, the app's fields in the query.
- Contracts 2 and 3 still run, unchanged and without links (DECISION-039).

**Rationale:** the owner's choices in chat, sessions 190–192 (PRP-036's WHY). Step B (images, wrapping rows, an accent style), meant for contract 3, hasn't landed: whether it now goes into contract 4 before any app outside this repository targets it, or makes contract 5, is the owner's call when step B is planned.

**Copied to MEMORY.md:** yes

---

### DECISION-041 — No back or forward in the frame

**Status:** resolved
**Raised:** 2026-10-23 — Session 173 (step B's discovery: do apps have pages?)
**Resolved:** 2026-10-23 — Session 175 (the owner: "okay drop back and forward", "yes drop home too")

**Question:** Should the frame have ← and → (and ⌂, the app's home), and so should apps declare pages, push a stack, or report positions for Rapidkoi to follow?

**Outcome:** No. In-app navigation is the app's own UI; Rapidkoi keeps only a cross-app return ("‹ Mail") for when a link opens another app, built with links. ←, ⌂ and → leave the header (PRP-032). MEMORY.md decision 22; decision 4 amended.

---

### DECISION-039 — Contract 3, and whether contract 2 apps keep running

**Status:** resolved
**Resolved:** 2026-10-22 — Session 167 (the owner approved PRP-030 and its recommendations: "approved")
**Raised:** 2026-10-22 — Session 166 (PRP-030: the new tab's reads, `on_resume`, and the `on_` event names on the wire, `docs/specs/app-events.md`)

**Question:** PRP-030 changes the guest contract: new event names on the wire, a new event (`on_resume`) and new asks. That is contract 3 (CLAUDE.md: any change is a decision and a bump). Two parts:
1. Does contract 3 start with PRP-030 (A), with step B (images, wrapping rows, an accent style) added to it before any app outside this repository targets it, or does B make contract 4?
2. Does Rapidkoi keep running contract 2 apps (the counter and quote published on the test VM), speaking both versions, or refuse them ("This app needs an older version of Rapidkoi") and republish ours at 3?

**Recommendation:** 1: contract 3 starts with A and is still open to B; no app outside this repository targets it until B lands. 2: keep running contract 2: published bundles can't be recompiled (CLAUDE.md architecture rule 6), and the difference is small (event names, three missing asks).

---

**Outcome:** contract 3 starts with PRP-030 (A) and stays open for step B; no app outside this repository targets it until B lands. Rapidkoi keeps running contract 2 apps: it speaks both, by the version each module exports.

---

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
