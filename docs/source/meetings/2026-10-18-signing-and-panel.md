# Meeting — 2026-10-18 — Signing, permissions, and the info panel

**Attendees:** the project owner, Claude (chat, after session 25)
**Type:** planning

The owner's panel sketch: `2026-10-17-ui/sketch-3-info-panel.png`.

---

## What Was Discussed

With Lugha's TLS landed (`std.tls` over OpenSSL 3, `https://` in `std.http`, milestone 112), the owner chose to finish the signing design parked in the packaging meeting. Then: how apps get permissions, and a redesign of the info panel (the owner: "it looks ugly right now").

## Decisions Made

**Signing (DECISION-009, design settled):**
1. **Where the key lives:**
   - The publisher's public key is a file at `https://HOST/.well-known/rapidkoi/key`, fetched over HTTPS the first time, then **remembered per host** (trust on first use).
   - Shared links will carry the key's **fingerprint** (the first 16 characters of its SHA-256, e.g. `mx7r-4f9a-2c81-e07b`), so a first open from a friend's link is checked against the key the friend trusts. The link syntax comes with the links milestone.
   - DNS stays an optional extra for later; a public log is out (central infrastructure).
2. **What's signed:**
   - Each build's **manifest** ("acme made this exact build"): it lists a hash for the module and every asset, so one signature covers the whole `.rkb`, checkable anywhere and offline.
   - The **index** ("acme's list now"): versions, current, withdrawn ones with a reason, and a **sequence number that only goes up**. Rapidkoi refuses a lower one, so there's no rollback to an old index.
   - Index **expiry** (against freezes) waits for a clock in std.
3. **What's on the server:** only three kinds of static files: `key`, `index`, and `builds/<sha256>.rkb` (one signed `.rkb` per version). The owner asked "why not just get counter.rkb and unpack it"; agreed, there are no loose pieces.
   - First open: the key (once), the index, the `.rkb` (hash checked against the index, signature against the key), then run.
   - Later opens run from the cache and fetch the index in the background.
4. **Key changes:**
   - A **rotation signed by the old key** is accepted silently.
   - An **unapproved change** (a lost key, a new domain owner, an attack) keeps the person's version and data, and offers the new one as a **separate app**. An app's identity is its **host and key**, so a domain buyer never inherits users' data.
   - **Revocation** of a stolen key comes later.
5. **Formats:**
   - **Sign bytes, not shapes:** a signature is checked over exact bytes before anything inside is read.
   - The key file: `{format, host, key, rotations: [{old, new, signature}]}`.
   - A signed `.rkb` (format 2): `{format: 2, manifest: bytes (with host and hashes), signature, key, module, assets}`.
   - The index's full hash pins a build. Format 1 bundles stay unsigned (local or development, never ✓).
   - **Ed25519** and **SHA-256** via OpenSSL 3, as a new **`std.crypto`** in Lugha (the owner chose OpenSSL over libsodium: already linked by `std.tls`, no new dependency). Prompt given to the owner.
6. **Developer tools:**
   - A **`rapidkoi.toml`** in the app's project describes the app (`[app]`, `[banners]`, `[assets]`, `[publish]`) instead of long flags, read with a new **`std.toml`** in Lugha (the owner's choice; prompt given).
   - Commands: `./main keygen`, `./main pack`, `./main publish` (into a `site/` folder), `./main serve site/` (verified locally as "development"), and `./main rotate`.
   - The secret key's path must be outside the project; `publish` never writes a secret into `site/`.
7. **What the person sees:**
   - The bar's marker: ✓ host, ✓ host · offline, ⚠ host · not confirmed (a signed file whose key isn't confirmed yet), 🛠 development, 📦 / 📄 unsigned.
   - A **failed check refuses, with no "run anyway"**.
   - Withdrawn versions warn; an unapproved key change gets a full-page choice.
   - **Updates install on the next open, with Undo** (a rollback).

**Permissions at the point of use (the owner's direction):**
- Apps reach devices and files only through Rapidkoi's API.
- When an app asks, Rapidkoi checks its permission and **asks the person then**, on its own screen: Allow once / Always allow / Don't allow. A file picker *is* the consent, for that file only.
- No permission screen at first open, and updates never ask.
- The manifest still **declares what an app may ask for** (a ceiling).
- **Network access is declaration-only:** named servers, no prompt, listed in the panel.
- Grants are per app (host and key), stamped with the build.
- The info panel toggles each one.

**The bar:** **⟳ Reload** (keeps the app's state; fetches a new build from a development server) and **☆ Pin** (in the address, filled when pinned) move to the top bar.

**The info panel** (the owner's sketch, with names agreed):
- A header: icon, name, version, the host with its marker (tap it for the publisher, key and build).
- The description.
- **Your settings:** Account · Notifications · Access.
- **On this device:** Where you left off · Data.
- **Forget this app** (removes everything it stored, with a confirm).
- Each row shows a summary and opens its own page in the panel, with a back arrow.
- **Account** is the person's identity with the app:
  - a per-app user ID (apps can't link the same person across apps)
  - a name the person chooses per app
  - sign-out
- **Start over** (clears state) lives on the "Where you left off" page, with a confirm.

## Open Questions Raised

- Accounts and sign-in (passkeys, per-app IDs): authentication, so a design talk of its own and the owner's security review (DECISION-022).
- Small std additions: `fs.set_mode` (secret key files), `fs.rename` (safe saves), a stdout `flush`, a clock (history times, index expiry).

## Action Items

- [x] Claude — record this; write PRP-005 (the info panel and bar actions)
- [ ] Owner — run the lugha sessions for `std.crypto` and `std.toml`

## Impact on the Project

Signing is designed and waits on `std.crypto`, `std.toml` and background loading (DECISION-021). The panel and bar can be built now (PRP-005), with Account, Notifications, Access and Data as honest placeholders until their features exist. Remembering becomes PRP-006.
