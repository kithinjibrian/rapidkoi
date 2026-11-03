## FEATURE: Rapidkoi's start as a chain of named stages, one Settle for start and every update, and compiled modules kept for the run

**Status:** approved (the owner, 2026-11-03: "approved"). DECISION-096. Builds on PRP-067 (committed in progress as 3c4db5a on prp-067-all-apps), which waits for this one.

## OBJECTIVE

- **Nothing the person sees changes**, except that Rapidkoi's own apps (the new tab, Settings, All apps) open without compiling again each time.
- **The code says the order.** Rapidkoi's start reads as its stages, top to bottom, each a function returning its result or the notice that explains it:

```
  ./main [args]
   0. COMMAND      pack │ keygen │ publish │ serve ──► runs it, exits
   1. SETUP        wasmtime engine, SQLite's shared memory cap, the clock
   2. DATA FOLDER  $XDG_DATA_HOME/rapidkoi, else ~/.local/share/rapidkoi
                   ✗ ──► nothing saved + notice; 3 and 4 skipped
   3. RESTORE      trust ─► pins ─► recent ─► people ─► identity ─►
                   preferences ─► notifications ─► apps
                   each part: Read │ Fresh (default) │ Broken (set aside
                   + notice) │ Newer (left alone + notice, never written)
   4. SESSION      the session that ended becomes "previous"
   5. BUILT-INS    compile the new tab, Settings and All apps once;
                   start the new tab  ✗ ──► that app can't open + notice
   6. WINDOW       the blank tab, the focus in the address bar
   7. SETTLE       tell what changed ─► answer ─► fetch pictures ─►
                   time zones ─► save   (the same passes as every update)
   8. RUNNING      subscriptions; Launched(items) ─► each opened
```

## CONTEXT

- **What exists:**
  - `src/main.la` (stage 0 and the application), `src/shell.la` (`boot`, `start`, `start_with` with numbered comments; `update`'s tail runs the passes after a message by hand);
  - `src/data/saving.la` (`session_store`, `restore`: each file read in turn by hand; `persist`), `src/data/sessions.la` (`begin_session`), `src/data/parts.la` (each file's `Part`: its name and its words);
  - the built-ins' openers (`window/newtab.la`, `window/settings_app.la`, `window/allapps.la`), each compiling its bundle (`bundles/loader.la` → `guest/guest.la`'s `compile`);
  - `std.crypto`'s `sha256` (`lughac spec`).
- **The drift this removes:** start answers apps and saves; `update` also tells apps what changed, fetches pictures, answers what those ask, and settles time zones. PRP-065's empty-command bug reached start and not updates for that reason.
- **Files at the limit:** `src/shell.la` (498 lines) gets shorter: `start_with` and `update`'s tail move out. `src/data/saving.la` (438) gets shorter: `restore`'s reads become the parts list. `src/data/remembered.la` (503) is untouched.
- **Decisions:** DECISION-096. None open touch it.
- **Changes:**
  - **new:**
    - `src/boot/boot.la`: the stages, chained;
    - `src/boot/settle.la`: the passes after start and after every message;
    - `src/data/restoring.la`: the parts list and reading it;
    - `src/guest/modules.la`: compiled modules kept for the run;
  - **changed:** `src/shell.la` (`boot`, `start_with` and `update` call the stages and `settle`), `src/data/saving.la`, the built-ins' openers (through the cache), `src/window/window.la` (`modules` in the model), CLAUDE.md (the layout's new `boot/` folder);
  - **tests and docs:** tests; CHANGELOG.md.
- **Then:** the app's start as one pipeline (its own PRP), then All apps closing when you leave it, then right-click (`on_context`).

## REQUIREMENTS

### Must Do

- **The stages** (`boot/boot.la`), in the order drawn above, each its own function: `setup`, `folder`, `restore`, `session`, `builtins`, `window`, then `settle`. `boot` is them chained; tests start at any stage with a model of their own, as `start_with` lets them today.
- **Restore reads a list of parts** (`data/restoring.la`): each its `Part`, its cap, its reader, its default (a function of what's read so far: Settings pinned on a first run; your apps seeded from Recent and pins), and where it goes in the model. The order is the list's. Each outcome (Read, Fresh, Broken, Newer) as today, with the same notices.
- **One Settle** (`boot/settle.la`): what `update`'s tail does now, in that order, as one function from the model before to the model after, giving the commands to send. `update` and Rapidkoi's start both call it.
- **Compiled modules kept for the run** (`guest/modules.la`): keyed by `sha256` of the module's bytes; at most 16 kept, the one used longest ago dropped. Built-ins compile through it: stage 5 starts the new tab through it; Settings and All apps compile into it the first time they open. (Built so, against this PRP's first words "stage 5 compiles all three": compiling them at start slows every start and every test that starts a window. The owner to choose.)
- **The same behaviour:** every test that passes today passes unchanged, except where a test named a function that moved.

### Must NOT Do

- **Change what any file holds, its notices, the subscriptions or `Launched`.**
- **Keep compiled code on disk:** DECISION-096.
- **The app's own pipeline** (Source → … → Shown) for every way of opening an app: the next PRP. Here only the built-ins use the module cache.
- **All apps closing when you leave it, or any other behaviour from the PRP-067 review:** they follow.

## DESIGN

- **`Stage` results:** each stage returns the model (with any notice already added, as now) and the commands it gives; a stage that can't do its work (no data folder, a built-in that won't start) says so with the notice it gives today and the chain carries on.
- **`Modules`:** `{ items: (u8[], Module, i64)[] }` (hash, module, last used), in `window.Model` as `modules`. `compiled(m, engine, bytes) -> Result<(Modules, Module), GuestError>`. A module is the engine's, so the cache holds only modules from the window's engine.
- **`Part` grows** a reader and a default only in `restoring.la`'s list (a struct of functions), so `parts.la`'s words stay where they are.
- **Security:** no new capability, nothing exposed to apps. The cache is keyed by the bytes' hash, so a changed module is never run from an older compile.

## TESTS

- [ ] `test/boot.la`: the stages in order on a fresh folder (the new tab started, the blank tab in front, Settings pinned); no data folder: nothing saved and its notice; a built-in that can't start: its notice, the rest starts.
- [ ] `test/restoring.la`: the parts in order; each outcome for one part (Read, Fresh, Broken set aside, Newer left alone) and its notice; a first run's defaults (Settings pinned, apps seeded).
- [ ] `test/settle.la`: start and an update run the same passes: what an app asks when it starts is answered, and what it then shows is fetched, at start as after a message.
- [ ] `test/modules.la`: the same bytes compile once; other bytes compile again; past 16, the one used longest ago goes.
- [ ] The existing suite: unchanged.

## ACCEPTANCE

- [ ] `shell.la`'s start and `update` read as the stages and `settle`; the order is said once.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass (the owner runs the tests).
- [ ] For the owner, by eye: Rapidkoi starts as before (the new tab, Restore, your pins on the dock); opening Settings or All apps a second time is quicker than the first.
