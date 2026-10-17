## FEATURE: The shell: load one local WebAssembly app and run it under UI contract v1

## OBJECTIVE
`./main counter.wasm` opens a window showing the guest app's screen. Clicking its buttons and typing into its fields runs the guest's `update`, and the window shows the new screen. A broken, hostile or incompatible app shows a plain error screen with a Restart button and never crashes Rapidkoi. This is build path step 1 (MEMORY.md decision 4), implementing contract v1 (decision 10, `docs/source/meetings/2026-10-17-ui-contract.md`) exactly.

```
$ cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown
$ lughac build src/main.la -o main && ./main guest/rust/target/wasm32-unknown-unknown/release/counter.wasm
```

## CONTEXT

- Starting state: `src/main.la` (hello-world placeholder), `lugha.toml` (kivuli only), docs and decisions. No `test/`.
- Ending state:

```
lugha.toml            — + wasmtime and cbor as path packages
src/main.la           — wiring: args → load → kivuli.sandbox (≤ 40 lines)
src/limits.la         — every contract limit and the fuel calibration, as named constants
src/theme.la          — docs/DESIGN.md tokens; contract words (sm, title, primary…) → kivuli values
src/guest.la          — a running guest: open a module, check exports and version, call init/update/view
                        through the calling convention, all bounds-checked; GuestError
src/contract.la       — pure: CBOR bytes ↔ contract v1 (Node tree, update result, event envelope),
                        node/depth counting; ContractError with a tree path
src/render.la         — pure: Node tree → kivuli View<Msg>; the unsupported-component placeholder
src/shell.la          — the kivuli app: Model (Running | Stopped), Msg (Event | Restart), update, view
test/contract.la test/render.la test/guest.la test/shell.la test/support.la
test/fixtures/*.wat   — hand-written guests (see TESTS)
guest/rust/           — Cargo workspace: crate `rapidkoi` (contract v1 for Rust) + example `counter`
CLAUDE.md             — COMMANDS (guest build), FILE ORGANIZATION, architecture rule 3 (see below)
```

- Related existing code: `../kivuli/README.md`, `../kivuli/src/lib.la`, `../kivuli/src/testing.la`; `../lugha-wasmtime/src/*.la` and `examples/log.la`; `../lugha-cbor/src/value.la`, `decoder.la`, `limits.la`.
- Related source files: `docs/source/stakeholder/vision-2026-10-17.md`, `docs/source/meetings/2026-10-17-architecture.md`, `docs/source/meetings/2026-10-17-ui-contract.md`.
- Lugha spec relied on: `args()`, `read_bytes`, `string_from_utf8`, `s.to_bytes()`, `u8[]`, enums with payloads, closures, `Ref`, `?`, package `path` dependencies (§6), `@test`/`skip`, `has_display` (via kivuli tests).
- kivuli API used: `sandbox`, `text`, `button`, `input`, `column`, `row`, `.padding`, `.spacing`, `.size`, `.bold`, `.dim`, `.style(ButtonStyle…)`, `.disabled`, `.placeholder`, `.window`; `kivuli.testing` (`click`, `is_disabled`, `type_into`, `texts`).
- lugha-wasmtime API used: `Engine`, `Store` (`set_fuel`, `limit`, `close`), `Limits`, `Module.new`/`from_text`, `Linker.new`/`instantiate`, `Instance.call`/`memory`, `Memory.size/read/write`, `Val`, `CallError`, `LinkError`, `MemoryError`.
- lugha-cbor API used: `decode`, `encode`, `Limits`, `Value` and its `get`/`as_*` helpers, `DecodeError`.
- Open decisions that must be resolved first: none. New deferred entries 014 and 015 (below) don't block.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Dependencies**
- `lugha.toml` `[packages]`: `wasmtime = { path = "../lugha-wasmtime", version = "0.1" }`, `cbor = { path = "../lugha-cbor", version = "0.1" }`. `lugha.lock` is only ever written by lughac.

