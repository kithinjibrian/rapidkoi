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
- Tests live in `test/`, mirroring `src/`: `src/update.la` → `test/update.la`. Keep test code out of `src/`.
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

Every spacing, padding, size and text size comes from the tokens in `docs/DESIGN.md`. App content's colours come only from the GTK theme through kivuli's styles (`ButtonStyle`, `.dim()`). Rapidkoi's own parts (bar, rail, home) take colours from its themes in `src/themes.la` and nowhere else. Never hardcode a colour outside `themes.la`, never give an app a way to set colours, never pass a bare number to `.padding`, `.spacing` or `.size`.

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
- **kivuli 0.10** (`kivuli = { git = "https://github.com/kithinjibrian/kivuli", version = "0.10" }`): the UI. 0.10 added `button_with` (a whole row as one button); 0.9 context menus (`.menu`); 0.8 `overlay`; 0.7 `wrap`; 0.6 added background tasks (`Cmd.task`, `Sub.receive`); 0.5 added images from bytes (`Image.Bytes(key, bytes)`, `.image(…)`); 0.4 added per-corner radius; 0.3 added `.header_style` and `Surface.flat`; 0.2 renamed `.style(ButtonStyle…)` to `.button_style(…)`; `.style([…])` takes looks. Import through the package entry: `import kivuli { View, text, sandbox };`. Only the entry and `kivuli.testing` are public.
- **lugha-gtk** comes in through kivuli. rapidkoi does not import `gtk` directly.
- **GTK 4** + GLib, through kivuli.
- **wasmtime** (C API v49.0.3) runs guests, through the package `wasmtime` (`wasmtime = { git = "https://github.com/kithinjibrian/lugha-wasmtime", version = "0.1" }`; source in `../lugha-wasmtime`). rapidkoi never declares a wasmtime `extern` itself.
- **CBOR** through the package `cbor` (`cbor = { git = "https://github.com/kithinjibrian/lugha-cbor", version = "0.1" }`; source in `../lugha-cbor`). The guest↔shell contract is version 2 (MEMORY.md decision 10; PRP-011): exports `rapidkoi_version`, `alloc`, `handle(state, event)` and `view`, events for everything. Implement it exactly; any change is a DECISIONS.md entry and a version bump.
- **Guests:** Rust (`wasm32-unknown-unknown`) under `guest/rust/`, built with cargo, never by `lughac`. The shell's tests use hand-written `.wat` fixtures and never need Rust.
- **SQLite** for app data, from build path step 3 or 4 (MEMORY.md decision 5). Not before.

---

## ARCHITECTURE RULES

