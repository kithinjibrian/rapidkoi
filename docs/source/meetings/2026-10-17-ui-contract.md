# Meeting — 2026-10-17 — The UI contract (v1)

**Attendees:** the project owner, Claude (chat, session 4)
**Type:** planning

---

## What Was Discussed

We first checked that `../lugha-wasmtime` covers everything the shell needs: its PRP-001 is merged, 56 of 56 tests pass, and a probe built kivuli and wasmtime together through a local `path` package. Then we designed the contract between a guest app and the shell in four parts: the byte format, the shape of the view tree, how events return to the app, and versions and limits. Lugha's std has no JSON or CBOR, so the decoder is written in Lugha either way.

## Decisions Made

**Format.** CBOR (RFC 8949). The decoder and encoder are a new standalone, pure-Lugha package, `../lugha-cbor` (package `cbor`), for the whole ecosystem.

**Tree shape.** Every node is a map with `kind`, `props` and (for containers) `children`, plus an optional `key` reserved for list identity later:

    { "kind": "column", "props": { "spacing": "md" }, "children": [
        { "kind": "text",   "props": { "text": "Count: 3", "style": "title" } },
        { "kind": "button", "props": { "label": "Add", "style": "primary", "on_press": <message> } },
        { "kind": "input",  "props": { "value": "Amina", "placeholder": "Your name", "on_change": <message> } }
    ]}

- Step 1 kinds: `text`, `button`, `input`, `column`, `row`.
- Props describe meaning, never pixels or colours (`"spacing": "md"`, `"style": "title"`). The shell maps them to `docs/DESIGN.md` tokens and the GTK theme. The owner accepted that apps can't choose exact sizes or colours.
- An unknown prop is ignored. An unknown kind renders a visible "unsupported component" placeholder; the rest of the screen still works.

**Events.**
- A message is any CBOR value the app chooses. The shell returns it untouched and never interprets it.
- The shell sends an envelope `{ "msg": <message>, "value": <event data> }`; `value` only when the event has data (an input's new text).
- No `on_press` means disabled.
- The guest is three byte functions: `init() → state`, `update(state, event) → result`, `view(state) → tree`. The shell keeps the state bytes between calls. Events are handled one at a time, in order.
- `update` returns `{ "state": <bytes>, "effects": [] }`; `effects` is reserved and stays empty in step 1.

**Versions.** Every guest exports `rapidkoi_version()`, returning the contract's major version (`1`). The shell checks it before any other call; an unsupported version shows "This app needs a newer version of Rapidkoi". Additive changes (new kinds or props) don't bump it.

**Limits** (starting values, in one place in the code):

| Limit | Value |
|---|---|
| Fuel per call | ≈ 100 ms of work (calibrated in the PRP) |
| Memory | 256 MB per app |
| View tree | 4 MB, 10,000 nodes, depth 64 |
| One string | 64 KB |
| State | 16 MB |

- The CBOR decoder checks every claimed length against the bytes actually present before allocating, and enforces a depth limit without unbounded recursion.
- A trap, a limit hit or bad data stops only that app: the shell shows "This app stopped working" with a **Restart** button.

## Open Questions Raised

- None. The exact byte layout of pointers and lengths across calls, and the fuel calibration, are details for the step 1 PRP.

## Action Items

- [x] Claude — record this, set up `../lugha-cbor`
- [ ] Owner — run the `lugha-cbor` PRP session, then the Rapidkoi step 1 session

## Impact on the Project

This is contract v1, the platform's core API (DECISION-012). The step 1 PRP implements it exactly, with nothing more. `lugha-cbor` comes first, because the shell can't read a guest without it (DECISION-013).
