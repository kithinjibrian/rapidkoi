## FEATURE: Remembering across restarts: trusted keys, pins and where you left off, saved safely in Rapidkoi's data folder

## OBJECTIVE
Close Rapidkoi and open it again, and it still knows:
- **the sites it trusts:** a signed site opened yesterday is checked against yesterday's key and sequence, and a swapped key still asks first
- **your pins**
- **where you left off:** Continue reopens that app with its state

Each change is saved as it happens, in three small files that a crash can't half-write. A file that can't be read is set aside with a notice, never lost or overwritten.

Designed in chat, session 35: `docs/source/meetings/2026-10-18-remembering.md` (topics 1–4, all settled).

```
~/.local/share/rapidkoi/        (or $XDG_DATA_HOME/rapidkoi/; 0700)
├── trust       host → key, highest sequence, separate?     (0600)
├── pins        name, path, icon                            (0600)
└── left-off    name, path, title, state, state version     (0600)
```

## CONTEXT

- **Starting state:** everything is in memory.
  - `window.la`: the Model's `pinned`, `left_off` and `ring`
  - `verify.la`: `Keyring` (a `Ref<Known[]>`)
  - `tabs.la`: `LeftOff` holds the compiled program, so Continue only works in the same session
- **Ending state:**
  - `src/store.la` (new): the data folder, safe writes, bounded reads, setting a broken file aside, `StoreError`
  - `src/remembered.la` (new): encoding and decoding the three files with their format numbers; `Recalled<T>`
  - `src/verify.la`: the keyring's entries out and in, plus a change count so saves know when trust moved
  - `src/tabs.la`: a `LeftOff` from disk, which has no program, reopens by loading its file or address again, with its saved state
  - `src/window.la`: the Model's `store: Option<Store>` (`None` in tests, so nothing touches the disk)
  - `src/shell.la`: `boot` loads the three files, with notices; `update` saves what changed, in one place, after each message
  - test files for the new modules; CLAUDE.md (rule 3, the tree); CHANGELOG.md
- **Related code:** `src/keys.la` `keygen` (the same safe-write steps, PRP-006 follow-up), `src/navigate.la` (Continue and Reload), `src/rules.la` `check_role` (icon checks)
- **Related source:** `docs/source/meetings/2026-10-18-remembering.md`; MEMORY.md decisions 14 (versioned state) and 16 (signing)
- **Lugha spec relied on** (all confirmed in `lughac spec`):
  - `std.fs`: `rename` and `set_mode` (m115), `sync` (m116), `make_dirs`, `exists`, `is_file`
  - `std.env`: `get`
  - `read_bytes`, `write_bytes`
  - `Ref`
- **kivuli API used:** none new. Notices use the existing notice line under the bar.
- **Open decisions:** none. DECISION-023 (reading a mode back) stays deferred; it doesn't block.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**The folder (`store.la`)**
- `$XDG_DATA_HOME/rapidkoi` when that's set and absolute (the XDG rule; a relative one is ignored); otherwise `$HOME/.local/share/rapidkoi`.
- Made on first save with `make_dirs`, then `set_mode` 0700.
- No `$HOME` and no `XDG_DATA_HOME`: nothing is saved this session, with a notice, and Rapidkoi works on in memory.

**Safe writes**, as keygen does:
1. write an empty `NAME.tmp`
2. `set_mode` it 0600
3. write the bytes into it
4. `fs.sync` it
5. `fs.rename` it to `NAME`
6. `fs.sync` the folder

A failure at any step leaves the old file as it was, and `update` shows a notice: "Rapidkoi couldn't save your pins: <why>".

**Reading:**
- Each file is read with its own size cap:
  - trust: 1 MB, at most 10,000 identities
  - pins: 32 MB, at most 256 pins
  - left-off: `MAX_STATE` + 1 MB
- It's decoded with CBOR limits and checked field by field: hostile or damaged bytes are a `Broken`, never a panic.

**Formats (`remembered.la`)**
- Each file is a CBOR map: `{format: 1, kind: "rapidkoi trust" | "rapidkoi pins" | "rapidkoi left-off", …}`.
  - **trust:** `entries: [{host, key (32 bytes), sequence, separate}]`. Each host passes `check_host` and each key is 32 bytes, or the file is broken.
  - **pins:** `pins: [{name, path, icon?}]`. A pin's icon is checked with the bundle's icon rule (`check_role`). A bad icon is dropped, not the pin.
  - **left-off:** `{name, path, title?, state?, state_version}`. The state is ≤ `MAX_STATE`.
- Reading gives `Recalled<T>`:
  - `Fresh`: no file
  - `Read(T)`
  - `Broken(set_aside_path, why)`
  - `Newer(n)`: a format number above 1

**Broken and newer files (topic 4)**
- **Broken:** renamed to `NAME.broken` (replacing an older `.broken`). That part starts empty, with a notice:
  - pins: "Rapidkoi couldn't read your pins, so it set them aside in <path>"
  - trust: "…your trusted keys…; sites will be trusted on first use again"
  - left-off: "…where you left off…"
