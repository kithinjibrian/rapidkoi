# CLAUDE.md — rapidkoi

Behavioral instructions for AI coding assistants. Each rule exists to prevent a specific mistake.

rapidkoi is the Rapidkoi shell: a native desktop app, written in Lugha with kivuli (Elm architecture over GTK 4), that runs sandboxed WebAssembly apps and renders the component trees they describe. The vision is `docs/source/stakeholder/vision-2026-10-17.md`; MEMORY.md decision 4 summarises it. Do not re-litigate it.

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

**A missing or broken lughac or kivuli feature is never worked around silently.** Stop, record it in `DECISIONS.md`, and report it. It is fixed in its own repository (`../lugha`, `../kivuli`) under that repo's `CLAUDE.md`, never from a rapidkoi session.

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

Every spacing, padding, size and text size comes from the tokens in `docs/DESIGN.md`. Colours come only from the GTK theme through kivuli's styles (`ButtonStyle`, `.dim()`). Never hardcode a colour, never write custom CSS, never pass a bare number to `.padding`, `.spacing` or `.size`.

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

No file exceeds 300 lines, tests included.

When a file reaches the limit: stop, propose a specific split (new file names and what moves where), wait for approval, split, then continue.

---

## PROTECTED FILES — NON-NEGOTIABLE

Never read, modify, or delete:

- `.env` and `.env.*`
- `lugha.lock`: written only by `lughac build`/`update`, never hand-edited
- Built executables (the paths in `.gitignore`)
- `../lugha/`, `../kivuli/`, `../lugha-gtk/`: other repositories with their own rules. Read for reference; never write

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
```

A task is done only when `fmt --check` is clean, `lughac test test/` passes (skips allowed only for `no display`), and `lughac build src/main.la -o main` succeeds.

> Needs: `lughac` (the user installs it from `../lugha` with `cargo install --path . --locked`; never do it yourself), `libgtk-4-dev`, `pkgconf`, `libgc-dev`, `cc`.

---

## STACK

- **Lugha** via `lughac` 0.1.0. Use only what `lughac spec` documents. **Never build or reinstall `lughac` yourself**: when rapidkoi needs a newer one, stop and ask the user.
- **kivuli 0.1** (`kivuli = { git = "https://github.com/kithinjibrian/kivuli", version = "0.1" }`): the UI. Import through the package entry: `import kivuli { View, text, sandbox };`. Only the entry and `kivuli.testing` are public.
- **lugha-gtk** comes in through kivuli. rapidkoi does not import `gtk` directly.
- **GTK 4** + GLib, through kivuli.

---

## ARCHITECTURE RULES

1. **The app is one kivuli program: a `Model`, a `Msg` enum, a pure `update`, and a pure `view`.** *Why:* that is kivuli's contract; state changed anywhere else is invisible to the diff and won't render.
2. **Never touch a GTK widget or import `gtk` from rapidkoi.** *Why:* kivuli owns the widgets; reaching past it breaks its diff and pinned-closure bookkeeping.
3. **`update` and `view` never do I/O.** File, process and network work goes through kivuli commands (`Cmd`) or a module that returns `Result`, called from a command. *Why:* blocking in `update` freezes the window.
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
├── src/               — main.la (entry: a placeholder hello-world today)
├── test/              — @test files mirroring src/ (created with the first PRP)
├── PRPs/              — prp-{NNN}-{feature_name}.md (+ TEMPLATE.md, DISCOVERY.md)
├── docs/
│   ├── CODE_STYLE.md  — documentation rules for .la code
│   ├── DESIGN.md      — spacing/text tokens, kivuli styles
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

- **`src/main.la` imports `kivuli` but doesn't use it.** It is the `lughac init` placeholder, kept until the first PRP replaces it.
