## FEATURE: Bundles: pack an app into one `.rkb` file, and open it by its path

## OBJECTIVE
A developer packs their app (the module, its name, version, icon and assets) into one file with `./main pack …`. Anyone opens it by typing its path in the address box, or with `./main app.rkb`, exactly as `.wasm` files open today. The tab and the info panel show the app's real name, version and description. Bundle format 1 is the local, unsigned first step of the packaging design (`docs/source/meetings/2026-10-18-packaging.md`). Hashes, signatures and addresses come with format 2.

```
$ ./main pack guest/rust/target/wasm32-unknown-unknown/release/counter.wasm \
      --name Counter --version 0.1.0 --description "Counts, and says hello" \
      --icon guest/rust/counter/icon.png -o counter.rkb
packed counter.rkb: Counter 0.1.0, 49 KB module, 1 asset
$ ./main counter.rkb
```

## CONTEXT

- Starting state: PRP-002 on `main`. `tabs.open_path` reads a `.wasm` and compiles it; `AppPage.name` is the file name.
- Ending state:

```
src/bundle.la      — pure: format 1 (Manifest, Bundle), read(bytes) and write(bundle) with every check; BundleError
src/pack.la        — the `pack` command: flags → files read → bundle.write → the .rkb written; messages, exit codes
src/main.la        — `./main pack …` runs pack.run and exits (no window); anything else starts the window
src/tabs.la        — open_path: `.rkb` → bundle.read → compile its module; AppPage gains `manifest: Option<Manifest>`
src/frame.la, src/panel.la — the manifest's name, version, description, and `📦 file · local bundle`
src/limits.la      — MAX_BUNDLE, MAX_ASSET, MAX_ASSETS, field lengths
test/bundle.la, test/pack.la, test/tabs.la (+rkb), test/counter.la (+rkb, skipped until built)
guest/rust/counter/icon.png — the demo's icon (a 128 × 128 PNG)
CLAUDE.md          — COMMANDS (pack), FILE ORGANIZATION, KNOWN ISSUES
```