**Opening (`main.la`, `guest.la`)**
- `./main <path>`: read the file with `read_bytes`. Over 32 MB → error screen. The size is checked after reading, because `std.fs` has no size query (DECISION-015).
- No argument, an unreadable file, a module that doesn't compile: the error screen, with the reason. Never a crash or panic.
- One `Engine` for the program. Per run: a new `Store` with fuel and `Limits { memory_bytes: 256 MB, instances: 1, memories: 1, tables: 1, … }`, and a `Linker` with **no host functions**.
- Required exports, checked before any call: `memory` (a memory), and functions `rapidkoi_version() -> i32`, `alloc(i32) -> i32`, `init() -> i64`, `update(i32, i32, i32, i32) -> i64`, `view(i32, i32) -> i64`. Missing or mistyped → error screen naming the export.
- `rapidkoi_version()` must return `1`; anything else → "This app needs a newer version of Rapidkoi" (or, for 0 and negatives, "This app isn't a Rapidkoi app").

**Calling convention (`guest.la`)**
- A guest returns `(ptr, len)` packed in one i64: `ptr = (r >> 32) as u32`, `len = r as u32` (both unsigned).
- To pass bytes, the shell calls `alloc(len) -> ptr`, then `Memory.write(ptr, bytes)`. A write out of bounds is a guest error, not a crash.
- `init()` → state bytes. `update(state_ptr, state_len, event_ptr, event_len)` → update-result bytes. `view(state_ptr, state_len)` → tree bytes.
- Every returned length is checked against its limit (state 16 MB, view 4 MB, update result 16 MB + 4 KB) **before** `Memory.read`, which bounds-checks the range.
- Every call gets a fresh fuel budget (`FUEL_PER_CALL`); out of fuel is its own error ("This app stopped responding").
- Inputs are valid only during a call; outputs are read immediately. The shell never frees guest memory (no `dealloc` in v1).

**Where the guest is called (`shell.la`)**
- `Model` is `Running { guest, state: u8[], tree: Node }` or `Stopped { reason: string, module_bytes }`.
- Startup runs `init` then `view`, decoding the tree into the model.
- The shell's kivuli `update` handles `Msg.Event(msg_bytes, value)`. It encodes the envelope, calls the guest's `update`, decodes `{state, effects}`, calls `view(state)`, decodes the tree, and stores all of it. Any error → `Stopped`. `Msg.Restart` closes the old store, then starts again from the module bytes in a fresh store.
- The shell's kivuli `view` only renders `model.tree` (or the error screen). It never calls the guest.
- **Architecture rule 3 changes** (part of approving this PRP). New wording: "*`view` never does I/O, and `update` does no file, process or network I/O. The one exception is the shell's `update`, which calls the running guest synchronously, each call bounded by fuel (`limits.la`), so a call can't block the window for long.*" Why: the guest is part of the shell's model, and fuel replaces the guarantee that blocking I/O would break.

**Contract v1 (`contract.la`, pure)**
- Tree:
  - A node is a map with `kind` (text) and `props` (map), plus `children` (array) for `column`/`row`. `key` is accepted and ignored in v1.
  - Kinds and props:
    - `text {text, style?: body|title|heading|dim}`
    - `button {label, style?: normal|primary|danger|subtle, on_press?}`
    - `input {value, placeholder?, on_change?}`
    - `column`/`row {spacing?, padding?: none|xs|sm|md|lg|xl}`
  - `on_press`/`on_change` are any CBOR value: kept as their encoded bytes, never interpreted.
  - An unknown prop is ignored. An unknown kind becomes `Node.Unsupported(kind)`; its children aren't read, but they still count toward the limits.
  - A missing required prop, or a known prop of the wrong type or with an unknown word, is `ContractError` with the path (`root > column[2] > button: "label" must be text`).
- Limits, three layers:
  - (a) By length alone, before decoding: view 4 MB, state 16 MB.
  - (b) `cbor.decode` with `cbor.Limits { max_string: 64 KB, max_bytes: 4 MB, max_depth: 2 × 64 + 8, max_items: 10,000 × 16 }`, with the arithmetic in a comment. Each node level is 2 CBOR levels (node map → `children` array); `+ 8` covers the props map and nested message values. 16 items is a generous bound per node (kind, props, a few props, a message).
  - (c) While translating, count nodes and node depth; over 10,000 nodes or depth 64 is `ContractError`.
- Update result: a map with `state` (byte string ≤ 16 MB) and `effects` (array, ignored in v1). Anything else is `ContractError`.
- Event envelope: encode `{ "msg": <message value as decoded>, "value": <text> }`; `value` only for `on_change`.

