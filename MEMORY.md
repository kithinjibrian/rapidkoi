# MEMORY.md — rapidkoi

Records resolved architectural decisions and current project state.
Read this at the start of every session before writing any code.

---

## ARCHITECTURAL DECISIONS

### 1. Lugha + kivuli

**Decision:** rapidkoi is written in Lugha and its UI is built with kivuli 0.1, pulled in as a git package in `lugha.toml`.

**Why:** The project was started this way (`lughac init` with kivuli listed as a package). kivuli gives a pure, testable Elm-architecture app over GTK 4.

**Rules out:** Calling GTK directly from rapidkoi, other UI toolkits, and mutable UI state outside `update`.

---

### 2. Return-based error handling

**Decision:** Expected failures return `Result<T, E>` with a per-module error enum; programmer errors `panic`.

**Why:** Failure paths show in the type and can't be ignored. Matches kivuli and lugha-gtk.

**Rules out:** `Option` as an error, string errors, `unwrap` on external input.

---

### 3. Context system

**Decision:** The project follows `docs/setup.md`: CLAUDE.md, MEMORY.md, CONTEXT.md, DECISIONS.md, CHANGELOG.md, .llmignore, PRPs/, docs/.

**Why:** Keeps AI sessions consistent across handoffs. Same setup as `../kivuli`.

**Rules out:** Feature code without a PRP; architectural choices made silently.

---

### 4. What rapidkoi is (DECISION-001)

**Decision:** This repository is the Rapidkoi shell: a native desktop app that runs sandboxed WebAssembly apps ("bundles"), each at an address, each screen linkable with its state. Guests describe their UI as a component tree; the shell renders it through kivuli and gives every app the platform features (links, permissions, storage, sync, live sessions, history). Work follows the vision's build path in order: 1 the shell, 2 links, 3 a real app, 4 storage and sync, 5 live sessions, 6 history.

**Why:** The owner's vision: `docs/source/stakeholder/vision-2026-10-17.md`.

**Rules out:** Guests drawing their own widgets or touching GTK; guests reaching any resource except through the shell; work from a later build-path step before the earlier ones are done.

**Amended (DECISION-041, session 175):** the vision's "navigate with back and forward, which follow each app's routes" is dropped. See decision 22.

---

### 5. Screen state in the model, app data in SQLite (DECISION-004)

**Decision:** Apps are Elm-shaped (`init`, `update`, `view`) and Rapidkoi holds their screen-state model. App data lives in a per-app SQLite database Rapidkoi owns, reached through effects and live-query subscriptions (build path step 3 or 4).

**Why:** Links, history and live sessions need the platform to see screen state; offline apps need a real database; owning it keeps sync, export and removal possible.

**Rules out:** Guests keeping hidden screen state, guests bundling their own database, the whole dataset in the model, DuckDB as the default.

---

### 6. One versioned UI description per update (DECISION-005)

**Decision:** The guest returns its whole view tree as one compact, versioned, language-neutral encoding; the shell turns it into kivuli `View`s. Events go back as the guest's own encoded messages.

**Why:** One crossing per update, inspectable, independent of guest language.

**Rules out:** Host builder imports, the Component Model for now, an unversioned format.

---

### 7. wasmtime through lugha-wasmtime (DECISION-003, DECISION-010)

**Decision:** The shell runs guests with wasmtime's C API, through the separate package `../lugha-wasmtime` (package name `wasmtime`).

**Why:** Hardened, with fuel and memory limits; separate bindings serve all of Lugha.

**Rules out:** wasm2c, wasm3, writing wasmtime externs inside rapidkoi.

---

### 8. Rust for the first guest (DECISION-006)

**Decision:** The guest UI library and demo app are Rust, under `guest/rust/`; the shell's tests use `.wat` fixtures.

**Why:** lughac can't target WebAssembly yet; Rust is the best-supported guest language.

**Rules out:** Rust in the shell's test suite; anything in the UI encoding that only suits Rust.

---

### 9. Grow Lugha's ecosystem (DECISION-010)