- **Newer:** never read, set aside or overwritten. That part works in memory this session, with a notice: "Your pins were saved by a newer Rapidkoi; changes this session won't be kept". Saves to that file are skipped.
- **Missing:** starts empty, silently.

**When it saves (topic 3): on every change, on the window's thread**
- `shell.update` compares the model before and after each message, and saves only what changed:
  - **pins:** the list, by value
  - **left-off:** its path, title, state and state version
  - **trust:** the keyring's change count
- `boot` loads all three once, before the first view.
- The keyring's change count moves only when trust is recorded (a first use, a rotation, a higher sequence, a separate app). So a plain re-open of a known site saves nothing.

**Continue after a restart**
- A `LeftOff` read from disk has no compiled program. Continue reloads it from its path, as Reload does, with the saved state:
  - a signed site is verified again
  - a state version that no longer matches starts over, with the usual notice
- A file or server that's gone shows the usual stopped page.

**Forget this app** removes its pin and its left-off, and those files are saved. It **keeps the app's trusted key**, so a swapped key still asks first if the app is opened again. Forgetting a key is a separate, later control.

**CLAUDE.md rule 3** gains an exception: `update` saves Rapidkoi's own small files in its data folder (trust ≤ 1 MB, pins ≤ 32 MB, left-off ≤ `MAX_STATE` + 1 MB), written safely (PRP-007). It moves to `Cmd.task` when std's file I/O stops blocking the thread.

