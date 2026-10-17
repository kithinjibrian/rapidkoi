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

## SESSION 18 — 2026-10-17 — kivuli 0.4.1: centered button content — open

Branch: kivuli-0.4.1

---

## NEXT SESSION START POINT

Open session 12 on a new branch. PRP-002 with Amendments 1 and 2 is on `main`. Wait for the owner's style feedback before restyling. Next is PRP-003 (remembering: an on-device store for history and pins, loose search, `banner`/`private`, generated art, DECISION-019), discussed in chat first. As kivuli ships the rest of DECISION-020, swap each placeholder out.
