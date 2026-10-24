## FEATURE: The Rust SDK and its examples move to their own repository; this one keeps the shell and its built-in apps

## OBJECTIVE
rapidkoi holds the shell and Rapidkoi's own apps (newtab, settings), nothing else. The Rust SDK (`rapidkoi`, `rapidkoi-derive`) and its examples (counter, styled) live in a new repository, `rapidkoi-rust`, with their git history. The tasks and quote apps (and quote's server) are deleted: Notes and Trip replace them later, as SDK examples, in their own PRPs. The shell's test suite never needs Rust: nothing skips for a missing build.

Nothing changes in the window. This is a restructure, not a feature.

## CONTEXT

- Starting state:
  - `guest/rust/`: one Cargo workspace of eight crates: `rapidkoi`, `rapidkoi-derive`, `counter`, `styled`, `newtab`, `settings`, `tasks`, `quote` (31 tracked files, about 4,400 lines). Every app depends on the SDK by `path = "../rapidkoi"`.
  - `examples/quote/`: the quote app's server (`server.la`, sign-in and `/quote`), its `rapidkoi.toml` and `Dockerfile`. Served on the VM at quote.dafifi.net.
  - `rapidkoi.toml` at the root: the counter's, as a signed app (`./main publish` from the root publishes the counter).
  - Three test files read built Rust: `test/counter.la` (6 tests), `test/tasks.la` (4), `test/saving.la` (6 of its tests use the counter as the app a signed site serves). All skip when the `.wasm` isn't built.
  - `scripts/newtab.sh`, `scripts/settings.sh` build from `guest/rust/`.
- Ending state:
  - **`../rapidkoi-rust`** (new repository, `github.com/kithinjibrian/rapidkoi-rust`):
    ```
    Cargo.toml            workspace: rapidkoi, rapidkoi-derive, examples/counter, examples/styled
    Cargo.lock
    rapidkoi/  rapidkoi-derive/
    examples/counter/     with its icon.png and rapidkoi.toml (moved from this repo's root, paths fixed)
    examples/styled/
    README.md             what it is, building an example, packing it with Rapidkoi's ./main
    ```
    Made with `git filter-repo` from a fresh clone of this repository, so those files keep their history. Its own CLAUDE.md and session log are its own first session's work, under its own rules, not this PRP's.
  - **This repository:**
    ```
    guest/rust/Cargo.toml   workspace: newtab, settings
    guest/rust/newtab/      rapidkoi = { git = "https://github.com/kithinjibrian/rapidkoi-rust", tag = "v0.1.0" }
    guest/rust/settings/    the same
    ```
    Deleted: `guest/rust/{rapidkoi,rapidkoi-derive,counter,styled,tasks,quote}`, `examples/` (all of it), the root `rapidkoi.toml`, `test/counter.la`, `test/tasks.la`.
- Related existing code: `test/support.la` (`fixed`, `linking`, `echo_v6`: the fixture pattern), `test/saving.la` (`site_of`, `write_toml`), `test/queries.la` and `test/links.la` (they already cover what `test/tasks.la` checked of the shell), `scripts/newtab.sh`, `scripts/settings.sh`, `../lugha-wasmtime/src/module.la` (`from_text` compiles WAT straight into a module; nothing returns its bytes).
- Related source files: chat with the owner, 2026-10-24 (sessions 214–215): "most of this guest apps should go into individual repos and leave builtins only. the rust lib i also feel should be outside the repo"; "let's just have two. one that is truly offline and another that needs multiple servers. you can delete task and quote".
- Lugha spec sections relied on: none new.
- kivuli API used: none new.
- Open decisions that must be resolved first: **Q1 below** (how a test gets a `.wasm` file from WAT).

## WHERE EACH TEST GOES

| Test | What it checks | After |
|---|---|---|
| `counter.la`: starts at zero, add and reset, greeting | The counter app | Already covered by the counter's 4 `cargo test`s, which move with it. Deleted here |
| `counter.la`: Home and Continue keep the count; a pinned counter opens; packed as a bundle runs the same | The shell | Rewritten on the `counting` fixture (below), in `test/counting.la` |
| `tasks.la`: adding, ticking, filtering, a task's own screen | The tasks app | Deleted with tasks |
| `tasks.la`: a link opens a new window where it says; tasks outlive the window | The shell | Already covered by `test/links.la` (`a_link_opens_a_new_window_and_a_bare_address_switches`) and `test/queries.la` (`data_outlives_the_window`). Deleted |
| `saving.la`: six signed-site tests | The shell: restore, a separate publisher, rotation, Forget, a changed state version, Continue | Unchanged, serving the `counting` fixture instead of the Rust counter. They stop skipping |

