## FEATURE: Identity: every signed app gets its own key pair for you, signed in automatically, and can show your name without ever knowing it (`{FIRST_NAME}`, `{LAST_NAME}`)

## OBJECTIVE
- **Keys:**
  - Opening a signed app gives it, on request, its own public key for you: a stable ID that means nothing outside it, so two apps can't tell they share a person.
  - The app can also ask Rapidkoi to sign a challenge, for the day it can reach its server.
  - This happens with no prompt (auto sign-in, on by default).
- **Names:** an app can write `Hello, {FIRST_NAME}!`, and Rapidkoi fills in your name while drawing the screen. The app never learns it.
- **The panel's Account page** shows "Signed in as User 4f9a". It has the auto sign-in switch, Sign out / Sign in, Start over as someone new (which asks first), and your name.

Designed in chat, session 55: `docs/source/meetings/2026-10-18-identity.md`. Authentication and secrets: **the owner's security review before merge** (CLAUDE.md security rule).

## CONTEXT

- **Starting state:**
  - Contract v1: `update` returns `{state, effects: []}`, and effects are ignored.
  - Guests have no host functions and no network.
  - Signed apps have an identity (host and key) in the keyring; `Identity.key` changes on a rotation.
  - The Account page is a placeholder ("Not signed in").
- **Ending state:**
  - **Contract v2** (DECISIONS.md entry, version bump):
    - `rapidkoi_version()` returns 2; a manifest's `contract` may be 1 or 2. v1 apps are unchanged.
    - **Effects** in `update`'s result: `{kind: "identity", msg}` and `{kind: "sign", msg, challenge: bytes ≤ 1024}`. Each is answered by one message `{msg, value}`:
      - the public key (32 bytes)
      - the signature (64 bytes) over the CBOR `{kind: "rapidkoi sign-in", host, challenge}`
      - `null` when not signed in, or for an unsigned app
    - **Placeholders:** `{FIRST_NAME}` and `{LAST_NAME}` in display text (`text` nodes' text and button labels), replaced by Rapidkoi while drawing. Not replaced in text fields (value or hint).
  - `src/identity.la` (new):
    - the master secret: 32 bytes from `std.crypto.random_bytes`, made on first need, kept in the data folder as `identity` (0600, written safely)
    - each app's seed: `hkdf_sha256(master, "", CBOR ["rapidkoi identity v1", host, origin key, generation], 32)` → `ed25519_public(seed)` and `ed25519_sign(seed, …)`
    - `sign_in(…)` builds the host-bound message
  - `src/people.la` (new): the `people` file (format 1), per app identity (host, origin key):
    - `generation` (starts at 0; Start over adds one)
    - `auto` (default true)
    - `signed_out`
    - per-app name overrides
    - the global first and last name
  - `src/verify.la`, `src/remembered.la`:
    - `Identity.origin`: the key first trusted, carried through rotations, so an app's identity, and your key with it, survives a rotation
    - a separate publisher has its own origin
    - trust file format 3 (format 2 reads with origin = key)
  - `src/contract.la`, `src/nodes.la`: v2's effects decoded and checked (kinds, sizes); v1's effects are still ignored.
  - `src/tabs.la` (or a new `src/effects.la` if tabs.la nears 500 lines): after an update, each effect is answered as a message sent back to the guest through `update`, as a click would be. That's bounded: at most 8 effects per update, and answers that cause effects are themselves capped.
  - `src/render.la`: placeholders filled from the names in effect for that app (its override, else the global name, else empty).
  - `src/panel.la`: the Account page:
    - "Signed in as User 4f9a" (the first 4 hex digits of the fingerprint), or "Not signed in"
    - a Sign in automatically toggle; Sign out / Sign in
    - Start over as someone new (asks first)
    - Your name, used by every app (first and last fields), and In this app only (first and last, empty = the name for every app)
    - "Apps can show your name; they never see it." An unsigned app's Account page says it can't have an account.
  - `guest/rust/rapidkoi`: contract v2; `update` can return effects (`Effects::identity(msg)`, `Effects::sign(msg, challenge)`); the `FIRST_NAME`/`LAST_NAME` constants for text.
  - `guest/rust/counter`:
    - asks for its identity at init
    - shows "You are User 4f9a" (from the key it got)
    - shows "Hello, {FIRST_NAME}!" when the name box is empty
  - Docs: CLAUDE.md (contract v2, the tree), MEMORY.md (contract decision), DECISIONS.md (022 partly resolved, contract v2), the contract meeting doc (an appendix for v2), CHANGELOG.md.
- **Related:**
  - `docs/source/meetings/2026-10-17-ui-contract.md` (v1)
  - `src/contract.la`, `src/guest.la`, `src/tabs.la` (`step`), `src/render.la`, `src/panel.la`, `src/saving.la`, `src/store.la`
- **Lugha spec relied on:** `std.crypto`
  - `hkdf_sha256` (m120)
  - `ed25519_public`, `ed25519_sign`, `random_bytes`
- **kivuli API used:** `toggle` (a switch), `input`, the existing buttons.
- **Open decisions:** none. Contract v2 is decided here, and gets its DECISIONS.md entry and version bump.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Keys**
- **The app identity** for keys is (host, origin key): the first key trusted for that app, carried through signed rotations (`Identity.origin`). A separate publisher (a changed key, opened as a separate app) has its own origin, so it's a new person.
- **The master secret** is made on first use (when an app first asks for its identity) with `random_bytes(32)`. It's kept in `identity` (format 1, kind "rapidkoi identity"), 0600, written safely, never printed or shown.
- **Each app's seed** is `hkdf_sha256(master, [], CBOR array ["rapidkoi identity v1", host, origin, generation], 32)`. The public key is `ed25519_public(seed)`. Nothing per-app secret is stored: seeds are worked out when needed.
- **Sign-in signatures** sign CBOR `{kind: "rapidkoi sign-in", host, challenge}`, with the host Rapidkoi verified, never one the app supplies.
- **Only signed apps have keys** (verified, trusted or not confirmed, by their host and origin). Unsigned apps get `null`.
- **Auto sign-in, on by default:**
  - an app that asks gets its key; there's no prompt
  - with auto off, or signed out, it gets `null`
  - Sign in (Account page) clears `signed_out`
- **Start over as someone new** asks first ("Counter will see you as a new person; your old account there can't be used again from Rapidkoi"), then adds one to `generation`.

**Contract v2**
- **Versions:** `rapidkoi_version()` may return 1 or 2. A manifest's `contract` may be 1 or 2, and must match the module's.
- **v2 effects:**
  - an array of at most 8 maps; any other kind is an error that stops the app with a plain reason, like any contract error
  - `identity`: `{kind, msg}`
  - `sign`: `{kind, msg, challenge}`, where the challenge is bytes, 1–1024 long
  - `msg` is any value the app chooses (like a button's)
- **Answers** are sent back as `update(state, {msg, value})`: the public key, the signature, or `null`. They're sent after the screen has been drawn for the update that asked, in order. The guest's update calls are bounded by fuel, as all calls are.
- **Placeholders:**
  - in v2 trees, `{FIRST_NAME}` and `{LAST_NAME}` in text nodes' text and button labels are replaced when drawing
  - anything else in braces is left as it is
  - an empty name is replaced by nothing
  - v1 trees are never rewritten
- **Nothing the guest receives contains a filled-in name:** messages carry the app's own values, and text fields are never rewritten.

**The Account page and names**
- **The `people` file:** saved on every change, like pins. A broken file is set aside with a notice, and a newer one is left alone (as PRP-007).
- **Name fields** sit on the Account page: "Your name (every app)" and "In this app only". Each has first and last, up to 64 characters each, and is saved as it's typed.
- **Forget this app** also removes its `people` entry (its generation and overrides).

### Must NOT Do
- **Nothing that identifies the person goes to an app:**
  - no private key, master secret, seed, name, email or phone
  - no key for an unsigned app
- **No guest host functions:** effects go through `update`'s result, as the contract reserved.
- **No network for apps** (a later capability). No recovery words, sync or device unlock (later).
- **No placeholders** in text fields, URLs, or anything sent back to the guest.

## MODEL AND MESSAGES
- **Model:**
  - `people: People` (private): loaded at start, saved by `persist`
  - the identity store is reached through `store`
- **Messages:**
  - `SetName(Scope, Part, string)` (`Scope { Everywhere, ThisApp }`, `Part { First, Last }`)
  - `ToggleAutoSignIn`, `SignOut`, `SignIn`
  - `Ask(Confirm.StartOverIdentity)` and `StartOverIdentity`
- **Guest answers** reuse `Msg.Tab(id, render.Msg.Event(value, None))`'s path internally (a new `render.Msg.Answer(Value)`).

## ERROR HANDLING REQUIREMENTS
- **`IdentityError`:**
  - `Store(StoreError)`
  - `Crypto(string)`, with std's message, never an object
  - `Broken(why)`
- **A failure to make or read the master secret** answers the app with `null`, and the notice line says why: "Rapidkoi couldn't read your identity: …".
- **Effect errors** (unknown kind, a bad challenge, too many) stop the app with "This app's update result is broken (…)", as other contract errors do.
- **Nothing panics** on a guest's effects or on any file.

## SECURITY CONSIDERATIONS (the owner reviews before merge: authentication, secrets, a contract change)
- **The master secret:**
  - 32 random bytes from OpenSSL, 0600, written safely, never logged, shown or sent
  - lost with the device until recovery exists (accepted for now; noted on the Account page: "Kept on this computer only")
- **Per-app keys** come from HKDF over the master secret and the app's (host, origin, generation). They're unrelated across apps, and an app's public key reveals nothing about the master or other apps.
- **Signatures bind the verified host,** so a fake host can't use a real app's signature. The challenge is the app's; its size is capped.
- **Unsigned apps get nothing.** Not-confirmed (⚠) apps get keys for their own (host, origin), never a trusted publisher's.
- **Placeholders** are only filled while drawing. The guest never receives the result; text fields and messages are untouched. Guests can't measure layout (contract v1's rule, kept).
- **Effects are bounded:** 8 per update, and fuel per call.

## TESTS TO WRITE
- [ ] **identity:**
  - the master is made once (0600) and read back
  - the same app gives the same public key; another host, origin or generation gives another
  - a signature verifies (`ed25519_verify`) over the host-bound message
  - the challenge goes into it unchanged
  - a broken identity file is refused
- [ ] **people:**
  - round trip
  - defaults (auto on, generation 0)
  - a broken file is set aside; a newer one is left alone
- [ ] **verify:** `Identity.origin` is the first key, kept through a rotation; a separate publisher has its own; trust format 2 reads with origin = key.
- [ ] **contract v2:**
  - effects decode
  - an unknown kind, a challenge of 0 or 1025 bytes, and 9 effects are each errors
  - v1 effects are still ignored
  - the version is checked (1 and 2 open, 3 refused)
- [ ] **answers** (a v2 WAT fixture that asks for identity and a signature):
  - it receives the key, then the signature, in order
  - an unsigned page receives `null`; auto off or signed out receives `null`
  - Start over gives a different key
  - a rotation keeps the key
  - a separate publisher gets another key
- [ ] **placeholders:**
  - `Hello, {FIRST_NAME} {LAST_NAME}!` draws the name
  - an empty name draws "Hello,  !"
  - other braces stay as they are
  - a text field's value isn't replaced
  - a v1 tree is never replaced
  - the guest's next message carries no name
- [ ] **Account page** (`kivuli.testing`):
  - "Signed in as User xxxx"
  - the toggle, Sign out and Sign in send their messages, and the state follows
  - Start over asks, then changes the key
  - name fields save, the per-app override wins, and Forget removes the app's entry
- [ ] **Rust:** `cargo test` for the library's v2 encoding and the counter.

## ROLLBACK PLAN
- Branch: `prp-011-identity`, from main after PRP-010.
- v1 apps are untouched. Deleting `identity` and `people` from the data folder returns to no keys and no names.

## ACCEPTANCE CRITERIA
- [ ] The counter (v2), published and opened, shows "You are User xxxx". Reopened, it's the same; after Start over, a new one *(the owner)*
- [ ] With your first name set in the Account page, the counter shows "Hello, Brian!" while its own name box is empty *(the owner)*
- [ ] Every test above
- [ ] `lughac test test/` and `cargo test` pass
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated
- [ ] **The owner's security review before merge**

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `cargo test --manifest-path guest/rust/Cargo.toml`
- `lughac build src/main.la -o main`
- `stat -c %a ~/.local/share/rapidkoi/identity` shows 600
- For the owner:
  1. Rebuild the counter, publish it, and open it.
  2. Check that it shows "You are User xxxx".
  3. Set your name in the Account page; the greeting should change.
  4. Start over as someone new; the ID should change.
