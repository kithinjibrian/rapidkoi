**Status:** approved by the owner, 2026-10-18 (session 34); the security review comes before merge

## FEATURE: Signed apps, published locally: `rapidkoi.toml`, keygen, signed bundles, `publish` to a site, and verified opening

## OBJECTIVE
A developer describes their app in a `rapidkoi.toml`, makes a key with `./main keygen`, and publishes with `./main publish`. That writes a static `site/` folder:
- the public **key**
- a signed **index** of versions
- each version as a **signed `.rkb`**

`./main serve site/` serves it, and Rapidkoi opening `rapidkoi://localhost:8000` **verifies everything** before running: the key, the index's signature and sequence, the build's hash and signature, the host. It shows **✓ localhost:8000 · verified (development)**, refuses with no bypass when anything fails, and offers a separate app when the key changes without approval. This is the signing design (`docs/source/meetings/2026-10-18-signing-and-panel.md`, DECISION-009, MEMORY.md decision 14), end to end on this machine.

**Not yet:** remembering keys across restarts (needs `std.fs.rename` for a safe on-disk store: PRP-007) and real `https://` hosts (need background loading, DECISION-021). Both slot in later without format changes.

```
$ cat rapidkoi.toml
[app]
name = "Counter"
version = "0.2.0"
host = "counter.example"
module = "guest/rust/target/wasm32-unknown-unknown/release/counter.wasm"
icon = "guest/rust/counter/icon.png"
[publish]
key = "~/.config/rapidkoi/keys/counter.example.key"
site = "site"
$ ./main keygen          # once
$ ./main publish         # → site/.well-known/rapidkoi/{key, index, builds/<sha256>.rkb}
$ ./main serve site/     # then: ./main rapidkoi://localhost:8000
```

## CONTEXT

- Starting state: format 1 bundles (unsigned), `./main pack` with flags, `./main serve BUNDLE.rkb`, and local development addresses opened by `loader`.
- Ending state:

```
src/project.la — read rapidkoi.toml (std.toml): [app], [banners], [assets], [publish]; ProjectError
src/keys.la    — the secret key file (keygen, read); fingerprints; the "outside the project" rule
src/signing.la — format 2: the signed manifest (host, sha256 of the module and each asset), sign and verify a bundle; the public key file (key, rotations)
src/index.la   — the signed index: encode, sign, read, verify (host, sequence, versions, withdrawn); add, prune, withdraw
src/publish.la — `./main publish` and `./main keygen` (the commands)
src/verify.la  — opening a signed site: fetch key, index, build; every check; trust decisions; VerifyError
src/bundle.la, src/rules.la — read format 2 as well as 1 (format 1 unchanged)
src/pack.la    — `./main pack` with no module reads rapidkoi.toml (flags keep working); signs when [app].host and [publish].key are set
src/serve.la   — `./main serve site/` serves a published site (key, index, builds/<64 hex>.rkb only)
src/loader.la, src/tabs.la, src/window.la — open signed sites; the in-session trust (host → key, highest sequence)
src/frame.la, src/panel.la — the ✓ / ⚠ markers; About shows the publisher's fingerprint, "verified (development)", the build hash
src/main.la    — keygen and publish subcommands
test/project.la, test/keys.la, test/signing.la, test/index.la, test/publish.la, test/verify.la (new); others as touched
CLAUDE.md (COMMANDS, tree, rule 3 unchanged), docs/DESIGN.md (markers), CHANGELOG.md, an example rapidkoi.toml for the counter
```

- Lugha spec relied on: `std.crypto` (`sha256`, `Sha256`, `ed25519_keypair`, `ed25519_public`, `ed25519_sign`, `ed25519_verify`, `random_bytes`, `equal`, `CryptoError`), `std.toml` (`parse_with`, `get`, `as_*`, `TomlError.message`), `std.env` (`get("HOME")`, `current_dir`), `std.fs`, `std.http`.
- Open decisions: none block. DECISION-009's later items (expiry, revocation, DNS) stay later.

## FORMATS (from the design)

**Sign bytes, not shapes:** everything signed is stored as its exact CBOR bytes plus a signature over them. The bytes are checked first, then decoded.