**Decision:** Generic pieces become standalone Lugha packages in their own repos; Rapidkoi-specific ones split out once stable; kivuli and lughac gaps are fixed upstream.

**Why:** A goal of the project, beside Rapidkoi itself.

**Rules out:** Generic bindings buried in rapidkoi; silent workarounds for Lugha gaps.

---

### 10. UI contract v1 (DECISION-012)

**Decision:** Guests and the shell speak CBOR. Nodes are `{kind, props, children?, key?}` with semantic props only. Messages are opaque, returned as `{msg, value?}`. Guests export `rapidkoi_version`, `init`, `update → {state, effects}` and `view`; the shell holds state. Fixed limits; a failing app stops alone. Detail: `docs/source/meetings/2026-10-17-ui-contract.md`.

**Why:** Language-neutral, inspectable, and every person's settings apply to every app.

**Rules out:** Pixel or colour props; the shell interpreting app messages; guest-held state; failing a whole screen over an unknown kind; changing v1 without bumping the version.

**Contract 2 (PRP-011, 2026-10-18), replacing v1:** one entry point, `handle(state, event)`, for every event (`init`, `opened`, `message`, `answer`; more later), plus `view`. Any result can carry effects (`identity`, `sign`, and `fetch` from PRP-012). `{FIRST_NAME}`/`{LAST_NAME}` are filled in display text while drawing.

---

### 11. lugha-cbor (DECISION-013)

**Decision:** CBOR lives in the standalone pure-Lugha package `../lugha-cbor` (`cbor`), built before the shell.

**Why:** Generic and useful to all of Lugha (decision 9).

**Rules out:** A CBOR decoder inside rapidkoi.

---

### 12. Rapidkoi's own UI (meeting 2026-10-17, ui-design)

**Decision:**
- **The frame:** the system title bar; Rapidkoi's bar (← ⌂ →, address/search, ⓘ, ⤴); a tab rail on the right (tabs, +, divider, settings).
- **Home:** logo, "Where you left off", a Pinned grid. Visual: big tiles and banners.
- **History:** on-device and searchable.
- **Pictures:** page banner → app banner → generated art. No screenshots.
- **App info:** a panel.
- **Contract:** optional root props `title`, `banner`, `private`.

Built in PRP-002 (frame and tabs) and PRP-003 (remembering), before links. Missing kivuli features get explicit placeholders (DECISION-018).

**Why:** The owner's sketches and direction (`docs/source/meetings/2026-10-17-ui-design.md`). With no search engine, Rapidkoi's memory is how people find things again.

**Rules out:** Screenshots of what people saw; apps drawing in the bar or rail; history leaving the device; silent workarounds for kivuli gaps.

---

### 13. Packaging (meeting 2026-10-18)

**Decision:**
- One module per app (room for prerendered screens later).
- **One app per host.** A build is identified by its manifest's hash; a version is the publisher's semver name for one build.
- The device keeps the previous build. Versions can be withdrawn.
- **Everything Rapidkoi holds for an app carries the version that wrote it:** `state_version`, an optional `migrate`, and starting over without one.
- Now: a single-file CBOR bundle `.rkb` (format 1), packed with `./main pack`. Signing comes later.

**Why:** The owner, after framing the problem first (`docs/source/meetings/2026-10-18-packaging.md`).

**Rules out:** Server-run apps by default; per-screen code chunks for now; several apps on one host; state without a version.

---

### 14. Signing design (meeting 2026-10-18, signing-and-panel)

**Decision:**
- A key file over HTTPS, remembered on first use per host.
- Signed manifests and a signed, rising-sequence index; the server holds `key`, `index` and `builds/<sha256>.rkb` only.
- Signed rotations; an unapproved key change makes a separate app (identity = host + key).
- Ed25519 and SHA-256 via OpenSSL as `std.crypto`; `rapidkoi.toml` via `std.toml`.
- No bypass on a failed check; automatic updates with Undo.

**Why:** The owner, step by step, choosing the simplest secure option at each point.