**Rendering (`render.la`, pure)**
- `text` → `text(s)`: `title` adds `.size(TEXT_TITLE).bold()`, `heading` adds `.size(TEXT_HEADING).bold()`, `dim` adds `.dim()`.
- `button` → `button(label, Msg.Event(msg))` with its `ButtonStyle`; no `on_press` → `.disabled(true)`.
- `input` → `input(value, fun(t) => Msg.Event(msg, Some(t)))`, plus `.placeholder`. No `on_change` → kivuli's read-only or disabled equivalent if one exists; if kivuli has none, record a kivuli gap in DECISIONS.md and stop.
- `column`/`row` → `.spacing`/`.padding` from `theme.la` tokens (`none` = 0).
- `Unsupported(kind)` → `text("Unsupported component: {kind}").dim()`.
- The root gets `.padding(SPACE_LG)` and `.window("Rapidkoi — {file name}", 480, 640)`.
- Error screen: `column([text("This app stopped working").size(TEXT_TITLE).bold(), text(reason).dim(), button("Restart", Msg.Restart).style(Primary)])`. When the module itself can't load, Restart isn't shown.

**Limits (`limits.la`)**
- Constants: `FUEL_PER_CALL`, `MEMORY_BYTES = 256 MB`, `MAX_STATE = 16 MB`, `MAX_VIEW = 4 MB`, `MAX_NODES = 10_000`, `MAX_NODE_DEPTH = 64`, `MAX_STRING = 64 KB`, `MAX_MODULE = 32 MB`.
- Fuel calibration: Lugha's std has no clock, so calibrate offline. Run a `.wat` loop with a fixed fuel budget to exhaustion under the shell's `time`, on this machine. Set `FUEL_PER_CALL` to ≈ 100 ms worth, and record the measurement, the machine and the command in a comment.

**Rust guest (`guest/rust/`)**
- A Cargo workspace with:
  - crate `rapidkoi`: contract v1 for Rust. It has a `View` builder (`text`, `button`, `input`, `column`, `row`, with style, spacing and padding), and an `App` trait with `init`, `update(state, msg, value)` and `view`. A macro exports `rapidkoi_version`, `alloc`, `init`, `update` and `view`. It uses a bump arena that resets on the first `alloc` after an export returns, so outputs survive until the shell has read them.
  - example `counter`: a title, the count, Add, and Reset (disabled at 0), plus a name input with "Hello, {name}".
- CBOR crate: **minicbor**: small, `no_std`, no allocator assumptions, fast. ciborium pulls in serde and is larger in wasm.
- Message encoding in the crate is the app's choice (the counter uses short text messages: `"add"`, `"reset"`, `"name"`).
- Built with `cargo build --release --target wasm32-unknown-unknown`, never by lughac. The command goes in CLAUDE.md COMMANDS.

### Must NOT Do
- No links, manifests, signing, storage, sync, effects (beyond checking `effects` is an array), host functions, multiple apps, file picker, or app themes. Each is a later build-path step.
- Don't change contract v1. Any change is a DECISIONS.md entry and a version bump.
- Don't declare a wasmtime `extern` or touch GTK; don't modify kivuli, lugha-wasmtime or lugha-cbor. A gap there is recorded and fixed upstream.
- Never panic or `unwrap` on anything from the guest or the file.
- Don't call the guest from kivuli's `view`.
- Don't print guest-provided text to the terminal. It may contain escape sequences.

## ERROR HANDLING REQUIREMENTS

- `guest.GuestError`, each becoming `Stopped { reason }` with a plain message:
  - `Unreadable(string)` and `TooLarge`
  - `Compile(string)`
  - `MissingExport(name)` and `WrongExport(name)`
  - `Version(i32)`
  - `Trap(string)`
  - `OutOfFuel`
  - `OutOfBounds`
  - `TooLong(what, len)`
  - `Link(string)`
- `contract.ContractError { path: string, problem: string }`, plus `Decode(cbor.DecodeError)`.
- The shell's `update` turns every `Err` into `Stopped`. Nothing panics on guest input. `panic` only for broken shell invariants, such as an `Event` arriving while `Stopped`. That case is ignored, in fact: the error screen has no event handlers.
- No display at startup: kivuli's/GTK's exit status from `main`, as CLAUDE.md says.

## SECURITY CONSIDERATIONS

