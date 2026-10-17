# Meeting — 2026-10-17/18 — How apps are packaged

**Attendees:** the project owner, Claude (chat, after session 18)
**Type:** planning

---

## What Was Discussed

How apps are packaged and delivered: the "apps from addresses" half of links (DECISION-009). The owner asked to define the problem before solutions, then to think about the person's and the developer's experience.

**The problem.** A Rapidkoi address has to turn into a running app on someone's device, and it has to be:
- fast
- genuinely the publisher's
- the same code for everyone on that version
- still working when the network or the publisher doesn't

**Who needs what:**
- **The person:** fast first and later opens, authenticity, offline once kept, no surprise updates, rollback.
- **The developer:** publishing without permission, cheap static hosting anywhere, a simple build → sign → publish flow, easy local testing.
- **Rapidkoi:** bytes it can check before trusting them, a stable id per bundle, permissions known before running.

**Constraints:** content-addressed builds; a verifiable publisher (whoever controls the domain); hostile input; no gatekeeper; offline by default.

**Out of scope:** payments, a store or directory, reviews.

**Code loading.** We compared four shapes:
- **A:** one module per app
- **B:** a module per screen
- **C:** htmx-style, the app running on a server
- **D:** A plus prerendered first screens

Scored against the constraints, C breaks offline, static hosting and privacy, and B is complex and larger in total. The owner chose A, with room to grow into D.

**Signing.** We discussed where the publisher's key lives (DNS, a well-known file, a public log), key rotation and loss, and the crypto library. The owner found it complex and asked to first bundle apps and open them by file path.

## Decisions Made

- **Code: one module per app (A).** The manifest keeps a `prerendered` slot, empty for now, so D (prerendered first screens, a tap queued until the module runs) can come later without a format change.
- **Vocabulary:**
  - an **app** is everything at one **host**: one app per host, like a web origin; a path is a route inside it
  - a **build** is one exact set of bytes, its id the hash of its manifest
  - a **version** is the publisher's semver name for one build, never reused
  - an **index** is a signed file at the host: versions → builds, and which one is current
- **Old versions:**
  - the publisher keeps only what they want; the build tool defaults to the last few
  - the device keeps the previous build, so rollback works offline
  - pinned links to deleted builds try the cache and mirrors, then offer the current version
  - the index can **withdraw** a version, with a reason
- **Everything Rapidkoi holds for an app is stamped with the version that wrote it:** state, snapshots, permission grants, and later the database and history.
  - The manifest has a `state_version`, bumped only when the state's shape changes.
  - An optional app export `migrate(state, from_version)` handles a change.
  - **With no `migrate`, the app starts over**, its old state kept aside and recoverable through rollback, with a one-line notice.
  - State is snapshotted before every update.
- **Hosting (for later):** static files under `/.well-known/rapidkoi/`, with `index` (signed, from the host only) and `pieces/<sha256>` (immutable, cacheable forever, mirrorable).
- **Now (PRP-003):** a single-file bundle, `.rkb`, opened by typing its path. A CBOR map with a format number, the manifest, the module and assets by name. The developer packs it with `./main pack …` and command-line flags (option B). A `rapidkoi.toml` comes later, with a `lugha-toml` package.

## Constraints Introduced

- Asset images are untrusted: a few plain formats only (PNG first), size caps, checked before use.

## Open Questions Raised

- Signing: where the key lives (DNS or a well-known file), rotation, loss and domain transfer, and the crypto library (libsodium bindings proposed as `lugha-sodium`). Deferred: needs the owner's review (security rule).
- Lazy, hash-addressed pieces and format 2 (hashes, signatures) arrive with addresses.

## Action Items

- [x] Claude — record this; write PRP-003 (bundles)
- [ ] Owner — review PRP-003

## Impact on the Project

DECISION-009 is partly settled: the vocabulary, one app per host, versions and withdrawal, state versioning, the hosting layout, and bundle format 1 for local files. Signing stays deferred. The "remembering" work (history, search, banners, art) becomes PRP-004.