### Must NOT Do
- **No restoring open tabs:** that's the next PRP, as agreed.
- No copies of apps or bundles on disk, and no SQLite.
- Nothing saved outside the data folder. No path from a file is ever written to or deleted; paths in pins and left-off are only opened, through the same checks as typing them.
- No deleting a broken file, and no overwriting a newer one.
- No secret keys here: `trust` holds public keys only.
- No disk in tests unless the test makes its own folder under `test/tmp/` (the Model's `store` is `None` by default).

## MODEL AND MESSAGES

- **Model fields added:**
  - `store: Option<Store>` (private): the data folder and which files are writable this session (`None`: nothing is saved)
- **Model fields changed:** none. `pinned`, `left_off` and `ring` are loaded from the store at boot.
- **Messages added:** none. Saving follows any message that changes a remembered part.

## ERROR HANDLING REQUIREMENTS

- **`StoreError`:**
  - `NoHome`
  - `Io(path, why)`
  - `TooLarge(file, size)`
- **The format readers' outcomes:** `Recalled.Broken(aside, why)` and `Recalled.Newer(n)`. Each field check names the field: "an entry's key isn't 32 bytes".
- **What `update` shows:** an `Err` from a save becomes the notice line; nothing panics on any file. `boot` gathers each part's notice, and the first one shows (the others are joined with " · ").
- **Panics:** none for file contents. `expect` only on invariants: none planned.

## SECURITY CONSIDERATIONS (the owner reviews before merge: it holds the trust store)

- **The trust file is what first-use trust rests on across restarts.** It holds public keys and sequences only, and nothing secret.
  - **Tampering:** anything that can write the data folder can already rewrite it, which is why a broken file is set aside rather than failing closed (topic 4).
  - **Permissions:** the folder is 0700 and the files 0600.
- **Every file is untrusted input:** size caps before decoding, CBOR limits, and every field checked. A host must pass `check_host`, a key must be 32 bytes, and an icon must pass the bundle icon rules.
- **Paths read from pins and left-off** are only opened, through `navigate`'s usual checks (local files, or local development addresses). They're never written to or deleted.
- **Safe writes:** a temporary file at 0600, then sync, rename and a sync of the folder. A crash leaves the old file or the new one.
- **Forget keeps the key** (above), so forgetting an app can't be used to slip a swapped key in.
- No new packages, externs or links.

## TESTS TO WRITE
- [ ] **store:**
  - the folder from `XDG_DATA_HOME` (absolute), with a relative one ignored, from `HOME`, and neither (`NoHome`)
  - a save then a read round trip
  - no `.tmp` left behind
  - an unwritable folder is an `Io` error and leaves the old file unchanged
  - a too-large file is refused before decoding
  - set aside: `NAME.broken` appears, replacing an older one
- [ ] **remembered:**
  - each file round trips
  - missing → `Fresh`
  - garbage bytes, the wrong `kind`, a bad host, a 31-byte key, too many entries → `Broken` with the field named
  - `format: 2` → `Newer(2)`, and that file is left byte-for-byte unchanged
  - a pin with a bad icon keeps the pin without it
- [ ] **trust across a restart** (live sockets, a test data folder):
  - open a site, then boot a new model from the same folder: the key is trusted (a swapped key asks)
  - a lower sequence is still refused
  - a separate app is still separate
- [ ] **pins and left-off across a restart:**
  - pin, then boot again: the pin shows on home
  - leave an app with state, then boot again: Continue reopens it with that state
  - a changed state version starts over with the notice
  - a missing file shows the stopped page
- [ ] **saves follow changes:**
  - pinning writes `pins`
  - reopening a known site doesn't rewrite `trust`
  - Forget removes the pin and left-off and keeps trust
- [ ] **notices:**
  - a broken file at boot shows the set-aside notice, for each of the three
  - a newer file shows the newer notice, and a pin made then isn't written
  - a failed save shows "Rapidkoi couldn't save …"
- [ ] **tests stay off the disk:** `with_pages` has no store, and nothing is written.

## ROLLBACK PLAN
- Branch: `prp-007-remembering`, from main after PRP-006.
- Nothing outside the data folder (and `test/tmp/` in tests) is written. Deleting `~/.local/share/rapidkoi/` returns Rapidkoi to today's behaviour.

## ACCEPTANCE CRITERIA
- [ ] Pin an app, open a signed site, leave an app with state, then quit and start again: the pin is there, Continue resumes the app, and the site opens ✓ against the remembered key *(the owner, on screen)*
- [ ] After a restart, a site whose key was swapped asks first *(the owner, on screen)*
- [ ] A hand-broken `pins` file is set aside with a notice, and Rapidkoi starts *(the owner)*
- [ ] Every rule and outcome above has a test
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated
- [ ] The owner's security review before merge

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- `stat -c %a ~/.local/share/rapidkoi ~/.local/share/rapidkoi/*` shows 700 and 600
- For the owner:
  1. `./main`: pin an app, open `rapidkoi://localhost:8000` (served signed), and leave an app mid-count.
  2. Quit and run `./main` again: the pin is there, Continue resumes the count, and the site opens ✓.
  3. Then write junk into `~/.local/share/rapidkoi/pins` and start again: the notice shows, and `pins.broken` is there.

## AMENDMENT 1 — 2026-10-18 (session 37, the owner after trying it: "go with A")

**What happened:** the owner quit with the counter open in its tab, mid-count. The pin survived but the state didn't. `left-off` was only saved when an app left its tab (closed, or replaced), and quitting leaves nothing: kivuli has no quit hook.

**Change:** what's remembered as "where you left off" is what home's card means: the app you used last.
- That's the current tab's app, else the last visited tab if it's still open, else the last app closed.
- It's saved whenever it changes, its state included, so quitting at any moment keeps it.
- After a restart, Continue reopens it with that state, as before.

**Cost, accepted:** a small save after each event in that app (the counter's state is a few bytes). Coalescing saves for apps with large states is a later refinement if it's ever noticeable.

**Tests added:**
- quitting with the app open keeps its state (the counter: Add twice, start again, Continue shows Count: 2)
- the app used last is the one remembered, across two tabs
- Forget still removes it

## AMENDMENT 2 — 2026-10-18 (session 38, the owner: "if we have host and key we can preserve state the rest just loose everything")

**What happened:** after amendment 1, the owner's counter state was still lost. Opening the app by its address (not Continue) started it fresh, and that fresh state became "the app used last", overwriting the saved one. The owner: an app's identity is its host and key (the signing design), so state belongs to that identity, whatever route opens it.

**Change:**
- **Signed apps keep their state across restarts, by identity: host + key.**
  - The same app opened from its site, as its signed `.rkb`, by pin or by Continue resumes the same state.
  - A separate publisher (another key) has its own state. A ⚠ not-confirmed file has its own key's state, so it can never reach a trusted publisher's.
  - **Rotation:** a key change signed by the old key moves the state to the new key, so a rotation doesn't lose it.
- **Unsigned apps** (a bare `.wasm`, an unsigned development server, a format 1 bundle) keep state only within a session, as before. After a restart they start fresh, and they're not offered by Continue. Their pins stay.
- **On disk:** `states/` holds one file per signed app, named by the SHA-256 of its host and key.
  - Each file is `{format: 1, kind: "rapidkoi state", host, key, state_version, state, used}`, where `used` is milliseconds since 1970 (`std.time.now_ms`).
  - It's checked like the others, and its host and key must be the identity asked for.
  - Each is written safely. At most 256 apps: past that, the one used longest ago is removed.
- **`left-off`** now says which app was used last (name, path, title, icon, and host and key for a signed app) and holds no state. It's only written for a signed app. Continue opens its path, which resumes from `states/`.
- **Opening a signed app by any route resumes its saved state** when the state version matches. Otherwise it starts over with the usual notice.
- **Saving:**
  - after every message, the app used last's state is saved when it changed, if it's signed
  - Start over saves the fresh state
  - Forget removes its state file
- **Rule 3:** the exception covers reading a signed app's state file when it opens (≤ `MAX_STATE` + 64 KB).

**Tests added:**
- a signed app resumes when opened by its address after a restart, and by Continue
- an unsigned app starts fresh after a restart and isn't offered by Continue
- a separate publisher doesn't get the first publisher's state
- a rotation keeps the state
- Forget removes the state file
- a changed state version starts over
- the cap removes the app used longest ago
- a state file whose host or key isn't the one asked for is broken
