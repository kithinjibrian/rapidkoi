# DECISIONS.md — rapidkoi

Tracks architectural and design questions that are open, deferred, or resolved.

Rules:
- Every open decision blocks implementation of the code it affects.
- The AI must not implement anything that depends on an open decision.
- When a decision is resolved, move it to the RESOLVED section and record the outcome.
- Once resolved, copy the outcome to MEMORY.md as an architectural decision.

---

## OPEN — Requires human input before implementation

### DECISION-001 — What rapidkoi is for

**Status:** open
**Raised:** 2026-10-17 — Session 1
**Resolved by:** human
**Blocks:** Every feature PRP, the module layout, and the design tokens' final values.

**Question:** What does rapidkoi do, and for whom? What does its first usable version have to let someone do?

**Options:**
- A) The user writes a short brief (purpose, users, first version's scope); the AI turns it into `docs/source/stakeholder/brief-{YYYY-MM-DD}.md`.
- B) A discovery interview, one question at a time, producing the same note.

**Notes:** The repo holds only the `lughac init` placeholder and a kivuli dependency, so nothing in the code answers this.

---

## DEFERRED — Acknowledged, not yet needed

### DECISION-002 — Publishing

**Status:** deferred
**Raised:** 2026-10-17 — Session 1
**Revisit when:** The user wants a remote, or a first release.

**Question:** Where is the repository hosted (e.g. `kithinjibrian/rapidkoi` on GitHub), public or private, and under what license?

**Notes:** Safe to defer: work is local-only until then, and no push happens without asking.

---

## RESOLVED

(none yet)