1. **The app is one kivuli program: a `Model`, a `Msg` enum, a pure `update`, and a pure `view`.** *Why:* that is kivuli's contract; state changed anywhere else is invisible to the diff and won't render.
2. **Never touch a GTK widget or import `gtk` from rapidkoi.** *Why:* kivuli owns the widgets; reaching past it breaks its diff and pinned-closure bookkeeping.
3. **`view` never does I/O, and `update` does no file, process or network I/O.** That work goes through kivuli commands (`Cmd`) or a module that returns `Result`, called from a command. Two exceptions in the shell's `update` (`src/shell.la`, `src/tabs.la`): it calls running guests synchronously, each call bounded by fuel (`src/limits.la`), it opens local app files the person chose (≤ 32 MB, PRP-002), and it reads and saves Rapidkoi's own small files in its data folder (`src/saving.la`, `src/states.la`: trust ≤ 1 MB, pins ≤ 32 MB, left-off ≤ 8 MB, each signed app's state ≤ `MAX_STATE` + 64 KB, written safely, PRP-007; it moves to `Cmd.task` when std's file I/O stops blocking the thread). Fetching an address is never done in `update`: it describes a `Cmd.task` (`src/loads.la`, `src/fetching.la`, PRP-008), and trust is recorded and the module compiled only when its result arrives. An app's requests are the same: each is checked (`src/network.la`) and sent in a `Cmd.task` (`src/requests.la`, PRP-012), never in `update`, and answered when its result arrives. *Why:* blocking in `update` freezes the window; the guest is part of the shell's model, and fuel keeps each call short (PRP-001).
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
├── src/               — main.la (entry: kivuli application, wiring only), shell.la (boot, update, view: composition only), layout.la (pure: the frame, PRP-014), addressbox.la (pure: the pill and its suggestions card, PRP-015), suggestions.la (pure: what the box suggests), content.la (pure: the current app in the content area), legacy.la (the pre-redesign view, for the old tests until the redesign replaces it),
│                        window.la (the Model, and changes that open nothing: tabs, home, pins, left off), navigate.la (opening: the address box, files, addresses, Continue, Reload), themes.la (Rapidkoi's colours as kivuli looks; dark first),
│                        bundle.la (pure: bundle format 1, read/write), rules.la (pure: every bundle rule, BundleError), loader.la (a path or address → manifest + compiled module, verified when signed),
│                        signing.la (format 2: signed bundles, read either format), keys.la (secret keys: keygen, read, fingerprints), index.la (the signed index and the public key file, rotations),
│                        project.la (rapidkoi.toml), publish.la (the `keygen` and `publish` commands, pack from rapidkoi.toml), verify.la (opening a signed site: every check, the keyring),
│                        store.la (the data folder: safe writes, bounded reads, set aside), remembered.la (the trust, pins and left-off files), states.la (each signed app's state, by host and key),
│                        identity.la (the master secret, per-app keys, sign-ins), people.la (sign-in settings and your name), effects.la (answering what apps ask for), account.la (the Account page), placeholders.la (pure: {FIRST_NAME}),
│                        network.la (pure: which requests an app may make), requests.la (apps' requests: sent in tasks, answered on arrival), dialing.la (looking a name up, refusing local addresses, connecting to exactly the checked ones; Network), access.la (the Access page: servers, switches, the notice),
│                        running.la (a guest's init and opened events, messages, answers and view), saving.la (restore at start, save on every change), fetching.la (a background load's work: fetch and check against a snapshot), loads.la (begin a load, take its result),
│                        pack.la (the `./main pack` command), serve.la (the `./main serve` development server: a bundle or a site), address.la (pure: local development addresses),
│                        tabs.la (Page, Tab, Pin; an app page's life: open, events, restart, resume), leftoff.la (LeftOff: leave, snapshot, recalled, signer_of), actions.la (the shell's Msg),
│                        frame.la (pure: the bar and the tab rail), home.la (pure: home and search), panel.la (pure: app info),
│                        guest.la (a running guest: store, limits, calling convention), contract.la (pure: CBOR ↔ contract v1; re-exports nodes' types),
│                        nodes.la (the decoded types: Node, style words, ContractError), props.la (reading one node's props),
│                        render.la (pure: Node → kivuli View; the error screen), theme.la (DESIGN.md tokens), limits.la (every limit)
│                        Imports run one way: nodes ← props ← contract ← theme ← render ← tabs ← actions ← frame ← home, panel ← window ← shell ← main; limits ← contract, guest ← tabs; themes ← window
├── test/              — @test files mirroring src/, support.la (CBOR builders, fixture guests as WebAssembly text, settle), fixtures/tls/ (a throwaway test CA: tests only),
│                        counter.la (end to end with the Rust counter; skips until it's built)
├── guest/rust/        — Cargo workspace: rapidkoi (contract 2 for Rust: View builder, App trait, export!, Request, SignIn), counter (the demo), quote (the network demo)
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
- **A second `./main other.wasm` raises the first window** instead of opening the app: kivuli makes GTK single-instance (DECISION-014).
- **The address box's focus ring is the field's, inside the pill**, not the whole pill's: kivuli can't style focus yet (DECISION-028).
- **Placeholders until kivuli and PRP-003 catch up (DECISION-018), each to be replaced, never extended:**
  - Opening a file means typing its path into the box, since there's no file dialog.
  - An app without a bundle icon shows its initial on its tab and tile; there's no generated art or banners yet (PRP-004).
  - Search is in memory only. Open tabs aren't restored on launch yet (the next PRP). Unsigned apps' state lasts only for the session (PRP-007 amendment 2).
  - Back, forward, share, settings and more are shown disabled.
  - Notifications and Data are placeholder pages until notifications and app data exist.
  - Remote apps open by host name over HTTPS only, signed only, with no redirects followed (PRP-009); plain http and unsigned bundles are for this machine's development servers. Loads run in the background (PRP-008), but a DNS lookup, a TLS handshake and compiling a module still briefly hold the window (kivuli's note). Reload is manual; there's no live reload yet.