**Rules out:** Loose pieces on servers; DNS or a log as requirements; libsodium; long pack flags for publishing.

---

### 15. Permissions at the point of use

**Decision:**
- Apps reach devices and files only through Rapidkoi's API, which asks the person when an app first asks (Allow once / Always / Don't).
- A file picker is the consent.
- The manifest declares a ceiling; network access is declaration-only.
- Toggles in the panel; no permission screens at install or update.

**Why:** The owner's direction, and the vision ("granted at the moment it's needed").

**Rules out:** Up-front permission screens; apps asking for anything undeclared.

---

### 16. The info panel and bar actions

**Decision:**
- **The bar:** ⟳ Reload (keeps state) and ☆ Pin.
- **The panel** (the owner's sketch): a header (icon, name, version, host); the description; Your settings (Account · Notifications · Access); On this device (Where you left off · Data); Forget this app.
- Rows open pages with a back arrow. Start over lives on the state page.

**Why:** The owner, who found the old panel ugly (`2026-10-17-ui/sketch-3-info-panel.png`).

**Rules out:** A flat list of lines and buttons; destructive actions without a confirm.

---

### 17. Identity (PRP-011)

**Decision:** Each signed app gets its own Ed25519 key pair for the person, derived from one master secret on the device (HKDF over the host, the app's origin key and a generation). Apps get only the public key; sign-ins are host-bound signatures. Auto sign-in is on by default; the Account page signs out, starts over as someone new, and holds a name that apps show through placeholders but never learn. Recovery (12 words, sync) later.

**Why:** No passwords, no cross-app tracking, no data apps don't need. Detail: `docs/source/meetings/2026-10-18-identity.md`.

**Rules out:** Rapidkoi accounts or servers in the middle; names, emails or phones sent to apps by Rapidkoi; keys for unsigned apps.

---

### 18. Apps reaching the network (PRP-012)

**Decision:** Declaration-only network access. A signed app may reach its own host and the servers in its signed manifest (`[network] servers`), over HTTPS by name, through the `fetch` effect; requests run in kivuli tasks, at most 4 at once per app, with no cookie jar, no redirects followed and nothing added by Rapidkoi. Plain HTTP only to this computer, and only for apps from it. The Access page switches each server off; an update adding one says so once. Signing in to the app's server is fetch → sign → fetch (`docs/server-sign-in.md`).

**Why:** No prompts, and nothing an app can reach that its publisher didn't sign. Detail: `docs/source/meetings/2026-10-19-network.md`.

**Rules out:** Sockets, WebSockets (later), raw IPs, ambient credentials, a remote app reaching this computer. Since PRP-013 (DECISION-026), Rapidkoi looks every name up itself, for fetches and for opening apps: a name with any address on this computer or the local network is refused whole, and connections go only to the addresses it checked.

---

### 19. The shell draws the frame; its pages are bundled apps (DECISION-027)

**Decision:** The shell draws the frame (bar, tab rail, address box, error screen) and the screens that grant, approve or verify (app info, key-change and sign-in approval, Access, Account). Content pages (new tab first, then search, notifications, harmless preferences) are apps bundled with the shell. Built on the branch `redesign`, merged to `main` only when the whole redesign is done.

**Why:** The shell dogfoods its own platform and keeps its trusted core small; anything an app can draw can be faked.

**Rules out:** A guest drawing a trust screen or holding the power to grant permissions or read the secret; merging the redesign to `main` piecemeal.

**Amended (DECISION-030, session 85):** the redesign is written on kivuli 0.17; the old view and its tests are deleted rather than ported.

---

### 20. Rapidkoi's theme for everything (DECISION-031)

**Decision:** Every colour Rapidkoi shows, apps' content included, comes from Rapidkoi's theme (`src/themes.la`) through kivuli's `.style`. The desktop's GTK theme is ignored. Apps name a button's kind (normal, primary, danger, subtle); Rapidkoi's theme decides its look.

**Why:** The owner (sessions 86–87): kivuli has no button looks and won't; one look Rapidkoi controls.

**Rules out:** Following the desktop's light or dark theme; apps setting colours.

---

### 21. Contract 3, and built-in permissions (DECISION-039, PRP-030)

**Decision:**
- Contract 3 names events `on_` on the wire (`on_init`, `on_open`, `on_resume`, `on_message`, `on_answer`, `on_response`, `on_fail`; `docs/specs/app-events.md`). Snake case is for the wire only; libraries name them as they like.
- It started with PRP-030 and stays open for step B (images, wrapping rows, an accent style); nothing outside this repository targets it until then.
- Rapidkoi keeps running contract 2 apps, by the version each module exports.
- A manifest's `permissions` list what an app reaches, fine grained. A built-in permission is only for bytes compiled into `./main`, decided by where the bytes came from, never by the manifest. An external app declaring one doesn't run; an app asking for one it didn't declare is stopped.
- Apps pull Rapidkoi's data (asks answered by events), never pushed; `on_resume` and `on_change` say when to ask again.
- Each declared permission is the person's to set (PRP-031): Always allowed (built-in ones' default), Needs approval (Rapidkoi's own dialog; Allow once is one ask), or Blocked (answered with nothing, the app runs on). A built-in app's manifest `controls` choose its address icons and card rows.

**Why:** The owner (sessions 165–167): the new tab reads your pins and where you left off without the shell drawing it (decision 19), and permissions cover it cleanly.

**Rules out:** A site holding a built-in permission; identifying Rapidkoi's own apps by name or manifest; pushing data to apps.

### 22. No back or forward in the frame (DECISION-041, PRP-032)

**Decision:** The frame has no ←, ⌂ or →. Moving between an app's screens is the app's own UI (as on iOS); Rapidkoi doesn't model an app's pages, routes or stack. Rapidkoi keeps only what apps can't do: a way back to the app you came from when a link opened another ("‹ Mail"), built with links between apps.

**Why:** The owner (sessions 173–175), from first principles: browsers own back and forward because their pages are untrusted documents from many sites that can't see each other. Rapidkoi gives each app one tab, and apps can draw their own buttons. Forward is rarely used anywhere.

**Rules out:** Frame back/forward; a route table in the manifest for navigation's sake. Links still need apps to open at a place and say where they are when shared; that's links' own question (build step 2).

---

## CURRENT PROJECT STATE

### Fully Working
- `src/main.la` prints a greeting; it passes `lughac check` and `fmt --check`.
- Context system in place (session 1). Vision recorded (session 2). Architecture decisions recorded (session 3). UI contract v1 designed (session 4).
- `../lugha-wasmtime` PRP-001 (core runtime) merged in that repo: everything the shell needs.
- **Build path step 1, the shell (PRP-001), merged in session 7:** `./main app.wasm` runs a guest under contract v1; a Rust crate and counter demo in `guest/rust/`. 47 Lugha tests, 7 Rust tests.
- **Rapidkoi's frame and tabs (PRP-002 + Amendment 1), merged in session 10:** the bar, an app-only tab rail, home (Where you left off, Pinned, search), the info panel, the dark theme defined (placeholder). 73 Lugha tests.
- **Bundles (PRP-003), merged in session 20:** `./main pack` makes a `.rkb` (format 1); `./main app.rkb` or typing its path opens it. 95 Lugha tests.
- **Local development (PRP-004), merged in session 23:** `./main serve app.rkb` (127.0.0.1 only); `rapidkoi://localhost:PORT` opens it; Reload. 115 Lugha tests.
- **The info panel and bar actions (PRP-005), merged in session 27:** the owner's panel sketch; ⟳ keeps state; ☆ pins. 121 Lugha tests.

### In Progress
- Nothing.

### Not Started
- Build path step 2 (links). Next: DECISION-016, then the links discovery interview.

---

## NEXT SESSION START POINT

After `../lugha-cbor`'s PRP-001 is merged: open a session, read the four files and `docs/source/meetings/2026-10-17-ui-contract.md`, then write `PRPs/prp-001-shell.md` implementing contract v1 exactly, and build it after approval.
