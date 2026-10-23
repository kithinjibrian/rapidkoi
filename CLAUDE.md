# CLAUDE.md — rapidkoi

Behavioral instructions for AI coding assistants. Each rule exists to prevent a specific mistake.

rapidkoi is the Rapidkoi shell: a native desktop app, written in Lugha with kivuli (Elm architecture over GTK 4), that runs sandboxed WebAssembly apps and renders the component trees they describe. The vision is `docs/source/stakeholder/vision-2026-10-17.md`; MEMORY.md decision 4 summarises it. Do not re-litigate it.

The project has a second goal: growing Lugha's ecosystem (MEMORY.md decision 9). Generic pieces become standalone Lugha packages, and gaps in Lugha's tools are fixed upstream as contributions.

---

## SESSION HANDOFF RULE — NON-NEGOTIABLE

Every session in `CONTEXT.md` must have:
- A **name**: a short title for what the session did
- A **state**: `open` while work is in progress, `closed` once the handoff is done
- A **branch**: the git branch the session's work lives on

Format:

    ## SESSION {n} — {YYYY-MM-DD} — {Name} — {state}
    Branch: {branch-name}

Rules:
- **Step 1 of every session, no exceptions:** append a new session entry to `CONTEXT.md` with state `open` and the current branch name. Do this before reading any other file, before planning, before writing any code. Commit it before any other work begins.
- Mark it `closed` only after CONTEXT.md is updated and committed, and pushed once a remote exists. There is no remote until the user says to publish (see OUTWARD-FACING ACTIONS).
- Never leave a session `open` at the end of a turn.
- Never start a new session without closing the previous one first.
- The NEXT SESSION START POINT block is rewritten at the end of every session.
- Sessions are never deleted. The full history stays in CONTEXT.md.

---

## PRP RULE — NON-NEGOTIABLE

Never write code for a new feature without a PRP file in `/PRPs`.

If a feature request is given without a PRP:
1. Do not write any code.
2. Run a discovery interview (`PRPs/DISCOVERY.md`), one question at a time, each with a recommended answer attached.
3. Write the PRP to `/PRPs/prp-{NNN}-{feature_name}.md` from `PRPs/TEMPLATE.md`.
   - `NNN` is the next unused number, zero-padded to three digits. Numbers are never reused or renumbered.
   - `feature_name` is lowercase snake_case: `prp-001-main_window.md`.
4. Present it to the user and build only after explicit approval.

A vague prompt is not a starting point. It is the beginning of a discovery.

---

## SCOPE RULE — NON-NEGOTIABLE

One PRP at a time. Never implement more than one feature's scope in a single session.

If mid-implementation the scope turns out larger than the PRP described: stop, document what was discovered, update or create a PRP, and get approval before continuing. The human decides what is "small enough to add", not the model.

**A missing or broken lughac, kivuli, lugha-wasmtime or lugha-cbor feature is never worked around silently.** Stop, record it in `DECISIONS.md`, and report it. It is fixed in its own repository (`../lugha`, `../kivuli`, `../lugha-wasmtime`, `../lugha-cbor`) under that repo's `CLAUDE.md`, never from a rapidkoi session.

**Generic code doesn't go in rapidkoi.** Anything another Lugha program could use unchanged (bindings to a C library, a general data structure) belongs in its own package. Propose it as a new package instead of writing it here.

---

## TESTING RULE — NON-NEGOTIABLE

Write the test before writing the implementation. No exceptions.

