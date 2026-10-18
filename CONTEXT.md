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

## SESSION 10 — 2026-10-17 — PRP-002 amendments: split, home not a tab, themes — closed

Branch: prp-002-frame → merged into main

### WHAT WAS DONE

After trying PRP-002, the owner asked for: the split (approved), home not being a tab, themes starting with dark (bars `#242424`, home `#161616`), and starting maximized ("full screen… because it is a browser", read as maximized with the title bar kept). Recorded as PRP-002 Amendment 1, built test-first, and merged. The theme and maximizing are placeholders until kivuli can apply a stylesheet and maximize (DECISION-020). The owner parked "what looked off in the style" until they raise it.

### FILES CREATED OR MODIFIED

```
src/window.la (new) — Model and what changes it: show_tab, go_home, open_file, go, close, reopen, pin, card, page_of, title_of
src/themes.la (new) — Theme, DARK, stylesheet(); rk-bar / rk-rail / rk-home
src/shell.la        — boot, update, view only (110 lines); re-exports Model, with_pages, title_of
src/tabs.la         — Tab holds an AppPage (home isn't a tab)
src/frame.la        — rail lists apps only, current is Option
src/home.la         — Card (name, title, msg): Select(id) for an open app, Continue for a closed one
test/shell.la (rewritten), test/views.la, test/counter.la
DECISIONS.md (020), docs/DESIGN.md (theme colours, rule), CLAUDE.md (design rule, tree, imports, known issues), CHANGELOG.md, PRPs/prp-002 (amendment)
```

### TESTS WRITTEN

- shell (14): no tabs on boot, + and ⌂ keep apps, the card selects an open app, a closed app reopens with its state, neighbours then home, the address box from home vs from an app, pins, panel, titles.
- views (+1, now 10): the app-only rail, cards, the dark stylesheet.
- counter e2e: ⌂ → Continue, and close → Continue, both keep the count.

### DECISIONS MADE

- `select` is a Lugha keyword, so it's `window.show_tab`.
- Typing an address on an app page replaces that tab's app (as a browser navigates a tab); from home it opens a new tab.
- "Full screen" means maximized: true full screen would hide the window controls the owner keeps.

### PENDING DECISIONS OPENED

- DECISION-020 (deferred): kivuli additions behind the placeholders.

### STILL OPEN AT CLOSE

- The kivuli work (DECISION-016, DECISION-020); the owner runs it.
- Style feedback: the owner will say what looked off.

---

## SESSION 11 — 2026-10-17 — PRP-002 Amendment 2: kivuli 0.2 (header, theme, maximized, disabled inputs) — closed

Branch: prp-002-amend2 → merged into main

### WHAT WAS DONE

kivuli 0.1.1 (disabled text fields) and 0.2.0 (typed looks, `.maximized()`, `.header`) shipped. The owner approved swapping the placeholders, recorded as PRP-002 Amendment 2. Rapidkoi now:
- has its bar in the window's header bar (the owner's screenshot request)
- opens maximized
- paints the rail `#242424` and home `#161616` through kivuli looks
- shows inputs without `on_change` disabled

