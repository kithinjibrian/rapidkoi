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

## SESSION 3 — 2026-10-17 — Record the architecture decisions — open

Branch: main

---

## NEXT SESSION START POINT

Open session 3 first. Then read CLAUDE.md, MEMORY.md, DECISIONS.md, CONTEXT.md and `docs/source/stakeholder/vision-2026-10-17.md`. Get the user's answers to DECISIONS 003–006 and move them to RESOLVED and into MEMORY.md. Then run a discovery interview for build path step 1 (the shell: load a local `.wasm` bundle, render a few basic components through kivuli, send events back) and write `PRPs/prp-001-shell.md`. Before any view code, read `../kivuli/README.md` and `docs/DESIGN.md`. If wasmtime is chosen, the user installs it; check its C header before writing any extern.
