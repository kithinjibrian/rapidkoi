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

**Amended (the owner, 2026-10-31, DECISION-084):** the Account page's Sign out, Sign in, Sign in automatically and Start over as someone new become one switch, "let this app know who I am"; becoming someone new is Forget, then opening it again.

**Amended (the owner, 2026-10-31, core decision 29):** "servers in the middle" means sign-in through a Rapidkoi account or server. The Pond holds only sealed data, reached with device keys and no account, and never stands between a person and an app's sign-in.

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

**Amended (DECISION-087, 2026-10-31):** an app's large bytes are blobs: files by hash beside its database, their keys in a reserved `rapidkoi_blobs` table committed with the app's rows, owned by a row when the app says so, shown with `image("blob:…")` and sent with `fetch`'s `body_blob` (`docs/specs/blobs.md`). Not built yet.

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

**Amended (the owner, PRP-048, 2026-10-30):** an app's windows share its database, so they all run one version. Upgrading or reloading any window (the arrow, Reload, the tab menu's Reload; signed, local development and built-in apps alike) loads the new version once and moves every window of the app onto it, each keeping its state; if it can't be loaded, no window changes. A second upgrade waits for the first. A link's new window of an app already open runs the version open. An older version opening a database a newer one changed is the app's to handle (DECISION-066).

**Amended (the owner, PRP-049, DECISION-064, 2026-10-30):** a copy is kept per app (host and first key), not per host. A site now signed by another key with no signed trail no longer removes the copy: it stays as the person's version, opens online or offline, and stops updating until its own key (or a rotation from it) signs the site again.

---

### 29. The Pond: Rapidkoi's server, default and self-hostable

**Decision:** Rapidkoi has one server, the **Pond**. Rapidkoi runs the default one (Rapidkoi Pond); anyone can run the same software as their own pond, launched with `./main pond` (beside `pack`, `publish` and `serve` in `commands/`). Switching ponds is changing an address. The rules:
1. **Optional:** everything works without a pond, in a reduced form (sync over the local network keeps working, DECISION-068).
2. **It can't read anything:** it holds only sealed data; the default pond can't read a person's data any more than their own can.
3. **One open protocol, the same software:** a device can move its sealed data from one pond to another with no loss.
4. **No accounts:** devices prove themselves with their keys, never an email or a password.
5. **It learns as little as possible:** who syncs, how much and when, and no more. One's own pond removes even that from Rapidkoi's view.

