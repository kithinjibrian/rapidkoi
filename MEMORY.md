# MEMORY.md — rapidkoi

Records resolved architectural decisions and current project state.
Read this at the start of every session before writing any code.

---

## ARCHITECTURAL DECISIONS

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

**Decision:** The project follows `docs/setup.md`: CLAUDE.md, MEMORY.md, CONTEXT.md, DECISIONS.md, CHANGELOG.md, .llmignore, PRPs/, docs/.

**Why:** Keeps AI sessions consistent across handoffs. Same setup as `../kivuli`.

**Rules out:** Feature code without a PRP; architectural choices made silently.

---

### 4. What rapidkoi is (DECISION-001)

**Decision:** This repository is the Rapidkoi shell: a native desktop app that runs sandboxed WebAssembly apps ("bundles"), each at an address, each screen linkable with its state. Guests describe their UI as a component tree; the shell renders it through kivuli and gives every app the platform features (links, permissions, storage, sync, live sessions, history). Work follows the vision's build path in order: 1 the shell, 2 links, 3 a real app, 4 storage and sync, 5 live sessions, 6 history.

**Why:** The owner's vision: `docs/source/stakeholder/vision-2026-10-17.md`.

**Rules out:** Guests drawing their own widgets or touching GTK; guests reaching any resource except through the shell; work from a later build-path step before the earlier ones are done.

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

## CURRENT PROJECT STATE

### Fully Working
- `src/main.la` prints a greeting; it passes `lughac check` and `fmt --check`.
- Context system in place (session 1). Vision recorded (session 2). Architecture decisions recorded (session 3).

### In Progress
- Nothing.

### Not Started
- Build path step 1, the shell: decisions made; needs its discovery interview and PRP-001.
- `../lugha-wasmtime`: repository scaffolded (session 3); no bindings yet.

---

## NEXT SESSION START POINT

Read CLAUDE.md, MEMORY.md, DECISIONS.md and CONTEXT.md. Run the discovery interview for build path step 1 (the shell) in chat, one question at a time, and write `PRPs/prp-001-shell.md`. The shell needs wasmtime bindings first: that work is a PRP in `../lugha-wasmtime`, under its own CLAUDE.md.
