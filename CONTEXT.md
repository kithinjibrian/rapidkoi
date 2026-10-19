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

## SESSION 39 — 2026-10-18 — Merge PRP-007 — closed

Branch: prp-007-remembering

### WHAT WAS DONE

The owner: "merge prp-007 then start background loading" (taken as the security review done). Full suite 211/211, fmt clean, build ok, no file over 500 lines (tabs.la 481). Merged prp-007-remembering into main. Also, at the owner's request (outside the repo): a static HTTPS site for the counter at counter.dafifi.net on the owner's VM (/opt/rapidkoi, nginx behind Traefik), published from ~/rapidkoi-sites/counter.dafifi.net with its own key.

### FILES CREATED OR MODIFIED

```
CONTEXT.md, lugha.lock (lughac's)
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The dots beside Reset (screenshot 17): not diagnosed; compare old builds when the owner wants.

---

## SESSION 40 — 2026-10-18 — Design background loading (PRP-008) — closed

Branch: prp-008-background_loading

### WHAT WAS DONE

Designed background loading in chat with the owner, three topics, each "yes": a loading page in the tab at once (spinner, elapsed time after 3 s, "· loading…" marker; closing or ⟳ ignores the old result; local files stay instant); the task gets a snapshot of trusted keys and returns the verified bytes plus a record, trust recorded only on arrival, a changed snapshot reloads once; 30 s per request, local addresses only (https next). Wrote PRP-008, including the step-0 split of tabs.la (481 lines) into leftoff.la and the kivuli 0.6 bump. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-background-loading.md
PRPs/prp-008-background_loading.md — for approval
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- DECISION-021 will resolve to option A (kivuli's Cmd.task) when PRP-008 merges.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-008 approved ("approved"), with the tabs.la split.

---

## SESSION 41 — 2026-10-18 — Build PRP-008: background loading — closed

Branch: prp-008-background_loading

### WHAT WAS DONE

Built PRP-008 test-first. Step 0: LeftOff and its functions moved unchanged to leftoff.la (tabs.la 481 → 366; suite unchanged). kivuli 0.6 (`lughac update`). verify: `check_site` over a snapshot (`known_for`), `record` on arrival; `verify_site` is both. fetching.la: the task's work (key, index, build checked against the snapshot; an unsigned server's bundle; "didn't answer in N seconds"; nothing listening is "Couldn't reach"). loader: `from_fetched` (records, then compiles) and `current` (the snapshot still the keyring's). tabs: `Screen.Loading(Pending)` (started, again, saved, kept), `AppPage.load`, `loading`, `from_loaded`, `failed_page`, `is_loading`, `refused` public. loads.la: `begin` (a loading page and a Cmd.task capturing only the address and snapshot), `arrive` (stale by tab and number ignored; a changed snapshot loads again once, then "Trust for HOST changed while it loaded; try again"; ⟳ keeps the open page until the new one is ready or a changed key is answered), `resumed` (moved from navigate). navigate's opens return (Model, Cmd). shell: Fetched and Tick, the loading page, `subscriptions` (120 ms only while loading). The marker shows "HOST · loading…". FETCH_TIMEOUT_MS 30 s.

### FILES CREATED OR MODIFIED

```
src/leftoff.la, src/fetching.la, src/loads.la — new
src/tabs.la, src/verify.la, src/loader.la, src/navigate.la, src/shell.la, src/window.la, src/actions.la, src/render.la, src/frame.la, src/main.la, src/limits.la (and imports in saving, remembered)
lugha.toml (kivuli 0.6), lugha.lock (lughac update)
test/fetching.la, test/loading.la — new; test/support.la (settle); test/verify.la (+2); test/opening.la, test/saving.la, test/loader.la (settle background loads); test/remembered.la, test/tabs.la (imports)
CLAUDE.md (rule 3, tree, known issues, stack), DECISIONS.md (021 resolved), CHANGELOG.md
```

### TESTS WRITTEN

- 226 pass: fetching 5 (live), loading 8 (a loading tab and task, the spinner and "Loading… 4 s", other actions meanwhile, stale results ignored, a result becomes the app or the reason with no Restart, ⟳ makes the old number stale, a changed snapshot reloads once then says so, end to end with trust saved, subscriptions only while loading), verify +2.

### DECISIONS MADE

- Tests run tasks with `support.settle`, matching kivuli's public `Cmd.Task`; no kivuli change needed (a `kivuli.testing` helper could come later).
- The checked bundle travels from the task to the window as data; it isn't re-read on arrival (the PRP suggested a second `read_any`; the bytes never leave the process, so it would only repeat the same check).
- ⟳ keeps the open page (running or not) as before, so a changed key still offers "Keep using my version".

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's review and on-screen checks before merge (the verification path moved into the task).

---

## SESSION 42 — 2026-10-18 — Merge PRP-008 — closed

Branch: prp-008-background_loading

### WHAT WAS DONE

The owner: "merge" (taken as the review done). Full suite 226/226, fmt clean, build ok. Merged prp-008-background_loading into main.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The dots beside Reset (screenshot 17): not diagnosed.

---

## SESSION 43 — 2026-10-18 — Design remote apps over HTTPS (PRP-009) — closed

Branch: prp-009-remote_apps

### WHAT WAS DONE

Designed remote apps in chat, three topics, each "yes": rapidkoi:// and https:// remote hosts over HTTPS only, names not IPs, signed only, no redirects, std's certificate checks; quiet first-use trust (remembered), "✓ HOST · verified" for remote, About's "Trusted since DATE" (trust file format 2); tests with a throwaway test CA through a test-only `Network` parameter, the owner's counter.dafifi.net on screen. Wrote PRP-009. Found a std gap (request_tls has no limits): DECISION-024, open; a Lugha prompt given to the owner. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-remote-apps.md
PRPs/prp-009-remote_apps.md — for approval
DECISIONS.md — DECISION-024 (open)
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None beyond the design.

### PENDING DECISIONS OPENED

- DECISION-024 (std.http: one request with limits and an extra CA).

### STILL OPEN AT CLOSE

- PRP-009 approved ("approved"); DECISION-024 resolved by Lugha m119 (`request_tls_with`).

---

## SESSION 44 — 2026-10-18 — Build PRP-009: remote apps over HTTPS — closed

Branch: prp-009-remote_apps

### WHAT WAS DONE

DECISION-024 resolved (Lugha m119 `request_tls_with`). Made the TLS fixtures (test/fixtures/tls: a throwaway CA, its key deleted after signing; localhost, wrong-host and expired certificates; make.sh, README). Built PRP-009 test-first: address.la (remote hosts by name over HTTPS, port 443 default, `remote`/`secure`, `site_url`; NotSecure, IpAddress; https://localhost allowed for TLS tests); fetching.la (one `request_tls_with` call for everything; `Network { ca_file }`, tests only; no key over HTTPS → "can't be verified"; a 3xx refused, never followed; "HOST's certificate couldn't be verified: …"); window.Model.network (always none) and `with_network` (tests only; no caller in src); verify `Identity.since`/`Trust.since` (first use dates, rotation and sequence keep it); trust file format 2 (format 1 reads with 0; the kind is checked before the format); the marker `✓ HOST · verified` for remote; About's "Since" line (`first_trusted`, a civil-date function, UTC). A one-off live check (not committed) fetched counter.dafifi.net: Cloudflare's certificate, the signed key, index and build all checked, Counter 0.2.0.

### FILES CREATED OR MODIFIED

```
src/address.la, src/fetching.la, src/loads.la, src/loader.la, src/window.la, src/verify.la, src/remembered.la, src/frame.la, src/panel.la
test/fixtures/tls/* — new; test/remote.la — new; test/address.la, test/fetching.la (+4 TLS), test/verify.la (+1), test/remembered.la (+1), test/views.la (+2), test/opening.la, test/loader.la, test/shell.la
CLAUDE.md, docs/DESIGN.md, CHANGELOG.md, DECISIONS.md
```

### TESTS WRITTEN

- 238 pass. TLS: opens; an untrusted CA, a wrong host and an expired certificate refused; no key over HTTPS refused; a 301 refused; end to end a first use saved with its date; the window with no test CA refuses the test server (the app passes no CA). Two old tests that expected remote hosts refused now use offline refusals (remote http, an IP), so no test touches the internet.

### DECISIONS MADE

- The unsigned fallback is for plain-HTTP development servers only (any HTTPS address must be signed), which also makes the rule testable offline.
- About's wording drops "this session" (trust has been remembered since PRP-007).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's security review and on-screen checks (counter.dafifi.net; a republished 0.2.1) before merge.

---

## SESSION 45 — 2026-10-18 — About's labels and values at the left — closed

Branch: prp-009-remote_apps

### WHAT WAS DONE

The owner's screenshot 18 ("left align text"): About's dim labels were centred, and short values sat in from the edge. `panel.hover_pair` now aligns both to the start (kivuli centres text unless told). Alignment isn't visible to kivuli.testing; the owner checks it on screen.

### FILES CREATED OR MODIFIED

```
src/panel.la
```

### TESTS WRITTEN

- None (visual); panel, views and opening still pass.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check, then PRP-009's merge.

---

## SESSION 46 — 2026-10-18 — Merge PRP-009 — closed

Branch: prp-009-remote_apps

### WHAT WAS DONE

The owner: "yes" (merge PRP-009, then design PRP-010; taken as the security review done, screenshot 18 showing counter.dafifi.net verified). Full suite 238/238, fmt clean, build ok. Merged prp-009-remote_apps into main.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- None.

---

## SESSION 47 — 2026-10-18 — Design PRP-010: retiring four placeholders with kivuli 0.10 — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

kivuli reached 0.10 (wrap, overlay, context menus, button_with; still no file dialog). Designed with the owner, four topics, each "yes": the panel as an overlay with a dim scrim (click or Escape closes; the app keeps its width and can't be clicked meanwhile); a tab's right-click menu (Reload, Pin/Unpin, About, Close, Forget… via the panel's question), selecting the tab first; home tiles' menu (Open, Unpin, Forget…, corrected in chat); a wrapping, equal-width home grid; panel rows as one Subtle button_with. Wrote PRP-010. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-kivuli-placeholders.md
PRPs/prp-010-kivuli_placeholders.md — for approval
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None beyond the design.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-010 approved ("build").

---

## SESSION 48 — 2026-10-18 — Build PRP-010: retiring four placeholders — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

Built PRP-010 ("build"), one commit each after the kivuli 0.10 bump: the info panel over the app (`overlay`, a `Look.Scrim` button "Click to close the panel" with Escape; the panel placed End/Fill; the app keeps its width); panel rows as one Subtle `button_with`; home's grid as `wrap(...).equal()` (TILES_PER_ROW removed); right-click menus: tabs (Reload greyed while loading, Pin/Unpin, About, Close, Forget… via the panel's question), pinned tiles (Open, Unpin, Forget…), the left-off card (Open, Forget…), with `Msg.OnTab(id, TabAction)` and `Msg.OnPin(path, PinAction)` reusing existing actions; `Card.path`; `rail` takes the pinned paths.

### FILES CREATED OR MODIFIED

```
lugha.toml, lugha.lock (kivuli 0.10)
src/shell.la, src/themes.la, src/panel.la, src/home.la, src/theme.la, src/frame.la, src/actions.la, src/window.la
test/menus.la — new; test/panel.la (+2), test/views.la (rail, Card)
CLAUDE.md (stack, known issues), DECISIONS.md (020), docs/DESIGN.md, CHANGELOG.md
```

### TESTS WRITTEN

- 245 pass: menus 5 (each item's message; Unpin when pinned; Reload greyed while loading; select-then-act for each; a gone tab; tile Open/Unpin/Forget; the card's menu); panel +2 (the dim layer closes by click and Escape, absent when closed; a whole row is one click target, found by its summary). The grid's layout is visual (the owner's resize check).

### DECISIONS MADE

- The scrim's tooltip is "Click to close the panel" (the panel's own Close already says "Close the panel").

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen checks, then merge.

---

## SESSION 49 — 2026-10-18 — The overlay panel: the lighter background and rounded corners — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner: "give the panel the lighter bg and rounded corners". Over the app the panel had no background of its own (the dim page showed through). New `Look.Panel`: the frame's colour (#242424, lighter than the page), radius `PAGE_RADIUS` on every corner, flat; the panel is inset by `SPACE_SM` so the corners show. The cards inside keep their look (Section, the frame's colour) as when the panel sat on the frame.

### FILES CREATED OR MODIFIED

```
src/themes.la, src/shell.la, docs/DESIGN.md
```

### TESTS WRITTEN

- None (visual); panel, views and menus pass.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of PRP-010, then merge.

---

## SESSION 50 — 2026-10-18 — The overlay panel: the dark background and a thin border — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner: "use the dark bg then a thin border". `Look.Panel` is now the page's colour (#161616) with a 1 px edge (`EDGE_WIDTH`, the theme's new `edge` #3a3a3a), still rounded on every corner and inset. Its cards (the frame's #242424) now stand out on it. DESIGN.md's theme table lists scrim and edge.

### FILES CREATED OR MODIFIED

```
src/themes.la, src/theme.la, docs/DESIGN.md
```

### TESTS WRITTEN

- None (visual); panel and views pass.

### DECISIONS MADE

- The edge colour #3a3a3a (a step above the frame), the owner to adjust on screen.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of PRP-010, then merge.

---

## SESSION 51 — 2026-10-18 — The overlay panel: padding, a fixed width, under ⓘ, no closing from outside — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner's screenshots 19–20, four points. 1) Padding: the dark look was on the padded node itself (kivuli's padding falls outside a node's background), so content met the border; the look moved to a wrapper. 2) Width: `PANEL_WIDTH` is a GTK minimum and the header's long marker ("✓ localhost:8000 · verified (development)") widened the panel; the marker is now a `button_with` text with `wrap_at(PANEL_CHARS)` and `ellipsize` (at most two lines, then …). 3) The panel sits at the top right just under ⓘ, as tall as its content (`.place(End, Start)`), not stretched to the bottom. 4) The dim layer is no longer a button: it blocks clicks to the app but closes nothing; Escape moved to the panel's Close button.

### FILES CREATED OR MODIFIED

```
src/shell.la, src/panel.la, test/panel.la, docs/DESIGN.md
```

### TESTS WRITTEN

- The overlay test now checks a stray click can't close it (no scrim button), Close and Escape do, and no Escape is left when closed. 245 pass.

### DECISIONS MADE

- No closing from outside (the owner). Escape stays, on Close.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of PRP-010, then merge.

---

## SESSION 52 — 2026-10-18 — The overlay panel: one height for every page — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner's screenshots 21–22: the width now holds, but the height followed each page's content. New token `PANEL_HEIGHT` (520, room for the main page, judged from screenshot 21 at about 1.4× scaling); the panel's dark wrapper takes it and the page inside expands, so Forget and Close stay at the bottom of the main page and shorter pages leave room below. (`.height` is a GTK minimum: a page taller than 520 would still grow.)

### FILES CREATED OR MODIFIED

```
src/theme.la, src/shell.la, docs/DESIGN.md
```

### TESTS WRITTEN

- None (visual); panel, views and menus pass.

### DECISIONS MADE

- PANEL_HEIGHT 520, for the owner to adjust on screen.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of PRP-010, then merge.

---

## SESSION 53 — 2026-10-18 — The marker: an icon instead of "verified" — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner: "instead of writing verified. just use a icon". The marker is now three parts in frame.la: `marker` (the short label: the host, "· new publisher" when separate, "HOST · not confirmed"; unsigned markers unchanged), `marker_icon` (security-high-symbolic for verified or trusted, dialog-warning-symbolic for not confirmed, none unsigned) and `marker_note` (the words: "HOST · verified", "(development)" on this machine, "HOST · trusted", "HOST · not confirmed"). The bar's marker button shows icon and host; its tooltip leads with the note, then "Click to search or type a path (Ctrl+K)". The panel header shows the same icon and host. Tests find the marker by that tooltip, so they still assert the exact standing.

### FILES CREATED OR MODIFIED

```
src/frame.la, src/panel.la, test/views.la, test/loading.la, test/opening.la, test/saving.la, test/remote.la, docs/DESIGN.md
```

### TESTS WRITTEN

- views: the marker's label, icon and note for remote verified, local verified, trusted, not confirmed and unsigned. 245 pass.

### DECISIONS MADE

- A trusted signed file's words are "HOST · trusted"; the icon is the same shield as verified.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's on-screen check of PRP-010, then merge.

---

## SESSION 54 — 2026-10-18 — Merge PRP-010 — closed

Branch: prp-010-kivuli_placeholders

### WHAT WAS DONE

The owner: "merge" (the on-screen checks done through screenshots 19–22 and the tweaks of sessions 49–53). Full suite 245/245, fmt clean, build ok, no file over 500 lines. Merged prp-010-kivuli_placeholders into main.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- None.

---

## SESSION 55 — 2026-10-18 — Design talk: identity — closed

Branch: design-identity

### WHAT WAS DONE

Talked identity through with the owner, slowly and with ASCII pictures: per-app key pairs (the public key to the app, the private key kept), host-bound challenges, a new key for a changed owner, only the public key given to apps, name placeholders filled while drawing (the owner's idea), email and phone typed into apps (relays later), no 2FA needed, auto sign-in on by default, recovery through one master secret (later). Lugha added `hkdf_sha256` (m120) at Claude's request. Wrote PRP-011 (keys + placeholders). No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-18-identity.md
PRPs/prp-011-identity.md — for approval (security review before merge)
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- Recorded in the meeting notes; contract v2 is decided in PRP-011.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-011 approved ("approved"). Note: links (the vision's step 2) still aren't built.

---

## SESSION 56 — 2026-10-18 — Build PRP-011: identity — closed

Branch: prp-011-identity

### WHAT WAS DONE

Built PRP-011 test-first. identity.la (the master secret, 32 random bytes, 0600, never replaced if broken; per-app Ed25519 seeds by hkdf_sha256 over host, origin and generation; host-bound sign-in messages; labels from the key's first four hex digits). people.la (per-app generation, auto, signed out, name overrides; your name for every app). Identity.origin and Trust.origin (an app's first key, kept through rotations; a separate publisher or a not-confirmed key is its own); trust format 3. Contract v2: guests speak 1 or 2; v2 init returns {state, effects}; read_update_v2 (identity, sign; at most 8; challenges 1–1024 bytes); answer {msg, value bytes|null}; a bundle's contract must match its module's. placeholders.la (fill, one pass, text and button labels only; titles left alone). effects.la (answer_all: signed apps you're signed in to; at most 32 answers in a row). The Account page (account.la). The Rust library (Ask, start/step/answer, FIRST_NAME/LAST_NAME) and the counter (asks who you are; "Hello, {FIRST_NAME}!"; "You are User xxxx"). A project's contract (default 2) for pack (--contract) and publish. Split (approved): running.la from tabs.la.

### FILES CREATED OR MODIFIED

```
src/identity.la, src/people.la, src/effects.la, src/account.la, src/placeholders.la, src/running.la — new
src/verify.la, src/remembered.la, src/nodes.la, src/contract.la, src/limits.la, src/guest.la, src/tabs.la, src/loader.la, src/render.la, src/window.la, src/saving.la, src/shell.la, src/panel.la, src/actions.la, src/project.la, src/pack.la, src/publish.la
test/identity.la, test/people.la, test/effects.la, test/account.la, test/placeholders.la — new; test/contract.la, test/verify.la, test/remembered.la, test/views.la, test/panel.la, test/guest.la, test/tabs.la, test/counter.la, test/pack.la, test/project.la, test/support.la (echo_v2)
guest/rust/rapidkoi (v2), guest/rust/counter
CLAUDE.md, MEMORY.md (10, 17), DECISIONS.md (025 resolved, 022 updated), the contract doc's v2 appendix, CHANGELOG.md
```

### TESTS WRITTEN

- 274 Lugha tests pass (identity 5, people 4, effects 5, account 5, placeholders 4, contract +3, verify +1, project +1, pack +1, and updated ones); Rust: library 8, counter 3.

### DECISIONS MADE

- v2 init returns {state, effects} (otherwise an app couldn't ask until a click).
- Screen titles aren't filled (window titles are visible outside Rapidkoi).
- "Sign in automatically" off: the app starts each run signed out (Sign in on the Account page).
- Start over as someone new restarts the app, so its init asks for the new key.
- The user label is the key's own first four hex digits (no hash library in apps).
- A broken identity file is refused and left alone (replacing it would make you a stranger everywhere).
- An unsigned app's Account row reads "No account".

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- **The owner's security review before merge** (authentication, secrets, a contract change), and the on-screen checks.

---

## SESSION 57 — 2026-10-18 — Account page: only this app's name — closed

Branch: prp-011-identity

### WHAT WAS DONE

The owner: "remove first name & last name those are rapidkoi settings. leave first name in app because that panel is for app settings". The Account page keeps only this app's name ("First name", "Last name", saved per app); the name for every app stays in the people file for Rapidkoi's settings later, with no field yet. Also found (screenshot 23): auto sign-in looked broken because a v2 app resumed from its saved state skips `init`, its only place to ask; proposed a `resumed` message (option A), awaiting the owner.

### FILES CREATED OR MODIFIED

```
src/account.la, test/account.la
```

### TESTS WRITTEN

- account: the name fields set this app's name. 274 pass.

### DECISIONS MADE

- A name for every app is a Rapidkoi setting (the owner); the panel holds only the app's own.

### PENDING DECISIONS OPENED

- The `resumed` message for v2 apps resumed from saved state (option A), awaiting the owner.

### STILL OPEN AT CLOSE

- Option A; the owner's security review and on-screen checks.

---

## SESSION 58 — 2026-10-18 — The Account page knows you after a restart — closed

Branch: prp-011-identity

### WHAT WAS DONE

The owner's screenshots 26–27: signed in, then after closing and reopening Rapidkoi the Account row said "Not signed in yet" though the counter (resumed with its key in its state) still showed "You are User 5189". The master secret was only read when an app asked; `saving.restore` now reads it if the file exists (`identity.existing`, never makes one), so the page names you at once.

### FILES CREATED OR MODIFIED

```
src/identity.la, src/saving.la, test/account.la
```

### TESTS WRITTEN

- account: after a restart the page shows the same label; no master secret is made until an app asks. 276 pass.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None (option A, the `resumed` message, still awaits the owner).

### STILL OPEN AT CLOSE

- Option A; the owner's security review and on-screen checks.

---

## SESSION 59 — 2026-10-18 — PRP-011 amendment 1: one entry point, events for everything — closed

Branch: prp-011-identity

### WHAT WAS DONE

With the owner: a resumed app never asked again (init was its only place); the owner asked for one entry point taking events ("why don't we just have event and then pass things like on_load, on_init, on_click etc"), the name `handle` (also in the Rust library: "the user will split if they want on their own"), and allowed breaking changes ("nothing has shipped"). Contract 2 now: exports `rapidkoi_version`, `alloc`, `handle(state, event)`, `view`; events `init` (fresh, empty state), `opened` (every open), `message`, `answer`; any result's effects answered. v1 removed (guest only speaks 2; manifests must say 2; the contract setting in pack and rapidkoi.toml removed again; placeholders always filled). running.la: fresh = init then opened, resumed = opened; resumed pages keep their asks (the bug). The Rust library: `App::handle(self, Event)` and `view`, a non-exhaustive `Event`, `Default` state for init. The counter asks on every Opened. Fixtures rebuilt (they tell init and opened apart by their 12- and 14-byte encodings).

### FILES CREATED OR MODIFIED

```
PRPs/prp-011-identity.md — amendment 1
src/guest.la, src/contract.la, src/running.la, src/tabs.la, src/loader.la, src/shell.la, src/limits.la, src/project.la, src/pack.la, src/publish.la
guest/rust/rapidkoi, guest/rust/counter
test/support.la (fixed, echo, echo_opening, crashing), test/guest.la, test/contract.la, test/effects.la, test/tabs.la, test/project.la, test/pack.la; manifests in tests to contract 2
docs/source/meetings/2026-10-17-ui-contract.md (contract 2), MEMORY.md, DECISIONS.md, CLAUDE.md, CHANGELOG.md
```

### TESTS WRITTEN

- 276 Lugha pass (new: events_say_what_happened; a resumed app asks again when opened; version 1 and 3 refused); Rust: library 7, counter 3 (it asks on every open).

### DECISIONS MADE

- One entry point `handle`, in the contract and the Rust library (the owner). v1 removed (the owner). `Event` is non-exhaustive (closing and others later).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's security review and on-screen checks of PRP-011.

---

## SESSION 60 — 2026-10-18 — Account page: name fields span the panel — closed

Branch: prp-011-identity

### WHAT WAS DONE

The owner's screenshot 28 ("make this span the panel width"): the panel aligns each part at the start, so the name fields kept their natural width. They now take the panel's inner width (PANEL_WIDTH less SPACE_MD on each side).

### FILES CREATED OR MODIFIED

```
src/account.la
```

### TESTS WRITTEN

- None (layout); account tests pass.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's security review and on-screen checks of PRP-011.

---

## SESSION 61 — 2026-10-19 — Merge PRP-011 — closed

Branch: prp-011-identity

### WHAT WAS DONE

The owner: "merge prp-011 then let's talk apps reaching the network" (taken as the security review done; screenshots 26–28 on screen). 276 Lugha tests, cargo 3 + 7, fmt clean, build ok, no file over 500 lines. Merged prp-011-identity (with design-identity's notes) into main.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- None.

---

## SESSION 62 — 2026-10-19 — Design talk: apps reaching the network — closed

Branch: design-network

### WHAT WAS DONE

Designed apps reaching the network with the owner, four topics, each "yes", building on the 2026-10-18 rule (declaration-only, no prompt, listed and switchable): servers declared in rapidkoi.toml and signed into the manifest, the own host implied, HTTPS names only, localhost only for apps from this machine; a `fetch` effect (methods, headers, body; no cookies, no redirects followed; 30 s, 1 MB up, 8 MB down, 4 in flight); signing in to the app's server through fetch + sign (a Rust helper, a server document); new servers in updates announced once and marked new; per-server switches. Wrote PRP-012. No code.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-19-network.md
PRPs/prp-012-network.md — for approval (security review before merge)
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- Only apps from this machine may reach localhost (Claude, from the owner's rules; stated in the PRP).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-012 approved ("approved").

---

## SESSION 63 — 2026-10-19 — Build PRP-012: apps reaching the network — closed

Branch: prp-012-network

### WHAT WAS DONE

Built PRP-012 test-first, in its order:
1. The manifest's `servers` (formats 1 and 2), `[network] servers`, `pack --server`, and `publish`.
2. `network.la`, the pure checks.
3. The `fetch` effect in contract 2. Answers now carry a CBOR value, and the network limits are in `limits.la`.
4. `requests.la`: a `Cmd.task` per request and `Msg.Responded`.
5. `effects` routing fetches; `answer_all` returns the commands.
6. People format 2 (`off`, `seen`, `fresh`).
7. `access.la`: the Access page, its switches, the one-time notice and the New marks.
8. Rust: `Request`, `Ask::fetch`, `Event::Response`/`Failed`, `SignIn`.
9. `docs/server-sign-in.md`, checked by `test/signin.la`.
10. The quote demo (`guest/rust/quote`, `examples/quote/`).
11. The docs.

The owner chose a Lugha demo server in this repo; it's in `examples/quote/` because lughac lets examples import the package. 304 Lugha tests plus the example's 2; cargo 3 + 2 + 9; fmt clean; build ok; no file over 500 lines. Not merged: it waits for the owner's security review and the demo on screen.

### FILES CREATED OR MODIFIED

```
src/network.la, src/requests.la, src/access.la (new)
src/bundle.la, src/project.la, src/pack.la, src/publish.la, src/limits.la, src/nodes.la, src/contract.la,
src/render.la, src/effects.la, src/actions.la, src/shell.la, src/panel.la, src/people.la, src/tabs.la
test/network.la, test/requests.la, test/access.la, test/signin.la (new); test/bundle.la, project, pack,
signing, contract, effects, account, people and the Manifest literals
guest/rust/rapidkoi/src/{lib,abi,fetch,signin}.rs, guest/rust/quote/, guest/rust/Cargo.toml
examples/quote/{rapidkoi.toml,server.la}, .gitignore
docs/server-sign-in.md, docs/DESIGN.md, docs/source/meetings/2026-10-17-ui-contract.md (the fetch section)
CLAUDE.md, MEMORY.md (decision 18), DECISIONS.md (026), CHANGELOG.md, CONTEXT.md
```

### TESTS WRITTEN

- **network:** own host and declared servers; switched off; HTTPS, IPs and localhost; method, headers and body; manifest names.
- **bundle, project, pack:** servers round-trip and are checked; `[network]`; `--server`.
- **contract:** the fetch shape; replies.
- **requests:**
  - live: GET and POST, a 301 not followed, timeout, too large, absent, TLS with and without the CA
  - in the window: a refusal answered at once, 4 in flight, through `handle`, closed tab and old load dropped, switched off
- **people:** format 2 round trip and bad lists.
- **access:** the listing, switches, the notice and New, Forget, unsigned apps.
- **signin:** a server following the doc accepts once and refuses a replay, another host's signature and an unknown challenge.
- **examples/quote/server.la:** its sign-in and quote.
- **Rust:** encoding, decoding, `SignIn`'s steps and failures, and the quote app.

### DECISIONS MADE

- The answer value is a CBOR `Value`: bytes or null for `identity` and `sign`, a map for `fetch`.
- `msg` travels in `Msg.Responded`, so the model needs no request numbers. `AppPage.in_flight` is the count.
- `Person.fresh` is added beside `off`/`seen`, so the notice comes once while New lasts until the Access page is left.
- Response headers are a text map (std's: lower case, repeated values joined with ", ").
- The demo server reuses `rapidkoi.serve` and `rapidkoi.identity` from `examples/`.

### PENDING DECISIONS OPENED

- DECISION-026 (deferred, for the security review): a declared name resolving to this computer or the local network; only the certificate check guards it.

### STILL OPEN AT CLOSE

- The owner's security review, and the demo on screen (locally first; deploying to the VM is asked before anything is touched there).

---

## SESSION 64 — 2026-10-19 — Merge PRP-012 — closed

Branch: prp-012-network

### WHAT WAS DONE

The owner: "approved" (the PRP-012 security review). The checks, by exit status:
- `fmt --check`: clean
- `lughac test test/`: 304 passed
- the example's tests: 2 passed
- cargo: 3 + 2 + 9 passed
- the build: ok; no file over 500 lines

Fixed CLAUDE.md's stale known issue (Access is real now; DECISION-026 listed). Merged prp-012-network into main. The demo is not on the VM: that waits for the owner's go.

### FILES CREATED OR MODIFIED

```
CLAUDE.md, CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Deploying the quote demo to the VM (`quote.dafifi.net`), when the owner says go.

---

## SESSION 65 — 2026-10-19 — Deploy the quote demo to the VM — closed

Branch: deploy-quote-demo

### WHAT WAS DONE

The owner: "deploy the demo to the vm".
- **Built:** the quote app with `QUOTE_SERVER=https://quote.dafifi.net` into `~/rapidkoi-sites/quote.dafifi.net/target`, so the local demo is unchanged.
- **Signed:** `rapidkoi.toml` there (host `quote.dafifi.net`), a new key `~/.config/rapidkoi/keys/quote.dafifi.net.key` (fingerprint ce26-d7e9-f80c-c7d6), published.
- **On the VM, `/opt/rapidkoi/`:**
  - `quote/` holds the Dockerfile and the `quote-server` binary; `sites/quote.dafifi.net/` holds the site.
  - the image `rapidkoi-quote` was trial-run on loopback
  - a `quote` service was added to `docker-compose.yml` (backup `docker-compose.yml.bak-before-quote`), and only it was started; the counter and the other apps were untouched
- **Checked live with Rapidkoi's own code:**
  - the site verified as quote.dafifi.net, Quote 0.1.0
  - a sign-in got 200, and its replay 401
  - `/quote` with the token said `User 4996`
  - counter.dafifi.net still answers

### FILES CREATED OR MODIFIED

```
examples/quote/Dockerfile (new), CLAUDE.md (tree), CONTEXT.md
```

### TESTS WRITTEN

- None (a throwaway live check under test/tmp/, deleted).

### DECISIONS MADE

- The image's base is `ubuntu:26.04`, matching the machine the binary is linked on (libgc, OpenSSL 3).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner opens `rapidkoi://quote.dafifi.net`: the quote and "Signed in as User xxxx", then switching quote.dafifi.net off on the Access page.

---

## SESSION 66 — 2026-10-19 — Design: refusing server names that point to the local network — closed

Branch: design-private-addresses

### WHAT WAS DONE

The owner: "refuse server names that point to local network" (DECISION-026). Four questions in chat, each answered:
1. Block every local range: "yes, block all of those".
2. Both fetches and opening apps: "both".
3. std gains `net.resolve` and a connect-to-address option: "A".
4. A mixed answer is refused whole: "A".

Recorded the meeting notes, resolved DECISION-026, and wrote PRP-013. Gave the owner a prompt for the Lugha session that adds the std pieces. Nothing built: the PRP waits for the owner's approval and for the std addition.

### FILES CREATED OR MODIFIED

```
docs/source/meetings/2026-10-19-local-addresses.md (new), PRPs/prp-013-local_addresses.md (new), DECISIONS.md, CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- DECISION-026 resolved (the design).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-013's approval.
- The std addition (`net.resolve`, `tls.Options.address`) in `../lugha`, by its own session.

---

## SESSION 67 — 2026-10-19 — Fix tls.Options literals for m121 — closed

Branch: fix-tls-options

### WHAT WAS DONE

The Lugha session merged `net.resolve` and `tls.Options.address` (m121, PR #83), and the installed lughac has them. The two `tls.Options` literals no longer compiled, since they listed every field. They now take the rest from `..tls.options()`:
- `src/fetching.la:96`
- `src/requests.la:124`

Also merged `design-private-addresses` into main first. Checks:
- fmt: clean
- `lughac test test/`: 304 passed
- the example's tests: 2 passed
- `./main` and `quote-server` build

lughac rewrote `lugha.lock` while building, and it was committed with the fix.

### FILES CREATED OR MODIFIED

```
src/fetching.la, src/requests.la, lugha.lock, CONTEXT.md
```

### TESTS WRITTEN

- None (no behaviour changed).

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-013's approval (std now has what it needs).

---

## SESSION 68 — 2026-10-19 — Build PRP-013: refusing names that point to the local network — closed

Branch: prp-013-local_addresses

### WHAT WAS DONE

The owner: "approved" (PRP-013). Built test-first:
1. **`network.la`:** `local_address` (IPv4/IPv6 text, every range, IPv4 inside IPv6 as IPv4, garbage refused) and `check_addresses` (all public, else refused whole). Three new variants, `PointsLocal`, `NoAddress` and `Lookup`, each with a sentence.
2. **`dialing.la` (new):**
   - `dial`: look up with `net.resolve` or the test names, check, then `try_each` over `send_to`, pinned with `tls.Options.address`
   - development addresses are never looked up
   - `Network` moved here, with a test-only `hosts`
3. **Wiring:** `requests.send` and `fetching.get` go through `dial`. A refusal sends nothing. A load shows the sentence via a "refused: " prefix, as with redirects. `fetching.network` is renamed `failure`.

Checks:
- fmt: clean
- `lughac test test/`: 316 passed (12 new)
- the example's tests: 2 passed
- the build: ok; no file over 500 lines

Live: quote.dafifi.net resolved to 4 Cloudflare addresses, opened verified, and a fetch got 200. Not merged: it waits for the owner's security review.

### FILES CREATED OR MODIFIED

```
src/dialing.la, test/dialing.la (new)
src/network.la, src/fetching.la, src/requests.la, src/loader.la, src/loads.la, src/window.la
test/network.la, test/requests.la, test/fetching.la, and every Network literal (hosts: [])
test/fixtures/tls/README.md, CLAUDE.md, MEMORY.md, DECISIONS.md, CHANGELOG.md, PRPs/prp-013-local_addresses.md, CONTEXT.md
```

### TESTS WRITTEN

- **network:** IPv4 range edges, IPv6 and embedded IPv4, garbage, `check_addresses`.
- **dialing:**
  - lookup
  - refusals with nothing sent
  - a failed lookup
  - the pinned connection, name verified, and a wrong name failing its certificate
  - addresses tried in order within one deadline
  - development addresses never looked up
- **requests and fetching:** a name pointing to the local network is refused.

### DECISIONS MADE

- `Network` lives in `dialing.la`, imported from there, not re-exported.
- Final failures that don't move to the next address: a timeout, a certificate, a response too large or malformed.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner's security review, then merge.

---

## SESSION 69 — 2026-10-19 — Merge PRP-013 — closed

Branch: prp-013-local_addresses

### WHAT WAS DONE

The owner confirmed the demo works exactly as before, then said "merge" (taken as the security review). The checks, by exit status:
- fmt: clean
- `lughac test test/`: 316 passed
- the example's tests: 2 passed
- cargo: passed
- the build: ok

Merged prp-013-local_addresses into main.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- None.

---

## SESSION 70 — 2026-10-19 — Discuss moving the shell's pages into bundled apps — closed

Branch: main

### WHAT WAS DONE

The owner proposed a UI redesign whose first step moves as much UI as possible out of the shell into apps bundled with it (new tab first), leaving the shell to draw only the frame. Discussed, no code:
- Agreed in principle, with one line: screens that grant, approve or verify (permission prompts, sign-in approval, Access, Account's secret, the ⓘ panel's verification) stay native, since an app-drawn copy of them can be spoofed by any app. Content screens (new tab/home, search, Notifications, Data) can be bundled apps.
- Gaps found: contract 2 has only Text, Button, Input, Column, Row (home needs images, a wrapping grid, menus); bundled apps need privileged capabilities (read pins and left-off, open an address), each a guest-exposed host capability under the SECURITY RULE; DESIGN.md forbids apps setting colours, so a bundled home loses `themes.la` unless that rule changes; where the bundled .wasm comes from (cargo build embedded in the binary?); where this sits in the build path.
- Next: a discovery interview (PRPs/DISCOVERY.md) for a PRP piloting the new tab as a bundled app.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None (the owner hasn't answered yet).

### PENDING DECISIONS OPENED

- None yet; the gaps above become DECISIONS.md entries during discovery.

### STILL OPEN AT CLOSE

- The owner's answer on the trusted-screens line, then discovery question 1.

---

## SESSION 71 — 2026-10-19 — Discovery: the new tab as a bundled app — closed

Branch: redesign

### WHAT WAS DONE

- The owner accepted the line from session 70 (trust screens native, content pages bundled apps) and said the redesign will regress a lot, so nothing merges to `main` until it's all built.
- Created the long-lived branch `redesign` from `main`; each redesign PRP branches from it and merges back into it.
- Recorded DECISION-027 (resolved) and MEMORY.md decision 19.
- Asked discovery question 2: what the pilot new tab does on screen.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
DECISIONS.md
MEMORY.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- DECISION-027: the shell draws the frame and trust screens; content pages become bundled apps; built on `redesign`, merged to `main` only when complete.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Discovery question 2 (the pilot's scope: same as today's home, or redesigned).

---

## SESSION 72 — 2026-10-19 — PRP for the bare frame: header, sidebar, content area — closed

Branch: redesign

### WHAT WAS DONE

- The owner sent a sketch (header on top, sidebar on the right, content area with one rounded top-right corner) and asked for only that on screen, with − □ × in the header. That replaces discovery question 2: the redesign starts from the frame, not the new tab app.
- Found today's layout already has that shape (`Look.Frame`, `Look.Page` with the top-right radius, `FRAME_THICKNESS`), so the change is emptying it.
- Wrote PRP-014 (`PRPs/prp-014-bare_frame.md`): a new pure `layout.view`; the old view moves unchanged to `legacy.la` so the old view tests keep guarding `update` until later PRPs replace each part. Waiting for approval; no code.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
PRPs/prp-014-bare_frame.md
```

### TESTS WRITTEN

- None (listed in PRP-014).

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-014's approval, in particular keeping the old view as `legacy.la` for the old tests.

---

## SESSION 73 — 2026-10-19 — Build PRP-014: the bare frame — closed

Branch: prp-014-bare_frame

### WHAT WAS DONE

The owner approved PRP-014. Built it test first:
- `test/layout.la` (4 tests) failed first (no `layout` module), then passed.
- `src/layout.la`: the bare frame. An empty header bar (an empty title view, so GTK shows only − □ ×; the window title stays for the taskbar), an empty `Look.Frame` sidebar `FRAME_THICKNESS` wide on the right, and an expanding `Look.Page` content area.
- `src/legacy.la`: the old `view`, `card` and `app_view`, moved unchanged out of `shell.la`. The 12 old view-driven test files import `view` from it.
- `shell.view` is now `layout.view`; `shell.la`'s view-only imports went with the view.
- Checks, by exit status: fmt clean; `lughac test test/` 320 passed; build ok. The only warning is the old W0104 in `dialing.la`.
- Not merged into `redesign`: the owner checks the window first.

### FILES CREATED OR MODIFIED

```
src/layout.la (new)
src/legacy.la (new)
src/shell.la
test/layout.la (new)
test/{account,access,effects,counter,loading,loader,panel,remote,menus,saving,opening,shell}.la (import view from legacy)
docs/DESIGN.md
CLAUDE.md (file tree)
CHANGELOG.md
PRPs/prp-014-bare_frame.md (checked off; the visual check is the owner's)
CONTEXT.md
```

### TESTS WRITTEN

- test/layout.la: a fresh window shows no text; open tabs and a notice show nothing and no New tab or Close tab; home with a typed search shows nothing; the shell's view is the bare frame.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- The owner opens `./main` to check the frame; then merge `prp-014-bare_frame` into `redesign` (not `main`).

---

## SESSION 74 — 2026-10-19 — Discovery: the address box — closed

Branch: prp-014-bare_frame

### WHAT WAS DONE

- The owner sent a sketch of the address box: at rest, a rounded field "Enter Address"; while typing, a rounded card holding the field (with → to go) and suggestions under it: what was typed, then a "Rapidkoi suggest" group (e.g. `chatapp.com/about`).
- Checked kivuli 0.10: no popover anchored to a node, and `overlay` covers the window's body only, not the header bar, so a card that covers the field in the header can't be drawn today. Focus can be moved (`Cmd.focus`, `.autofocus`) but there's no focus or blur event.
- Asked discovery question 1: where the box sits and how the suggestions card appears. No code. PRP-014's visual check is still the owner's.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- None.

### PENDING DECISIONS OPENED

- None yet. A kivuli popover, if chosen, is a kivuli change in its own repo.

### STILL OPEN AT CLOSE

- Discovery question 1 for the address box PRP; the owner's look at PRP-014 before merging it into `redesign`.

---

## SESSION 75 — 2026-10-19 — Discovery: the address box (2) — closed

Branch: prp-014-bare_frame

### WHAT WAS DONE

- Question 1 answered: option A. The field stays in the header bar, centred; while typing, a rounded card of suggestions floats over the top of the page, lined up under it. A kivuli popover (option B) may come later as polish.
- The owner asked for the field to be a pill: ends fully round ("50% rounded"). kivuli radii are pixels, so the PRP fixes the field's height as a token and its radius at half of it, in a new `themes.la` look. kivuli looks apply to any node, a text field included; the look on screen is the owner's to check.
- Asked question 2: where the suggestions come from.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- Address box placement: in the header, suggestions dropping below it over the page (option A).
- The field is a pill (radius = half its height).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Discovery question 2; PRP-014's visual check and merge into `redesign`.

---

## SESSION 76 — 2026-10-19 — Discovery: the address box (3) — closed

Branch: prp-014-bare_frame

### WHAT WAS DONE

- Question 2 answered: option A, local suggestions only. First what was typed (Enter or → goes there), then "Rapidkoi suggest": the open tabs, pins and where-you-left-off that match (home's search, moved into the box). An app's screens join once links exist; nothing typed leaves the computer.
- Read today's box: `Msg.Draft`, `Msg.Go` (Enter), `Msg.EditAddress` (the marker on an app page), `Msg.FocusSearch` (Ctrl+K).
- Asked question 3: how the box and card behave (opening, closing, keys).

### FILES CREATED OR MODIFIED

```
CONTEXT.md
```

### TESTS WRITTEN

- None.

### DECISIONS MADE

- Suggestions are local only (open tabs, pins, left off).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- Discovery question 3; PRP-014's visual check and merge into `redesign`.

---

## SESSION 77 — 2026-10-19 — PRP-015: the address box — closed

Branch: prp-014-bare_frame

### WHAT WAS DONE

- Question 3 answered: the behaviour as proposed. The card shows while the box has text; Enter, → or a suggestion opens and closes it; Escape or a click outside closes it and puts back what the box showed; no arrow keys until kivuli has key events in a field.
- Wrote PRP-015 (`PRPs/prp-015-address_box.md`), with the remaining discovery answered in it as recommendations for the owner to approve:
  - failed opens show their reason in the card;
  - the content area shows the current app again (`app_view` moves from `legacy.la` to `content.la`), since an opened app must show somewhere;
  - the overlay spans content and sidebar so the card centres under the pill.
- No code.

### FILES CREATED OR MODIFIED

```
CONTEXT.md
PRPs/prp-015-address_box.md
```

### TESTS WRITTEN

- None (listed in PRP-015).

### DECISIONS MADE

- The address box's behaviour (question 3).

### PENDING DECISIONS OPENED

- None.

### STILL OPEN AT CLOSE

- PRP-015's approval; PRP-014's visual check and merge into `redesign`.

---

## SESSION 78 — 2026-10-19 — Build PRP-015: the address box — open

Branch: prp-015-address_box

### WHAT WAS DONE

- In progress.

---

## NEXT SESSION START POINT

PRP-014 (the bare frame) is built on `prp-014-bare_frame`, waiting for the owner's look at `./main`, then a merge into `redesign` (never `main`: DECISION-027). PRP-015 (the address box) is written and waiting for approval; once approved and PRP-014 is merged, branch `prp-015-address_box` from `redesign`, tests first.