Its first use is sync: what earlier entries call "the relay" (DECISION-068 to 076) is the Pond. Candidates, each decided in its own entry: recovery (the sealed sync key and device list, DECISION-075), waking phones, live sessions (step 5), and web links (the vision's `open.rapidkoi.app`).

**Why:** The owner (2026-10-31): "the browser will need to have a centralized server for many needs so rapidkoi the org can run the default one while users can host the same and guarantee total ownership of their data"; the name plays on koi ("yes pond"). The rules make self-hosting real ownership: the data's safety is the same on either pond; who sees the metadata and who keeps it running is what differs.

**Rules out:** a pond that's required; a pond that reads data; accounts; a protocol only Rapidkoi's pond speaks. It reverses the general stance against central infrastructure taken when signing was designed (a public log, `docs/source/meetings/2026-10-18-signing-and-panel.md`), as a default under these rules.

---

### 30. Apps can't fingerprint the person

**Decision:** An app, or its server, mustn't be able to recognise a person without asking, across apps, after Start over, or over time, by combining small signals. The rules, for every capability now and later:
1. **Apps never get measurements:** no window size, fonts or pixel data. Rapidkoi draws; apps describe. A canvas can be drawn on but never read back.
2. **Every value an app can read comes from a short list many people share:** steps, not exact numbers (text size in a few steps; time, when apps get it, at coarse precision, and a time zone only if needed).
3. **Requests from every Rapidkoi look the same:** the same headers, nothing about the device.
4. **Every capability's PRP says what it reveals:** what an app can learn, to whom it goes, and how many people share each value.
5. **Visible:** the inspector shows what each app read and which servers it contacted.
6. **Starting over means a new person to the app:** a new key and a clean database: Forget (DECISION-084).

**Why:** The owner (2026-10-31), asked how Rapidkoi avoids fingerprinting: "yes record it". On the web most fingerprinting comes from a page measuring the machine (fonts, screen, canvas, audio); Rapidkoi apps describe a tree and already never learn Rapidkoi's sizes or colours (contract 7), and per-app keys stop matching by key (core decision 17). What's left leaks through what apps can read and how requests look.

**Rules out:** capabilities that hand apps exact device facts; reading pixels back; per-device request headers.

**Amended (the owner, 2026-10-31: "think of the places where apps require a person info to customize something", then "yes record it"):** personal values are shown or adapted by Rapidkoi, asked only when a server needs them, and handed over only by the person. Five patterns, for every capability that touches who the person is or where:
1. **Show:** Rapidkoi fills the value into the screen; the app never learns it (apps can't read the screen). The default for anything only displayed. The name's placeholders already work so.
2. **Adapt:** Rapidkoi adjusts the screen to the person (look, layout, motion, the week's first day, sorting) from rules the app gives; the app never learns why.
3. **Ask:** when a server truly needs the value, a permission in plain words, with a coarse option where one exists.
4. **Hand over:** the person gives it by their own act (filling a form, picking a contact).
5. **Never:** fonts, device model, exact screen, whether a screen reader or other assistive technology is used.

| Personal value | Pattern |
|---|---|
| Name | Show (placeholders, built) |
| Time zone, date style | Show (a time node); a calendar's day logic: Ask |
| Language | Show (the app ships its translations, Rapidkoi picks): its own design, a contract change |
| Number style, units | Show (`number` with a `unit`, formatted by Rapidkoi) |
| First day of the week | Adapt (Rapidkoi's date picker and calendar) |
| Theme, contrast, text size, reduced motion | Adapt (the first three already resolved by Rapidkoi) |
| Screen reader | Adapt; never told |
| Screen width | Adapt (rules like "when narrow, stack", applied by Rapidkoi) |
| Location shown | Show and adapt (Rapidkoi draws "you are here", labels distances, sorts by distance) |
| Location sent | Ask (with a coarse, city option) |
| Contacts | Show by reference; details only through the picker (DECISION-078) |
| Email, phone, address, card | Hand over |
| Region | Ask, or the person picks it in the app |

**Time, the first** (contract 8 or later, when apps get time): "now" in UTC at coarse precision; a time node (`{ "time": …, "show": "time" | "date" | "date_time" | "relative" | "weekday" }`) drawn in the person's zone and date style, "relative" kept current by Rapidkoi; a date and time picker that answers an instant in UTC; a "Know your time zone" permission for apps that need local day logic; schedules in local terms ("every day at 09:00") run by Rapidkoi. Data, never code: Rapidkoi runs no expressions an app sends.

**Accepted leaks:** which item a person taps (their own act), what they ask a server for, and the hours they're active, which hint at a time zone over weeks.

**Amended (the owner, 2026-10-31: "yes record it"):** **anything Rapidkoi works out from the person's values may only become text on the screen:** never structure (which boxes, buttons or typing boxes exist), addresses (a picture's or a request's), messages, or anything else the app or a server can observe, errors and timing included. A general interpreter, where an app's expressions make decisions on the person's values, is ruled out: any decision can be steered into a message the app receives (a button only in one branch), an address a server receives, or a shape the app can later notice. A small formatting language whose only output is text, which always finishes and touches nothing else (like ICU MessageFormat's plurals: `{count, plural, one {# note} other {# notes}}`), keeps the rule and is allowed, most likely with the language and translations design. Parts come first.

**Still leaks, noted:** the IP address on every request (two apps using one tracking server can match people by it: requests through the Pond is DECISION-080); ids an app puts in links that open another app (can't be stopped; the inspector shows them). To check: what `std.http` adds to a request by default.

---

## OPEN — Requires human input before implementation

### DECISION-044 — What "Continue where you left off" means now that a window's state goes with it

**Status:** open (parked by the owner, session 189)
**Raised:** 2026-10-23 — Session 189

**Question:** State lives with its window (the owner, session 189): closing a window deletes its state, and an app opened anew starts fresh. Continue used to reopen the last closed app with its state. Should it become a browser's "reopen closed tab" (the last closed window with its state, this run only, held in memory), or go?

**Meanwhile:** Continue works as it did within a run (a window closed this run is held in memory with its state). After a restart it reopens the app fresh, since its state file is gone. Not redesigned until the owner decides.


---

### DECISION-081 — Who Rapidkoi is for first, among developers

**Status:** open
**Raised:** 2026-10-31, the owner: "what would incentivize devs to build for rapidkoi" (`docs/source/meetings/2026-10-31-developers-and-users.md`)

**Question:** Which developers to aim at first, and what to lead with?

**Recommended (Claude):** independent developers and small teams building local-first tools, led by "a whole app with no backend: offline, synced, multiplayer, private, free to run". Then the gaps in their way, in order: sync and live sessions built; a TypeScript SDK; what a link shows someone without Rapidkoi. Among capabilities, time, files and clipboard next.

---

### DECISION-082 — Who Rapidkoi is for first, among users, and Rapidkoi's own everyday apps

**Status:** open
**Raised:** 2026-10-31, the owner: "what about users" (`docs/source/meetings/2026-10-31-developers-and-users.md`)

**Question:** What is the first reason to install Rapidkoi, and for whom? Should Rapidkoi ship its own everyday apps (notes, tasks, calendar, contacts)?

**Recommended (Claude):** yes: Rapidkoi's own everyday apps, private, offline, synced and sharing data with each other (DECISION-078), each in its own repository like Notes (core decision 27) and on the same contract as everyone's, so they prove the platform too; privacy-minded people who want their data back as the first audience; and an early decision on what a link shows someone without Rapidkoi.

---

### DECISION-083 — Name tokens become parts, in the contract that brings time

**Status:** open
**Raised:** 2026-10-31, from core decision 30's show pattern (`docs/source/meetings/2026-10-31-privacy.md`)

**Question:** `{FIRST_NAME}` and `{LAST_NAME}` inside strings (`src/guest/placeholders.la`) predate parts: a text made of plain strings and values Rapidkoi fills in (`["Hello, ", { "name": "first" }, "!"]`). Replace them, and when?

**Recommended (Claude):** replace them outright (nothing outside the shell uses them: `rapidkoi-rust`, `../notes` and `guest/rust` checked), in the contract version that brings time, so name and time parts arrive together and apps are rebuilt once. A plain string's braces are then shown as typed. Steps: the contract's spec; the decoder; `placeholders.la` filling parts; `test/placeholders.la`; comments in `src/people/` and `src/window/actions.la`; `first_name()` and `last_name()` in `rapidkoi-rust` first; the built-ins repacked.

---

### DECISION-085 — Questions about an app stay on its screen (a future PRP-053)

**Status:** open
**Raised:** 2026-10-31, the owner, checking PRP-052: "does the modal need to have a bg. or of it does can it be contained in the app screen only"

**Question:** Rapidkoi's dialogs cover the whole window (kivuli's root `.dialog`, which dims and freezes everything behind). Should questions about one app (Forget, an app asking for a permission) cover only that app's screen instead, leaving the frame, the address box and other tabs usable, and staying with its tab when you switch away and back (as notices do, PRP-051)?

**Recommended (Claude):** yes, as PRP-053: an app's questions built with kivuli's `overlay` and `.inert(true)` over the app's area, dimmed there only; Rapidkoi's own (About, renaming a window, naming a separate app) stay window-wide. The care needed: Forget closes the tab it asks about, so a question is tied to its own tab, never to whichever tab is in front.

**Meanwhile:** the dialog stays window-wide, and ⚙'s card steps aside while it asks (PRP-052: a popover is drawn above the window).

---

## DEFERRED — Acknowledged, not yet needed

### DECISION-065 — The address box's pills sit 12 px in from its left end, flush at top and bottom (kivuli)

**Status:** deferred (requested: `docs/specs/kivuli-requests.md` § 16)
**Raised:** 2026-10-30, the owner, checking the chip (PRP-050) on screen

**Question:** the owner wants the room around the address box's pills (the chip, the shield's pill) equal on every side. `ui.input_group` insets its add-ons a fixed 12 px from the frame's edge, with only the 1 px edge above and below, and Rapidkoi can't change it: `.style` on the group reaches its outer row only.

**Meanwhile:** the pills keep the 12 px at their start. Nothing is worked around (no address box built without `input_group`). When kivuli lets the caller set the inset, the box insets its add-ons as much as their room above and below.

---

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

### DECISION-080 — Requests through the Pond, to hide the IP address

**Status:** deferred
**Raised:** 2026-10-31, from core decision 30
**Revisit when:** The Pond is built.

**Question:** Every request an app makes shows the person's IP address to its server, and two apps using one tracking server can match people by it. Should Rapidkoi offer to send requests through the Pond, as Apple's Private Relay does? It costs the Pond bandwidth, and the Pond would then see which servers a person's apps contact.

---

## RESOLVED

### DECISION-087 — Blob storage: an app's large bytes in files beside its database

**Status:** resolved (the owner, 2026-10-31: "okay let's build", then "yes" to approving the whole spec, its defaults and its security review); not built yet
**Raised:** 2026-10-31, the owner: "should rapidkoi offer blob storage" ("gap noticed in the platform")

**Question:** An app can keep bytes in a `BLOB` column (4 MB a value, 100 MB a database, DECISION-052), but it can't show a picture it stored (pictures come only from the bundle or a URL, DECISION-056 Q1); large values crowd SQLite's shared memory and the 1 s call limit; and once sync logs changes with the session extension (DECISION-070), every edit of a large value is copied whole into the log's 100 MB. Should Rapidkoi give each app a blob store, and how?

**Decision:** yes, as `docs/specs/blobs.md` sets out, approved whole. In short:
- **Files by hash beside the database:** `databases/<sha256 hex>.blobs/<hash>`, the SHA-256 Rapidkoi computes; nothing the app sends becomes a path. Any bytes; Rapidkoi only sniffs a type to show a picture; a blob is never run, handed to the system or shared with another app.
- **Keys committed with the rows:** `rapidkoi_blobs (key, hash, size, owner_table, owner_key)`, a reserved table in the app's own database. `blob_set` and `blob_delete` are steps of `sql_batch`; the file is written first, then one transaction; a crash leaves at most an orphan file, swept on open.
- **Owners:** a blob may belong to a row by table and single-column primary key; deleting the row deletes its blobs in the same transaction, built only from names read back from the app's schema (amended below: a check, not triggers).
- **Asks:** `blob_get {key, from?, len?, hash?}` (at most 4 MB an answer, each piece checked against the blob's hash), `blob_list {prefix?}`; `image("blob:<key>")` under every picture rule; `fetch` with `body_blob`. Bytes come from the app (A) and go out as a request body (D); a fetch into a blob (B), the file picker (C), write sessions past 16 MB and sync are designed, not built.
- **Defaults:** 16 MB a blob, 1 GB an app apart from its database, 10,000 blobs; signed and local development apps, not built-ins; ⚙'s Data row one total; Clear and Forget remove them.
- **A new contract version:** whichever of blobs and sync (DECISION-071) is built first is contract 8.
- **The SDK:** a helper writing a row and its blob in one batch, and `blob_lines(key)`.

**Why:** files keep large bytes out of SQLite's memory and sync's log, and SQLite itself reads them faster as files past about 100 KB ([sqlite.org/intern-v-extern-blob](https://sqlite.org/intern-v-extern-blob.html)). Committing keys with the rows is the one place both can be atomic, and the one place sync already watches. Owners clean up without Rapidkoi reading the app's data (DECISION-046). Reads checked by hash never join two versions.

**Security review (the owner, approving the spec):** the blob asks and their limits, `blob:` pictures, `body_blob`, the owner check and the reserved table.

**Needs first, under their own rules:** lugha-sqlite's reserved names in `Guard` and the program's own steps in a batch (`docs/specs/lugha-sqlite-reserved-names.md`): built, `v0.2.0`. Later, for write sessions only: lughac file handles.

**Amended (the owner, 2026-10-31: "go with B", building PRP-054):** owners are kept by a check, not triggers. lugha-sqlite `v0.2.0` refuses the app's `DROP TABLE` on a table carrying a reserved trigger, which would block the usual migration (new table, copy, drop, rename). Instead, right after every `sql_batch` commits (not inside it: a batch dropping an owner table would make the check fail and roll the batch back), Rapidkoi deletes the mappings whose owner key no longer exists in its table, and those of a table that's gone. A crash in between leaves only a leak, which the next write's check removes; a table whose primary key is no longer one column keeps its blobs. It asks only whether a key exists, inside SQLite, never returned: DECISION-046's "Rapidkoi doesn't read it" is amended this far. Gives up: changing a row's primary key drops its blobs; a table dropped in one batch and made again later has lost them.

**Rules out:** blobs as values in the app's own tables as the way to keep large bytes; a type the app declares; Rapidkoi reading the app's rows to find what's still used; joining pieces of a blob that changed mid-read.

**A core decision:** yes (decision 26, amended)

---

### DECISION-084 — What a person can do to an app: Forget and one sign-in switch

**Status:** resolved (the owner, 2026-10-31: "can this be simplified to a single action", "how many actions do we actually need by neccessity", then "yes record it"); not built yet
**Raised:** 2026-10-31, from DECISION-079

**Question:** An app's settings offer Start over (its screen), Clear data, Start over as someone new, Sign out and Sign in, Sign in automatically, and Forget. Which are needed?

**Decision:** two, by necessity, plus closing a window (which stops an app and drops that window's state) and, when built, Export.
- **Forget:** closes its windows and deletes their state; removes its pin, its database, its kept copy and its settings here; keeps its trusted key (a swapped key still asks first, PRP-007); and makes the next open someone new: the generation goes up and survives Forget, kept as one number under a hash of the app's identity (no name, not a readable list of apps). Forgetting never brings an old identity back: today's Forget drops the generation to 0, so reopening gives the app the very first key, undoing every Start over.
- **One switch per app, "let this app know who I am":** on, the app gets the person's key and signatures whenever it asks, every run; off, it gets nothing, every run, and works as a guest with its data on this device kept. Back on, the same key.
- **Export** (the vision's data ownership) when it exists, so Forget needn't mean losing data.

**Replaces:** Start over (closing and reopening a window does it, since a window's state goes with it), Clear data (part of Forget), Start over as someone new (Forget, then open it again), Sign out and Sign in (the switch), Sign in automatically (the switch's on), and DECISION-079.

**Why:** one destructive action with one promise ("this app never knew me") is simpler and is what Rapidkoi's privacy rests on (core decision 30, rule 6). Each dropped control was a convenience the remaining ones cover.

**Gives up:** clearing an app's data while staying the same person to its server (rare, mostly repair: a Repair action could come later); signing in for one run only, signed out again at the next start (a shared-computer habit; Rapidkoi is for a personal device; could return as an option on the switch).

**Not decided here:** the words of Forget's dialog and the switch (the owner's); whether Forget on one device forgets on all, when sync is built.

---

### DECISION-079 — Start over clears the app's database

**Status:** superseded by DECISION-084 (Forget does it, and Start over goes)
**Raised:** 2026-10-31, from core decision 30

**Question:** Start over gives an app a new key for the person (`StartOverIdentity`, `src/people/account.la`), but the app's database stays: an id the app stored there recognises the person again. Clear it, or ask?

**Decision:** Start over clears that app's database too, as ⚙ → Data → Clear does, without asking. To the app it's a new person: a new key and no data.

**Why:** a new identity the app can link to the old one through its own storage isn't new (core decision 30, rule 6).

**Not decided here:** the words on Start over saying the app's data goes with it; how it meets sync (a cleared database on one device, when sync is built).

---

### DECISION-078 — Data shared between apps, with the person's consent

**Status:** resolved as a direction (the owner, 2026-10-31: "yes add it to the vision"); designed after sync and live sessions
**Raised:** 2026-10-31, from a talk on what capabilities would draw developers to Rapidkoi

**Question:** Should a person's common data (contacts, calendar, files, tasks) be something apps share, with consent, instead of each app's silo?

**Decision:** Yes, added to the vision as a signature feature ("Your data, across apps") and a built-in service ("Shared data"). Common kinds of personal data have shapes every app understands; an app reads or adds to them only with the person's consent, given in Rapidkoi's own screens (a picker), and receives only what the person chose.

**Why:** today every app rebuilds the same data and moving between apps means exports; it fits the vision's data ownership. Phones share contacts and calendars this way; the web and desktop apps mostly don't. Worth fixing early: once apps keep contacts their own way, sharing them later is hard.

**Not decided here:** which kinds of data, who defines their shapes and how they change, how consent works (each time, or remembered), how it meets sync and storage, and how it's exposed in the contract. Its design is its own discussion, after sync and live sessions. Anything an app can read of another's data is a capability given to untrusted code: the owner's review.

---

### DECISION-077 — Trimming the log

**Status:** resolved (the owner, 2026-10-31: "yes"); the numbers are first guesses, revisited when sync is built
**Raised:** 2026-10-31, from DECISION-070 ("the log only grows")

**Question:** The log catches devices up, keeps deletes until every device has seen them, and holds what history shows (build step 6). When can a change be removed, on a device and on the Pond?

**Decision:**
- **Every change is kept until every device on the list, and the Pond, has it** (known from the summaries devices exchange).
- **The Pond drops changes everyone has.** Devices keep a sealed copy of the current data on the Pond; a new device, or one recovering with its words (DECISION-075), starts from it and takes the changes after it.
- **A device not seen for 90 days isn't waited for.** When it comes back it starts from a fresh copy and sends the changes it made while away; they merge by time, and any for a row that's gone stays in its log, unapplied.
- **On a device, a change everyone has is history alone.** How long losing and replaced changes stay is history's question (step 6); until then they're kept within the log's own cap, 100 MB per app (the size of its storage, DECISION-046), oldest history trimmed first.

**Why:** catching up and deletes need a change only until everyone has it; past that, only history wants it. Waiting forever for a device that's gone would mean never trimming; a fresh copy brings it back without losing what it made.

**Rules out:** trimming a change some device still lacks; a log without a cap.

---

### DECISION-076 — Devices finding and trusting each other on a network

**Status:** resolved as a direction (the owner, 2026-10-31: "yes record it"); cryptography and networking, so the design gets the owner's explicit review when built
**Raised:** 2026-10-31, after DECISION-075

**Question:** On a local network, how do a person's devices find each other, prove who they are, and talk privately; what fills in while `std.net` has "no UDP yet"; and on which networks does a device announce itself?

**Decision:**
- **Mutual TLS:** each side proves it holds a device key on the signed device list (DECISION-075), checked against that list, and the connection is encrypted by OpenSSL through `std.tls`, as Syncthing does. `std.tls` has "no client certificates, pinning": both are a request to lughac, never worked around here.
- **Announcing needs UDP** (mDNS, multicast): a request to lughac, written in `docs/specs/` when sync is built.
- **Remembered addresses:** each device remembers where it last reached each of the person's devices and tries there first. A feature of its own (quicker; works where announcements are blocked), not a stand-in: it can't find a device for the first time on a new network.
- **A device announces itself on every network,** carrying only a tag derived from the person key that looks random to strangers and changes over time. One switch, "Sync over local networks", on by default.
- **Every connection:** prove device keys against the list, check the encryption, exchange summaries then the missing changes, with size caps. Anything fails: the connection closes and nothing is sent.

**Why:** the cryptography stays in OpenSSL, widely used and audited; Noise would need a new package doing cryptography, written or bound here. Per-network choices need Rapidkoi to recognise networks and a screen, for little gain while announcements reveal only that a Rapidkoi device is present.

**Rules out:** Noise, for now; a device's identity in the clear in its announcements; per-network settings, until someone asks.

**Not decided here:** the announcement's format and how often it's sent, the tag's derivation, ports, and the switch's words.

---

### DECISION-075 — Adding and removing a device

**Status:** resolved as a direction (the owner, 2026-10-31: "yes record it"); every part is cryptography or identity, so its design gets the owner's explicit review when built
**Raised:** 2026-10-31, after DECISION-074

**Question:** How does a new device get what it needs to sync, and how is a lost one shut out? Each app's key for the person is derived from the device's master secret (core decision 17), so a new device needs the same master secret to be the same person in every app.

**Decision:**
- **What a device holds:** the master secret (the same on all the person's devices), a sync key (the same on all; seals changes), its own device key (proves which device it is, signs its changes), and the device list (which devices are the person's, signed).
- **Pairing, every day:** on the new device, Add this device; it meets a device already set up, on the Wi-Fi or through the relay; they set up an encrypted connection and both show the same short code worked out from it; the person checks they match and approves on the old device, which sends the master secret, the sync key and the list. No internet or camera needed; a device in the middle would make the codes differ.
- **Recovery words, when no other device is left:** shown once when sync is first turned on, 12 words that are the master secret. Typed on a new device, they give it the master secret; it then fetches the sync key and the list, kept on the relay sealed with the master secret.
- **Removing a device:** from any remaining device; the others make a new sync key and drop it from the list, so it reads nothing sent after, and the relay and the other devices refuse it.
- **The sync key is separate from the master secret,** so removing a device can lock it out of new data.
- **The person's Rapidkoi settings sync too** (per app: Start over's generation, signed out, the name): otherwise one person looks different in an app on another device.

**Weak spot, for DECISION-022:** a removed device still has the master secret and can still sign in to apps as the person; changing it is a new identity in every app (Start over). Sync can't solve it.

**Rules out:** each device its own master secret; the master secret sealing the changes; pairing that needs a camera.

**Not decided here:** the cryptography (the key exchange, how the code is worked out, how the words encode the secret), what `std.crypto` offers for it, and every screen's words.

---

### DECISION-074 — Devices on different versions of an app

**Status:** resolved (the owner, 2026-10-31: "i think option 1 and option 3", then "yes record it")
**Raised:** 2026-10-31, after DECISION-073: one device upgraded an app whose new version adds a column; another, offline, still runs the old one

**Question:** A change for a column the other device's version doesn't have: stop syncing across versions, apply what fits and hold the rest, or bring the other device up to the new version?

**Decision:**
- **Two devices sync an app only when both run the same version of it.**
- **The older device gets the newer version:** from the app's site, as kept apps update (core decision 28); with no internet, directly from a device on the same network that has it, asked for only when two devices meet and one is behind. It's checked exactly as from the site: the same app (host and first key), its signed index, withdrawals.
- **The relay never carries apps,** only sealed changes: it mustn't learn which apps a person uses, apps are large and changes small, and a device that reaches the relay can reach the app's site.
- **Open windows aren't swapped:** the newer version is kept as from the site, used at the next open or when the person upgrades (PRP-044, PRP-048); that app syncs on this device from then. Other apps keep syncing.
- **An app that can't upgrade** (the newer version withdrawn, or its publisher's key changed with no signed trail, DECISION-064) doesn't sync between those devices, and the person is told once.
- **A change for a column or table this device doesn't have is kept in the log, not applied,** and breaks nothing. Changes are untrusted input; this also covers edits made on an old version that arrive after the upgrade.
- **Guidance for developers** (the contract's Storage section, beside `user_version`): add columns, never rename: to rename, add the new column and copy into it; drop an old column in a later version whenever you like (late edits to it are kept in the log and ignored); never reuse an old name for something new.

**Why:** syncing across versions would leave changes for columns one side lacks; bringing the older device up, from the site or a neighbour, lets them meet on one version without anyone waiting long. Migrations can't translate edits that arrive after they've run, and a translator hook would add contract surface and per-rename code for every developer: add-and-copy with the keep-not-apply rule costs nothing new, and unused columns are cheap in SQLite and can be dropped later.

**Rules out:** syncing an app between different versions; the relay storing apps; a hook for apps to translate old changes (it can come later as its own contract change, if a real app needs it).

**Needs the owner's review when built:** a bundle passed between devices (`trust/`).

---

### DECISION-073 — Synced tables have no rules a merge can break

**Status:** resolved (the owner, 2026-10-31: "yes", to option 3)
**Raised:** 2026-10-31, after DECISION-011

**Question:** Merged changes can break a table's rules: two devices apart each make a tag "work" in a table whose names are UNIQUE, and each then refuses the other's row, so they never match; an item added to a list another device deleted (DECISION-070) breaks a FOREIGN KEY; two fields moved apart break a CHECK (start before end). Rapidkoi picking a loser, the app resolving it, or no such rules in synced tables?

**Decision:**
- **A table that syncs may have no UNIQUE (besides its id), no FOREIGN KEY and no CHECK.** Rapidkoi checks its definition when the app creates it and gives the developer a sentence, as for ids (DECISION-071). Part of contract 8's rules.
- **`local_` tables (DECISION-072) keep every rule.**
- **The app handles what a merge can leave in what it shows:** two tags of one name (the person can combine them), an item whose list is gone.

**Why:** it can be checked before anything goes wrong, and devices always end up the same (DECISION-068, goal 3); nothing silently disappears, since a duplicate is visible and fixable. Picking a loser hides a row the person made and is hard to get right for every kind of rule; asking the app breaks goal 5 and leaves devices apart until it answers. cr-sqlite, from memory and unchecked, likewise allows no unique index besides the primary key on the tables it syncs.

**Rules out:** Rapidkoi resolving a broken rule; a conflict message to apps.

---

### DECISION-072 — Which tables sync: all but `local_` ones

**Status:** resolved (the owner, 2026-10-31: "yes", to option 2)
**Raised:** 2026-10-31, after DECISION-071

**Question:** Does every table of an app sync, or does the app mark which do? Some data belongs to one device: a cache, a draft kept here, "last opened on this device".

**Decision:**
- **Every table syncs, except one whose name starts with `local_`,** which stays on its device.
- The mark is in the app's own SQL: no new message in the contract for it (it's part of contract 8's rules, with DECISION-071).
- A `local_` table may number its rows the SQLite way; DECISION-071's check applies only to tables that sync.

**Why:** sync by default keeps DECISION-068's goal 5 (apps don't have to do anything); opting out is one visible word. Syncing everything would carry caches to every device and set "last opened" fighting between them; syncing only marked tables would leave a forgetful developer's data on one device, silently.

**Rules out:** an opt-in list of synced tables; a separate message declaring tables.

---

### DECISION-071 — Ids of synced rows: random, chosen by the app

**Status:** resolved (the owner, 2026-10-31: "yes", to option 1)
**Raised:** 2026-10-31, after DECISION-011: two offline devices both creating row 5

**Question:** SQLite numbers rows as the highest so far plus one (Notes: `id INTEGER PRIMARY KEY AUTOINCREMENT`, `../notes/src/store.rs`). Two devices apart both make note 5; merged field by field (DECISION-011), two notes become one mixed-up note. Random ids, Rapidkoi renumbering, or a range of numbers per device?

**Decision:**
- **A synced table's rows have ids the app chooses at random,** time-ordered so rows still sort by creation. Two devices practically never pick the same; a row has the same id on every device, so links work everywhere.
- **Rapidkoi checks a table's definition when the app creates it.** A table whose ids SQLite assigns (`INTEGER PRIMARY KEY`, `AUTOINCREMENT`) doesn't sync, and the developer gets a sentence saying why.
- **It's a contract change: contract 8,** made when sync is built (a DECISIONS.md entry and a version bump, as every contract change).
- **Elsewhere, under their own rules:** `rapidkoi-rust` gains a helper for new ids; Notes moves to them, and its links change.

**Why:** renumbering behind the app's back breaks links (a link to note 5 from one device opens another note on the next); a range per device needs the app to give ids anyway, since SQLite counts on from the highest number it has, which after a sync is another device's, so it costs as much and protects less. Random ids are what local-first systems use.

**Rules out:** Rapidkoi renumbering rows; ids that mean different rows on different devices.

**Next:** which tables sync: DECISION-072.

---

### DECISION-070 — Deleting a synced row

**Status:** resolved (the owner, 2026-10-31: "yes", to rule 2)
**Raised:** 2026-10-31, after DECISION-011: one device deletes a row that another edited while they were apart

**Question:** When a delete meets an edit made without knowing about it, does the latest time win (the row comes back) or the delete?

**Decision:**
- **A delete wins over edits made by a device that hadn't seen it.** The row stays deleted everywhere, whatever the edit's time. The losing edit stays in the log, as any losing change does (DECISION-011), for history to show and restore.
- **A re-creation after seeing the delete brings the row back.** A device that has the delete no longer has the row, so writing it again is deliberate: the row returns everywhere.
- **The delete is a change in the log** and stays there, so a device that was away learns the row is gone instead of sending it back. The row's earlier changes stay too.

**Why:** people expect what they deleted to stay deleted; rows coming back when another device comes online is a well-known complaint about sync, and it grows with the number of devices. It stays silent (DECISION-011) and loses nothing.

**Rules out:** the latest time deciding between a delete and an edit; removing a deleted row's changes from the log.

**Later:** the log only grows: trimming it is DECISION-077.

---

### DECISION-069 — The time on a synced change

**Status:** resolved (the owner left it to Claude's judgement, 2026-10-31: "here just use your best judgement")
**Raised:** 2026-10-31, after DECISION-011: the latest time wins, but a device's clock can be wrong

**Question:** What time does a change carry, so that a slow or fast clock can't undo edits?

**Decision:**
- **A hybrid logical clock.** Each device remembers the latest time on any change it has seen. A new change takes the later of the device's own clock and that latest time, plus one tick. So an edit made after seeing another change always counts as later, whatever the device's clock says. Edits made apart, neither seeing the other, keep their devices' times: either winning is acceptable, and both stay in the log.
- **A change stamped more than 5 minutes ahead of this device's clock waits** in the log, unapplied, until the clock catches up. It isn't lost, and it can't win against later edits or drag every device's clock forward.
- **The device with the wrong clock says so,** in one short notice, when other devices' changes look far off to it.

**Why:** a slow clock made a later correction lose to the edit it corrected (the worked example: C two hours slow); a fast one would win every conflict for as long as it's wrong. The hybrid logical clock is the usual answer and small; 5 minutes is wide for ordinary drift and narrow for a wrong clock.

**Rules out:** trusting each device's clock alone; a time from a server (sync works without one, DECISION-068).

**Not decided here:** the notice's words, and where it shows, when it's built.

---

### DECISION-011 — How synced app data resolves conflicts

**Status:** resolved (the owner, 2026-10-31)
**Raised:** 2026-10-17 — Session 3

**Question:** When two devices or two people edit the same data, what wins: last write per row, a change log with merge rules per table, or CRDTs on SQLite (e.g. cr-sqlite)?

**Decision (the owner: "a", after a worked example of three devices editing one task while apart):**
- Every change is kept as an entry in a log: which row and field, the new value, the time, and the device that made it.
- For each field, the change with the latest time wins. Equal times are settled by the device, the same way on every device, so all devices pick the same winner.
- It's silent: the person isn't asked or told. Changes to different fields of a row never fight (one device's new title and another's tick both stay).
- The losing changes stay in the log, so nothing is gone (DECISION-068, goal 3). Seeing them comes with history (build step 6).
- The time is a hybrid logical clock (DECISION-069). Deletes: DECISION-070. Row ids: DECISION-071. Table rules: DECISION-073. Versions: DECISION-074.
- Order of arrival doesn't matter: devices that have the same changes show the same data.

**Rules out:** last write per row (it drops another device's edit to a different field); asking the person to pick between versions; deleting a losing change.

**Not decided here:** sharing between people isn't in scope (DECISION-068 is one person's devices).

---

### DECISION-068 — What sync is for

**Status:** resolved (the owner, 2026-10-31)
**Raised:** 2026-10-31, the owner: "let's discuss sync from scratch"; then, lost in the mechanics, "simply answer what do we want"

**Question:** Before choosing how sync works (DECISION-011 and the rest), what must it do?

**Decision (the owner: "yes that's right, record it"):**
1. Every app's data is on all of a person's devices.
2. It works with or without the internet: devices that can reach each other (the same Wi-Fi) sync directly; apart, they sync across the internet.
3. Nothing is lost: devices that changed things while apart end up the same, and no change silently disappears.
4. Only the person's own devices can read it: anything in between sees sealed data only.
5. Apps don't have to do anything: they keep writing to their database, and Rapidkoi syncs it.
6. No single server is required: syncing across the internet goes through a relay, a default one or the person's own, and without any relay, local sync still works.

**Rules out:** sync that needs the internet or one particular server; a relay or network that can read what it carries; sync an app has to code for.

**Not decided here:** how. Conflicts (DECISION-011, since resolved), row ids, sharing with other people, choosing which apps sync, pairing and recovery, how devices find and trust each other on a network, and who runs the default relay are their own questions. Sync stays build step 4; this records what it's for.

**Found while discussing (2026-10-31):** the system's SQLite has the session, changeset and rebaser functions, but `lugha-sqlite` doesn't bind them; `std.net` has TCP but "no UDP yet" (finding devices on a network usually needs it); `std.tls` has no client certificates or pinning. Each is fixed in its own repository when sync is built.

---

### DECISION-067 — A popover over a big app's screen is slow (kivuli)

**Status:** resolved (kivuli 0.64.1 and 0.65.0, PRP-069 and PRP-070 there, 2026-10-31)
**Raised:** 2026-10-31, the owner: ⚙ and the shield open noticeably slower on Notes than on the new tab

**Question:** every message rebuilt the whole window, the app's screen included, though a popover changes only the frame. Rapidkoi now wraps each app's screen in kivuli's `lazy`, keyed on everything it shows (`ui/content.la`, `screen_key`: the tab, a hash of the app's state, the theme and text size, the looks switches, the name filled in, picture progress), so its own share of a message is under 10 ms whatever the app's size. Measured with Notes from notes.dafifi.net (`perf`, then a probe logging each message), kivuli still cost 70–180 ms a message: kivuli 0.64 re-added every rule an unchanged lazy part held, and its allocations brought full collections over Notes' big heap. And one popover was five messages: GTK reported the window inactive and active twice as its own popover took focus, and each return to the front started a check against the app's site (PRP-044).

**Outcome:** kivuli 0.64.1 (PRP-069) makes a message cost the same whatever an unchanged lazy part holds: the checks outside it are one pass. kivuli 0.65.0 (PRP-070) makes `Sub.window_active` report only the window coming to and leaving the front; its own popovers, menus and drop-downs send nothing, so a popover starts no site check. Rapidkoi moves to 0.65 with no code change. What's left per message grows with the nodes outside the lazy part (about 70 µs a node), not with the app's size.

---

### DECISION-066 — An older version opening data a newer one changed: the app's to handle

**Status:** resolved (the owner, 2026-10-30)
**Raised:** 2026-10-30, after PRP-048: the second half of "what happens when an app changes its schema"

**Question:** PRP-048 stops two versions of an app using its database at once. An older version can still open a database a newer one changed: a withdrawal (the site's current falls back), a link naming an older version (PRP-036), or an older build published again. Should Rapidkoi keep copies, refuse older versions, or leave it to the app?

**Decision (the owner):** app developers take care of it. The database is the app's: Rapidkoi doesn't read it, migrate it, keep copies of it or compare versions against it. Rapidkoi guarantees one version per database at a time (PRP-048) and that `PRAGMA user_version` is the app's to read and write (lugha-sqlite's `Guard.untrusted()` allows it). The contract's Storage section says how: record the shape in `user_version`, migrate in one `sql_batch`, and check it on open before writing.

**Rules out:** Rapidkoi copying databases before upgrades; Rapidkoi refusing an older version by its version number; Rapidkoi running migrations.

**Later, in rapidkoi-rust (its own repository and rules):** a migrations helper that reads `user_version` first and refuses data newer than the app knows, and Notes no longer setting `user_version = 1` unconditionally.

---

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

**Amended (the owner, 2026-10-30, after checking it on screen):** "Open as a separate app" opens the new app in place, in the tab it was asked from, not in a new one: the person's version becomes "where you left off" (Continue brings it back with its window's state), and the chip's list moves between them. Caution is amber (`caution` in the theme), not the brand orange; the apps' palette has cyan in place of yellow.

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
