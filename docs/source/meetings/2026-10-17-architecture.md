# Meeting — 2026-10-17 — Step 1 architecture

**Attendees:** the project owner, Claude (chat, sessions 2–3)
**Type:** planning

---

## What Was Discussed

Where an app's state lives. The owner agreed the platform should hold it, and raised that offline apps need real storage, such as SQLite or DuckDB, not just an in-memory model. That split state into two kinds: small screen state (route, filters, drafts, selection) held by Rapidkoi as the app's model, and app data in a database Rapidkoi provides.

How an app hands over its UI: a whole, versioned description per update, rendered through kivuli, with events sent back as the app's own messages.

Which WebAssembly engine runs apps, and which language the first test app is written in.

The owner added a second goal for the project: improving Lugha's ecosystem.

## Decisions Made

- Screen state is a platform-held model; apps are Elm-shaped (`init`, `update`, `view`). (DECISION-004)
- App data lives in a per-app SQLite database owned by Rapidkoi, not compiled into bundles; it arrives at build path step 3 or 4. DuckDB is not the default. How sync handles conflicts is decided at steps 4–5. (DECISION-004)
- Apps send a whole, compact, versioned UI description per update. Step 1 components: text, button, text input, column, row. (DECISION-005)
- wasmtime is the engine, used through its C API. (DECISION-003)
- The first guest app and guest UI library are in Rust under `guest/rust/`; the shell's tests use hand-written `.wat` fixtures. (DECISION-006)
- Rapidkoi also exists to grow Lugha's ecosystem. Generic pieces become standalone Lugha packages; `lugha-wasmtime` is a separate repository from day one. (DECISION-010)

## Constraints Introduced

- The UI description format must stay language-neutral, so a future Lugha guest library is as natural as the Rust one.

## Open Questions Raised

- How sync and live sessions resolve conflicting edits to app data (last write wins, change log, or CRDTs such as cr-sqlite). Deferred to steps 4–5: DECISION-011.

## Action Items

- [ ] Owner — install wasmtime's C API — before the shell's PRP is built
- [ ] Owner — `rustup target add wasm32-unknown-unknown` — before the guest app is built

## Impact on the Project

DECISIONS 003–006 are resolved and build path step 1 is unblocked, pending its discovery interview and PRP. The shell depends on a new sibling package, `../lugha-wasmtime`. Any lughac or kivuli gap found along the way is fixed upstream as a contribution, never worked around. A WebAssembly target in lughac, plus a Lugha guest library, is a long-term milestone, done in the lugha repository.
