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

## CURRENT PROJECT STATE

### Fully Working
- `src/main.la` prints a greeting; it passes `lughac check` and `fmt --check`.
- Context system in place (session 1).

### In Progress
- Nothing.

### Not Started
- Deciding what rapidkoi does (DECISION-001).
- Any feature.

---

## NEXT SESSION START POINT

Read CLAUDE.md, MEMORY.md, DECISIONS.md and CONTEXT.md. Resolve DECISION-001 with the user (what rapidkoi is for), write a stakeholder note in `docs/source/stakeholder/`, then run a discovery interview for the first feature and write `PRPs/prp-001-….md`.
