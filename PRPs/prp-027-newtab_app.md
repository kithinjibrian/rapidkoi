## FEATURE: The new tab is an app that ships inside Rapidkoi and shows the Rapidkoi wordmark.

## OBJECTIVE
From chat with the owner (sessions 157–158). An empty tab runs Rapidkoi's own new-tab app, a bundle built into `./main`, instead of drawing an empty content area (DECISION-027: content pages are bundled apps). For now the app shows only the word "Rapidkoi" as a title; a real logo waits for images in the contract.

## CONTEXT

- Starting state: an empty tab (PRP-024) draws nothing in the content area; `guest/rust/` holds the counter and quote apps; bundles are packed with `./main pack`.
- Ending state:
  - **Part 1, now:** `guest/rust/newtab/` (a Rust crate in the workspace, contract 2) whose view is one title text, "Rapidkoi". `scripts/newtab.sh` builds it with cargo and packs it (`./main pack` with `--name "New tab" --version 0.1.0`, no key, so unsigned; amended while building: flags, not a `rapidkoi.toml`, since the repo's own `rapidkoi.toml` is the counter's) into `assets/apps/newtab.rkb`, which is committed so building `./main` never needs cargo.
  - **Part 2, once Lugha has `include_bytes` (DECISION-038):** `src/window/newtab.la` holds the bundle's bytes; an empty tab runs it as a page of its own; `src/ui/content.la` draws it.
- Related existing code: `guest/rust/counter/` (the crate to copy), `src/commands/pack.la`, `src/bundles/loader.la`, `src/window/tabs.la` (`open_module`), `src/ui/content.la`.
- Lugha spec sections relied on: Part 2 needs `include_bytes`, which doesn't exist yet (`docs/specs/lugha-include-bytes.md`).
- kivuli API used: none new.
- Open decisions that must be resolved first: DECISION-038 (deferred) blocks Part 2 only.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- The app uses only contract 2: a `text("Rapidkoi").title()` in a column. No messages, no asks, no state.
- The bundle is **unsigned**, trusted because it is part of `./main` (owner, session 158). An empty tab still shows no shield and no ⚙.
- The embedded app runs under the same limits as any guest (fuel, memory, tree size): it is a guest like the others.
- Part 2: an empty tab is still an empty tab (`blanks`, at most one, filled in place by opening something, PRP-024/026); only its content changes. The running new-tab app is never "where you left off", never pinned, never saved, never in the sidebar as an app.
- Part 2: if the embedded bundle fails to load or stops, the empty tab shows the empty content area, never an error screen: Rapidkoi's own page failing isn't the person's problem to read.

### Must NOT Do
- Change the contract to add images, alignment or colours (the wordmark sits where contract 2 puts text, at the start; a centred logo is a later contract).
- Sign it or add a Rapidkoi signing key.
- Read it from a file beside `./main` while waiting for Lugha (the owner chose embedding).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- Part 1: none.
- Part 2: `Model.newtab: Option<AppPage>`, the running new-tab app, shared by the one empty tab (started at launch, kept while the window lives). Messages from it are dropped: it sends none.

## ERROR HANDLING REQUIREMENTS

- Part 2: loading the embedded bundle returns `Result` through the existing loader (`BundleError`); an `Err` leaves `newtab` `None`, and the empty tab draws as today. A stop or trap is the same.

## SECURITY CONSIDERATIONS

- No new host functions, dependencies, externs or keys. The bundle is built from this repo's source; it's still run as an untrusted guest under every cap.
- The packed `assets/apps/newtab.rkb` is a committed binary: the script rebuilds it, and a test checks it's a valid unsigned bundle named "New tab".

## TESTS TO WRITE

Part 1:
- [x] `cargo test` in `guest/rust/newtab`: the view encodes to one column holding one title text, "Rapidkoi"; any event leaves the state as it was.
- [x] `test/newtab.la`: `assets/apps/newtab.rkb` reads as an unsigned bundle named "New tab", version 0.1.0 (skips if the file is missing, like `test/counter.la`).

Part 2 (written when DECISION-038 resolves):
- [ ] An empty tab's content shows "Rapidkoi".
- [ ] Opening an address from the empty tab fills it as before; the new-tab app isn't a tab, a pin or "left off".
- [ ] A bundle that fails to load leaves the empty content area, no error screen.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits.

## ACCEPTANCE CRITERIA
- [ ] Part 1: the crate builds for `wasm32-unknown-unknown`; `scripts/newtab.sh` writes `assets/apps/newtab.rkb`; `./main assets/apps/newtab.rkb` shows "Rapidkoi"
- [ ] Part 2: every empty tab shows "Rapidkoi"
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them)
- [ ] `lughac build src/main.la -o main` succeeds; `lughac fmt --check src test` clean
- [ ] No file over 500 lines; CLAUDE.md tree, DESIGN.md and CHANGELOG.md updated

## VALIDATION
- `cargo test --manifest-path guest/rust/Cargo.toml`
- `scripts/newtab.sh`
- `lughac test test/newtab.la`
- For the user: `./main assets/apps/newtab.rkb` shows the wordmark (Part 1); `./main` shows it in the empty tab (Part 2).
