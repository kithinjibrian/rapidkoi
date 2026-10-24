# rapidkoi-rust request: contract 7's base (`v0.2.0-pre.1`)

From Rapidkoi, PRP-045 (DECISION-056). For a session in `../rapidkoi-rust`, under its own rules. Rapidkoi's side is built and tested on its branch `prp-045-contract_7_base`; it waits on this tag to move its built-in apps (the new tab, Settings) and merge.

## What changed in the contract

Contract 7 is contract 6 with two additions, and Rapidkoi runs **only** contract 7 (`rapidkoi_version` 7; anything older is refused with "This app needs to be rebuilt for this Rapidkoi"). Events, state, links, the tree, styles, roles and storage are as in contract 6.

1. **`kinds` in `handle`'s result.** A map from a field's path (as `link` names it: 1–4 segments of `[a-z0-9_]`, joined by `.`) to `"number"`, `"text"` or `"bool"`, at most 16. When a link opens a fresh window, Rapidkoi sets each link field where the fresh state holds a value of the same kind (as before) **or holds null and `kinds` names its kind**. So a field that starts empty, an `Option` in Rust, can now be set by a link. Rapidkoi reads `kinds` from the `on_init` result. A bad `kinds` stops the app.
2. **`label` on a typing box.** An `input` may carry `label` (text, 1–200 bytes): Rapidkoi names the box by it (and screen readers will hear it once kivuli can set accessible labels). An empty or over-long label stops the app.

## What the SDK needs

1. **`__CONTRACT_VERSION` = 7** (`abi.rs`).
2. **`kinds`, written for the app:**
   - `Simple` gains its kind: integers (every `i*`/`u*`) and floats → `"number"`, `String` → `"text"`, `bool` → `"bool"`, `Option<T>` → `T`'s.
   - `#[derive(State)]` also writes `fn kinds() -> Vec<(String, &'static str)>`: every simple field by its name, and every nested `State` field's kinds under its name as a prefix (`editor.task`), up to `MAX_PATH` segments and 16 in all (the first 16, in field order, when there are more).
   - The `Init` result always carries them: the export that answers `on_init` writes `"kinds": {path: kind, …}` beside `state`, `effects` and `link`. Other results may leave it out.
3. **`label` on inputs:** `View::label` already writes `label` on any node, typing boxes included; nothing to change, but a test that an `input(...).label("Search notes")` encodes it would pin it.
4. **Docs:** the crate's description and `lib.rs`'s top say contract 7; an `Option` link field is the way to say "none" (Notes moves from `note: u32` with 0 to `note: Option<u32>`).
5. **Tests:** `kinds()` for flat, nested and `Option` fields; the `Init` result carries `kinds`; version 7.
6. **Release:** the examples (counter, styled) rebuilt and their tests passing; tag **`v0.2.0-pre.1`** (asking the owner before pushing the tag).

## How Rapidkoi will use it

`guest/rust/newtab` and `guest/rust/settings` move to `tag = "v0.2.0-pre.1"`, are repacked (`scripts/newtab.sh`, `scripts/settings.sh`), and PRP-045 merges. PRP-046 (scrolling) and PRP-047 (pictures) follow the same way (`pre.2`, `pre.3`); `v0.2.0` after them.