- For every new function, write a failing `@test` first, then the minimum code to make it pass.
- Tests live in `test/`, flat, each file named for the module it tests: `src/ui/layout.la` → `test/layout.la` (a test in a subfolder can't import `test/support.la`, DECISION-033). Keep test code out of `src/`.
- `update` and `view` are pure: test them with no display. Use `kivuli.testing` to find and press buttons and type into inputs in a view tree.
- Anything that opens a GTK window must start with `if !has_display() { skip("no display"); }` so it is skipped, not failed, without one.
- Test behavior visible to callers: inputs, outputs, and every error variant. Do not test kivuli's, GTK's, or lughac's own behavior.
- Every new `public` function gets at least one happy-path test and one test per error variant it can return.
- While working, run only the test files that cover the change. Run the full suite once before a PRP's branch is merged.
- Opening real windows is the user's job. Never claim a visual behavior works because it compiled.

If the PRP does not describe what to test, add the test cases to the PRP before writing code.

---

## SECURITY RULE — NON-NEGOTIABLE

Never implement without explicit human review and approval:
- Authentication, authorization, or session management
- Cryptographic operations (hashing, signing, encrypting)
- Payment processing
- Secrets or credential handling
- Running external processes (`std.process`)
- **Any host function exposed to a WebAssembly guest.** Each one is a capability that untrusted code can call

For everything else:
- Never hardcode credentials, API keys, tokens, or secrets, not even in comments or examples.
- Never build SQL, shell commands, or file paths by concatenating untrusted input.
- Never trust external input (files, CLI args, env, network) without validating it first.
- Never log sensitive data (passwords, tokens, PII).
- Never show a raw internal error to the user. Map it to a message in the view.
- Never add a package dependency, an `extern`, or a `[link]` library without stating why in the PRP. An `extern` must match its C prototype exactly: check the header, do not guess.

**Guests are untrusted.** Everything a bundle sends (view trees, messages, lengths, pointers into its memory) is hostile input:
- Bounds-check every pointer and length against the guest's memory before reading. A bad offset is a decode error, never a crash.
- Cap what a guest can make the shell do: tree size, depth, string length, runtime fuel/time and memory. Exceeding a cap stops that app with an error the person sees.
- A guest never gets a GTK handle, a file path, or anything the shell didn't explicitly decide to give it.

If unsure whether something has a security implication, stop and ask.

---

## OUTWARD-FACING ACTIONS — NON-NEGOTIABLE

Ask before anything that leaves this machine: creating the GitHub repository, adding a remote, `git push`, tagging a release, opening PRs or issues anywhere. Before any push or tag, check the test run's real exit status: `lughac test … | tail` hides a failure.

---

## DESIGN RULE — NON-NEGOTIABLE

Read `docs/DESIGN.md` before writing any `view` code.

Every spacing, size and text size in Rapidkoi's own screens comes from the tokens in `docs/DESIGN.md`. Every colour Rapidkoi draws comes from its theme in `src/ui/themes.la`, through kivuli's `.style`; the desktop's GTK theme is ignored (DECISION-031). Never hardcode a colour outside `ui/themes.la`, never write a bare number in a `Spacing`, `Sizing`, `Font` or `Border`: use a token.

Apps style their own content freely (DECISION-049, contract 5, `docs/specs/contract-5.md`): their raw values are checked and bounded by `src/guest/styles.la`, and Rapidkoi's names they use are resolved from the theme and tokens by `src/ui/paint.la`. Rapidkoi's frame, address and dialogs stay Rapidkoi's, where apps can't draw.

---

## CODE DOCUMENTATION RULE — NON-NEGOTIABLE

Read `docs/CODE_STYLE.md` before writing any function, type, or module.

Every `public` item has a `/** … */` doc comment. Every non-obvious decision inside a function has a `//` comment explaining *why*, not what.

---

## PENDING DECISIONS RULE — NON-NEGOTIABLE

Before writing code that depends on an unresolved architectural question, check `DECISIONS.md`.

- `open`: stop. Do not implement. Ask the user to resolve it.
- `resolved`: follow the recorded outcome. Do not re-litigate.
- A new unresolved question mid-implementation: add it to `DECISIONS.md` as `open` and stop. Do not guess.

Never make an architectural choice silently.

---

## FILE SIZE RULE

No file exceeds 500 lines, tests included (raised from 300 by the owner, session 25).

When a file reaches the limit: stop, propose a specific split (new file names and what moves where), wait for approval, split, then continue.

---

## PROTECTED FILES — NON-NEGOTIABLE

Never read, modify, or delete:

- `.env` and `.env.*`
- `lugha.lock`: written only by `lughac build`/`update`, never hand-edited
- Built executables (the paths in `.gitignore`)
- `../lugha/`, `../kivuli/`, `../lugha-gtk/`, `../lugha-wasmtime/`, `../lugha-cbor/`: other repositories with their own rules. Read for reference; never write from a rapidkoi session

If a task seems to require touching a protected file, stop and ask. See also `.llmignore`. `.claude/settings.json` denies Claude's Read and Edit tools on the same paths. Keep all three in sync.

---

## COMMANDS

```bash
lughac check src/main.la              # lex, parse, type-check
lughac build src/main.la -o main      # compile and link to ./main (needs gtk4 via pkg-config)
lughac run src/main.la                # build and run (opens a window, so it's the user's)
lughac test test/                     # every @test; GTK tests skip themselves without a display
lughac fmt --check src test           # formatting; drop --check to rewrite
lughac update src/main.la             # re-resolve git packages, rewrite lugha.lock
lughac explain E0312                  # what an error code means
lughac spec                           # the full Lugha language reference: check it before assuming a feature exists

# The Rust guest (guest/rust/): cargo, never lughac
cargo test --manifest-path guest/rust/Cargo.toml
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown
./main guest/rust/target/wasm32-unknown-unknown/release/counter.wasm   # opens a window: the user's

# Bundles: pack the counter into one .rkb, then open it
./main pack guest/rust/target/wasm32-unknown-unknown/release/counter.wasm --name Counter --version 0.1.0 \
    --description "Counts, and says hello" --icon guest/rust/counter/icon.png -o counter.rkb
./main counter.rkb                                                     # opens a window: the user's

# Rapidkoi's new-tab app (PRP-027): build and pack it into assets/apps/newtab.rkb (after building ./main)
./scripts/newtab.sh
# Rapidkoi's Settings app (PRP-038): the same, into assets/apps/settings.rkb; rebuild ./main after
./scripts/settings.sh
./main assets/apps/newtab.rkb                                          # opens a window: the user's

# Local development: serve a bundle, open it at its address (Reload in ⓘ after a repack)
./main serve counter.rkb --port 8000                                   # 127.0.0.1 only; Ctrl+C to stop
./main rapidkoi://localhost:8000                                       # opens a window: the user's

# Signed apps: rapidkoi.toml describes the app (the repo's own is the counter's)
./main keygen          # once: the secret key at [publish].key, outside the project, mode 0600
./main publish         # signs and writes site/.well-known/rapidkoi/{key, index, builds/<sha256>.rkb}
./main publish --withdraw VERSION --reason TEXT
./main pack            # one .rkb from rapidkoi.toml, signed when it has a host and a key
./main serve site/     # serves the site; ./main rapidkoi://localhost:8000 opens it verified

# The network demo (PRP-012): the quote app and its Lugha server, in examples/quote/
cd examples/quote && ../../main keygen && ../../main publish && cd ../..
lughac test examples/quote/server.la                                   # the server's own tests
lughac build examples/quote/server.la -o quote-server
./quote-server --site examples/quote/site --host localhost:8000 --port 8000
./main rapidkoi://localhost:8000                                       # opens a window: the user's
```

`test/counter.la` drives the built counter end to end; it skips itself until the guest is built.

A task is done only when `fmt --check` is clean, `lughac test test/` passes (skips allowed only for `no display`), and `lughac build src/main.la -o main` succeeds.

> Needs: `lughac` (the user installs it from `../lugha` with `cargo install --path . --locked`; never do it yourself), `libgtk-4-dev`, `pkgconf`, `libgc-dev`, `cc`.

---

## STACK

- **Lugha** via `lughac` 0.1.0. Use only what `lughac spec` documents. **Never build or reinstall `lughac` yourself**: when rapidkoi needs a newer one, stop and ask the user.
- **kivuli 0.54** (`kivuli = { git = "https://github.com/kithinjibrian/kivuli", version = "0.54" }`): the UI. Group states `Group.mark()`, `State.GroupHover`/`GroupFocusWithin` (0.52), `scroll_fit` (0.53), `Cmd.scroll_to(name)` (0.54). 0.19–0.49: clicking away clears focus, `kivuli.ui` components and palettes (Rapidkoi's header uses them), SVG images (0.48), styling the window controls (0.49), `Sizing.icon(px)` (0.50), popovers `popover(trigger, open, content, on_close)` / `ui.popover` (0.51). 0.11–0.17 made styling one door, `.style([...])` of typed values (`Font`, `Text`, `Background`, `Border`, `Spacing`, `Sizing`, `Shadow`, `Effects`, `Outline`, `When(State, …)`), removing `.bold`, `.dim`, `.padding`, `.spacing`, `.width`, `.height`, `.button_style` and named looks; there are no button looks (Rapidkoi styles buttons itself, DECISION-031). 0.18 made the header bar a node: `.header(header_bar(start, title, end).style([...]))`. Earlier: `button_with` (0.10), context menus `.menu` (0.9), `overlay` (0.8), `wrap` (0.7), background tasks `Cmd.task`/`Sub.receive` (0.6), images from bytes (0.5). Import through the package entry: `import kivuli { View, text, sandbox };`. Only the entry and `kivuli.testing` are public.
- **lugha-gtk** comes in through kivuli. rapidkoi does not import `gtk` directly.
- **GTK 4** + GLib, through kivuli.
- **wasmtime** (C API v49.0.3) runs guests, through the package `wasmtime` (`wasmtime = { git = "https://github.com/kithinjibrian/lugha-wasmtime", version = "0.1" }`; source in `../lugha-wasmtime`). rapidkoi never declares a wasmtime `extern` itself.
- **CBOR** through the package `cbor` (`cbor = { git = "https://github.com/kithinjibrian/lugha-cbor", version = "0.1" }`; source in `../lugha-cbor`). The guest↔shell contract is version 5 (MEMORY.md decisions 10, 21, 24, 25; PRP-011, PRP-030, PRP-036, PRP-037: boxes, texts, styles and roles, `docs/specs/contract-5.md`; closed with PRP-039), on top of version 4: exports `rapidkoi_version`, `alloc`, `handle(state, event)` and `view`, events for everything, named `on_` on the wire (`docs/specs/app-events.md`), and a state of named fields whose `link` names what goes in the window's address (`docs/specs/contract-4.md`). Contract 2, 3 and 4 apps still run (DECISION-039, DECISION-048, DECISION-049). After contract 5 closes, any change is a DECISIONS.md entry and a version bump.
- **Guests:** Rust (`wasm32-unknown-unknown`) under `guest/rust/`, built with cargo, never by `lughac`. The shell's tests use hand-written `.wat` fixtures and never need Rust.
- **SQLite** for app data, from build path step 3 or 4 (MEMORY.md decision 5). Not before.

---

## ARCHITECTURE RULES

1. **The app is one kivuli program: a `Model`, a `Msg` enum, a pure `update`, and a pure `view`.** *Why:* that is kivuli's contract; state changed anywhere else is invisible to the diff and won't render.
2. **Never touch a GTK widget or import `gtk` from rapidkoi.** *Why:* kivuli owns the widgets; reaching past it breaks its diff and pinned-closure bookkeeping.
3. **`view` never does I/O, and `update` does no file, process or network I/O.** That work goes through kivuli commands (`Cmd`) or a module that returns `Result`, called from a command. Two exceptions in the shell's `update` (`src/shell.la`, `src/window/tabs.la`): it calls running guests synchronously, each call bounded by fuel (`src/guest/limits.la`), it opens local app files the person chose (≤ 32 MB, PRP-002), and it reads and saves Rapidkoi's own small files in its data folder (`src/data/saving.la`, `src/data/states.la`: trust ≤ 1 MB, pins ≤ 32 MB, left-off ≤ 8 MB, each signed app's state ≤ `MAX_STATE` + 64 KB, written safely, PRP-007; it moves to `Cmd.task` when std's file I/O stops blocking the thread). Fetching an address is never done in `update`: it describes a `Cmd.task` (`src/net/loads.la`, `src/net/fetching.la`, PRP-008), and trust is recorded and the module compiled only when its result arrives. An app's requests are the same: each is checked (`src/net/network.la`) and sent in a `Cmd.task` (`src/net/requests.la`, PRP-012), never in `update`, and answered when its result arrives. *Why:* blocking in `update` freezes the window; the guest is part of the shell's model, and fuel keeps each call short (PRP-001).
4. **Keep `src/main.la` to wiring:** `main` calls kivuli with the initial model, `update` and `view` imported from their modules. *Why:* `main.la` can't be tested on its own; everything worth testing lives in a module.

5. **Follow the vision's build path in order** (shell → links → a real app → storage and sync → live sessions → history). Never build part of a later step early. *Why:* each step shapes the next; a speculative later piece locks in guesses.
6. **The guest↔host contract carries a version from its first commit.** *Why:* it is the platform's public API, and published bundles can't be recompiled.

Add rules here as PRPs settle the module layout.

---

## ERROR HANDLING — NON-NEGOTIABLE

Expected failures return `Result<T, E>` (Lugha's prelude). Programmer errors `panic`.

```
Expected failure  →  return Err(...)
Truly unexpected  →  panic("...")   (an invariant broke)
```

```
// Good: the caller sees the failure in the type
public fun load_settings(path: string): Result<Settings, SettingsError> { … }

// Bad: hides "missing file" and "bad format"
public fun load_settings(path: string): Settings { … }
```

- Each module that can fail defines its own error enum. Errors are typed, never a bare `string`.
- **Never** use `Option` to mean an error. `None` can't say why.
- **Never** `unwrap`/`expect` on anything derived from user or external input. `expect("reason")` is only for true invariants, and its message states the invariant.
- `update` turns an `Err` into model state (an error message the view shows). It never panics on one.
- Startup failures (no display) end the program with GTK's exit status from `main`. They are not caught.

---

## FILE ORGANIZATION

```
.
├── CLAUDE.md          — this file
├── MEMORY.md          — resolved decisions + project state
├── CONTEXT.md         — session handoff log
├── DECISIONS.md       — open/deferred/resolved questions
├── CHANGELOG.md       — what shipped
├── .llmignore         — protected paths
├── .claude/           — settings.json: deny rules mirroring .llmignore
├── lugha.toml         — package `rapidkoi`, source `src`, depends on kivuli
├── src/               — main.la (entry: kivuli application, wiring only), shell.la (boot, update, view: composition only), and nine folders (PRP-018):
│                        window/   window.la (the Model, and changes that open nothing: app and empty tabs, pins, left off), actions.la (the shell's Msg), tabs.la (Page, Tab, Pin; an app page's life, its link fields), restarts.la (resume, with_saved, load_again: a page started again from saved state),
│                                  leftoff.la (LeftOff: leave, snapshot, recalled, signer_of), navigate.la (opening: the address box, files, addresses, Continue, Reload, another instance), newtab.la (the new-tab app, built in with include_bytes), settings_app.la (the Settings app, built in the same way), instances.la (named instances: names, last used, renaming, closing), pins.la (pinning and Forget, split from window.la)
│                        ui/       layout.la (pure: the frame: header bar, sidebar, content area), addressbox.la (pure: the address box), suggestions.la (pure: what the box will suggest; Card, matches),
│                                  content.la (pure: the current app; the new-tab app on the empty tab), header.la (pure: the header bar), strip.la (pure: the header's instance strip and the rename dialog), shield.la (pure: the shield's facts, look and card), settings.la (pure: site settings' card and the confirm dialog), grants.la (pure: the Permissions row and page, `controls`, the approval dialog), sidebar.la (pure: the apps down the right), cards.la (pure: the pieces both cards share), icons.la (Rapidkoi's Iconsax icons, built in from assets/icons/, recoloured), frame.la (pure: names, the trust marker, the changed-key question), render.la (pure: Node → kivuli View; error screens), paint.la (pure: an app's look resolved and turned into kivuli styles),
│                                  tokens.la (DESIGN.md tokens and style helpers), schemes.la (the four colour sets, the person's Prefs, contrast), themes.la (the looks, from the theme), frame_looks.la (the frame's looks: header, sidebar, window controls, tooltips)
│                        guest/    guest.la (a running guest: store, limits, calling convention), running.la (init and opened events, a link's fields, messages, answers, view), links.la (pure: contract 4's link names, an address's fields, a link set in a fresh state), events.la (pure: events named by contract version), home.la (pure: the new tab's reads), limits.la (every limit),
│                                  contract.la (pure: CBOR ↔ contract; re-exports nodes' types), nodes.la (the decoded types), tree5.la (pure: contract 5's screens), styles.la (pure: contract 5's style map, checked), roles.la (pure: roles and their states), props.la (reading one node's props), placeholders.la (pure: {FIRST_NAME}), effects.la (answering what apps ask for, behind their permissions)
│                        bundles/  bundle.la (pure: format 1), rules.la (pure: every bundle rule, BundleError), permissions.la (pure: permissions, their tiers), signing.la (format 2: signed bundles), loader.la (a path or address → manifest + module, verified when signed), project.la (rapidkoi.toml)
│                        trust/    keys.la (secret keys: keygen, read, fingerprints), index.la (the signed index and public key file, rotations), verify.la (opening a signed site: every check, the keyring)
│                        net/      network.la (pure: which requests an app may make), requests.la (apps' requests, in tasks), dialing.la (lookups, refusing local addresses; Network),
│                                  fetching.la (a background load's work), loads.la (begin a load, take its result), address.la (pure: addresses and links: parse, write, without_link)
│                        data/     store.la (the data folder: safe writes, bounded reads), remembered.la (the trust, pins, left-off and open files), sessions.la (the current and previous sessions: launch, Restore), states.la (each signed app's state), preferences.la (the person's choices: the file, what apps read, a change), saving.la (restore at start, save on every change)
│                        people/   identity.la (the master secret, per-app keys, sign-ins), people.la (sign-in settings and your name), account.la (an app's account: the model side), access.la (what an app can reach: the model side)
│                        commands/ pack.la (`./main pack`), publish.la (`keygen`, `publish`), serve.la (`./main serve`: a bundle or a site), landing.la (pure: the landing page a browser shows)
│                        Imports name the folder (`import ui.layout;` binds `layout`). They run one way: guest.nodes ← props ← contract ← ui.tokens ← themes ← render ← window.tabs ← actions ← ui.frame ← suggestions ← window.window ← ui.content, addressbox ← layout ← shell ← main
├── test/              — @test files, flat, each named for the module it tests (DECISION-033), support.la (CBOR builders, fixture guests as WebAssembly text, settle), fixtures/tls/ (a throwaway test CA: tests only),
│                        counter.la (end to end with the Rust counter; skips until it's built)
├── guest/rust/        — Cargo workspace: rapidkoi (contract 5 for Rust: raw primitives, style, widgets, typed messages (Message, View<M>, .map), App trait, State (nested), Next, Field paths (then), export!, Request, SignIn, Place), rapidkoi-derive (#[derive(State)]), styled (contract 5's styling shown), settings (Rapidkoi's Settings, PRP-038), counter (the demo: its count in its address), quote (the network demo), newtab (Rapidkoi's new tab, PRP-027)
├── assets/apps/       — newtab.rkb and settings.rkb: Rapidkoi's built-in apps, packed unsigned by scripts/newtab.sh and scripts/settings.sh and committed; built into ./main by src/window/newtab.la and settings_app.la (repack, then rebuild ./main)
├── scripts/           — newtab.sh, settings.sh: build and pack the built-in apps (their --permission and --control flags)
├── assets/icons/      — the Iconsax SVGs, the only copy: `src/ui/icons.la` builds each in with include_str (PRP-029); change a file, then rebuild
├── examples/quote/    — the network demo's rapidkoi.toml, its server (server.la: the signed site, sign-in, /quote) and Dockerfile (the VM)
├── PRPs/              — prp-{NNN}-{feature_name}.md (+ TEMPLATE.md, DISCOVERY.md)
├── docs/
│   ├── CODE_STYLE.md  — documentation rules for .la code
│   ├── DESIGN.md      — spacing/text tokens, kivuli styles
│   ├── server-sign-in.md — what an app's server checks to sign a person in (PRP-012)
│   ├── setup.md       — the context-system guide this setup follows
│   ├── source/        — meetings/, research/, stakeholder/, constraints/: read on demand
│   └── specs/ decisions/ incidents/ status/
└── reports/           — EOD reports (YYYY-MM-DD.md)
```

Update this tree when modules are added.

---

## ANTI-PATTERNS

1. **Never use a Lugha feature without checking `lughac spec`.** The language is young, and invented syntax wastes a session. If documented syntax is rejected, check the installed `lughac` is current first.
2. **Never use a kivuli API from memory.** Read `../kivuli/README.md` or its `src/` first. A guessed modifier fails to compile or, worse, is silently ignored.
3. **Never name a function `of`.** It is a keyword.
4. **Never give a method the same name as a field (E0310).**
5. **Never name a local or parameter after an imported module (E0311).**
6. **Never reassign a captured variable in a closure (E0511).** Capture a `Ref` and `set`/`update` it.
7. **Never compare messages or views with `==`.** A `View` holding closures isn't `Eq`; test views through `kivuli.testing`.
8. **Never let a guest's failure crash the shell.** A trap, a decode error or a cap hit ends that app, not Rapidkoi.

---

## KNOWN ISSUES — DO NOT FIX

- **A missing `update`, `alloc` or `view` export is only found on its first call**, not at load: lugha-wasmtime can't list exports yet (DECISION-017). The error screen still shows.
- **A second `./main other.wasm` raises the first window** instead of opening the app: kivuli makes GTK single-instance (DECISION-014). So there's one window, and instances can't be side by side yet (DECISION-042).
- **Instances don't share data** until storage (build step 4): each is the app with its own whole state (PRP-033).
- **The instance strip doesn't adapt to the window** (DECISION-043, kivuli): it's a fixed 520 px with at most three pills, so in a narrower window the address box shifts, and the window can't shrink past the strip.
- **After a pick in ⚙'s Permissions page, the card doesn't close on a click inside the window or Escape** (DECISION-040, kivuli): ⚙ again, or a click outside the window, closes it.
- **No Ctrl+K for the address box**: kivuli gives shortcuts to buttons only, and the address is one field now (PRP-025).
- **Placeholders until kivuli and PRP-003 catch up (DECISION-018), each to be replaced, never extended:**
  - Opening a file means typing its path into the box, since there's no file dialog.
  - An app without a bundle icon shows its initial on its tab and tile; there's no generated art or banners yet (PRP-004).
  - Search is in memory only. A window's state lives with the window (session 189): closing it deletes its state, and an app opened anew starts fresh. Only Restore brings a signed window's state back; an unsigned one's lasts only for the run. What should last waits for storage (build step 4). Continue's meaning is parked (DECISION-044).
  - Share and account are shown disabled (share needs kivuli's clipboard, DECISION-047; the address box's link can be selected and copied). There's no back, home or forward: in-app navigation is the app's own (DECISION-041).
  - Notifications and Data are placeholder pages until notifications and app data exist.
  - Remote apps open by host name over HTTPS only, signed only, with no redirects followed (PRP-009); plain http and unsigned bundles are for this machine's development servers. Loads run in the background (PRP-008), but a DNS lookup, a TLS handshake and compiling a module still briefly hold the window (kivuli's note). Reload is manual; there's no live reload yet.