- Related source files: `docs/source/meetings/2026-10-18-packaging.md`, DECISION-009, MEMORY.md decision 13.
- Lugha spec relied on: `args()`, `read_bytes`, `write_bytes`, `exit` (or `main`'s return code), `std.fs.is_file`, `parse_i64`, `s.split`, `s.starts_with`.
- Open decisions: none block. DECISION-009 (signing) is deliberately not touched.

## BUNDLE FORMAT 1

One CBOR map, decoded with `cbor.decode` under bundle limits:

```
{ "format": 1,
  "manifest": {
      "contract": 1,                    // the UI contract the module speaks
      "name": "Counter",                // 1–64 bytes, no control characters
      "version": "0.1.0",               // semver: MAJOR.MINOR.PATCH, digits only
      "description": "Counts…",         // optional, ≤ 280 bytes
      "state_version": 1,               // ≥ 1, bumped when the state's shape changes
      "icon": "icon.png",               // optional: an asset name
      "banners": { "home": "banner/home.png" },   // optional: page → asset name
      "permissions": [],                // must be empty in format 1
      "prerendered": [] },              // must be empty in format 1 (room for D)
  "module": <bytes>,                    // the .wasm
  "assets": { "icon.png": <bytes>, … } }
```

- **Asset names:** 1–128 bytes of `a–z 0–9 . _ - /`. They don't start or end with `/`, and contain no `..` and no `//`. Names are only ever keys, never file paths.
- **Asset types:** PNG only in format 1, checked by its 8-byte signature. Anything else is refused.
- **References:** `icon` and every banner name an asset that exists. Assets nothing refers to are allowed (apps may use them later).
- **Unknown keys** are ignored at both levels, so later formats can add fields. An unknown `format` number is refused: "This bundle needs a newer Rapidkoi".
- **Limits:** the whole file ≤ `MAX_BUNDLE` (32 MB); the module ≤ the existing `MAX_MODULE`; each asset ≤ `MAX_ASSET` (4 MB); at most `MAX_ASSETS` (256) assets. `cbor.Limits`: depth 8, items 4,096, strings `MAX_ASSET`, bytes `MAX_BUNDLE`.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**`bundle.la` (pure)**
- `Manifest { contract, name, version, description: Option, state_version, icon: Option, banners: (string, string)[] }`. `Bundle { manifest, module: u8[], assets: (string, u8[])[] }`.
- `read(bytes) → Result<Bundle, BundleError>` applies every rule above.
- `write(bundle) → Result<u8[], BundleError>` applies the **same checks** before encoding, so `pack` can never produce a bundle `read` refuses.
- `asset(bundle, name) → Option<u8[]>`.

**`pack.la` (the command)**
- Usage: `./main pack MODULE --name N --version V [--description D] [--state-version N] [--icon FILE] [--banner PAGE=FILE]… [--asset NAME=FILE]… -o OUT.rkb`.
  - `--icon FILE` adds the asset `icon.png` and points `icon` at it. `--banner PAGE=FILE` adds `banner/PAGE.png`.
  - `--state-version` defaults to 1.
- It reads each file (refusing missing ones), builds the bundle, `write`s it, writes OUT, and prints one summary line.
- Any error prints `rapidkoi pack: <what>` to stderr and exits 1: a bad flag, a missing value, an unreadable file, a check failing (naming the field). Success exits 0.
- It never overwrites OUT unless `--force` is given. It never runs the module. No window opens.

**Opening (`tabs.la`)**
- `open_path` on `*.rkb`: `read_bytes` (≤ `MAX_BUNDLE`) → `bundle.read` → compile `module` → the page as today.
  - The page's `name` is the manifest's name, and `manifest` is kept on the page.
  - A `contract` other than 1 gives "This app needs a newer version of Rapidkoi".
  - Any `BundleError` gives the error page with a plain reason, with no Restart.
- `.wasm` keeps working exactly as before (`manifest: None`).
- The state's version: `LeftOff` keeps the page's `state_version` with its state. This PRP only records it. `migrate`, and starting over on a mismatch, come with PRP-004, when state outlives a session, the first time a mismatch can happen.

**Showing it**
- **Tab tooltip and window title:** the manifest's name (and screen title, as now).
- **Info panel:**
  - name
  - version (`Version 0.1.0`)
  - description
  - `📦 {file} · local bundle`
  - `Contract v1 · state v{n}`
  - permissions, as now
- **Address on an app page:** `📦 counter.rkb · local bundle` (`📄 … · local file` stays for `.wasm`).
- **The icon** isn't drawn yet: kivuli shows images from files only, and Rapidkoi won't write untrusted bytes to disk to get around that. It's a placeholder (DECISION-018): the initial letter, as now, until kivuli shows an image from bytes (DECISION-020, a new item). The icon is read, checked and kept on the page, ready.

**The demo**
- `guest/rust/counter/icon.png`: a 128 × 128 PNG made once, committed.
- CLAUDE.md COMMANDS gains the pack command for the counter.

### Must NOT Do
- No hashes, signatures, keys, indexes, addresses or network (format 2, DECISION-009).
- No `migrate` calls, state persistence or snapshots (PRP-004).
- No writing assets to disk. Pack writes only its OUT file.
- No image decoding by Rapidkoi. PNG is checked by its signature only; GTK decodes later, once kivuli can show bytes.
- No permissions or prerendered screens beyond checking that they're empty.
- No change to UI contract v1.

## ERROR HANDLING REQUIREMENTS
- `BundleError`, each with a plain message naming the field:
  - `Decode(cbor.DecodeError)`, `TooLarge(what, len)`, `Format(n)`
  - `Missing(field)`, `Wrong(field, problem)`, `BadName(name)`, `NotPng(name)`, `MissingAsset(name)`
  - `TooManyAssets(n)`, `NotEmpty(field)`
- Opening maps each to the error page. Pack maps each to `rapidkoi pack: …` and exit 1.
- Nothing panics on a bundle's bytes or on pack's input.

## SECURITY CONSIDERATIONS
- A `.rkb` is hostile input: size first, then `cbor.decode` with limits, then every field checked before anything is used. The module still runs in its sandbox with PRP-001's limits.
- Asset names are keys, never paths. Nothing is written from a bundle.
- PNG signature check only. No decoding until kivuli does it (through GTK) from bytes.
- No crypto in this PRP.
- Pack reads only the files named on its command line, and writes only OUT, refusing to overwrite without `--force`.

## TESTS TO WRITE
`test/bundle.la` (pure):
- [ ] A round trip: write → read gives the same manifest, module and assets.
- [ ] Each rule:
  - format 2 refused; missing manifest or module
  - contract not an integer
  - name empty, too long, or containing a control character
  - bad semver (`1.0`, `1.0.0-beta`, `a.b.c`)
  - state_version 0
  - description too long
  - icon naming a missing asset; a bad asset name (each: `../x`, `/x`, `x//y`, `X.png`, 129 bytes)
  - a non-PNG asset; too many assets; an asset too large
  - non-empty `permissions` and `prerendered`
  - unknown keys ignored
- [ ] Hostile: truncated bytes, not a map, a huge claimed length, the whole file over `MAX_BUNDLE`.
- [ ] `write` refuses what `read` refuses (a bad name, a non-PNG asset).

`test/pack.la`:
- [ ] Packing a fixture module with every flag, then `bundle.read` of the output, gives what was asked.
- [ ] Errors: no module, no `--name`, a bad `--version`, an unknown flag, a missing icon file, a non-PNG icon, an existing OUT without `--force`. Each is a message and exit 1. Tests write under `test/tmp/` (gitignored) and clean up.

`test/tabs.la`:
- [ ] Opening a `.rkb` gives the manifest's name and keeps the manifest; a bad bundle gives the error page with no Restart; a `contract: 2` bundle says "needs a newer version"; `.wasm` is unchanged.

Views:
- [ ] The panel shows version, description, `📦 … · local bundle` and `state v1`. The address shows the bundle marker.

`test/counter.la`:
- [ ] Packing the built counter, then opening the `.rkb`, behaves as the `.wasm` did (skipped until built).

## ROLLBACK PLAN
- Branch: `prp-003-bundles`. `main` stays at PRP-002 until merge. No data, no migrations.

## ACCEPTANCE CRITERIA
- [ ] `./main pack …` (the command above) produces `counter.rkb`; `./main counter.rkb` opens it, the panel showing "Counter", "Version 0.1.0" and the description *(the owner checks on screen)*
- [ ] Typing `counter.rkb` in the address box opens it too *(on screen)*
- [ ] Every bundle rule has a test; nothing panics on hostile bundles
- [ ] `lughac test test/` passes; `lughac fmt --check src test` is clean; `lughac build src/main.la -o main` succeeds
- [ ] Every `public` item has a doc comment; no file over 300 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- `cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown`
- For the owner: the pack command above, then `./main counter.rkb`; open ⓘ.
