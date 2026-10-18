## FEATURE: Loading in the background: an address opens at once on a loading page while its key, index and build download and are checked, and the window stays usable

## OBJECTIVE
Opening `rapidkoi://localhost:8000` (typed, pinned, Continue, ⟳, or `./main ADDRESS`) opens its tab at once on a loading page: the address, a spinner and "Loading…", then the time so far after 3 seconds. Meanwhile you can switch tabs, go home and open other apps. When the download is done and every check has passed, the page becomes the app, marked ✓ as today, or the refused or stopped page with the reason. It's the groundwork for remote `https://` apps (the next PRP), and it resolves DECISION-021 with its option A.

Designed in chat, session 40: `docs/source/meetings/2026-10-18-background-loading.md` (topics 1–3, all settled).

## CONTEXT

- **Starting state:**
  - `loader.load` fetches synchronously in `update` (3 s per request, CLAUDE.md rule 3's exception).
  - `verify.verify_site` reads and records trust in the shared `Keyring` as it goes.
  - `tabs.la` is 481 lines.
  - kivuli is 0.5; `main.la`'s subscriptions are `Sub.none()`.
- **Ending state:**
  - **Step 0, a split** (the file size rule; approved with this PRP): `LeftOff` and the functions about it move unchanged from `tabs.la` to a new `src/leftoff.la`. That's `leave`, `snapshot`, `compiled`, `recalled` and `signer_of`, about 120 lines. `resume` stays in `tabs.la`, because it starts a guest. Imports are updated in window, navigate, saving, remembered and the tests.
  - `lugha.toml`: kivuli `version = "0.6"`, then `lughac update src/main.la` rewrites `lugha.lock`.
  - `src/verify.la`: `check_site(known, host, get)` is pure over a snapshot of identities. It returns the verified build and a `Record` of what to trust. `record(ring, r)` applies a record. `verify_site` becomes these two together, so today's callers and tests keep working.
  - `src/fetching.la` (new): what runs in the task. `fetch_app(address, known)` gets the key file, index and build (or an unsigned server's `app.rkb`), 30 s per request. It returns `Fetched` (plain data): bytes checked against the snapshot, or a refusal.
  - `src/loader.la`: `from_fetched(engine, ring, fetched)` turns an arrived result into `Loaded` (applying the record, then compiling). Files keep `load`.
  - `src/tabs.la`:
    - `Screen.Loading(started_ms)`
    - `AppPage.load: i64`: the load's number, so a stale result is ignored
    - `loading(address, number, now)` and `arrive(page, …)`
  - `src/navigate.la`: opening an address returns a `Cmd.task`. `open_file`, `reopen`, `reload` and `separate_app` return `(Model, Cmd<Msg>)`.
  - `src/actions.la`: `Msg.Fetched(tab, number, Fetched)` and `Msg.Tick(now)`.
  - `src/shell.la`:
    - `update` handles both new messages
    - `subscriptions` ticks every 120 ms while any tab is loading
    - `start` and `boot` return the task for `./main ADDRESS`
  - `src/frame.la`: the "loading…" marker. `src/render.la`: the loading page.
  - CLAUDE.md: rule 3 (the fetch exception removed), the tree, known issues; CHANGELOG.md; DECISIONS.md (DECISION-021 resolved).
- **Related code:**
  - `src/loader.la`, `src/verify.la`, `src/navigate.la`, `src/shell.la`
  - `../kivuli/README.md` (Effects, Background work), `../kivuli/examples/download.la`
- **Lugha spec relied on:**
  - `std.http` `request_with` with `Limits { timeout_ms }`
  - `std.time.now_ms`
  - closures capturing plain data
- **kivuli API used:**
  - `Cmd.task(work, done)`, `Cmd.batch`
  - `Sub.every(ms, f)`, `Sub.none`
  - all confirmed in kivuli 0.6.0's README
- **Open decisions:** none. DECISION-021 resolves to option A (kivuli's `Cmd.task`).

## IMPLEMENTATION REQUIREMENTS

### Must Do

**The loading page (topic 1)**
- Opening any `rapidkoi://` or `http://` local address puts the page in its tab at once, as `Screen.Loading(started)`, and starts one `Cmd.task`. That applies to typing it, a pin, Continue, ⟳, Open as a separate app, and `./main ADDRESS`.
- The page shows the address, a spinner frame ("◐ ◓ ◑ ◒", turned by the 120 ms tick) and "Loading…". From 3 s on, it shows "Loading… N s".
- The bar's marker reads "ADDRESS · loading…". There's no ✓ or ⚠ until the result has been checked.
- Every other action works while a tab loads: other tabs, home, the panel, more opens. Each loading tab has its own task.
- A result for a closed tab, or for an older load number (⟳ started another), is ignored.
- Local files (`.rkb`, `.wasm`) load as today, synchronously.

**The task (topic 2)**
- The task's function captures only plain data: the address, and a snapshot of the identities trusted for that host (`Identity[]`). It never holds a `Ref`, the model, kivuli or GTK.
- It runs the same checks as today, in the same order, failing closed: key → index → build. It uses the snapshot for first use, rotations and the sequence.
- It returns `Fetched`: the build's bytes and what it verified (or a format 1 bundle from an unsigned server), plus the `Record` to apply. Or a refusal (`Failed`, `Unverified`, `Unapproved`), with the same sentences as today.
- **On arrival** (`Msg.Fetched`) the window:
  1. Ignores the result if it's stale (above).
  2. Compares that host's identities in the keyring with the snapshot. If they differ, the tab loads again with a fresh snapshot: a new task, the page staying on Loading. That's at most once per open; a second difference shows "Trust for HOST changed while it loaded; try again" (⟳).
  3. Applies the record (`verify.record`), so trust is recorded only here.
  4. Reads the bundle (`read_any`, which checks the signature again over the same bytes; cheap) and compiles it.
  5. Starts the page, resuming a signed app's saved state as today (`navigate.resumed`).
- Nothing is compiled before every check has passed.

**Time (topic 3)**
- Each request: `timeout_ms` 30,000; the body caps are unchanged (`MAX_BUNDLE` for a build).
- A timeout reads "ADDRESS didn't answer in 30 seconds".
- The tick (`Sub.every(120, Msg.Tick)`) runs only while some tab is loading. An idle Rapidkoi has no subscription.

**Saving** keeps working: `persist` runs after `Msg.Fetched` like any message, so a first use is saved as it's recorded.

### Must NOT Do
- No remote hosts or https: still localhost only (the next PRP).
- No checks or compiling in the task's result handler before staleness and the snapshot are checked. No trust recorded inside the task.
- No progress bar (std.http can't report one), and no cancelling (kivuli can't). A dropped result costs only its download.
- No background reading of local files (it would block anyway).
- No change to the formats, the contract, or the checks themselves.

## MODEL AND MESSAGES

- **`AppPage`:**
  - `load: i64` (private): this page's load number
  - `Screen.Loading(i64)`: when the load started, in ms
- **`Model`:** `now: i64` (private), the last tick, for the elapsed time.
- **Messages:**
  - `Fetched(i64, i64, Fetched)`: a task's result, with its tab id and load number
  - `Tick(i64)`: the loading tick's time

## ERROR HANDLING REQUIREMENTS

- `Fetched` carries the loader's `Refusal` variants: the same sentences as today, plus the timeout sentence.
- A changed snapshot twice gives the "Trust for HOST changed while it loaded; try again" stopped page (⟳ only).
- A task never panics on network input: everything it touches already returns `Result`.
- A panic inside a task would end the program (kivuli). The task's code has no `unwrap` on external data; there's a test that hostile responses give refusals.

## SECURITY CONSIDERATIONS (the owner reviews before merge: it moves the verification path)

- The same checks, in the same order, fail closed. Only where the bytes are fetched changes.
- **Trust is recorded in one place** (on arrival). The snapshot check closes the race of two concurrent first uses.
- **Stale results are ignored by tab id and load number**, so an old download can't replace a newer page.
- **The task holds only plain data:** no keyring box, no model, no GTK.
- **Compiling** only follows full verification, on the window's thread.

## TESTS TO WRITE
- [ ] **split:** the suite passes unchanged after step 0 (imports only).
- [ ] **verify:**
  - `check_site` over a snapshot gives the same outcomes as `verify_site` for every existing case
  - `record` applies a first use, a higher sequence and a rotation
  - `verify_site` still passes its tests
- [ ] **fetching** (live sockets, as test/opening.la):
  - a signed site gives its bytes and a record
  - an unsigned server gives `app.rkb`
  - a server that never answers times out (a short timeout in the test) with the sentence
  - a tampered build is refused, as today
- [ ] **pages, pure** (`update` with hand-made `Fetched` values; no network):
  - an address opens a Loading tab and returns a task
  - the spinner turns on Tick, and "Loading… 4 s" shows after 4 s
  - the marker reads "· loading…"
  - other messages work meanwhile (select, home, new tab)
  - a result for a closed tab, or an older load number, changes nothing
  - a result with a changed snapshot reloads once, then refuses with the sentence
  - a good result becomes the app and records trust; a refusal becomes the refused page
- [ ] **end to end** (live sockets): `start(["./main", ADDRESS])` returns a task. Running it and sending its message gives the verified app, a signed app's saved state resumes, and trust is saved.
- [ ] **subscriptions:** none when idle; a tick while loading.

## ROLLBACK PLAN
- Branch: `prp-008-background_loading`, from main after PRP-007.
- Step 0 (the split) and the kivuli bump are separate commits and can stay even if the rest is abandoned.

## ACCEPTANCE CRITERIA
- [ ] `./main rapidkoi://localhost:8000` shows the loading page, then the counter ✓ *(the owner)*
- [ ] With a slow server (`./main serve` behind a pause, or stopped mid-load), the window stays usable and shows "Loading… N s", then the reason *(the owner)*
- [ ] Every test above
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated
- [ ] The owner's review before merge

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner: `./main serve site/`, then `./main rapidkoi://localhost:8000`. Then open the address with the server stopped: the loading page shows, then "didn't answer"/"couldn't reach", while the other tabs still work.
