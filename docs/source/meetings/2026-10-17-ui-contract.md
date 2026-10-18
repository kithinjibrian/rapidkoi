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


## Contract 2 (PRP-011, 2026-10-18): one entry point, events for everything

Replaces v1 outright (nothing had shipped). A module's `rapidkoi_version()` returns 2, and a bundle's manifest says `contract: 2`; anything else is refused ("This app needs a newer version of Rapidkoi").

- **Exports:** `rapidkoi_version`, `alloc`, `handle(state, event) → {state, effects}`, `view(state) → tree`. There's no `init` or `update` export.
- **Events,** CBOR maps with an `event` field (an app ignores kinds it doesn't know; new ones, like closing, come later as new kinds):
  - `{event: "init"}`: a fresh start, with an empty state; make the first one
  - `{event: "opened"}`: every open, right after init on a fresh start, or straight away when resumed from a saved state; the place to ask for things
  - `{event: "message", msg, value?}`: the app's own messages (a click; typing, with the text as `value`)
  - `{event: "answer", msg, value}`: Rapidkoi's reply to an effect; `value` is bytes, or null
- **The order:**
  - fresh: `handle([], init)`, `handle(state, opened)`, `view`
  - resumed: `handle(saved, opened)`, `view`
  - then `handle` and `view` for each message or answer
- **Effects,** from any event's result (at most 8; anything else stops the app with a plain reason):
  - `{kind: "identity", msg}`: the app's own public key for the person (32 bytes)
  - `{kind: "sign", msg, challenge: bytes 1–1024}`: an Ed25519 signature (64 bytes) over the CBOR `{kind: "rapidkoi sign-in", host, challenge}`, where `host` is the one Rapidkoi verified
  - each is answered by an answer event, at most 32 in a row, or the app is stopped
- **Placeholders:** `{FIRST_NAME}` and `{LAST_NAME}` in text nodes' text and button labels are filled while drawing with the app's name for the person (or nothing). Never in text fields or screen titles, never in anything sent back.
- **Keys:** each app's Ed25519 seed is `hkdf_sha256(master, "", CBOR ["rapidkoi identity v1", host, origin, generation], 32)`. `origin` is the key first trusted for the app (kept through rotations); `generation` counts "Start over as someone new".

### Contract 2's fetch effect (PRP-012, 2026-10-19)

Added to contract 2 without a version bump (nothing had shipped); an app that never asks for it is unchanged.

- **The effect:** `{kind: "fetch", msg, method, url, headers?: {text: text}, body?: bytes}`.
  - Its shape is checked when the result is read: method and url are text, the url at most 8192 bytes, at most 64 headers each at most 8192 bytes, and the body is bytes. A wrong shape stops the app, like any bad effect.
  - Whether it may be sent is checked next (`src/network.la`). A refusal is answered at once and nothing leaves the computer.
- **What it may reach:**
  - a signed app: its own host and the manifest's `servers` (signed with it), minus those switched off on the Access page
  - HTTPS by name only: plain `http://` only to this computer, and only for an app from this computer; never an IP address
  - an unsigned app: only `localhost` servers it declares, and its own development server
- **The request:**
  - GET, POST, PUT, PATCH, DELETE
  - no `host`, `cookie`, `content-length`, `connection`, `transfer-encoding` or `upgrade` headers; header names are tokens, values have no control characters
  - a body of at most 1 MB
- **Sending:** in the background, at most 4 at once per app. No redirects are followed, no cookies are kept, and Rapidkoi adds no credentials. Each request has 30 s, and the response body is at most 8 MB.
- **The answer:** `{event: "answer", msg, value}`, where `value` is:
  - `{status, headers: {text: text}, body: bytes}` from the server, any status: a redirect comes back with its `location`; header names are in lower case, and repeated headers are joined with ", "
  - or `{error: text}`, Rapidkoi's sentence ("api.acme.com isn't one of Notes's servers", "You turned off api.acme.com for Notes", "Too many requests at once; try again", "api.acme.com didn't answer in 30 seconds", …)
  - a result for a closed tab, or for a page loaded again since, is dropped
- **Signing in to the app's server:** `docs/server-sign-in.md`.
