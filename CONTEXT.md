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

## NEXT SESSION START POINT

Open session 2 first. Then read CLAUDE.md, MEMORY.md, DECISIONS.md, CONTEXT.md. Ask the user to resolve DECISION-001 (what rapidkoi does). Record the answer in `docs/source/stakeholder/`, move DECISION-001 to RESOLVED and into MEMORY.md, then run a discovery interview for the first feature and write `PRPs/prp-001-….md`. Before any view code, read `../kivuli/README.md` and `docs/DESIGN.md`.
