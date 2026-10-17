# CONTEXT.md — rapidkoi

Session handoff file. Updated at the end of every session.
Read at the start of the next session alongside CLAUDE.md, MEMORY.md, and DECISIONS.md.

Every session has a name and a state: open | closed.
A session is closed only after CONTEXT.md is committed (and pushed, once a remote exists).

---

## SESSION 1 — 2026-10-17 — Set up the context system — closed

Branch: main

### WHAT WAS DONE

Set up the context engineering system from `docs/setup.md`, adapted to Lugha and kivuli (modelled on `../kivuli`'s setup). Initialised the git repository. No app code changed.

### FILES CREATED OR MODIFIED

```
CLAUDE.md               — behavioral rules for Lugha/kivuli
MEMORY.md               — resolved decisions 1–3, project state
CONTEXT.md              — this log
DECISIONS.md            — DECISION-001 (purpose, open), DECISION-002 (publishing, deferred)
CHANGELOG.md            — Keep a Changelog skeleton
.llmignore              — protected paths
.claude/settings.json   — deny rules mirroring .llmignore
PRPs/TEMPLATE.md        — PRP template for a kivuli app
PRPs/DISCOVERY.md       — discovery interview protocol
docs/CODE_STYLE.md      — doc-comment rules for .la code
docs/DESIGN.md          — spacing/text tokens, kivuli style use
docs/setup.md           — moved from the repo root
docs/source/…, docs/specs, docs/decisions, docs/incidents, docs/status, reports/ — empty folders (.gitkeep)
```

### TESTS WRITTEN

- None (no code).

### DECISIONS MADE

- File size limit set to 300 lines (setup.md default).
- Built executable is `./main` (`lughac build src/main.la -o main`), matching `.gitignore`.

### PENDING DECISIONS OPENED

- DECISION-001 — what rapidkoi is for (open).
- DECISION-002 — publishing (deferred).

### STILL OPEN AT CLOSE

- DECISION-001 blocks all feature work.
- No remote; nothing pushed.

---

## SESSION 2 — 2026-10-17 — Record the vision — closed

Branch: main

### WHAT WAS DONE

The owner gave the Rapidkoi vision. Recorded it verbatim as a stakeholder note with its implications, resolved DECISION-001, and opened the four architecture questions that block build path step 1 (the shell). CLAUDE.md now describes the project and treats WebAssembly guests as untrusted input. No code changed.

### FILES CREATED OR MODIFIED

```
docs/source/stakeholder/vision-2026-10-17.md — the vision, verbatim, with summary and implications
DECISIONS.md — DECISION-001 resolved; 003–006 opened; 007–009 deferred
MEMORY.md    — decision 4 (what rapidkoi is); state and next step
CLAUDE.md    — project description, guest security rules, architecture rules 5–6, anti-pattern 8
CONTEXT.md   — this entry; NEXT SESSION START POINT moved below the sessions
```

### TESTS WRITTEN

- None (no code).

### DECISIONS MADE

- DECISION-001 resolved: this repo is the Rapidkoi shell (see MEMORY.md decision 4).

### PENDING DECISIONS OPENED

- DECISION-003 — WebAssembly runtime (recommended: wasmtime).
- DECISION-004 — where app state lives (recommended: platform-held model, Elm-shaped guests).
- DECISION-005 — guest↔host UI contract (recommended: a versioned serialised tree per render).
- DECISION-006 — first guest language (recommended: Rust, plus `.wat` test fixtures).
- Deferred: 007 platforms beyond GTK, 008 process isolation, 009 manifest/bundle/signing.

### STILL OPEN AT CLOSE

- DECISIONS 003–006 block step 1.
- No remote; nothing pushed.

---

## SESSION 3 — 2026-10-17 — Record the architecture decisions — closed

Branch: main

### WHAT WAS DONE

Discussed step 1's architecture with the owner in chat (sessions 2–3) and recorded the outcome: DECISIONS 003–006 resolved, a new project goal (grow Lugha's ecosystem, DECISION-010), conflict handling for synced data deferred (DECISION-011). Created the sibling repository `../lugha-wasmtime` (package `wasmtime`) with its own context system, on the owner's go. No rapidkoi code changed.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-17-architecture.md — the chat's decisions and impact
DECISIONS.md — 003–006 and 010 resolved; 011 deferred
MEMORY.md    — decisions 5–9; state; next step
CLAUDE.md    — ecosystem goal, generic-code rule, stack (wasmtime, Rust guests, SQLite later), lugha-wasmtime protected
.llmignore, .claude/settings.json — ../lugha-wasmtime added
../lugha-wasmtime/ — new repo: lughac init --lib, CLAUDE.md, MEMORY.md, CONTEXT.md, DECISIONS.md (001 install/version, 002 API layers, 003 license; 004 WASI deferred), templates
```

### TESTS WRITTEN

- None (no code).

### DECISIONS MADE

- See DECISIONS 003, 004, 005, 006, 010 and the meeting note.

### PENDING DECISIONS OPENED

- DECISION-011 — conflict handling for synced data (deferred to step 4).
- In lugha-wasmtime: its DECISIONS 001–003 (open), 004 (deferred).

### STILL OPEN AT CLOSE

- Step 1 needs its discovery interview and PRP-001.
- The shell's PRP depends on lugha-wasmtime's first PRP, which is blocked on its DECISIONS 001–002.
- The owner installs wasmtime's C API and adds Rust's `wasm32-unknown-unknown` target.

---

## SESSION 4 — 2026-10-17 — Record the UI contract — closed

Branch: main

### WHAT WAS DONE

Checked `../lugha-wasmtime` against the shell's needs: everything is there (56/56 tests, example runs, a probe built kivuli + wasmtime together via a `path` package). Designed UI contract v1 with the owner in chat and recorded it. Set up `../lugha-cbor` (package `cbor`) with its own context system, on the owner's go. No rapidkoi code changed.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-17-ui-contract.md — contract v1 in full
DECISIONS.md — 012 (contract v1), 013 (lugha-cbor) resolved
MEMORY.md    — decisions 10–11; state; next step
CLAUDE.md    — stack: wasmtime path package, cbor, contract v1 rule; lugha-cbor protected
.llmignore, .claude/settings.json — ../lugha-cbor added
../lugha-cbor/ — new repo: lughac init --lib, context system, hostile-input rules
```