**The `counting` fixture** (`test/support.la`): a hand-written contract 5 guest. Its state is `{count: n}` (n under 24, one CBOR byte). Its view is a box with the text `Count: n` and a pressable box labelled `Add`, and any `on_message` adds one. It's just enough to stand in for the counter: a state that changes on a press and shows on screen.

## Q1 — A `.wasm` file from WAT (needs the owner)

`saving.la`'s sites and the pack test need the fixture as a **file of WebAssembly bytes**, since `publish` and `pack` read a module from disk. lugha-wasmtime's `Module.from_text` compiles text straight into a module, and its `wasmtime_wat2wasm` is in the private `raw`.

- **(a) Recommended: lugha-wasmtime gains a public `wat_to_wasm(text): Result<u8[], CompileError>`**, a few lines over the extern it already has. The tests write `counting.wasm` into `test/tmp/` when they need it. Recorded in DECISIONS.md and done in `../lugha-wasmtime` under its own rules first. This PRP waits for it.
- (b) Commit `test/fixtures/counting.wasm`, made by `wat2wasm` (wabt, installed here) from a committed `counting.wat`. Nothing waits, but a binary sits in the repository and can drift from its source.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **In order**, each step leaving this repository building and its tests passing:
  1. **The fixture and its tests first.** Write the `counting` fixture (Q1's way) and `test/counting.la`. Switch `saving.la` from `counter()` to it. Delete `test/counter.la` and `test/tasks.la`. The owner runs the tests.
  2. **The new repository, locally:**
     - clone this repository into a scratch folder;
     - `git filter-repo --path guest/rust/rapidkoi --path guest/rust/rapidkoi-derive --path guest/rust/counter --path guest/rust/styled --path guest/rust/Cargo.toml --path guest/rust/Cargo.lock --path rapidkoi.toml` with renames into the layout above;
     - in a new commit there: the workspace's members, `examples/counter/rapidkoi.toml` paths, the SDK's description (contract 6, not 5), README.md;
     - move it to `../rapidkoi-rust`;
     - `cargo build` and `cargo test --no-run` there succeed; the owner runs `cargo test`.
  3. **Publish it (asks first: OUTWARD-FACING ACTIONS):** create the GitHub repository, push, tag `v0.1.0`.
  4. **Point the built-ins at it:** `guest/rust/Cargo.toml` (members: newtab, settings), both `Cargo.toml`s to the git tag. `cargo build` the built-ins. Repack both with `scripts/newtab.sh` and `scripts/settings.sh`, check the `.rkb`s still load (`test/newtab.la`, `test/settings_app.la`), rebuild `./main`.
  5. **Delete:** `guest/rust/{rapidkoi,rapidkoi-derive,counter,styled,tasks,quote}`, `examples/`, the root `rapidkoi.toml`, and the stale untracked `tasks.rkb` and `quote-server` at the root (after looking at them). Drop `/examples/quote/site` and `/quote-server` from `.gitignore`.
  6. **The docs:**
     - CLAUDE.md: COMMANDS (the counter, tasks, bundles and quote blocks go; one line points to `../rapidkoi-rust` for building example apps), STACK (Guests: the SDK is a git dependency), the tree (`guest/rust/`, `examples/`, `test/` lines), PROTECTED FILES (add `../rapidkoi-rust/`);
     - `.llmignore` and `.claude/settings.json`: `../rapidkoi-rust/**`, in step with CLAUDE.md;
     - MEMORY.md: a decision recording the split and the two coming examples (Notes, Trip); project state;
     - DECISIONS.md: Q1's outcome;
     - `docs/server-sign-in.md`: it names `examples/quote/server.la` as the worked example, so say the example is in git history (before this PRP's commit) until Trip's server replaces it;
     - CHANGELOG.
  7. **The VM (asks first):** stop and remove quote.dafifi.net's container. The exact commands go to the owner before anything runs. counter.dafifi.net stays as it is.
- Use `git rm` for every deletion, so the files stay in this repository's history.
- Each step's commit is its own, so any step can be undone alone.

### Must NOT Do
- Never write in `../rapidkoi-rust` after step 2. From then on it's a protected sibling, changed only in its own sessions.
- Don't change the SDK's code or the built-ins' code: only where they live and how they're depended on. The repacked `.rkb`s must hold the same apps.
- Don't start Notes or Trip. Each gets its own PRP.
- Don't change `docs/specs/contract-6.md`'s tasks examples: they're SQL examples, still valid without the app.
- Don't push, tag, create a repository or touch the VM without the owner saying yes at that step.
- Don't rewrite this repository's history: deleted files stay in its past commits.

## MODEL AND MESSAGES

- No changes.

## ERROR HANDLING REQUIREMENTS

- No new code paths in `src/`. The fixture is test code.

## SECURITY CONSIDERATIONS

- **A new git dependency for the built-ins:** `rapidkoi-rust`, the owner's own, pinned to a tag. It only affects building the built-ins (cargo); `./main` still embeds the committed `.rkb`s and never builds Rust.
- **Q1(a) adds a public function to lugha-wasmtime** that turns text into bytes. It runs nothing and exposes nothing to guests: no SECURITY RULE review needed beyond the owner's yes to Q1.
- **`examples/counter/rapidkoi.toml`** keeps `key = "~/.config/rapidkoi/keys/counter.key"`, a path outside the repository. No key moves into either repository.
- **Removing quote.dafifi.net** takes down the only public example of a server checking Rapidkoi sign-ins. Its code stays in git history, and Trip's server takes it over.

## TESTS TO WRITE

- [ ] `test/counting.la`: the fixture opens at `Count: 0`, and `Add` makes it `Count: 1`
- [ ] `test/counting.la`: Home and Continue keep the count; closed and reopened it still has it (from `home_and_continue_keep_the_count`)
- [ ] `test/counting.la`: pinned, it opens from Home at `Count: 0` (from `a_pinned_counter_opens_from_home`)
- [ ] `test/counting.la`: packed as a bundle with an icon and description, it runs the same, and ⟳ keeps the count (from `the_counter_packed_as_a_bundle_runs_the_same`; the icon is one of `test/support.la`'s `png(w, h)`)
- [ ] `test/saving.la`: its six signed-site tests run on the fixture and no longer skip
- [ ] `test/newtab.la` and `test/settings_app.la` pass after the repack (built from `rapidkoi-rust` v0.1.0)

## ROLLBACK PLAN

If this needs to be abandoned mid-way:
- Branch to return to: `prp-017-kivuli_0_17`, at the commit before this PRP's first.
- State: every step is its own commit. Revert from the last back. Before step 3, `../rapidkoi-rust` is local only and can be deleted. After it, the GitHub repository stays, and archiving or deleting it is the owner's call.

## ACCEPTANCE CRITERIA
- [ ] `../rapidkoi-rust` exists with the layout above and the moved files' history (`git log --follow rapidkoi/src/lib.rs` reaches before the split)
- [ ] It builds, and its `cargo test` passes (the owner runs it)
- [ ] It's on GitHub, tagged `v0.1.0` (the owner said yes)
- [ ] This repository's `guest/rust/` holds only newtab and settings, depending on the tag
- [ ] `examples/`, the root `rapidkoi.toml`, `test/counter.la` and `test/tasks.la` are gone
- [ ] No test skips for a missing Rust build (`grep -rn "build guest/rust" test` finds nothing)
- [ ] The repacked built-ins load and run (`test/newtab.la`, `test/settings_app.la`), and the owner sees the new tab and Settings unchanged
- [ ] quote.dafifi.net is down (the owner said yes)
- [ ] CLAUDE.md, `.llmignore` and `.claude/settings.json` protect `../rapidkoi-rust/`
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated

## VALIDATION
Run these commands to verify completion:
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- `cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown`
- In `../rapidkoi-rust`: `cargo test`, then `cargo build --release --target wasm32-unknown-unknown`
- For the user: `./main`: the new tab and ⚙ Settings look and work as before. In `../rapidkoi-rust`, pack and open the counter (its README's commands): it counts.