- The guest is untrusted:
  - every pointer and length is checked (limits first, then `Memory` bounds)
  - fuel per call; memory, instance, memory and table counts capped through `Store.limit`
  - the CBOR decoder is limited
  - no host functions at all, so the guest can reach nothing
- No new `extern` or `[link]` in rapidkoi. Dependencies are the two owner-approved sibling packages (DECISIONS 010 and 013).
- The file path comes from the command line (the user). It's only read, never executed or written.
- Guest text reaches GTK labels only (no markup: confirm kivuli's `text` doesn't parse Pango markup, otherwise record a gap). It is never printed to the terminal.
- No auth, crypto, payments or secrets.

## TESTS TO WRITE

Pure, no display (`test/contract.la`, `test/render.la`):
- [ ] Each kind decodes, with every prop and style word; `key` is accepted.
- [ ] An unknown prop is ignored. An unknown kind gives `Unsupported`, and its subtree still counts toward the limits.
- [ ] Each contract error gives the right path: a missing `kind`; `props` not a map; a wrong-typed `label`; an unknown style word; `children` on `text` is ignored; non-array `children`.
- [ ] Limits: 10,001 nodes; depth 65; a 64 KB + 1 string; a view over 4 MB by length; `cbor.Limits` hits → `Decode`.
- [ ] Update result: valid; missing `state`; `state` not bytes; `effects` not an array; non-map.
- [ ] Envelope: a button gives `{msg}`, an input gives `{msg, value}`, and a message value round-trips byte for byte.
- [ ] Render (kivuli.testing):
  - texts appear
  - `click` on a button gives `Event(msg)`
  - a button with no `on_press` is disabled
  - `type_into` gives `Event(msg, Some(text))`
  - the unsupported placeholder text is there
  - the error screen has Restart, and not when the module didn't load

Runtime, no display, `.wat` fixtures compiled with `Module.from_text` (`test/guest.la`, `test/shell.la`):
- [ ] `good.wat`, fixed CBOR blobs in data segments: start → tree; an event → the next tree; `update` receives the envelope bytes the shell wrote (the fixture echoes them back inside its state).
- [ ] `version2.wat` → Version(2); `version0.wat` → not a Rapidkoi app.
- [ ] `no_view.wat` → MissingExport("view"); `bad_alloc_type.wat` → WrongExport("alloc").
- [ ] `trap.wat` (`unreachable` in update) → Trap; `spin.wat` (a loop in view) → OutOfFuel.
- [ ] `oob.wat` (returns a ptr past memory) → OutOfBounds; `huge.wat` (returns len 2^31) → TooLong before any read.
- [ ] `bad_cbor.wat` → Decode; `unknown_kind.wat` → placeholder; `nonmap_update.wat` → ContractError.
- [ ] `alloc_oob.wat` (alloc returns a bad ptr) → OutOfBounds on write.
- [ ] Shell: an error gives `Stopped`, and `Restart` gives `Running` again with a fresh store (the fixture's state is back to `init`'s).
- [ ] Unreadable path and over-32 MB bytes → error, no crash (the size check is tested on bytes, not a real 32 MB file).

Rust (`cargo test` in `guest/rust/`, not part of `lughac test`):
- [ ] The view builder encodes the exact CBOR of the meeting note's example; the arena keeps outputs until the next call's first alloc.

## ROLLBACK PLAN
- Branch to return to: `main` at the commit before `prp-001-shell` is merged.
- Work happens on branch `prp-001-shell`; abandoning it means deleting the branch. No migrations, no changes outside this repo.

## ACCEPTANCE CRITERIA
- [ ] `./main counter.wasm` shows the counter. Add, Reset (disabled at 0) and the name greeting work. *(The owner checks this on screen.)*
- [ ] Every fixture in TESTS gives its error screen, or its result, without a crash
- [ ] Architecture rule 3 is reworded in CLAUDE.md as above
- [ ] `FUEL_PER_CALL` is calibrated, with the measurement recorded
- [ ] `lughac test test/` passes; `lughac build src/main.la -o main` succeeds
- [ ] `cargo test` and the release wasm build pass in `guest/rust/`
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 300 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- `cargo test --manifest-path guest/rust/Cargo.toml && cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown`
- For the owner: `./main guest/rust/target/wasm32-unknown-unknown/release/counter.wasm`. Click Add a few times, then Reset, and type a name. Then `./main README.md` (not a module) should show the error screen.