The header bar itself keeps the system's colour: kivuli can't style it yet (DECISION-020). Not verified on screen by me: the owner checks.

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock — kivuli 0.2 (lock: kivuli 0.2.0, lugha-gtk 0.1.23, by lughac update)
src/themes.la  — rewritten: Look { Rail, Home }, Theme, DARK, kivuli_theme(), surface()
src/frame.la   — Bar { start, title, end } for the header; rail styled Look.Rail
src/shell.la   — .maximized().theme(…).header(…); home styled Look.Home
src/contract.la, src/render.la — inputs without on_change: Node.Input(…, None), rendered .disabled(true)
src/*.la       — .style(ButtonStyle…) → .button_style(…)
test/views.la, test/contract.la, test/render.la — bar parts, theme surfaces, disabled inputs
CLAUDE.md, DECISIONS.md (016 shipped, 020 progress), docs/DESIGN.md, CHANGELOG.md, PRPs/prp-002 (Amendment 2)
```

### TESTS WRITTEN

- views: the theme's surfaces (backgrounds only); the bar tests read the header parts in one row.
- contract: an input without on_change decodes as disabled.
- render: a disabled input is greyed out and silent (`is_field_disabled`, `Lookup.Silent`).

### DECISIONS MADE

- kivuli's `Theme`/`theme` are imported as `Looks`/`looks` in themes.la, which has its own `Theme`.
- Only backgrounds are set; text stays the GTK theme's (dark on the owner's desktop).

### PENDING DECISIONS OPENED

- None. DECISION-020 now lists what's left: a header bar look, a file dialog, an overlay, context menus, a wrapping grid.

### STILL OPEN AT CLOSE

- The owner's on-screen check, and their style feedback when they're ready.

---

## SESSION 12 — 2026-10-17 — The frame look: one frame colour, the page as a card — closed

Branch: frame-look → merged into main

### WHAT WAS DONE

The owner sent a screenshot ("topbar and sidebar should have same color. remove that shadow under topbar") and an Opera-style inspiration. Measured the screenshot:
- header `#272727` with a 2 px `#0a0a0a` shadow
- rail `#242424`, with a `#2c2c2c` strip around it: the rail's margin showed GTK's window background
- home `#161616`

Rapidkoi now paints the whole window `Look.Frame` (`#242424`), which removes the strip and puts the rail on the frame, and the page is a rounded `Look.Page` card (`#161616`, 12 px) inset by `SPACE_SM`. The header bar's colour and shadow need kivuli (DECISION-020 item 1, prompt given to the owner).

### FILES CREATED OR MODIFIED

```
src/themes.la  — Look { Frame, Page }; Theme { frame, page }; Page has PAGE_RADIUS
src/theme.la   — PAGE_RADIUS = 12
src/shell.la   — root .style([Look.Frame]); card(): the page as an inset rounded card
src/frame.la   — the rail has no look of its own (it sits on the frame)
test/views.la  — the theme's surfaces
docs/DESIGN.md, DECISIONS.md (020 item 1 measured), CLAUDE.md, CHANGELOG.md
docs/source/meetings/2026-10-17-ui/screenshot-3-after-amendment-2.png, inspiration-opera-frame.png
```

### TESTS WRITTEN

- views: frame and page surfaces (colour, radius; text untouched).

### DECISIONS MADE

- App pages get the same card as home (`#161616`), as in the inspiration.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- kivuli: the header bar's look and no shadow.

---

## SESSION 13 — 2026-10-17 — The header bar takes the frame look (kivuli 0.3) — closed

Branch: header-look → merged into main

### WHAT WAS DONE

kivuli 0.3.0 shipped `.header_style([…])` and `Surface.flat`. Rapidkoi now depends on kivuli 0.3, gives the header bar `Look.Frame`, and makes the frame flat. The top row and the rail are one `#242424` surface, with no shadow (the owner's session 12 request).

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock — kivuli 0.3 (0.3.0, by lughac update)
src/themes.la — Frame is flat; doc comment
src/shell.la  — .header_style([Look.Frame])
test/views.la — the frame is flat, the page isn't
CLAUDE.md, DECISIONS.md (020: item done, renumbered), docs/DESIGN.md, CHANGELOG.md
```

### TESTS WRITTEN

- views: Frame flat, Page not.

### DECISIONS MADE

- None new.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 14 — 2026-10-17 — The page flush with the frame; one top-right curve (placeholder) — closed

Branch: flush-page → merged into main

### WHAT WAS DONE

The owner (screenshot 4): "remove that margin/padding. only the top right curve should be visible." The page's `SPACE_SM` margin is gone: it now meets the header bar, the rail and the window's edges. kivuli's `Surface.radius` rounds all four corners or none, so the page is square for now, and per-corner radius is DECISION-020 item 1 (kivuli prompt given to the owner).

### FILES CREATED OR MODIFIED

```
src/shell.la  — card(): no margin
src/themes.la — Page has no radius until one corner can curve
src/theme.la  — PAGE_RADIUS documented as the top-right curve
test/views.la — Page radius None
docs/DESIGN.md, DECISIONS.md (020 item 1), CLAUDE.md, CHANGELOG.md, docs/source/meetings/2026-10-17-ui/screenshot-4-after-header-look.png
```

### TESTS WRITTEN

- views: the page is square for now.

### DECISIONS MADE

- Square corners rather than four curves while waiting: four would add curves at the window's edges, which the owner doesn't want.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- kivuli: per-corner radius.

---

## SESSION 15 — 2026-10-17 — The rail as thick as the top bar — closed

Branch: even-frame → merged into main

### WHAT WAS DONE

The owner: "make the top bar and sidebar have the same width". Read as thickness: the rail's width equals the header bar's height. Measured screenshot 4: header 70 px, rail 113 px (the old 64 + 2 × 8 = 80), so the header is about 50. New tokens: FRAME_THICKNESS = 50, TAB_SIZE = 34 (the header's icon-button size), RAIL_WIDTH = FRAME_THICKNESS − 2 × SPACE_SM = 34. The header bar's height is GTK's: if the owner's next screenshot shows a mismatch, adjust FRAME_THICKNESS.

### FILES CREATED OR MODIFIED

```
src/theme.la   — FRAME_THICKNESS, TAB_SIZE 34, RAIL_WIDTH derived
docs/DESIGN.md — the frame tokens
```

### TESTS WRITTEN

- None (sizes only; all 74 pass).

### DECISIONS MADE

- "Same width" means the rail's width equals the header bar's height.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner checks it on screen.

---

## SESSION 16 — 2026-10-17 — The page's top-right curve (kivuli 0.4) — closed

Branch: page-curve → merged into main

### WHAT WAS DONE

kivuli 0.4.0 (per-corner radius, content clipped to the curve) was tagged, and the owner pushed it. Rapidkoi now depends on kivuli 0.4 (lock: kivuli 0.4.0, lugha-gtk 0.1.24), and the page curves only at its top-right corner (`radius_top_right: PAGE_RADIUS`, 12 px), as the owner asked in session 14.

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock — kivuli 0.4
src/themes.la, src/theme.la — the page's top-right curve
test/views.la — only the top-right corner curves
docs/DESIGN.md, DECISIONS.md (020: done, renumbered), CLAUDE.md, CHANGELOG.md
```

### TESTS WRITTEN

- views: radius_top_right 12, the other corners unset.

### DECISIONS MADE

- None new.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- DECISION-020's remaining kivuli items: a file dialog, an overlay, context menus, a wrapping grid.

---

## SESSION 17 — 2026-10-17 — Center the frame's icons — closed

Branch: center-icons → merged into main

### WHAT WAS DONE

The owner: "center align all icons". The rail's items are now `.align(Align.Center)`: each at its own size on the rail's center line. The separator keeps the rail's width. Found the cause of off-center icons: kivuli builds a button's content in a GtkBox left at fill alignment, so content sits at the start of any button wider than it. It's a kivuli bug (DECISION-020 item 0); a fix prompt was given to the owner. Header icons are GTK's header bar layout, centered vertically already.

### FILES CREATED OR MODIFIED

```
src/frame.la — rail items centered; separator full width
DECISIONS.md (020 item 0), CLAUDE.md (known issue)
```

### TESTS WRITTEN

- None (layout only; all 74 pass).

### DECISIONS MADE

- Centering the rail's items is correct layout, not a workaround; the button-content fix belongs to kivuli.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- kivuli: center button content.

---

## SESSION 18 — 2026-10-17 — kivuli 0.4.1: centered button content — closed

Branch: kivuli-0.4.1 → merged into main

### WHAT WAS DONE

kivuli 0.4.1 (button content centered) was tagged and pushed. `lughac update` locked it. The tab squares and home's tiles now center their letter. The known issue and DECISION-020 item 0 are gone. The rail keeps its centered items.

### FILES CREATED OR MODIFIED

```
lugha.lock — kivuli 0.4.1
DECISIONS.md (020), CLAUDE.md (known issue removed)
```

### TESTS WRITTEN

- None (all 74 pass).

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- DECISION-020: a file dialog, an overlay, context menus, a wrapping grid.

---

## SESSION 19 — 2026-10-18 — Record the packaging discussion; propose PRP-003 (bundles) — closed

Branch: prp-003-bundles

### WHAT WAS DONE

Recorded the packaging discussion: the problem first, code shapes A–D (A chosen, room for D), vocabulary (one app per host), keeping and withdrawing versions, versioned state (start over without `migrate`), the hosting layout, and signing (deferred). Wrote PRP-003 (bundle format 1: a single-file `.rkb`, `./main pack` with flags, opening by path). No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-packaging.md — the discussion and decisions
DECISIONS.md — DECISION-009 partly settled
MEMORY.md    — decision 13 (packaging)
PRPs/prp-003-bundles.md — for approval
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- See the meeting note. "Remembering" becomes PRP-004.

### PENDING DECISIONS OPENED

- None (signing stays in DECISION-009).

### STILL OPEN AT CLOSE

- PRP-003 awaits approval. kivuli: an image from bytes (for bundle icons).

---

## SESSION 20 — 2026-10-18 — Build PRP-003: bundles — closed

Branch: prp-003-bundles → merged into main

### WHAT WAS DONE

Built PRP-003 test-first:
- **Bundle format 1** (`bundle.la` read/write, `rules.la` every check, used both ways).
- **The `./main pack` command** (`pack.la`; main runs it without a window).
- **Opening `.rkb` by path** (`loader.la`, new: a path → manifest + compiled module; `tabs.open_path` now calls it).
- **The manifest shown:** the name in the tab and title; version, description, `📦 … · local bundle` and the state version in the panel and address.
- **The counter demo:** a 128 × 128 icon.png.

95 Lugha tests pass; the real `./main pack` works (summary and exit 0; refusing to overwrite; unreadable files, exit 1).

### FILES CREATED OR MODIFIED

```
src/bundle.la, src/rules.la, src/pack.la, src/loader.la (new)
src/tabs.la    — AppPage.manifest; LeftOff.state_version and manifest; open_path via loader (281 lines)
src/main.la    — `pack` subcommand
src/frame.la   — marker(): 📦 bundle / 📄 file
src/panel.la   — version, description, marker, "Contract v1 · state vN"
src/limits.la  — MAX_BUNDLE, MAX_ASSET, MAX_ASSETS, MAX_NAME, MAX_DESCRIPTION, MAX_ASSET_NAME
test/bundle.la, test/pack.la (new); test/tabs.la, test/views.la, test/shell.la, test/counter.la (+bundle e2e)
test/tmp/.gitignore — scratch for tests
guest/rust/counter/icon.png
.gitignore (/*.rkb), CLAUDE.md (commands, tree, known issue), DECISIONS.md (020: image from bytes), CHANGELOG.md
```

### TESTS WRITTEN

- bundle (12): round trip, minimal, unknown keys, format, fields, semver, asset names, PNG and references, permissions/prerendered, sizes and counts, hostile bytes, write refuses what read refuses.
- pack (4): every flag, the simplest, plain errors, no overwrite without --force.
- tabs (+3): opening a bundle uses its manifest (state_version 4 recorded); a newer contract; a broken bundle.
- views (+1): the panel and address for a bundle.
- counter (+1): the real counter packed with its icon runs the same; the panel shows version and description.

### DECISIONS MADE

- Decoder `max_string` is MAX_MODULE (a module is one byte string); each asset's 4 MB cap is checked after decoding. This corrects the PRP's "strings MAX_ASSET".
- `loader.la` is a new module rather than growing tabs.la past 300 lines.
- Each pack test uses its own scratch files (tests run in parallel).

### PENDING DECISIONS OPENED

- DECISION-020 item 1: an image from bytes in kivuli (for icons and banners).

### STILL OPEN AT CLOSE

- The owner's on-screen check of `./main counter.rkb`.

---

## SESSION 21 — 2026-10-18 — App icons from bundles (kivuli 0.5) — closed

Branch: app-icons → merged into main

### WHAT WAS DONE

kivuli 0.5.0 (images from bytes, keyed) was tagged and pushed. Rapidkoi moved to it (lock: kivuli 0.5.0, lugha-gtk 0.1.25).
- **The bundle rules read each PNG's declared size** from its IHDR header: every asset ≤ 4096 px a side, the icon ≤ 1024 × 1024, banners ≤ 2048 × 1024, and zero sizes or a missing IHDR refused.
- **The icon's bytes travel** with the page, its pin, "where you left off" and home's card.
- **The rail tab and the home tile show the icon** (`Image.Bytes("{path}#icon", bytes)`), with the initial when there's none.

99 Lugha tests. counter.rkb was rebuilt at the repo root.

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock — kivuli 0.5
src/rules.la  — png_size (IHDR), check_asset image size, check_role; ImageTooLarge
src/bundle.la — icon and banner role caps
src/limits.la — MAX_IMAGE_PX, MAX_ICON_PX, MAX_BANNER_WIDTH/HEIGHT
src/loader.la — Loaded.icon
src/tabs.la   — AppPage.icon, LeftOff.icon, Pin.icon
src/window.la — pins and cards carry icons
src/frame.la  — icon_of (keyed by path); tabs show the icon
src/home.la   — Card.icon; tiles show the icon
src/theme.la  — TAB_ICON 22, TILE_ICON 64
test/support.la (png(w, h)), test/bundle.la, test/pack.la, test/views.la, test/counter.la
CLAUDE.md, DECISIONS.md (020 item done), docs/DESIGN.md, CHANGELOG.md
```

### TESTS WRITTEN

- bundle (+2): sizes by role; IHDR required.
- views (+2): the icon on the tab and tile instead of the letter; icons keyed by file.
- counter: the bundled counter's tab has its icon.

### DECISIONS MADE

- `icon_of` lives in frame.la (home and window import it; frame importing home would be a cycle).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 22 — 2026-10-18 — Propose PRP-004: local development (serve, open localhost) — closed

Branch: prp-004-dev-server

### WHAT WAS DONE

The owner wanted to serve counter.rkb and open it over localhost. Lugha gained `std.net` (m110) and `std.http` (m111: an HTTP/1.1 server and client with limits, http only; TLS is proposed as lugha PRP-144). Wrote PRP-004: `./main serve`, opening `rapidkoi://localhost:PORT` (local hosts only), and a manual Reload. Remembering becomes PRP-005. No code.

### FILES CREATED OR MODIFIED

```
PRPs/prp-004-local_development.md — for approval
DECISIONS.md — DECISION-021 (loading in the background), deferred
MEMORY.md    — next steps renumbered
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- `std.http` instead of a `lugha-http` package (Lugha's std has it now).
- `./main serve` built in, rather than Python's http.server.

### PENDING DECISIONS OPENED

- DECISION-021.

### STILL OPEN AT CLOSE

- PRP-004 awaits approval. It includes extending rule 3 to a localhost fetch.

---

## SESSION 23 — 2026-10-18 — Build PRP-004: local development — closed

Branch: prp-004-dev-server → merged into main

### WHAT WAS DONE

Built PRP-004 test-first:
- `address.la`: local development addresses, with typed errors.
- `serve.la`: `./main serve`, std.http on 127.0.0.1 only, one fixed route, re-reading and re-checking the bundle on every request.
- The loader fetches `rapidkoi://localhost:PORT` (std.http, 32 MB, 3 s) and loads the bundle like a file.
- The address box opens local addresses and gives a notice for other hosts and https.
- Reload refetches and starts fresh; the address and panel are marked `🛠 … · development`.

115 Lugha tests pass, including live-socket tests (a server on a spawned green thread per test, each on its own port). The real `./main serve` was checked with curl: 127.0.0.1 only (ss), 200 for the bundle, 404 for anything else, exit 1 for a missing bundle.

### FILES CREATED OR MODIFIED

```
src/address.la, src/serve.la (new); src/loader.la (fetch, from_bundle); src/main.la (serve)
src/window.la (go for addresses, reload); src/actions.la (Reload); src/shell.la
src/frame.la (marker: 🛠 development); src/panel.la (Served from…, Reload); src/limits.la (FETCH_TIMEOUT_MS)
test/address.la, test/serve.la, test/loader.la (new); test/shell.la (notice)
CLAUDE.md (rule 3, commands, tree, known issues), CHANGELOG.md
```

### TESTS WRITTEN

- address (6), serve (4), loader (6, live sockets: load, http, 404 / unreachable / not local, too large, reload, panel).

### DECISIONS MADE

- Typed AddressError variants instead of strings.
- "serving …" goes to stderr: std has no flush, and stdout stays buffered while the server runs (a small lughac gap).
- `tabs.la` untouched: reload lives in window.la.

### PENDING DECISIONS OPENED

- None (DECISION-021 was opened in session 22).

### STILL OPEN AT CLOSE

- The owner's on-screen check: serve plus open, then repack and Reload.

---

## SESSION 24 — 2026-10-18 — Split window.la (308 lines) into window and navigate — closed

Branch: split-window → merged into main

### WHAT WAS DONE

`src/window.la` reached 308 lines in PRP-004 and was merged that way, a miss caught after the merge and reported to the owner. With the owner's approval, the four functions that open something (`go`, `open_file`, `reopen`, `reload`) moved, unchanged, into a new `src/navigate.la` (85 lines); `window.la` is 234. Callers: shell.la (`navigate.…`) and test/loader.la.

### FILES CREATED OR MODIFIED

```
src/navigate.la (new), src/window.la, src/shell.la, test/loader.la, CLAUDE.md (tree)
```

### TESTS WRITTEN

- None (a move; all 115 pass).

### DECISIONS MADE

- Before every merge, check `wc -l` on every changed file, not just the largest few.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Nothing new.

---

## SESSION 25 — 2026-10-18 — File size limit to 500 lines — closed

Branch: file-size-500 → merged into main

### WHAT WAS DONE

The owner raised the file size limit from 300 to 500 lines. Changed in CLAUDE.md (FILE SIZE RULE) and PRPs/TEMPLATE.md. Older PRPs keep their "300" as written: they were approved under it.

### FILES CREATED OR MODIFIED

```
CLAUDE.md, PRPs/TEMPLATE.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- The limit is 500 lines per file, tests included.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Nothing new.

---

## SESSION 26 — 2026-10-18 — Record signing, permissions and panel designs; propose PRP-005 (info panel) — closed

Branch: prp-005-panel

### WHAT WAS DONE

Recorded the chat after session 25:
- **The signing design:** key file and first use, signed manifest and index, static key/index/builds layout, rotations and unapproved changes, formats, OpenSSL `std.crypto`, `rapidkoi.toml` and `std.toml`, markers.
- **Permissions at the point of use.**
- **The info panel redesign** (the owner's sketch, agreed names) and ⟳ / ☆ in the bar.

Prompts for `std.crypto` and `std.toml` were given to the owner. Wrote PRP-005. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-signing-and-panel.md, docs/source/meetings/2026-10-17-ui/sketch-3-info-panel.png
DECISIONS.md — 009 design settled; 022 (accounts) deferred
MEMORY.md    — decisions 14–16
PRPs/prp-005-info_panel.md — for approval
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- See the meeting note. Remembering becomes PRP-006.

### PENDING DECISIONS OPENED

- DECISION-022 (accounts).

### STILL OPEN AT CLOSE

- PRP-005 awaits approval. The lugha std work (crypto, toml) is the owner's.

---

## SESSION 27 — 2026-10-18 — Build PRP-005: the info panel and bar actions — closed

Branch: prp-005-panel → merged into main

### WHAT WAS DONE

Built PRP-005:
- **The bar:** ‹ ⌂ › ⟳ [address ☆] ⓘ ⤴.
- **⟳ Reload keeps state:** `tabs.load_again` loads again from the file or server and resumes when state_version matches; otherwise it starts fresh with a notice.
- **The panel as the owner sketched it:** a header with icon, name, version and marker (opens About); the description; Your settings and On this device as `Look.Section` cards of rows with summaries; pages with Back; Forget.
- **Start over and Forget ask first**, in place.
- **`window.forget`** removes the tab, the pin and a matching left-off, and keeps the previous left-off otherwise.

121 Lugha tests; the counter e2e checks ⟳ keeps its count.

### FILES CREATED OR MODIFIED

```
src/panel.la (rewritten), src/actions.la (PanelPage, Confirm, new messages; TogglePin replaces Pin/Unpin)
src/window.la (panel, confirm, forget, toggle_pin), src/navigate.la (reload keeps state), src/tabs.la (load_again, state_version_of)
src/frame.la (⟳ ☆), src/themes.la (Look.Section), src/shell.la
test/panel.la (new); test/views.la, test/shell.la, test/counter.la, test/loader.la
docs/DESIGN.md, CLAUDE.md, DECISIONS.md (020 item 0: clickable rows), CHANGELOG.md
```

### TESTS WRITTEN

- panel (8): bar ⟳ ☆, the main page, each page and Back, Start over asks, Forget asks and removes, a confirm dropped on tab change, reload from a non-file, sizes.
- counter: ⟳ keeps the count.

### DECISIONS MADE

- `tabs.load_again`: navigate already had `reopen`, for Continue.
- The panel's "main" function is `front` (`main` is reserved for the entry file).
- Sizes round to the nearest tenth of a MB (a test caught truncation).

### PENDING DECISIONS OPENED

- DECISION-020 item 0: clickable rows in kivuli.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 28 — 2026-10-18 — Text aligned left; the panel on the page colour — closed

Branch: align-left → merged into main

### WHAT WAS DONE

The owner (screenshot 5): "text align left".
- kivuli centers labels by default. Rapidkoi now aligns text to the start: contract text nodes (render), the error screen, home's headings, messages and card, and the whole info panel.
- Rows lay out as label, spacer, summary, arrow; an expanded label centers its text.
- The screenshot also showed the panel's section cards invisible: the panel sat on the frame (`#242424`, measured), the same colour as the cards. The panel now has `Look.Page` with a `SPACE_SM` strip of frame, as PRP-005 intended.

### FILES CREATED OR MODIFIED

```
src/render.la, src/panel.la (left()), src/home.la, src/shell.la, docs/DESIGN.md, CHANGELOG.md
docs/source/meetings/2026-10-17-ui/screenshot-5-panel.png
```

### TESTS WRITTEN

- None (layout; all 121 pass).

### DECISIONS MADE

- Start-aligned text everywhere Rapidkoi draws, and for apps' text by default.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 29 — 2026-10-18 — The panel back on the frame; its close button at the bottom — closed

Branch: panel-back → merged into main

### WHAT WAS DONE

The owner (screenshot 6): "previous design was better. move the close button at the bottom". Session 28's page-coloured panel (also stretched to half the window by its scroll wrapper) was undone: the panel sits on the frame at PANEL_WIDTH again. Left-aligned text stays. The × close moved from the header to a **Close** button at the bottom, under Forget. The section cards are the frame's colour on the frame again, so they're not drawn as separate cards; the owner prefers it that way.

### FILES CREATED OR MODIFIED

```
src/shell.la, src/panel.la, docs/DESIGN.md, CHANGELOG.md, docs/source/meetings/2026-10-17-ui/screenshot-6-panel-page-colour.png
```

### TESTS WRITTEN

- None (all 121 pass).

### DECISIONS MADE

- The panel stays on the frame colour (the owner's preference over visible cards).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 30 — 2026-10-18 — Forget and Close in one row — closed

Branch: panel-buttons-row → merged into main

### WHAT WAS DONE

The owner: "forget and close in one row". The panel's bottom is one row: Forget this app (danger) and Close, each taking half. While Forget asks for confirmation, its question with Cancel and Forget takes that place.

### FILES CREATED OR MODIFIED

```
src/panel.la, docs/DESIGN.md
```

### TESTS WRITTEN

- None (layout; all tests pass).

### DECISIONS MADE

- None new.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check.

---

## SESSION 31 — 2026-10-18 — The panel at a fixed width — closed

Branch: panel-fixed-width → merged into main

### WHAT WAS DONE

The owner (screenshots 12 and 13): "the panel should have a fixed width no matter what". Measured: about 510 px on the main page and 816 px on Account. `.width(n)` is GTK's minimum size, so long text widened it. Now:
- PANEL_WIDTH is 360 (about the main page's look)
- every wrapped panel line uses `.wrap_at(PANEL_CHARS)` (40), the app's name at 20
- About's values (paths, addresses) are ellipsized with the whole value as a tooltip
- the header's marker shows the file name only

### FILES CREATED OR MODIFIED

```
src/theme.la (PANEL_WIDTH 360, PANEL_CHARS 40), src/panel.la, docs/DESIGN.md
docs/source/meetings/2026-10-17-ui/screenshot-12-panel-width.png, screenshot-13-panel-width.png
```

### TESTS WRITTEN

- None (layout; all 121 pass).

### DECISIONS MADE

- Text is capped, rather than the widget given a maximum: GTK has no maximum width.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of every page's width.

---

## SESSION 32 — 2026-10-18 — The panel's width, firmer — closed

Branch: panel-width-firm → merged into main

### WHAT WAS DONE

The owner: the panel still shifted "a tiny bit but still noticeable". Some page's content was still slightly wider than 360, so its width won. Now every page's content is kept well under PANEL_WIDTH, so the panel's own width always decides:
- PANEL_CHARS 30 (from 40)
- the app's name wraps at 14
- the header's file name is cut at 18 characters with …

### FILES CREATED OR MODIFIED

```
src/theme.la, src/panel.la (NAME_CHARS, MARKER_CHARS, shorten), docs/DESIGN.md
```

### TESTS WRITTEN

- None (layout; all 121 pass).

### DECISIONS MADE

- Content capped well below the width, rather than near it.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check. If it still shifts, measure each page's width from a screenshot.

---

## SESSION 33 — 2026-10-18 — Propose PRP-006: signed apps, published locally — closed

Branch: prp-006-signing

### WHAT WAS DONE

Lugha landed `std.crypto` (m113, reviewed) and `std.toml` (m114). Wrote PRP-006: rapidkoi.toml, keygen, format 2 signed bundles, publish to site/ (key, signed index, builds/<sha256>.rkb), serve site/, and verified opening of `rapidkoi://localhost` sites (in-session trust; no bypass; an unapproved key change becomes a separate app). Prompts were given to the owner for `std.fs.rename`/`set_mode`, `flush`, a clock (lugha) and background tasks (kivuli, DECISION-021). No code.

### FILES CREATED OR MODIFIED

```
PRPs/prp-006-signed_apps.md — for approval (crypto: the owner's security review before merge)
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- Trust is in-session until the on-disk store (PRP-007, needs std.fs.rename). Remote hosts stay off until DECISION-021. No `rotate` command yet; rotations are verified.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-006 awaits approval.

---

## SESSION 34 — 2026-10-18 — Build PRP-006: signed apps, published locally — closed

Branch: prp-006-signing

### WHAT WAS DONE

Built PRP-006 test-first, in the planned order. `rapidkoi.toml` (std.toml, unknown keys refused), secret keys (keygen outside the project, never overwriting), format 2 signed bundles (the manifest's exact bytes signed; module and asset hashes both ways), the signed index and the public key file (sequence, withdrawn, prune, rotations signed by the old key), `./main keygen` / `publish` / `publish --withdraw` / `pack` from the toml, `./main serve SITE/` (key, index, builds by 64 lowercase hex only), and verified opening: key → index → build, failing closed, trust recorded only after every check passes, in a session keyring in the Model. The ✓ / ⚠ markers, the refused page (no Restart), the changed-key page (Keep using my version / Open as a separate app), and About's signed lines. Smoke-tested the commands with the real counter in the scratchpad, and probed the site server with raw `..` paths (all 404).

### FILES CREATED OR MODIFIED

```
src/project.la, keys.la, signing.la, index.la, publish.la, verify.la — new
src/bundle.la, rules.la — format 2 support, new BundleError variants, check_host
src/loader.la — Refusal (Failed / Unverified / Unapproved), verify_site for rapidkoi:// sites, read_any for files
src/tabs.la — AppPage.trust; Screen.Refused, Screen.Unapproved(change, kept page)
src/window.la — the Model's keyring; navigate.la — reload keeps the open page on a key change, keep_mine, separate_app
src/actions.la — KeepMine, SeparateApp; shell.la — wiring; render.la — refused; frame.la — markers, unapproved; panel.la — About
src/pack.la — no arguments packs rapidkoi.toml; serve.la — site folders; main.la — keygen, publish
test/project.la, keys.la, signing.la, index.la, publish.la, verify.la, opening.la — new; serve, loader, tabs — updated
rapidkoi.toml — the counter's (host localhost:8000, key in ~/.config/rapidkoi/keys); .gitignore — /site
CLAUDE.md (commands, tree), docs/DESIGN.md (markers and pages), CHANGELOG.md
```

### TESTS WRITTEN

- 170 tests pass (was 136 + new): project 7, keys 6, signing 6, index 7, publish 8, serve +2, verify 9 (in-memory sites, real signatures), opening 4 (live sockets: verified, a changed byte refused with no Restart, a changed key asks, a signed file ⚠ then ✓).

### DECISIONS MADE

- On the changed-key page, when nothing was open before, the keep button reads "Don't open it" (it closes the tab) rather than "Keep using my version". For the owner to confirm.
- The repo's `rapidkoi.toml` uses `host = "localhost:8000"`: in development, the key file's host must be the address's host.
- A site already holding another key is refused by `publish` (it would look like an unapproved change to everyone).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner reviewed and checked it on screen: "Done it..it's fine". Merged to main, with "Don't open it" kept.
- keygen can't set 0600 itself until std.fs.set_mode; it prints the chmod reminder.

---

## SESSION 35 — 2026-10-18 — Keys at 0600, and designing PRP-007: remembering — closed

Branch: prp-007-remembering

### WHAT WAS DONE

Lugha m115 (`fs.rename`, `fs.set_mode`, `flush`, `std.time`) and m116 (`fs.sync`) landed; kivuli 0.6.0 brought `Cmd.task` and `Sub.receive` (unblocks DECISION-021). keygen now writes the secret key at 0600 through a temporary file made owner-only before the secret goes in, synced and renamed into place (the PRP-006 follow-up it promised); the chmod reminder is gone. Designed PRP-007 in chat, one topic at a time (all four settled), and wrote it.

### FILES CREATED OR MODIFIED

```
src/keys.la, src/publish.la — keygen at 0600, safe write; message
test/keys.la — keygen leaves only the key; an unwritable folder is reported
DECISIONS.md — DECISION-023 (std.fs can't read a mode back), deferred
docs/source/meetings/2026-10-18-remembering.md — the design talk
PRPs/prp-007-remembering.md — for approval
CLAUDE.md (keygen command), CHANGELOG.md
```

### TESTS WRITTEN

- keys: 2 new (8 pass). The 0600 itself was checked by hand (`stat`: 600): no Lugha test can read a mode.

### DECISIONS MADE

- PRP-007 remembers trust, pins and left-off; tabs come next in their own PRP; app copies come with remote hosts.
- `~/.local/share/rapidkoi/` (XDG), three CBOR files with format numbers, safe writes; saved on every change on the window's thread (a new rule 3 exception, written in the PRP); broken files set aside with a notice; newer files left untouched.
- Proposed in the PRP, for approval: Forget keeps the app's trusted key.

### PENDING DECISIONS OPENED

- DECISION-023 (deferred).

### STILL OPEN AT CLOSE

- PRP-007 approved ("approved"). Then: background loading (kivuli 0.6, DECISION-021) after the tabs PRP.

---

## SESSION 36 — 2026-10-18 — Build PRP-007: remembering — closed

Branch: prp-007-remembering

### WHAT WAS DONE

Built PRP-007 test-first. `store.la`: the data folder (XDG, then HOME), safe writes (0600 temp file, sync, rename, folder sync; the folder 0700), capped reads, set aside as `.broken`, skipping a newer file's name, remove. `remembered.la`: the trust, pins and left-off formats (format 1, a kind each), read as hostile input with every field checked; `recall` gives Fresh / Read / Broken (set aside) / Newer / Failed. `verify.la`: public `Identity`, `identities`, `keyring_of`, and a change count that moves only when trust is recorded. `tabs.la`: `recalled` and `compiled`; `navigate.reopen` loads a remembered left-off again from its path, with its state. `saving.la`: `restore` at start (notices joined), `persist` after every message (only what changed). `shell.la`: `boot` uses the real folder; `start(argv, store)` for tests; `update` saves after each message.

### FILES CREATED OR MODIFIED

```
src/store.la, src/remembered.la, src/saving.la — new
src/verify.la, src/tabs.la, src/navigate.la, src/window.la, src/shell.la
test/store.la, test/remembered.la, test/saving.la — new; test/verify.la (+2); test/shell.la, test/counter.la (start(…, None))
CLAUDE.md (rule 3 exception, tree, known issues), CHANGELOG.md
```

### TESTS WRITTEN

- 197 pass: store 8, remembered 6, verify +2, saving 9 (restarts over live sockets: trust, a swap asks, separate stays separate; a known site writes nothing, checked with a read-only folder and a mutation of the no-change rule; pins and left-off; the counter resumes Count: 1; Forget keeps trust; broken and newer notices; a failed save; no store). The test run never touched ~/.local/share/rapidkoi.

### DECISIONS MADE

- The left-off file also keeps the app's icon (not in the PRP's field list), so home's card shows it after a restart.
- A file that can't even be read (Failed: e.g. permissions) is never written this session either, like a newer one: nothing unread is overwritten.
- Tests reach the window through `start(argv, None)`; only `boot` (main) uses the person's folder.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- **The owner's security review before merge** (the trust store), and the on-screen checks in the PRP's VALIDATION.

---

## SESSION 37 — 2026-10-18 — PRP-007 amendment 1: where you left off includes the open app — closed

Branch: prp-007-remembering

### WHAT WAS DONE

The owner tried PRP-007: "the pin is there but state is lost and ui has regressed". The state: they quit with the counter open, and left-off was only saved when an app left its tab (kivuli has no quit hook). The owner chose option A: remember the app used last (the current tab's, else the last visited open tab's, else the last closed), saved as it changes. Wrote amendment 1 and built it test-first. Also found and fixed: Continue after a restart that fails to load (file gone, server down) would have overwritten the remembered state with nothing; a page that isn't running now never replaces a remembered app, and reopen keeps the left-off until the app runs.

### FILES CREATED OR MODIFIED

```
PRPs/prp-007-remembering.md — amendment 1
src/tabs.la — snapshot; src/window.la — used_last; src/saving.la — saves used_last; src/navigate.la — reopen keeps left-off until it runs
test/saving.la — 3 new
```

### TESTS WRITTEN

- quitting with the counter open keeps Count: 2; the app used last across two tabs and home; a failed Continue keeps the saved state. 200 pass.

### DECISIONS MADE

- Option A (the owner). Coalescing saves for large states stays a later refinement.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- **The UI regression:** not reproduced; PRP-007 touched no view code. Waiting for the owner's screenshot or description.
- The owner's security review and on-screen checks before merge.

---

## SESSION 38 — 2026-10-18 — PRP-007 amendment 2: state belongs to signed apps — closed

Branch: prp-007-remembering

### WHAT WAS DONE

The owner: state still lost. Cause: opening the app by its address started it fresh, and the fresh state overwrote the saved one as "the app used last". The owner: identity is host + key, so state belongs to that; unsigned apps lose state across restarts. Wrote amendment 2 and built it test-first: `states.la` (one file per signed app under states/, named by the SHA-256 of [host, key], checked to hold that identity, at most 256 apps, the oldest removed; move after a rotation); the keyring records rotations; left-off holds which app (and host and key) but no state, written only for signed apps; every open of a signed app resumes its saved state (navigate.resumed); saving keeps the used-last signed app's state; Forget removes it. A rotation's state is moved as the app opens, before its state is read.

### FILES CREATED OR MODIFIED

```
PRPs/prp-007-remembering.md — amendment 2
src/states.la, test/states.la — new
src/store.la (ensure), src/verify.la (moves), src/tabs.la (signer, signer_of, recalled, with_saved), src/remembered.la (left-off format), src/navigate.la (resumed, arrived), src/saving.la (Mark, persist, forgotten), src/shell.la
test/store.la, test/verify.la, test/remembered.la, test/saving.la (reworked)
CLAUDE.md (rule 3, tree, known issues), CHANGELOG.md
```

### TESTS WRITTEN

- 211 pass: states 6; saving 15 (a signed counter resumes by address and by Continue; unsigned starts fresh; a separate publisher starts at 0; a rotation keeps Count: 1; Forget removes the state; a changed state version starts over; the signed app used last stays the one continued; a failed Continue keeps Continue).

### DECISIONS MADE

- The owner: only apps with a host and key keep state across restarts. Pins stay for every app (the owner said "everything", read as state; flagged in the summary).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The UI regression the owner reported: still not seen; needs a screenshot.
- `src/tabs.la` is at 481 lines: the tabs PRP must start with a split.
- The owner's security review and on-screen checks before merge.

---

## SESSION 39 — 2026-10-18 — Merge PRP-007 — open

Branch: prp-007-remembering

---

## NEXT SESSION START POINT

PRP-007 (amendments 1 and 2) is built on `prp-007-remembering`, not merged. First: the owner's UI regression (ask for a screenshot). Then the security review, on-screen checks, full suite, merge. Next: the tabs PRP (start by splitting src/tabs.la, 481 lines), then background loading (DECISION-021).