### TESTS WRITTEN

- None (no code).

### DECISIONS MADE

- DECISIONS 012 and 013.

### PENDING DECISIONS OPENED

- None here. lugha-cbor DECISION-001 (streaming) deferred.

### STILL OPEN AT CLOSE

- lugha-cbor PRP-001, then rapidkoi PRP-001 (the shell).

---

## SESSION 5 — 2026-10-17 — PRP-001: the shell — closed

Branch: main

### WHAT WAS DONE

Checked `../lugha-cbor` (merged, 86/86 tests, hostile-input probe passed). Researched kivuli's API and GTK startup, lughac's `args`/`read_bytes`/`std.fs`, and the wasmtime and cbor APIs. Wrote PRP-001 (the shell) for the owner's approval. No code written.

### FILES CREATED OR MODIFIED

```
PRPs/prp-001-shell.md — the shell: contract v1, calling convention, modules, tests, Rust guest
DECISIONS.md          — 014 (kivuli single-instance) and 015 (std.fs size) deferred
```

### TESTS WRITTEN

- None (PRP only).

### DECISIONS MADE

- Proposed in PRP-001, pending approval: the i64 (ptr, len) convention; no dealloc; the guest called from the shell's kivuli `update` only, with rule 3 reworded; minicbor for the Rust crate; offline fuel calibration (no clock in std).

### PENDING DECISIONS OPENED

- DECISION-014, DECISION-015 (deferred, gaps upstream).

### STILL OPEN AT CLOSE

- PRP-001 awaits approval.

---

## SESSION 6 — 2026-10-17 — Build PRP-001: the shell — closed

Branch: prp-001-shell (not merged)

### WHAT WAS DONE

