# DECISIONS.md — rapidkoi

Every architectural and design decision: the core ones that shape the project (numbered, formerly MEMORY.md's: "MEMORY.md decision 26" in older PRPs is core decision 26 here), then the questions that are open, deferred or resolved.

Rules:
- Every open decision blocks implementation of the code it affects.
- The AI must not implement anything that depends on an open decision.
- When a decision is resolved, move it to the RESOLVED section and record the outcome. One that shapes the whole project also becomes a core decision, or amends one.

---

## CORE DECISIONS

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

**Decision:** CLAUDE.md (the rules and commands), DECISIONS.md (every decision, with its why), PRPs/ (a spec per feature), CHANGELOG.md, .llmignore and docs/.

**Why:** Keeps AI work consistent. **Amended (2026-10-24):** sessions, CONTEXT.md, MEMORY.md and `docs/setup.md` were removed by the owner as slowing work down: git's log and the PRPs carry the history, and MEMORY.md's decisions moved here.

**Rules out:** Feature code without a PRP (unless the owner says a chore needs none); architectural choices made silently.

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

### 23. Named instances (PRP-033)

**Decision:** An app can be open more than once. Each instance is a tab with its own state, a stable key ("" for the first, then "2", "3"…) under which a signed app's state is saved, and a name: the app's name and number until the person renames it. The sidebar is a dock, one entry per app (its last-used instance) with a count. The header's start, where ← ⌂ → were, is the current app's instance strip. A pin is the app (it opens the last-used instance); where you left off is an instance. Instances don't share data until storage (build step 4).

**Why:** The owner (sessions 175–177): people need the same app on different screens. Several tabs per app crowd the dock, and titles change with the screen, so they can't name an instance. Panes are deferred; side by side waits on several windows (DECISION-042).

**Rules out:** Naming instances from screen titles; an instance's state keyed by its name (renaming would move it).

**Amended (PRP-034, session 181):** what's open is saved (`open` file), and every open signed instance's state as it changes.

**Amended (PRP-035, session 184), from the basics with the owner:** launch always lands on the new tab, which offers "Restore previous session" all session; Restore stacks it under the current session; the current wins per app (an app open now keeps only its current windows, session 187). Unrestored, it's replaced at the next launch by the session after it, unless that one was empty (session 186). An instance is a window onto its app: it lives as long as it's open, and an extra window's state goes when it closes. An app's data (its first window's state) outlives every window; a closed app reopens where you last were. A crash leaves what a quit does.

**Amended (session 189, the owner: "devs are going to treat state like persistent storage"):** every window is the same, and state lives with its window. Closing any window deletes its state; an app opened anew starts fresh; only Restore brings a saved window back. What should last belongs in storage (build step 4). Session 188's special first window is undone. Continue's meaning is parked (DECISION-044).

### 24. Links and contract 4 (DECISION-048, PRP-036)

**Decision:**
- A link is preserved state in text: `https://host/@version?field=value`, a real web address on the app's own domain. The version is in the path (never `host@version`, a web login); Rapidkoi's parts live in the path (`@`, `~` reserved), the app's fields in the query.
- A window's state is named fields (contract 4). After each event the app names which fields go in its address (`link`); their values are always read from the state. Only simple values: numbers, text, true/false.
- Opening a link with fields opens a new window of its app at that version (while listed), fields set in the fresh state where they fit by name and kind, then `on_open`. A bare address of an open app switches to it.
- The address box shows the whole link, as the `https://` web address (the owner, session 197: a `rapidkoi://` scheme was tried in session 196 and dropped as "egoistical"). `./main publish` writes a landing page every other path on the site serves.

**Why:** The owner (sessions 190–193): which fields go out is chosen when the new state is returned, never declared up front; naming fields (not values) leaves one place for them to diverge from.

**Rules out:** Apps writing their own URLs; fields at the struct's definition; lists or maps in an address; a shared Rapidkoi domain for links. Cross-app links come next (proposed PRP-037).

### 25. Apps style themselves; Rapidkoi is the engine (DECISION-049, PRP-037)

**Decision:**
- The person always controls bigger text, light/dark and contrast, less motion, knowing which app they're in, and using any app without seeing it or a mouse. Apps follow by default; an emergency switch forces it (PRP-038).
- Apps style freely through one door (`style`), within what Rapidkoi can draw. Rapidkoi's names (colours, sizes, spacing, corners) follow the person, and the app never learns their values. Nothing about looks goes in the manifest.
- Contract 5 carries boxes, texts, typing boxes, styles and roles (AccessKit's names). Widgets live in the Rust library, the framework; the contract gains only what an app can't build without painting.
- State is a tree with one owner per piece, others holding IDs; links will be paths into it (PRP-039).

**Why:** the owner (sessions 197–198), from the goals: the developer wants their look, the person wants apps that work for them. Research: the Extensible Web Manifesto, Flutter's layers, the first rule of ARIA, AccessKit.

**Rules out:** apps never setting colours (DECISION-031's app rule); a component per app subject; looks in the manifest.

**Amended (PRP-038, sessions 200–201):**
- Settings is a built-in app (☰ → Settings), holding `change_settings`.
- Light or dark, with More contrast as a switch: four colour sets.
- Text size 80%–200% sets the base, and Rapidkoi's spacing names follow; it isn't zoom.
- Any app reads theme, contrast and text size, nothing more.
- Each app's ⚙ card has Use my colours and Use my text size.
- ☰ holds Settings, About and Quit.

**Amended (PRP-039, sessions 203–204):**
- Link fields are paths through named fields (`editor.task`), at most 4 deep, never list positions.
- Nested `State` structs are named maps; link fields chain (`App::editor.then(Editor::task)`).
- Messages are typed per part (`#[derive(Message)]`, `View<M>`, `.map`), with `String` still allowed.
- Contract 5 is closed.

### 26. Storage: SQL, a database per app (DECISION-046, DECISION-051)

**Decision:** Each app gets its own SQLite database on the device. Apps send SQL with parameters and get rows back as events; a write is a batch in one transaction. Other windows hear `on_change` with the changed tables. A window copies what it asked for into its state, and the database stays the truth. Signed apps keep it until Forget; local development apps keep theirs too. SQLite's own guardrails apply: authorizer, progress handler, limits. It comes through a new package, `lugha-sqlite`, built first.

**Why:** The owner (session 207): a real database is a selling point for offline apps. Researched: WebSQL's end was a standards rule, and SQLite's compatibility is promised through 2050.

**Rules out:** A key/value store as the only storage; holding transactions open between events; pasting values into SQL text.

**Amended (PRP-041, DECISION-052, session 209):** contract 6 carries it: the `sql` and `sql_batch` asks, answered `on_answer`/`on_fail`, and `on_change`'s `tables`. Each call runs in a task. ⚙'s Data row shows the size and clears it; Forget removes it. The tasks app keeps its tasks there, so a task's link opens it in any window.

---

### 27. The Rust SDK lives on its own; this repository is the shell and its built-ins (PRP-042)

**Decision:** The Rust library (`rapidkoi`, `rapidkoi-derive`) and its examples live in `rapidkoi-rust` (`../rapidkoi-rust`, github.com/kithinjibrian/rapidkoi-rust), with their history. This repository keeps the shell and Rapidkoi's own apps (newtab, settings), which depend on the SDK by git tag. Two example apps there replace tasks and quote: Notes (offline: storage, links, search) and Trip (its own server with sign-in, plus open-meteo and Frankfurter). The shell's tests use WebAssembly text fixtures only.

**Why:** The owner (session 215): "most of this guest apps should go into individual repos and leave builtins only. the rust lib i also feel should be outside the repo"; "let's just have two. one that is truly offline and another that needs multiple servers."

**Rules out:** Example or demo apps in this repository; shell tests that need a Rust build; changing the SDK from a rapidkoi session (a contract change is tagged there first).

**Amended (the owner, 2026-10-24):** Notes has its own repository, `../notes` (github.com/kithinjibrian/notes), on the SDK by tag (`v0.1.0`), not as an SDK example. It's live as a signed app at notes.dafifi.net, deployed with its own `scripts/deploy.sh`. The SDK's examples stay counter and styled; where Trip lives isn't decided yet.

---

### 28. Signed apps are kept on the device (PRP-043)

**Decision:** Every signed app opened is kept: its site's key and index and the one build they name, in `kept/<sha256 of host>/`. It opens from there at once, re-checked exactly as a site is (`check_site_at` over the kept files), then the site is checked in the background. A newer version is used from the next open, silently. A withdrawn running version stops its window once Rapidkoi sees the withdrawal; offline, the kept copy runs. 500 MB in all, least recently opened first, pins never evicted; one build per app; Forget removes it. ⟳ asks the site first. Compiled code is never kept.

**Why:** The owner (2026-10-24): kept apps must run without a connection (the vision's "offline by default"); every open fetching and compiling was slow and broke offline use.

**Rules out:** Trusting a file because it's on disk; swapping code under a running window (except stopping a withdrawn one); keeping a link's older version; caching compiled machine code.

**Amended (the owner, 2026-10-24):** updates are silent. A person hears about an update only when it changes something for them: their window stopped (a withdrawal), their state lost (its shape changed), the app asking for more (a new server or permission), the publisher's key changed, or a link not getting the version it named. A routine new version says nothing; its version is on the Signed app page. What's new in an app is the app's own to say.

**Amended (the owner, PRP-044):** still no notice, but the address box offers it: once a check keeps a newer version and reached the site, an upgrade button (the refresh arrow, a red dot, "A new version of this app is available: upgrade") shows left of the shield in every window of the app on the old version; clicking it upgrades that window, its state carried over. A kept app in front is checked every 15 minutes too.

**Amended (the owner, PRP-048, 2026-10-30):** an app's windows share its database, so they all run one version. Upgrading or reloading any window (the arrow, Reload, the tab menu's Reload; signed, local development and built-in apps alike) loads the new version once and moves every window of the app onto it, each keeping its state; if it can't be loaded, no window changes. A second upgrade waits for the first. A link's new window of an app already open runs the version open. An older version opening a database a newer one changed is a separate question, not decided here.

---

## OPEN — Requires human input before implementation

### DECISION-044 — What "Continue where you left off" means now that a window's state goes with it

**Status:** open (parked by the owner, session 189)
**Raised:** 2026-10-23 — Session 189

**Question:** State lives with its window (the owner, session 189): closing a window deletes its state, and an app opened anew starts fresh. Continue used to reopen the last closed app with its state. Should it become a browser's "reopen closed tab" (the last closed window with its state, this run only, held in memory), or go?

**Meanwhile:** Continue works as it did within a run (a window closed this run is held in memory with its state). After a restart it reopens the app fresh, since its state file is gone. Not redesigned until the owner decides.


---

## DEFERRED — Acknowledged, not yet needed

### DECISION-058 — Pictures kivuli can't show yet: WebP, over 4,096 px, a decode that fails (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-requests.md` § 6)
**Raised:** 2026-10-24, PRP-047

**Question:** DECISION-056 Q3 agreed PNG, JPEG and WebP from servers, up to 8,000 px a side and 40 MP, and `on_fail` for every picture that can't be shown. kivuli 0.54 shows PNG, JPEG and SVG only, up to 4,096 × 4,096 px and 16 MB, and shows nothing, silently, for one it can't decode.

**Meanwhile (the owner, PRP-047 Q1 (a)):** Rapidkoi refuses WebP with its sentence ("This picture is WebP, which Rapidkoi can't show yet") and pictures over 4,096 px a side or 16 MP; a picture whose header passes but whose decoding fails shows nothing and sends no `on_fail`. Raising the limits later only refuses less: no contract change.

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

**Status:** deferred (requested: `docs/specs/kivuli-requests.md` § 3)
**Raised:** 2026-10-23 — Session 177 (the owner's screenshot: a narrower window shifts the address box and won't shrink further; "keep taking off instances and putting them in the more menu")

**Question:** For the strip to move pills into ⋯ as the window narrows, Rapidkoi needs the window's width, or a kivuli row that shows what fits and reports the rest. kivuli 0.54 has neither.

**Meanwhile:** the strip is a fixed `STRIP_WIDTH` (520) with at most three pills. A narrower window shifts the address box and can't shrink past it (CLAUDE.md KNOWN ISSUES). No workaround.

**Revisit when:** kivuli offers either; then the strip shows as many pills as fit.

---

### DECISION-042 — Several Rapidkoi windows (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-requests.md` § 2)
**Raised:** 2026-10-23 — Session 177 (PRP-033; the owner: people who want two instances side by side "can open multiple windows")

**Question:** A second `./main` only raises the first window (DECISION-014): kivuli makes the GTK application single-instance. Should kivuli support several windows in one program (one model, a view per window), or let a second launch run separately?

**Meanwhile:** one window; instances are switched in the header's strip. Panes were weighed and deferred by the owner (session 176).

**Revisit when:** kivuli offers several windows; then a "New window" for an instance.

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

**Status:** deferred (reported: `docs/specs/kivuli-requests.md` § 5; Rapidkoi uses kivuli's documented exact `Sizing` meanwhile)
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

**Status:** deferred (part done; Rapidkoi uses placeholders for the rest, DECISION-018; the file dialog requested: `docs/specs/kivuli-requests.md` § 1)
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

### DECISION-064 — Two apps at one address: an app is its host and its first trusted key everywhere

**Status:** resolved (the owner, 2026-10-30; built by PRP-049, then PRP-050)
**Raised:** 2026-10-30, while fixing "Open as a separate app" running the old publisher's kept copy (found during PRP-048)

**Question:** DECISION-009 and the signing meeting (2026-10-18) say an unapproved key change "keeps the person's version and data, and offers the new one as a separate app"; an app's identity is its host and key. Rapidkoi follows that for data and saved state only. Kept copies, tabs ("one tab per app"), pins, Continue, the saved session and Forget go by address, and the background check deletes the kept copy on a changed key. So "Keep using my version" lasts only while its window is open, the old app's data becomes unreachable, and "Open as a separate app" can run the old publisher's kept copy.

**Decision (the owner, with the journey at https://claude.ai/artifact/Nx55CYNApso8E7GjWYGyvN):**
- **An app is its host and its first trusted key** (`Identity.origin`, kept through signed rotations), wherever Rapidkoi remembers an app: kept copies, tabs, pins, Continue, the session, Forget and Clear. Files and development addresses stay keyed by their path.
- **A changed key never deletes your version.** It keeps opening from its kept copy, with its data, and no longer updates; the shield offers the new one as a separate app.
- **A separate app is named by the person before it opens** (empty field; the instance-name rules; unique among the apps at that address). It loads from its site and gets its own kept copy, data and tab.
- **Several apps at one address:** a chip at the start of the address box (the app's colour and name) opens a dropdown of every app there (colour, name, key fingerprint, a default). With no default, typing the address asks which; with one, it opens it. The dock marks each app with its colour; the colour comes from its key and shows already while naming it. "· new publisher" leaves the address box.
- **Whoever signs the site decides which app is live:** a publisher returning with its key, or a signed rotation from it, makes its app update again and freezes the other; a new key with no trail is one more app. Rapidkoi never moves data between apps on its own.

**Rules out:** deleting a kept copy because the site's key changed; one kept copy per host; matching apps by address alone; a separate app named by Rapidkoi; moving an app's data to another app (not now: its own feature, if ever).

---

### DECISION-063 — Opening ⚙'s card can crash Rapidkoi (kivuli's popover cleanup)

**Status:** resolved (kivuli 0.62.2, 2026-10-25)
**Was:** deferred (reported: `docs/specs/kivuli-requests.md` § 7)
**Raised:** 2026-10-25, the owner: a segfault after clicking ⚙ in the address box

**Question:** kivuli's popover and context-menu nodes unparent their popover from their box's `destroy` handler; when GTK has already taken the popover apart, that's a freed pointer (`gtk_widget_unparent: assertion 'GTK_IS_WIDGET (widget)' failed`), and sometimes a segfault. Traced with `G_DEBUG=fatal-criticals` to `render/patch/apply` dropping a node.

**Meanwhile:** nothing in Rapidkoi can avoid it: every popover is kivuli's. Fixed in kivuli; nothing is worked around.

**Outcome:** kivuli 0.62.2 holds each popover until its widget's `destroy` and unparents it only while it's still attached (kivuli DECISION-044). Rapidkoi is on 0.63.1; the owner checked: ⚙ no longer crashes.

---

### DECISION-040 — A drop-down inside a popover breaks the popover's dismissal (kivuli)

**Status:** resolved (kivuli 0.63.1 with lugha-gtk 0.1.58, 2026-10-25)
**Was:** deferred (reported: `docs/specs/kivuli-requests.md` § 4)
**Raised:** 2026-10-22 — Session 172 (the owner: after changing a permission in ⚙'s card, it couldn't be dismissed; the owner's run of a kivuli-only repro: "it's stuck here too")

**Question:** After a pick in a `ui.dropdown` inside a `popover`, a click inside the window and Escape no longer close the popover. Only a click on another surface does, and kivuli's model and the widget drift apart. kivuli 0.54, no Rapidkoi code in the repro.

**Meanwhile:** no workaround. The card still closes with ⚙ or a click outside the window (CLAUDE.md KNOWN ISSUES). Session 171's `ClosePanel` stays: it's how a popover's on_close should read anyway (the shield's does), not a fix for this. Session 171's "approvals wait while a card is open" stays as a rule of its own (the owner left it to me, session 173: no dialog over the settings you're changing), not as a fix for this.

**Revisit when:** kivuli fixes it; then check ⚙'s Permissions page closes normally after a pick.

**Outcome:** GTK drops the seat's grab when a drop-down's list hides; a kivuli popover now takes its grab again when a drop-down, context menu or popover inside it closes (kivuli DECISION-045). The owner checked: ⚙'s Permissions card closes normally after a pick.

---

### DECISION-062 — An overlay layer with a maximum size swallows the clicks under it (kivuli 0.62)

**Status:** resolved (kivuli 0.62.1, 2026-10-25)
**Raised:** 2026-10-25, the owner: "update button not clickable"

**Question:** kivuli 0.62's clamp fills its parent when its node has a maximum size, so an overlay layer's clamp covers the whole base and takes its clicks: the update button (under its red dot) can't be pressed, and the overflow band in development would cover a whole app. kivuli fixes it. Meanwhile: stay on 0.62 (centring works, the update button doesn't), or go back to 0.61 (the update button works, a centred box with a maximum width shrinks again, as Notes 0.3.5's note would)?


**Meanwhile (the owner):** wait for kivuli's fix; Rapidkoi stays on 0.62 on its branch, unmerged, so `main` keeps 0.58 and a working update button. Nothing is worked around.

**Outcome:** kivuli 0.62.1 with lugha-gtk 0.1.55: a clamp takes the pointer only where its node is, so an overlay layer with a maximum size no longer takes the clicks under it; the update button works again.

---

### DECISION-061 — A box with a maximum width can't be centred (kivuli)

**Status:** resolved (kivuli 0.62, 2026-10-25)
**Raised:** 2026-10-24, Notes 0.3.3

**Question:** An app's `max_width` with `align: center` should give a box as much room as there is, at most its maximum, in the middle. kivuli's clamp puts its child at the start, and a centred one shrinks to its content's natural width: Notes' note wrapped in a narrow strip.

**Meanwhile (the owner):** apps' maximum-width boxes fill from the start; Notes doesn't centre its note. Nothing is worked around.

**Outcome:** kivuli 0.62 (PRP-066): a node with a maximum size and an alignment other than fill is as big as there's room for, up to its maximum, where its alignment says. Nothing changes in Rapidkoi; Notes centres its note again.

---

### DECISION-059 — Rapidkoi crashes on a screen nested about 13 deep (lughac, kivuli)

**Status:** resolved (lughac PRP-167, 2026-10-25)
**Raised:** 2026-10-24, opening Notes 0.3.0

**Question:** kivuli's `resolve` uses 289 KB of stack a call in Rapidkoi's `./main` (lughac's `-O 0`; 122 KB at `-O 2`), and a screen costs two or three calls a level, so about 13 levels of nesting overflow the 8 MB main stack and Rapidkoi segfaults. Apps may nest 64 deep: any app can crash the shell. The fix is lughac's (frames that don't grow with every `match` arm's temporaries) or kivuli's (boxing `Style` in `View`, or walking without deep recursion). What does Rapidkoi do until then?

**Options:** (a) nothing in Rapidkoi: wait for lughac or kivuli; Notes can't be opened meanwhile. (b) build `./main` with `-O 2` (frames 122 KB: about 30 levels), as a build setting, until the fix. (c) (b) and lower `MAX_NODE_DEPTH` for now. Any of these leaves the root cause to lughac and kivuli.

**Meanwhile (the owner: (b)):** `./main` is built with `lughac build -O 2` (CLAUDE.md, the scripts): about 30 levels fit, so Notes opens. An app nesting deeper still crashes the shell until lughac or kivuli is fixed; nothing else is worked around.

**Outcome:** lughac's PRP-167 (milestone 135): stack slots shared between `match` arms, and stack probes, so running out of stack panics with `stack overflow` instead of a segfault. Measured on `./main` (2026-10-25): `resolve` 73 KB at `-O 0` (was 289 KB), 51 KB at `-O 2` (was 122 KB). `./main` stays built with `-O 2` (the owner): a 64-deep screen needs about 7 MB of the 8 MB stack, by estimate; drawing one is for the owner to check.

---

### DECISION-047 — Share needs a clipboard command (kivuli)

**Status:** resolved (kivuli 0.60, 2026-10-25)
**Raised:** 2026-10-23 — Session 193 (PRP-036: links)

**Question:** Share should copy the window's link. kivuli 0.54 has no command that writes to the clipboard, and rapidkoi never touches GTK itself (architecture rule 2).

**Meanwhile:** share stays disabled. The address box shows the full link, and its text can be selected and copied. No workaround.

**Revisit when:** kivuli offers a clipboard command; then share copies `address_of(page)`.

**Outcome:** kivuli 0.60's `Cmd.copy_text`: on an app, share copies its link (`address_of`), or a local file's path; on the empty tab there's nothing to share, so it stays greyed. Nothing is shown when it's copied (no unasked copy).

---

### DECISION-050 — Screen readers need accessible roles from kivuli

**Status:** resolved (kivuli 0.59, 2026-10-25)
**Raised:** 2026-10-23 — Session 198 (PRP-037: roles in contract 5)

**Question:** Contract 5 lets any node say what it is (`role`, `checked`, `selected`, `label`), so a library-built checkbox can read as "Buy milk, checkbox, ticked". kivuli 0.54 can't set a GTK accessible role, state or label on a node.

**Meanwhile:** roles drive what Rapidkoi does itself: focus, pressing by keyboard, the `Checked` look. A screen reader hears what GTK infers from the widget. No workaround through visible text.

**Revisit when:** kivuli sets accessible roles and states; then they're passed through with no contract change.

**Typing boxes (the owner, 2026-10-24):** contract 5 gives an `input` no `label`, and Rapidkoi's `Typing` has none, so an app's label on a typing box (the SDK lets it write one) is ignored. When kivuli lands accessible labels, typing boxes gain `label` in the contract (a DECISIONS entry and a version bump) and it's passed through with the rest.

**Outcome:** kivuli 0.59's `Accessible` style: `ui.render.heard` gives each box and text the app's role, checked, selected and label; a typing box is heard by its label (else its placeholder), a picture by its label.

---

### DECISION-014 — Running two apps at once (kivuli single-instance)

**Status:** resolved (kivuli 0.61, 2026-10-25)
**Raised:** 2026-10-17 — Session 5
**Revisit when:** The shell runs more than one app, or a second launch must open a second app.

**Question:** kivuli starts GTK with `gtk_application_new(id, 0)` (`../kivuli/src/render/live.la:109`), so GTK makes the program single-instance: a second `./main other.wasm` raises the first window and drops the new app. Should kivuli accept application flags (e.g. `G_APPLICATION_NON_UNIQUE`, or `HANDLES_OPEN` for links later)?

**Notes:** A kivuli gap, fixed upstream when needed. Step 1 runs one app, so it's a known limitation. Build path step 2 (links) will need `HANDLES_OPEN`-style behaviour anyway.

**Outcome:** kivuli 0.61's `App.on_open`: what a launch names arrives as `Msg.Launched` and opens as the address box would (`navigate.open_launched`); a second `./main …` hands its items to the running window and exits. Instances side by side in separate windows are still DECISION-042.

---

### DECISION-060 — An app's long text: wraps, or is truncated

**Status:** resolved (the owner, 2026-10-24)
**Raised:** 2026-10-24, Notes 0.3.0: a long preview pushed its 296 px list wider

**Question:** An app's text was one unwrapped line, and GTK never lets a box be narrower than its widest line, so long text pushed its box (and the window) wider. Apps lay themselves out (PRP-046), so they need to say what long text does.

**Outcome:** as on the web, text wraps to its box's width; a new style key `truncate: true` keeps it on one line, cut with "…" (kivuli's `.wrap()` and `.ellipsize()`). Contract 7 isn't released, so no version bump; the SDK gets `Text::truncate()`.

---

### DECISION-054 — Knowing when Rapidkoi's window comes back to the front (kivuli)

**Status:** resolved (kivuli 0.58, 2026-10-24)
**Raised:** 2026-10-24, PRP-044

**Question:** PRP-044's Q1 (the owner: (a)) checks a kept app against its site every 15 minutes while it's in front, and when Rapidkoi's window comes back to the front. kivuli 0.54 has no event for the window becoming active again (GTK's `notify::is-active`), so only the 15-minute check is built.

**Meanwhile:** the 15-minute check, and the check on every open from the kept copy. Nothing is worked around.

**Outcome:** kivuli 0.56's `Sub.window_active`: while a kept app is in front, Rapidkoi checks it against its site when the window comes back to the front (`Msg.WindowActive(true)`), as well as every 15 minutes.

---

### DECISION-057 — Clipping an app's page, showing its overflow, and scrolling one way (kivuli)

**Status:** resolved (kivuli 0.58, 2026-10-24)
**Raised:** 2026-10-24, PRP-046

**Question:** Rapidkoi never scrolls an app's page (the owner: "we are building apps not sites"). What doesn't fit must be clipped, silently when published and with a striped band in development; and a box scrolling `vertical` shouldn't scroll sideways. kivuli 0.54 has no clipping container, no overflow measure, and no one-axis scroll.

**Meanwhile:** an overflowing app pushes the window's minimum size (visible, the developer's to fix); a scrolling box scrolls whichever way its content overflows; `ui.frame.in_development` already says where a band would show. Nothing is worked around.

**Outcome:** kivuli 0.57 and 0.58: a scrolling box keeps to its axis (`Overflow.scroll_y`/`scroll_x`); a running app's page is in `clip`, so it never pushes the window bigger; an app in development reports its overflow (`on_overflow` → `Msg.Overflowed`) and `ui.overflow` draws the striped band and "Bottom overflowed by 42 px".

---

### DECISION-055 — A text area's placeholder (kivuli)

**Status:** resolved (kivuli 0.58, 2026-10-24)
**Raised:** 2026-10-24 (the owner, after Notes: its empty text box shows nothing)

**Question:** Contract 5's typing box of several lines has a `placeholder`, and kivuli 0.54 draws placeholders on one-line fields only (`GtkTextView` has none). How does an empty text area say what it's for?

**Meanwhile:** `ui.render.typing_box` names the text area by its placeholder (kivuli `.name`), so it's found as a one-line field is. Nothing is drawn in its place: no workaround with visible text.

**Outcome:** kivuli 0.55 draws a text area's placeholder: `ui.render.typing_box` passes it, and still names the box by its label or placeholder.

---

### DECISION-056 — Contract 7: one contract, scrolling, pictures

**Status:** resolved (discovery with the owner, 2026-10-24); built in PRP-045, PRP-046, PRP-047
**Raised:** 2026-10-24, after Notes: its list and note can't scroll apart, and it has no icons

**Outcome (the owner, Q1–Q7, "don't be afraid of breaking changes"):**
- **Q1, where pictures come from:** the app's bundle (`image("asset:trash.svg")`), or a URL in the view (`image("https://…")`) on the app's own or declared servers, under `fetch`'s rules, loaded by Rapidkoi as a browser loads an `<img>` (Elm's model). Optional `on_load` and `on_fail` messages. Never a URL anywhere else: no picture is fetched but one a view names.
- **Q2, sizing:** the existing `Sizing` styles; `fit` `contain` (default) or `cover`; the box's styles (corners, borders, shadows) apply; `.color(...)` recolours a one-colour SVG from the bundle only; `label` for screen readers.
- **Q3, limits:** 10 MB downloaded a picture; 8,000 px a side and 40 megapixels, checked from the header before decoding; 256 MB decoded a window, least recently shown first; 6 downloads at once a window; 30 s each; PNG, JPEG and WebP from servers, SVG (≤ 1 MB) only from the bundle. A failed picture shows Rapidkoi's broken-picture mark, sends `on_fail` with the sentence, and the app runs on; a broken view (a bad URL, a missing asset, a size over the limits) stops the app.
- **Q4, scrolling** (researched: CSS's `min-height: auto`, Flutter's unbounded viewports, Compose's infinite height): the page is the window and never scrolls; `Layout::scroll()` / `scroll_x()` on a box; every box on the way to a scrolling one has a minimum size of 0, so it never pushes its parents bigger; it's as tall as its content until it runs out of room (`scroll_fit`), `grow` to fill; a scrolling box inside another scrolling the same way stops the app with a sentence.
- **Q5:** contract 7 only; contracts 2–6 and their code deleted; every app rebuilt (the SDK, its examples, the built-ins, Notes; the counter on the VM republished, asked first).
- **Q6:** `label` on typing boxes; link fields with declared kinds (`kinds` in the `on_init` result, written by the SDK from the state's types), so an `Option` field can be set by a link.
- **Q7:** three PRPs, one contract: PRP-045 (the base), PRP-046 (scrolling), PRP-047 (pictures); the SDK released as v0.2.0 after the third.

**Why:** Notes needs both; a breaking change costs one rebuild per app while every app is the owner's.

**Supersedes:** DECISION-039, 048, 049, 052's "older contracts still run".

**Amended (the owner, PRP-047's build):** a picture's `on_load` and `on_fail` arrive as `on_message` (the failure's sentence as `value`), so typed messages arrive whole; WebP and the 8,000 px / 40 MP limits wait on kivuli (DECISION-058).

---

### DECISION-053 — WebAssembly text to bytes (lugha-wasmtime)

**Status:** resolved (lugha-wasmtime 0.1.1, session 217)
**Raised:** 2026-10-24 — Session 216

**Question:** PRP-042 takes the Rust apps out of this repository, so the tests that serve or pack a module from a `.wasm` file need to make one from WebAssembly text. lugha-wasmtime's `Module.from_text` compiles text straight into a module; its `wasmtime_wat2wasm` is in the private `raw`. How do the tests get the bytes?

**Outcome (the owner, session 216):** lugha-wasmtime gains a public `wat_to_wasm(text): Result<u8[], CompileError>`, in its own repository first. Not chosen: a committed binary fixture made by `wat2wasm`, which can drift from its source.

**Done:** lugha-wasmtime 0.1.1 (`wat_to_wasm`); `test/support.la`'s `write_counting` uses it.

---

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

**A core decision:** yes (decision 26)

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

**A core decision:** yes (decision 26, amended)


**Superseded in part (DECISION-056, PRP-045):** older contracts no longer run; contract 7 is the only one, and its spec is `docs/specs/contract-7.md`.

---

### DECISION-051 — SQLite comes through a new package, lugha-sqlite

**Status:** resolved
**Raised:** 2026-10-24 — Session 207
**Resolved:** 2026-10-24 — Session 207 (the owner: "package first then storage prp")

**Question:** Where do the SQLite bindings live?

**Outcome:** Its own package, `lugha-sqlite` (package `sqlite`, `../lugha-sqlite`, later `github.com/kithinjibrian/lugha-sqlite`), generic like lugha-wasmtime and lugha-cbor (CLAUDE.md: bindings to a C library are their own package). It's built first, under its own rules, then Rapidkoi's storage PRP. It links the system's `libsqlite3`, whose session extension, authorizer, progress handler and heap limit are all present here; building it needs `libsqlite3-dev` (the owner installs it). Publishing the repository is asked first.

**A core decision:** yes (decision 26)

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

**A core decision:** yes (decision 25)


**Superseded in part (DECISION-056, PRP-045):** older contracts no longer run; contract 7 is the only one, and its spec is `docs/specs/contract-7.md`.

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

**A core decision:** yes


**Superseded in part (DECISION-056, PRP-045):** older contracts no longer run; contract 7 is the only one, and its spec is `docs/specs/contract-7.md`.

---

### DECISION-041 — No back or forward in the frame

**Status:** resolved
**Raised:** 2026-10-23 — Session 173 (step B's discovery: do apps have pages?)
**Resolved:** 2026-10-23 — Session 175 (the owner: "okay drop back and forward", "yes drop home too")

**Question:** Should the frame have ← and → (and ⌂, the app's home), and so should apps declare pages, push a stack, or report positions for Rapidkoi to follow?

**Outcome:** No. In-app navigation is the app's own UI; Rapidkoi keeps only a cross-app return ("‹ Mail") for when a link opens another app, built with links. ←, ⌂ and → leave the header (PRP-032). Core decision 22; core decision 4 amended.

---

### DECISION-039 — Contract 3, and whether contract 2 apps keep running

**Status:** resolved
**Resolved:** 2026-10-22 — Session 167 (the owner approved PRP-030 and its recommendations: "approved")
**Raised:** 2026-10-22 — Session 166 (PRP-030: the new tab's reads, `on_resume`, and the `on_` event names on the wire, `docs/specs/app-events.md`)

**Question:** PRP-030 changes the guest contract: new event names on the wire, a new event (`on_resume`) and new asks. That is contract 3 (CLAUDE.md: any change is a decision and a bump). Two parts:
1. Does contract 3 start with PRP-030 (A), with step B (images, wrapping rows, an accent style) added to it before any app outside this repository targets it, or does B make contract 4?
2. Does Rapidkoi keep running contract 2 apps (the counter and quote published on the test VM), speaking both versions, or refuse them ("This app needs an older version of Rapidkoi") and republish ours at 3?

**Recommendation:** 1: contract 3 starts with A and is still open to B; no app outside this repository targets it until B lands. 2: keep running contract 2: published bundles can't be recompiled (CLAUDE.md architecture rule 6), and the difference is small (event names, three missing asks).


**Superseded in part (DECISION-056, PRP-045):** older contracts no longer run; contract 7 is the only one, and its spec is `docs/specs/contract-7.md`.

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

**A core decision:** no (a look)

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

**A core decision:** no (CLAUDE.md's TESTING RULE says it)

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

**A core decision:** yes (decision 20)

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

**A core decision:** yes (decision 19, amended)

---

### DECISION-028 — The address box's focus ring on the whole pill (kivuli)

**Status:** resolved
**Raised:** 2026-10-19 — Session 79
**Resolved:** 2026-10-19 — Session 80 (the owner, in chat)

**Question:** Focus showed on the field inside the pill, and kivuli can't style focus (an outline, `:focus-within`).

**Outcome:** No kivuli change. The pill is scrapped: the address box is the field itself, rounded (`Look.Address`: radius half its height), with no box around it and no → button (Enter goes). GTK's focus ring then goes round the whole box.

**Rationale:** The owner's call: the simplest shape that gets the focus right.

**A core decision:** no (a look, not architecture)

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

**A core decision:** yes (decision 19)

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

**A core decision:** yes (decision 18)

---

### DECISION-025 — UI contract 2: one entry point, effects and name placeholders

**Status:** resolved
**Raised:** 2026-10-18 — Session 55 (designing identity)
**Resolved:** 2026-10-18 — Session 56 (PRP-011, the owner approved)

**Question:** How does an app ask Rapidkoi for things (its key, a sign-in signature), and show the person's name without learning it, without host functions?

**Outcome:** Contract 2, replacing v1 (amendment 1: the owner, "you can do breaking changes nothing has shipped"): one export `handle(state, event)` for every event (`init`, `opened`, `message`, `answer`) and `view`; the `identity` and `sign` effects from any result, answered as answer events; `{FIRST_NAME}`/`{LAST_NAME}` filled while drawing. Detail: the contract 2 appendix of `docs/source/meetings/2026-10-17-ui-contract.md`.

**A core decision:** yes (decisions 10 and 17)

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

**Rationale:** It keeps momentum without silent workarounds: every placeholder is visible, named and temporary. The real features are fixed upstream (core decision 9).

**A core decision:** yes

---

### DECISION-016 — A read-only text input (kivuli)

**Status:** resolved
**Raised:** 2026-10-17 — Session 6
**Resolved:** 2026-10-17 — Session 8 (the owner chose A)

**Question:** How should the shell show a contract `input` with no `on_change`?

**Outcome:** A: kivuli gains `.disabled(true)` for text fields (its PRP-006, prompt given to the owner). Until kivuli 0.1.1 is tagged, `contract.read_tree` keeps refusing such inputs with a visible error.

**Rationale:** It matches "no handler means disabled" everywhere, with no contract change.

**Shipped:** kivuli 0.1.1; Rapidkoi renders such inputs disabled since session 11.

**A core decision:** yes

---

### DECISION-012 — UI contract v1

**Status:** resolved
**Raised:** 2026-10-17 — Session 4
**Resolved:** 2026-10-17 — Session 4

**Question:** What exactly passes between a guest and the shell?

**Outcome:** CBOR. Nodes are `{kind, props, children?, key?}`; step 1 kinds are text, button, input, column, row. Props are semantic only (no pixels or colours). Unknown props are ignored; unknown kinds render a placeholder. Messages are opaque app-chosen CBOR values, returned in an envelope `{msg, value?}`. No handler means disabled. The guest exports `rapidkoi_version() → 1`, `init`, `update(state, event) → {state, effects: []}` and `view(state)`; the shell holds the state bytes. Limits: fuel ≈100 ms per call, 256 MB memory, view 4 MB / 10,000 nodes / depth 64, strings 64 KB, state 16 MB. A failing app stops alone, behind a Restart screen. Full detail: `docs/source/meetings/2026-10-17-ui-contract.md`.

**Rationale:** Language-neutral, inspectable, one crossing per call; semantic props let the person's settings apply to every app; reserved `key` and `effects` avoid breaking changes later.

**A core decision:** yes

---

### DECISION-013 — lugha-cbor

**Status:** resolved
**Raised:** 2026-10-17 — Session 4
**Resolved:** 2026-10-17 — Session 4

**Question:** Where does the CBOR code live?

**Outcome:** A standalone, pure-Lugha package `../lugha-cbor` (package `cbor`), built before the shell. Its decoder is safe on hostile input.

**Rationale:** Core decision 9: generic pieces are their own packages.

**A core decision:** yes

---

### DECISION-003 — Which WebAssembly runtime the shell embeds

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** Which C-API WebAssembly runtime does the shell embed?

**Outcome:** wasmtime, through its C API, wrapped by the separate Lugha package `lugha-wasmtime` (DECISION-010).

**Rationale:** The most mature and hardened option, with fuel/epoch and memory limits for untrusted code. WAMR is the fallback to reconsider for Android (DECISION-007).

**A core decision:** yes

---

### DECISION-004 — Where an app's state lives

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** Does the guest own its state, or does the platform hold it?

**Outcome:** Two kinds of state. Screen state (route, filters, drafts, selection) is a serialisable model Rapidkoi holds; apps are Elm-shaped (`init`, `update`, `view`). App data lives in a per-app SQLite database that Rapidkoi owns and provides (from build path step 3 or 4); queries and writes are effects, with live-query subscriptions. Apps don't bundle their own database. Conflict handling is DECISION-011.

**Rationale:** Platform-held screen state gives links, history and live sessions to every app. A real database is needed for offline apps (the owner's point), and Rapidkoi owning it keeps sync, export, inspection and removal possible. SQLite fits many small reads and writes; DuckDB is for analytics.

**A core decision:** yes

---

### DECISION-005 — The guest↔host UI contract

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** How does a bundle hand its view tree to the shell, and how do events get back?

**Outcome:** The guest writes its whole view tree into its memory as one compact, versioned encoding per update; the shell decodes it into kivuli `View`s, and kivuli diffs. Events go back to the guest as its own encoded messages. Step 1 components: text, button, text input, column, row. The exact encoding is designed in the step 1 PRP and stays language-neutral.

**Rationale:** One crossing per update, simple, inspectable, and independent of any guest language.

**A core decision:** yes

---

### DECISION-006 — The first guest language and test bundles

**Status:** resolved
**Raised:** 2026-10-17 — Session 2
**Resolved:** 2026-10-17 — Session 3

**Question:** What language is the first guest UI library and demo bundle written in?

**Outcome:** Rust (`wasm32-unknown-unknown`), in this repo under `guest/rust/`. The shell's own tests use hand-written `.wat` fixtures, so `lughac test` never needs Rust. A lughac WebAssembly target and a Lugha guest library are a later milestone in the lugha repository.

**Rationale:** Rust is the best-supported WebAssembly language and already installed; lughac emits native code only today.

**A core decision:** yes

---

### DECISION-010 — Rapidkoi grows Lugha's ecosystem

**Status:** resolved
**Raised:** 2026-10-17 — Session 3
**Resolved:** 2026-10-17 — Session 3

**Question:** Where do general-purpose pieces built for Rapidkoi live?

**Outcome:** A second project goal is improving Lugha's ecosystem. Generic pieces (runtime bindings, database bindings) are standalone Lugha packages in their own repositories, starting with `lugha-wasmtime` (package `wasmtime`) from day one. Rapidkoi-specific pieces (link parser, UI encoding) start in this repo and split out once their shape settles. kivuli and lughac gaps are fixed upstream.

**Rationale:** The owner's direction (`docs/source/meetings/2026-10-17-architecture.md`).

**A core decision:** yes

---

### DECISION-001 — What rapidkoi is for

**Status:** resolved
**Raised:** 2026-10-17 — Session 1
**Resolved:** 2026-10-17 — Session 2

**Question:** What does rapidkoi do, and for whom?

**Outcome:** A native app for running apps: each app lives at an address, every screen has a link carrying route and state, and apps are sandboxed, signed WebAssembly bundles that describe their UI as a component tree Rapidkoi renders. Build path: shell → links → a real app → storage and sync → live sessions → history.

**Rationale:** The project owner's vision, recorded in `docs/source/stakeholder/vision-2026-10-17.md`.

**A core decision:** yes
