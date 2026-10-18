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

**Copied to MEMORY.md:** at PRP-013's merge

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