Built PRP-001 test-first on `prp-001-shell`. The shell loads a `.wasm` app, checks memory and version, and runs init/update/view under contract v1 with every limit. It decodes CBOR into a Node tree, renders through kivuli, and stops a failing app behind an error screen with Restart. A Rust crate (`guest/rust/rapidkoi`) and counter demo work end to end (`test/counter.la`). On the owner's word, wasmtime and cbor come from GitHub (v0.1.0) instead of path packages. Not merged: `src/contract.la` is 368 lines (limit 300) and its split needs approval; DECISION-016 is open.

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock — wasmtime and cbor git packages (lock written by lughac)
src/main.la            — wiring: sandbox(boot(args()), update, view)
src/limits.la          — every limit; FUEL_PER_CALL calibrated (700M ≈ 100 ms, i5-4308U)
src/theme.la           — DESIGN.md tokens, Space → pixels
src/contract.la        — pure CBOR ↔ contract v1, three-layer limits (368 lines: over the limit)
src/render.la          — Node → kivuli View, error screen, Msg
src/guest.la           — store, limits, calling convention, GuestError, reason()
src/shell.la           — Model (Running | Stopped), boot, launch, update, view
test/support.la contract.la render.la guest.la shell.la counter.la — 47 tests
guest/rust/            — workspace: rapidkoi crate (View, App, export!, arena), counter
CLAUDE.md              — rule 3 reworded (shell's update calls the guest), COMMANDS (cargo), tree, known issues
DECISIONS.md           — 016 (kivuli read-only input, open), 017 (wasmtime export listing, deferred)
CHANGELOG.md           — the shell, safe failure, Rust library + demo
PRPs/prp-001-shell.md  — marked approved; deps line → git
```

### TESTS WRITTEN

- test/contract.la (15): kinds, props, styles, unknown props/kinds, error paths, node/depth/string/size limits, update results, envelopes.
- test/render.la (7): texts, button message/disabled, typing, placeholder, error screen with/without Restart.
- test/guest.la (14): good guest, bytes written reach update, versions, missing memory/version, missing/mistyped export, trap, out of fuel (and recovery), out-of-bounds pointer, huge length, bad alloc pointer, closed, compile errors.
- test/shell.la (8): start, envelope reaches update, state stored, crash → Stopped → Restart, bad tree, bad update result, boot errors.
- test/counter.la (3): the real Rust counter end to end (skips until built).
- guest/rust (7): exact CBOR of a text and a column, disabled button, envelope decode, arena lifetime, counter logic and view.

### DECISIONS MADE

- Messages are kept as decoded CBOR values and re-encoded in preferred serialisation: the same value, not always the same bytes as the guest sent (byte-exact would need raw item spans from lugha-cbor).
- Fixture guests are generated WebAssembly text in `test/support.la` instead of `test/fixtures/*.wat` files: the blobs come from `cbor.encode`, so they can't drift from the contract.
- Fuel calibrated offline with `/usr/bin/time` (std has no clock).

### PENDING DECISIONS OPENED

- DECISION-016 (open): kivuli read-only text input.
- DECISION-017 (deferred): lugha-wasmtime export listing.

### STILL OPEN AT CLOSE

- Split of `src/contract.la`, proposed to the owner: `nodes.la` (the types, ≈90 lines), `props.la` (prop readers, ≈75), `contract.la` (read_tree/read_update/envelope and the walk, ≈210, re-exporting nodes' types so no caller changes).
- Merge of `prp-001-shell` after the split, with the full VALIDATION run again.
- The owner opens the window: `./main guest/rust/target/wasm32-unknown-unknown/release/counter.wasm`.

---

## SESSION 7 — 2026-10-17 — Split contract.la; merge PRP-001 — closed

Branch: prp-001-shell → merged into main

### WHAT WAS DONE

The owner approved the split. `src/contract.la` (368 lines) became `nodes.la` (types, 78), `props.la` (prop readers, 81) and `contract.la` (decoding and the walk, 232), with `contract` re-exporting the types so no caller changed. Every VALIDATION command passed, and PRP-001 was merged into `main`.

### FILES CREATED OR MODIFIED

```
src/nodes.la    — new: Node, TextStyle, ButtonLook, Space, ContractError, invalid()
src/props.la    — new: text, optional_text, word, text_style, button_look, space
src/contract.la — read_tree, read_update, envelope, the walk; re-exports nodes' types
CLAUDE.md       — file tree and import order
```

### TESTS WRITTEN

- None new; all 47 Lugha and 7 Rust tests pass after the split.

### DECISIONS MADE

- Inside `contract.la`, `invalid` is called as `nodes.invalid` (a module is imported once, and the re-export binds `nodes`); the local `props` became `given` (E0311 with the `props` module).

### PENDING DECISIONS OPENED

- None. DECISION-016 still awaits the owner (recommended: A, a disabled text field in kivuli).

### STILL OPEN AT CLOSE

- DECISION-016.
- No remote for rapidkoi (DECISION-002); nothing pushed.

---

## SESSION 8 — 2026-10-17 — Record Rapidkoi's UI design; propose PRP-002 — closed

Branch: prp-002-frame

### WHAT WAS DONE

Recorded the UI design the owner sketched in chat (frame, tab rail, home, finding things again, pictures, info panel), with their two sketches. Recorded DECISION-016 as A. On the owner's "placeholders where you can, don't get blocked", recorded DECISION-018 and wrote PRP-002 (frame and tabs) with a placeholder table. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-17-ui-design.md   — the design discussion and decisions
docs/source/meetings/2026-10-17-ui/*.png       — the owner's two sketches
docs/DESIGN.md                                 — the frame's tokens and icons
DECISIONS.md — 016 resolved (A), 018 resolved (placeholders), 019 deferred (state across restarts)
MEMORY.md    — decision 12 (Rapidkoi's own UI)
PRPs/prp-002-frame_and_tabs.md — for approval
```

### TESTS WRITTEN

- None (design only).

### DECISIONS MADE

- DECISION-016 (A), DECISION-018 (placeholders).

### PENDING DECISIONS OPENED

- DECISION-019 (deferred to PRP-003).

### STILL OPEN AT CLOSE

- PRP-002 awaits approval. It includes extending rule 3 to opening a local app file in `update`.
- kivuli work for the owner: disabled text fields (prompt given), then a file dialog, an overlay, context menus and a wrapping grid.

---

## SESSION 9 — 2026-10-17 — Build PRP-002: frame and tabs — closed

Branch: prp-002-frame (not merged)

### WHAT WAS DONE

Built PRP-002 test-first: the contract's root `title`, tabs.la (pages and their life, moved out of shell.la), actions.la, the pure frame/home/panel views, a new shell.la (tabs, address box, pins, left off, Continue restoring state), and main.la on kivuli `application` (for Ctrl+K focus). The Rust crate gained `screen_title`. 70 Lugha tests and 8 Rust tests pass, and the build works. Not merged: `src/shell.la` is 305 lines (limit 300), and the owner's approval of its split is pending.

### FILES CREATED OR MODIFIED

```
src/tabs.la (new)    — Screen, Program, AppPage, Page, Tab, Pin, LeftOff; open_path/open_module/handle/leave/resume
src/actions.la (new) — the shell's Msg
src/frame.la (new)   — bar, rail, label, initial, icon_button
src/home.la (new)    — home.view, matches; tiles in rows of 4
src/panel.la (new)   — panel.view
src/shell.la         — rewritten: Model, boot, with_pages, update, view, title_of (305 lines)
src/main.la          — kivuli application
src/contract.la, nodes.la, props.la — Described { root, title }, maybe_text
src/theme.la         — frame tokens; window 960 × 700
test/tabs.la test/views.la (new); test/shell.la test/counter.la (rewritten); test/contract.la, support.la
guest/rust           — View::screen_title; the counter titles itself "Counter"
CLAUDE.md            — rule 3's second exception, file tree, imports, placeholder KNOWN ISSUES
docs/DESIGN.md, CHANGELOG.md
```

### TESTS WRITTEN

- tabs (6): start with title, events/restart, crash then ignore, leave/resume with state, resume after a crash, open paths.
- views (9): bar on home and on an app, rail, labels, empty home, left off and pinned, search, matching, panel.
- shell (12): boot, events per tab, crash isolation and restart, open/select/close, close all, home → Continue restores state, address box notices, editing, pins, panel, titles.
- counter (5, e2e): adds Home→Continue keeping the count, and opening a pin.
- contract (+2): root title, non-text title, nested title ignored.

### DECISIONS MADE

- Continue keeps the compiled module in memory (LeftOff.program), so it needn't re-read the file. That also makes it testable with fixtures.
- On home, ⌂ carries Ctrl+K (it moves to the search box); on an app page the address button does. There's only ever one enabled Ctrl+K.
- Rust: `screen_title` rather than `title` (`title()` already meant the text style).
- `home.view` / `panel.view` instead of `home()` / `panel()` (a function can't share its module's name when imported).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The split of `src/shell.la`, proposed: `src/window.la` gets Model, with_pages, title_of and the helpers (go, open_here, close, leave_current, with_page, page_of, has, pinned), about 150 lines. `shell.la` keeps boot, update, view and app_view, about 160, re-exporting Model, with_pages and title_of so no caller changes.
- The owner checks the window on screen (PRP-002 VALIDATION).

---

## NEXT SESSION START POINT

Open session 10 on `prp-002-frame`. If the owner approved the split, move Model and the helpers into `src/window.la` (shell.la re-exports them), run every VALIDATION command, update the CLAUDE.md tree, and merge into `main`. Then PRP-003 (remembering), and the kivuli prompt for the placeholders' real features when the owner wants it.
