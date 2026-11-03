## FEATURE: An app's start as one pipeline: every way of opening an app enters at Source, every app compiles through the modules kept for the run

**Status:** approved (the owner, 2026-11-03: "approved"). DECISION-096. Builds on PRP-068 (`prp-068-boot-sequence`, committed, not merged).

## OBJECTIVE

- **Nothing the person sees changes**, except that any app opened again in a run (a local file, a signed site, a kept copy) compiles nothing, as Rapidkoi's own apps already don't.
- **One path.** However an app is opened, it goes through the same stages, each a function from one value to the next, a failure ending in the page state it ends in today:

```
  built-in │ local file │ signed site │ kept copy │ upgrade
        │
        ▼
  1. SOURCE     its bytes
                built-in → inside ./main
                file     → read (≤ 32 MB)                     ✗ Stopped
                site     → Loading; fetched in a task          ✗ Stopped / offline: kept copy
                kept     → the data folder's copy
        │
        ▼
  2. CHECKED    bundle rules, contract 11, permissions
                (built-in ones only from ./main), controls     ✗ Refused
                site: its signature and key                    ? Unapproved → back to 2
        │
        ▼
  3. COMPILED   through the modules kept for the run           ✗ Stopped
        │
        ▼
  4. STARTED    instantiate; saved state (signed, resumed)
                or on_init; on_open                            ✗ Stopped (Restart)
        │
        ▼
     its page: Running, with what it asked
     ──► Settle answers it (PRP-068), as after any message

  another window │ Restart │ restoring a window ──► re-enter at 4 with the page's Program
```

## CONTEXT

- **The entry points today**, each running its own subset of these steps:
  - built-ins: `window/newtab.la`, `window/settings_app.la`, `window/allapps.la` (`open`, `open_kept`, `open_bundle`);
  - a local file: `window/tabs.la`'s `open_path` (`bundles/loader.la`'s `load`), from `window/navigate.la`'s `open_file` and `open_launched`;
  - a signed site: `net/loads.la`'s `begin`, `begin_app`, `begin_link`, `begin_place`, `begin_at`, `begin_restoring`, then `arrive`; its kept copy, `net/keeping.la`;
  - re-entries: `window/instances.la`'s `another`, Restart in `window/tabs.la`, `window/restarts.la`, `window/upgrades.la`;
  - the guest's own start: `guest/running.la`'s `start` (already a chain: open → on_init or saved state → on_open → shown).
- **What PRP-068 left:** the modules kept for the run (`guest/modules.la`), the loader's `from_bytes_with` (a compile function given), and Settle, which already answers what an app asks when it starts.
- **Files near the limit:** `src/net/loads.la` and `src/window/navigate.la` (≈480) shrink as their entry points call the pipeline.
- **Decisions:** DECISION-096. None open touch it.
- **Changes:**
  - **new:** `src/boot/appboot.la` (the stages and `Source`; `window/apps.la` is taken), `test/appboot.la`;
  - **changed:** the entry points above call it; `bundles/loader.la` (its file and site paths through `from_bytes_with`); CHANGELOG.md.

## REQUIREMENTS

### Must Do

- **`Source`**: `BuiltIn(file, bytes)`, `File(path)`, `Site(...)` (what `net.loads` begins with), `Kept(...)`, `Upgrade(...)`. Every entry point names one and calls the pipeline; none runs a stage itself.
- **The stages** (`boot/appboot.la`): `sourced`, `checked`, `compiled`, `started`, each its own function, returning the next value or the page state it ends in today (Stopped with or without Restart, Refused, Unapproved), with the same sentences.
- **A site is two halves of the same pipeline:** `sourced` gives a Loading page and the task; when its result arrives (`arrive`), the pipeline carries on at `checked`. A result for a closed tab or an older load is still ignored, and one checked against changed trust still loads again once.
- **Every app compiles through the modules kept for the run** (stage 3): a file or a site opened again compiles nothing; a changed file or a new version has new bytes, so it compiles fresh.
- **Re-entries** (another window, Restart, restoring a window) enter at `started` with the page's own `Program`, as today.
- **The same behaviour:** every test that passes today passes, except where a test named a function that moved.