- **Secret key file** (developer's machine only; CBOR):
  - `{ format: 1, kind: "rapidkoi secret key", host, secret: 32 bytes, public: 32 bytes }`
  - `keygen` refuses to overwrite one, and refuses a path inside the project.
- **Public key file** `site/.well-known/rapidkoi/key` (CBOR):
  - `{ format: 1, host, key: 32 bytes, rotations: [ { old, new, signature } ] }`, where each rotation is signed by `old` over the CBOR of `{host, old, new}`.
  - The trail must lead from a trusted key to `key`.
- **Signed index** `site/.well-known/rapidkoi/index` (CBOR):
  - `{ format: 1, body: bytes, signature: 64 bytes }`
  - `body` is the CBOR of `{ host, sequence, current, versions: [ { version, sha256: 32 bytes, size, withdrawn?: text } ] }`.
- **Signed bundle, format 2** `site/.well-known/rapidkoi/builds/<sha256 hex>.rkb` (CBOR):
  - `{ format: 2, manifest: bytes, signature: 64 bytes, key: 32 bytes, module, assets }`
  - The manifest (format 1's fields plus `host`, `module_sha256`, and `assets` as name → sha256) is signed. The file's own sha256 is its name and its id in the index.
- **Fingerprint:** the first 8 bytes of sha256(public key), as 4 groups of 4 hex digits: `4f9a-2c81-e07b-19d3`.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**`rapidkoi.toml` (`project.la`)**
- **`[app]`:** `name`, `version`, `module` (required); `description`, `host`, `state_version` (default 1), `icon`.
- **`[banners]`:** page → file. **`[assets]`:** name → file.
- **`[publish]`:** `key` (path; `~/` expands from `$HOME`), `site` (default `site`), `keep` (default 3).
- Paths are relative to the toml's folder. Unknown keys are refused with a clear message (typos shouldn't pass silently).
- `std.toml.parse_with` with limits of 64 KB, depth 8 and 10,000 values. Errors give std.toml's own message, prefixed `rapidkoi.toml: `.

**Keys (`keys.la`, `publish.la`)**
- `./main keygen`:
  - reads rapidkoi.toml's `host` and `[publish].key`
  - refuses if the path is inside the project folder (after `~` expansion, compared with the project's absolute path) or already exists
  - makes the folder, writes the secret key file
  - prints `wrote <path> · fingerprint 4f9a-…` and **"Keep it secret. Run: chmod 600 <path>"** (std has no set_mode yet; once it does, keygen sets 0600 itself)
- `read_secret(path, host)` refuses a file whose host isn't this app's.

**Signed bundles (`signing.la`, `bundle.la`, `pack.la`)**
- `./main pack` with no module argument reads `./rapidkoi.toml`.
  - With `host` and `[publish].key` set, it writes a **format 2** signed bundle (`name-version.rkb`). Otherwise it writes format 1, as today.
  - The flag form keeps working unchanged (format 1).
- `bundle.read` accepts format 2:
  - it checks the signature over the manifest bytes with the bundle's own `key` field, then decodes the manifest
  - it checks every hash: the module, and each asset, both ways (no asset missing from the manifest, none extra)
  - then it applies every format 1 rule
- The result records the host, the public key and the file's sha256. Whether that key is *trusted* is decided elsewhere (verify).

**Publishing (`index.la`, `publish.la`)**
- `./main publish`:
  - packs signed (as above)
  - reads `site/.well-known/rapidkoi/index` if there is one, and **verifies it with the key**: it never builds on a tampered or foreign index
  - refuses a version that exists with different bytes (the same bytes again is a no-op)
  - adds the version, makes it current, bumps `sequence`, and keeps the newest `keep` versions (never pruning `current` or a withdrawn entry)
  - writes `builds/<hex>.rkb`, the signed index and the key file, and deletes the pruned builds' files
  - prints a summary
- `./main publish --withdraw VERSION --reason TEXT` marks a version withdrawn, bumps the sequence and re-signs.
- It never writes the secret key, or anything outside `site/`.

**Serving a site (`serve.la`)**
- `./main serve DIR` (a directory) serves exactly:
  - `/.well-known/rapidkoi/key` and `/.well-known/rapidkoi/index`
  - `/.well-known/rapidkoi/builds/<64 lowercase hex>.rkb`
  - 404 for anything else
- It never maps any other path to a file. 127.0.0.1 only. `./main serve BUNDLE.rkb` stays as today.

**Verified opening (`verify.la`, `loader.la`, `window.la`)**
- `rapidkoi://localhost:PORT` first asks for the key file. A 404 means an unsigned development server, handled as today (format 1 at `app.rkb`). Otherwise:
  1. **Key:** the key file's host must equal the address's host (`localhost:PORT` counts as the host for development).
     - No trusted key for this host in this session → trust it (first use).
     - A trusted key that differs → follow the rotations. A signed trail to the new key → trust the new one silently.
     - No signed trail → **an unapproved change** (below).
  2. **Index:** its signature checks with the trusted key, and its host matches. Its `sequence` is ≥ the highest seen for this host in this session (a lower one is refused: "an older index than one already seen"). `current` is in `versions`. A withdrawn current version is refused fresh: "withdrawn by its publisher: <reason>".
  3. **Build:** fetch `builds/<hex>.rkb`.
     - Its size matches the index.
     - Its sha256 matches the index.
     - `bundle.read` (format 2) passes.
     - Its `key` equals the trusted key, and its manifest's host equals the host.
  4. **Then it runs** as a page, with `verified: Some(Trust { host, fingerprint, build_hash })`.
- **Any failure** shows the error page: "This app couldn't be verified, so Rapidkoi didn't run it: <reason>". **No run-anyway button**, only ⟳ (try again).
- **An unapproved key change** shows a page in that tab: "localhost:8000 is now signed by a different key. This can mean the site changed owners."
  - **Keep using my version** keeps the open page if there is one; otherwise it closes.
  - **Open as a separate app** trusts the new key as a separate identity for this session; the page is marked "new publisher".
- **A signed `.rkb` file opened by path** passes `bundle.read` (its signature checks with its own key).
  - If that key is trusted for its host in this session: ✓ host.
  - Otherwise: **⚠ host · not confirmed**. It runs, as a stranger.
- **Trust lives in the Model for this session:** host → (key, highest sequence). PRP-007 keeps it on disk.

**Showing it (`frame.la`, `panel.la`)**
- **Markers:**
  - `✓ localhost:8000 · verified (development)`
  - `⚠ counter.example · not confirmed`
  - `✓ counter.example` (a signed file whose key is trusted)
  - `🛠 …` and `📦 …` as today
- **About:** Publisher "counter.example · key 4f9a-2c81-e07b-19d3"; "Verified (development) this session" or "Not confirmed"; Build "sha256 4f9a2c81…" (16 hex characters, full on hover); Bundle format 2.

### Must NOT Do
- **No run-anyway** for a failed check. No trusting a key that isn't reached by first use or a signed trail.
- **No secret key in `site/`,** in a bundle or in logs. No secret key path inside the project.
- No remote hosts or https (DECISION-021), no on-disk trust store (PRP-007), no expiry, revocation or DNS (later).
- No `./main rotate` command yet. Rotations are verified (tests build them); the command comes when key handling gets its own small PRP.
- No change to UI contract v1. Format 1 bundles behave as before.

## ERROR HANDLING REQUIREMENTS
- **`ProjectError`** (a missing file or field, a bad value, an unknown key, TOML errors with their line and column).
- **`KeyError`** (an existing file, a path inside the project, a wrong host, a malformed file, crypto failures).
- **`SignError` / `BundleError` additions:** a bad signature, a hash mismatch, a missing or extra asset hash, a host mismatch.
- **`IndexError`:** a bad signature, a wrong host, a lower sequence, a current version missing, a version reused with different bytes.
- **`VerifyError`:** each step's failure, with a plain sentence.
- All are `Result`s with plain messages. **Nothing panics** on any file, response or toml. CryptoError's message is shown, never an OpenSSL object.

## SECURITY CONSIDERATIONS (the owner reviews before merge: crypto)
- Every check fails closed, in the order key → index → build, before anything runs. Signatures are checked over exact bytes before decoding.
- Hashes are compared with `std.crypto.equal` (constant time).
- First-use trust is per session in this PRP. That's acceptable for local development, and **not** for remote hosts, which stay disabled.
- Secret keys: a path outside the project, never written to `site/`, a chmod reminder until `std.fs.set_mode` exists, and never printed. `random_bytes` comes only from `std.crypto` (keygen uses `ed25519_keypair`).
- The site server serves only well-formed key, index and build paths.
- Index sequence: no going back within a session.

## TESTS TO WRITE
- [ ] **project:** a full toml; defaults; `~` expansion; relative paths; each missing field; an unknown key; a TOML syntax error with its line; the limits.
- [ ] **keys:** keygen writes, refuses to overwrite, refuses inside the project, refuses a wrong host on read; fingerprint format.
- [ ] **signing:**
  - sign then read a format 2 bundle round trip
  - a flipped byte in the manifest, signature, module or an asset is refused, each with its own reason
  - an asset added or removed is refused
  - the wrong key field; a host mismatch
  - format 1 still reads
- [ ] **index:**
  - encode, sign, read, verify round trip
  - tampered body; a wrong key; a wrong host
  - a lower sequence refused against a seen one
  - add, re-add the same bytes (a no-op), reuse a version with different bytes (refused)
  - prune keeps current and withdrawn; withdraw
- [ ] **publish** (test/tmp sites):
  - publish twice (two versions, sequence 2, both builds on disk)
  - keep = 1 prunes the old build file
  - withdraw
  - a tampered existing index is refused
  - the secret key never appears in site/
- [ ] **serve:** a site directory serves key, index and builds by hex; refuses other paths, uppercase hex and `..`.
- [ ] **verify** (live sockets, a spawned server on a site):
  - opens, marked verified
  - a tampered build, a tampered index, a swapped key without a trail (the unapproved page), and a key with a signed rotation (accepted)
  - a lower sequence after a higher one, a withdrawn current, and a build whose host differs
  - each with its message and no run
- [ ] **files:** a signed .rkb opened by path is ⚠ not confirmed, or ✓ once its key is trusted this session.
- [ ] **views:** the markers; About's lines.

## ROLLBACK PLAN
- Branch: `prp-006-signing`. Format 1 is untouched; nothing on disk outside test/tmp and the developer's own site/ and key.

## ACCEPTANCE CRITERIA
- [ ] keygen, then publish, then serve site/, then opening `rapidkoi://localhost:8000` runs the counter, marked ✓ verified (development) *(the owner checks on screen)*
- [ ] Editing one byte of the build in site/ makes Rapidkoi refuse it with a reason and no bypass *(on screen)*
- [ ] Every check has a test that fails it; `lughac test test/` passes; `fmt --check` is clean; the build succeeds
- [ ] The owner's security review before merge
- [ ] Every `public` item has a doc comment; no file over 500 lines; CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner: the four commands above, then tamper with `site/.well-known/rapidkoi/builds/*.rkb` (one byte) and ⟳.
