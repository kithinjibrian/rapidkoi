# CLAUDE.md — rapidkoi

rapidkoi is the Rapidkoi shell: a native desktop app, written in Lugha with kivuli (Elm architecture over GTK 4), that runs sandboxed WebAssembly apps and renders the screens they describe. The vision is `docs/source/stakeholder/vision-2026-10-17.md` (core decision 4): don't re-litigate it.

The project also grows Lugha's ecosystem (core decision 9): generic pieces become their own packages, and gaps in Lugha's tools are fixed upstream.

Where things are recorded: **DECISIONS.md** (every decision and its why: core decisions, then open, deferred and resolved), **PRPs/** (a spec per feature), **CHANGELOG.md**, and git's log. Commit messages say what changed and why.

---

## HOW WE WORK

- **Discuss before building.** Research rather than guess, and cite what you checked.
- **A feature needs an approved PRP** in `PRPs/prp-{NNN}-{feature_name}.md` (next unused number, from `PRPs/TEMPLATE.md`). Without one: no code; run a discovery (`PRPs/DISCOVERY.md`), one question at a time, each with a recommended answer, then write the PRP and wait for approval. Small fixes and chores skip the PRP when the owner says so.
- **One PRP at a time.** If the scope grows mid-way: stop, say what you found, and get approval before going on.
- **Work on a branch per PRP or chore**, from `main`. Merge into `main` only when the owner asks, after the full suite passes.
- **Never make an architectural choice silently.** Check DECISIONS.md first: `open` blocks the work (ask the owner); `resolved` is followed, not re-litigated; a new question is added as `open`, and you stop.
- **Missing or broken lughac, kivuli, lugha-wasmtime, lugha-cbor, lugha-sqlite or rapidkoi-rust features are never worked around.** Record it in DECISIONS.md (a request in `docs/specs/` when it helps), report it, and stop. It's fixed in that repository under its own rules, never from here.
- **Generic code doesn't go in rapidkoi.** Anything another Lugha program could use unchanged belongs in its own package: propose one.
- **No copy in screens beyond what the owner asked for.**
- **Files stay at or under 500 lines**, tests included. At the limit, propose a split and wait for approval.

## TESTING

- **Test first.** For every new function, a failing `@test` first, then the least code that passes it. Every function used outside its module gets a happy-path test and one per error it can return.
- Tests live in `test/`, flat, named for the module (`src/ui/layout.la` → `test/layout.la`, DECISION-033). No test code in `src/`.
- `update` and `view` are pure: test them without a display, through `kivuli.testing` (press buttons, type into inputs, read texts).
- Anything that opens a GTK window starts with `if !has_display() { skip("no display"); }`.
- The shell's tests never need Rust: stand-in apps are WebAssembly text in `test/support.la` (`counting()` for a real-looking one).
- **Run the headless tests yourself** while working (the files covering the change), and the full suite before saying a task is done. The owner checks anything visual: never claim something looks or works on screen because it compiled.
- A task is done when `lughac fmt --check src test` is clean, `lughac test test/` passes (skips only for "no display"), and `lughac build -O 2 src/main.la -o main` succeeds.

## SECURITY — NON-NEGOTIABLE

Never build these without the owner's explicit review: authentication, sessions, cryptography, payments, secrets, running processes (`std.process`), and **any host function exposed to a WebAssembly guest** (each is a capability untrusted code can call). If unsure whether something is security-sensitive, stop and ask.

- Never hardcode credentials, keys or tokens, not even in examples. Never log sensitive data.
- Never build SQL, shell commands or file paths from untrusted input: values go in as parameters.
- Validate all external input (files, args, env, network). Never show a raw internal error: map it to a sentence in the view.
- A new dependency, `extern` or `[link]` library needs its reason in the PRP. An `extern` matches its C prototype exactly: check the header.
- **Guests are untrusted.** Everything a bundle sends (trees, messages, lengths, pointers) is hostile: bounds-check every pointer and length (a bad one is a decode error, never a crash); cap tree size, depth, string length, fuel, time and memory (`src/guest/limits.la`), and a cap hit stops that app with a sentence; a guest never gets a GTK handle, a file path, or anything the shell didn't decide to give it.

## OUTWARD-FACING ACTIONS

Ask before anything that leaves this machine: creating repositories, adding remotes, pushing, tagging, opening PRs or issues, and anything on the VM (show the exact commands first). Before a push or tag, check the test run's real exit status: `… | tail` hides a failure.

## PROTECTED FILES

Never read, modify or delete: `.env` and `.env.*`; `lugha.lock` (written only by `lughac build`/`update`); built outputs (the paths in `.gitignore`). Sibling repositories are read for reference and never written from here: `../lugha/`, `../kivuli/`, `../lugha-gtk/`, `../lugha-wasmtime/`, `../lugha-cbor/`, `../lugha-sqlite/`, `../lugha-imagesize/`, `../rapidkoi-rust/`. Keep `.llmignore` and `.claude/settings.json` in step with this list. Never build or install `lughac` yourself: when a newer one is needed, ask the owner.

---

## COMMANDS

```bash
lughac check src/main.la              # type-check
lughac build -O 2 src/main.la -o main # build ./main (needs gtk4 via pkg-config); -O 2 for smaller stack frames (DECISION-059)
lughac test test/                     # every @test (or name files: lughac test test/tabs.la)
lughac fmt --check src test           # formatting; drop --check to rewrite
lughac update src/main.la             # re-resolve git packages (rewrites lugha.lock)
lughac explain E0312                  # an error code
lughac spec                           # the language reference: check it before using any feature

# Rapidkoi's own apps (guest/rust/: newtab, settings), on rapidkoi-rust by tag: cargo, never lughac
cargo test --manifest-path guest/rust/Cargo.toml
./scripts/newtab.sh && ./scripts/settings.sh   # build and repack into assets/apps/ (after building ./main), then rebuild ./main

# Running apps (opening a window is the owner's)
./main app.wasm | ./main app.rkb | ./main rapidkoi://localhost:8000
./main pack app.wasm --name App --version 0.1.0 --description "…" --icon icon.png -o app.rkb
./main serve app.rkb --port 8000      # 127.0.0.1 only
# Signed apps, from the folder whose rapidkoi.toml describes the app (e.g. ../rapidkoi-rust/examples/counter/)
./main keygen      # once; the key goes outside the project, mode 0600
./main publish     # writes site/.well-known/rapidkoi/; --withdraw VERSION --reason TEXT
./main serve site/
```

Needs: `lughac` (the owner installs it from `../lugha`), `libgtk-4-dev`, `libsqlite3-dev`, `pkgconf`, `libgc-dev`, `cc`; Rust with `wasm32-unknown-unknown` for the built-ins.

## STACK

- **Lugha** via `lughac` 0.1.0: only what `lughac spec` documents.
- **kivuli 0.65** (git package): the UI. Import through its entry (`import kivuli { View, text };`); only the entry and `kivuli.testing` are public. Styling is one door, `.style([...])` of typed values (`Font`, `Spacing`, `Sizing`, `Background`, `Border`, `When(State, …)`, …). Read `../kivuli/README.md` or its source before using any API.
- **GTK 4** only through kivuli. rapidkoi never imports `gtk`.
- **wasmtime** (C API 49) through `lugha-wasmtime` 0.1 (`wat_to_wasm` since 0.1.1). rapidkoi declares no wasmtime `extern`.
- **Picture headers** through `lugha-imagesize` 0.1 (PRP-047): a picture's format and size before anything decodes it.
- **CBOR** through `lugha-cbor` 0.1. **SQLite** through `lugha-sqlite` 0.1: one database per app, every connection `Guard.untrusted()`, every call in a `Cmd.task`.
- **The contract is version 8, the only one** (`docs/specs/contract-8.md`, DECISION-056, PRP-053): named-field state, links (with `kinds`), `on_` events, boxes, texts (with `parts`: your name and times, filled in by Rapidkoi) and typing boxes (with `label`), styles, roles, storage, `now`. Older apps are refused, never run. Any change to it is a DECISIONS.md entry and a version bump, and the guest↔host contract carries its version from its first commit: published bundles can't be recompiled.
- **The Rust SDK is `rapidkoi-rust`** (`../rapidkoi-rust`, a git dependency by tag, now `v0.3.0-pre.1`: contract 8, with parts and `now`) with the examples counter and styled. Notes is its own repository, `../notes` (core decision 27). A contract change is tagged there first; then the built-ins move to the tag and are repacked.

## ARCHITECTURE

1. **The app is one kivuli program:** a `Model`, a `Msg` enum, a pure `update` and a pure `view`. State changed anywhere else doesn't render.
2. **Never touch a GTK widget or import `gtk`.** kivuli owns the widgets.
3. **`view` does no I/O, and `update` does no file, process or network I/O**: that goes through `Cmd` (`Cmd.task`) or a module returning `Result` called from one. Fetches, apps' requests and apps' database calls are checked, then run in tasks, and answered when the result arrives. The exceptions, in `update`: calling running guests (bounded by fuel), opening a local app file the person chose (≤ 32 MB), Rapidkoi's own small files in its data folder (`data/saving.la`, `data/states.la`), Forget removing a database and an app's kept copy (PRP-043, PRP-052), and reading the system's time zone offset (`time.local_offset`, in `window/zones.la` only; the C library caches it, PRP-053).
4. **`src/main.la` is wiring only**: everything worth testing lives in a module.
5. **Follow the build path in order** (shell → links → a real app → storage and sync → live sessions → history). Steps 1–3 and storage are done; sync is parked by the owner. Never build part of a later step early.

**Layout:** `src/main.la` (wiring), `src/shell.la` (boot, update, view: composition), and folders: `window/` (the model and its changes), `ui/` (pure views, the theme, tokens), `guest/` (running guests and the contract), `bundles/`, `trust/` (keys, signed sites), `net/`, `data/` (the data folder, storage), `people/` (identity, accounts), `commands/` (`pack`, `publish`, `serve`). Imports name the folder (`import ui.layout;` binds `layout`) and run one way: guest.nodes ← props ← contract ← ui.tokens ← themes ← render ← window.tabs ← actions ← ui.frame ← suggestions ← window.window ← ui.content, addressbox ← layout ← shell ← main. Rapidkoi's built-in apps are `assets/apps/*.rkb` (committed; repack, then rebuild `./main`); icons are `assets/icons/` (built in with `include_str`).

**Errors:** expected failures return `Result<T, E>` with the module's own error enum; programmer errors `panic`. Never `Option` for an error, never a bare `string` error, never `unwrap`/`expect` on anything external (`expect("…")` only for true invariants). `update` turns every `Err` into model state the view shows; it never panics on one.

## DESIGN AND CODE STYLE

- **Read `docs/DESIGN.md` before writing any view.** Every spacing, size and text size in Rapidkoi's own screens is a token from it; every colour comes from `src/ui/themes.la` (the desktop theme is ignored, DECISION-031). Never a hardcoded colour, never a bare number in `Spacing`, `Sizing`, `Font` or `Border`.
- Apps style their own content (contract 7): their values are checked by `src/guest/styles.la`, and Rapidkoi's names they use are resolved by `src/ui/paint.la`. The frame, address and dialogs stay Rapidkoi's.
- **Read `docs/CODE_STYLE.md` before writing code.** Rapidkoi is a program, so nothing is `public` (an item with no keyword is visible to the whole package, tests included). Every item used outside its module has a `/** … */` comment; every non-obvious decision inside a function has a `//` comment saying why. Code reads like the code around it.

## ANTI-PATTERNS

1. Never use a Lugha feature without checking `lughac spec`, or a kivuli API from memory.
2. Never name a function `of` (a keyword), a method like a field (E0310), or a local after an imported module (E0311).
3. Never reassign a captured variable in a closure (E0511): capture a `Ref` and `set`/`update` it.
4. Never compare messages or views with `==`: test views through `kivuli.testing`.
5. Never let a guest's failure crash the shell: a trap, a decode error or a cap hit ends that app.

Known gaps waiting on kivuli, lughac or the packages are DECISIONS.md's deferred entries (DECISION-018 lists the placeholders): don't fix or work around them here.