### Must NOT Do

- **Change what's checked, refused or asked**, the order of checks, the page states or their sentences, or how trust, kept copies and upgrades work.
- **Answer or show anything in the pipeline:** that's Settle's (PRP-068), so starting and every update answer the same way.
- **Keep compiled code on disk** (DECISION-096), or hide apps instead of closing them.
- **All apps closing when you leave it, or anything else from PRP-067's review:** they follow this PRP.

## DESIGN

- **Values between stages:** `Sourced { bytes, origin: Origin, path, link, trust snapshot }` → `Checked { Loaded without its module, the module's bytes }` → `Compiled { Program }` → the page from `started` (today's `ran(page, running.start(...))`).
- **Where failures go:** each stage returns `Result<Next, Ended>`, `Ended` being the page state it ends in (`Stopped(why, restart)`, `Refused(why)`, `Unapproved(change, before)`); `page_of_ended` makes the page, as `refused` and `failed_page` do today.
- **The cache:** `compiled` takes the window's `Modules` and gives them back; entry points store them in the model, as PRP-068's built-ins do.
- **Security:** no new capability, nothing exposed to apps. The checks run in the same order, still before anything is compiled: a refused app's module is never compiled.

## TESTS

- [ ] `test/appboot.la`:
  - [ ] each source reaches a running page: a built-in, a local `.wasm`, a local `.rkb`;
  - [ ] each failure ends as it does today: an unreadable file (Stopped, no Restart), a bundle for another contract (Refused, its sentence), built-in permissions from outside `./main` (Refused), a module that won't compile (Stopped);
  - [ ] a local file opened twice compiles once; the same file changed compiles again;
  - [ ] re-entering at `started` (another window) starts fresh from the same `Program`.
- [ ] `test/loading.la`, `test/keeping.la`, `test/upgrades.la`, `test/instances.la`, and `test/saving.la` and `test/effects.la` (which cover `window/restarts.la`): unchanged, as the sites', kept copies', upgrades' and re-entries' behaviour.
- [ ] The full suite: unchanged.

## ACCEPTANCE

- [ ] Every way of opening an app calls the pipeline; no entry point compiles or starts an app itself.
- [ ] `lughac test test/` (the owner runs it), `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: apps open as before (a local file, a signed site, offline from its kept copy, an upgrade); opening the same app again is quicker the second time.

## AS BUILT

- **The pipeline is `src/boot/appboot.la`** (`window/apps.la` was taken): `Source` (`BuiltIn`, `File`, `Fetched`), `checked`, `loaded` (Source to Compiled, through the modules kept for the run), `started`, and `opened` for a local file. `tabs.open_path` moved there as `opened`: `tabs.la` can't import the pipeline, which imports it.
- **The stages' work stays in `bundles/loader.la`**, split so each stage is one function: `checked_file`, `checked_bytes`, `checked_fetched` (Source and Checked, nothing compiled), `vetted` (the bundle's rules), `compiled_with` (Compiled). `load`, `from_bytes` and `from_fetched` are those chained with a plain compile, for tests; `from_bytes_with` is gone.
- **Every entry point calls it:** the three built-ins' openers, local files (`navigate`'s `open_file` and restoring a window, `choosing`), ⟳ on a local file (`restarts.load_again`), and a site's result (`loads.arrive`). The six `begin_*` still place the tab and start the fetch; their result enters at Checked.
- **Re-entries** (another window, Restart, upgrades, restoring state) start from the page's own `Program` through `guest.running.start`, stage 4, as before.
- **Kept copies** arrive through `loads.arrive` as a site's result does, so they go through the pipeline with it.
