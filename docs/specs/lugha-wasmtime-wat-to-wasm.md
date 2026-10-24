# lugha-wasmtime request: WebAssembly text to bytes

From Rapidkoi, session 216 (DECISION-053, PRP-042). For a lugha-wasmtime session in `../lugha-wasmtime`, under its own CLAUDE.md and PRP process.

## What Rapidkoi needs

Rapidkoi's tests stand in for real apps with hand-written WebAssembly text (`test/support.la`), compiled by `Module.from_text`. Some tests need the module as a **file of bytes**: a signed site is published from a `.wasm` on disk (`./main publish`), and `./main pack` packs one into a bundle. Today those tests use a Rust app built with cargo and skip when it isn't built. PRP-042 moves the Rust apps out of Rapidkoi's repository, so the tests need to write the bytes themselves.

`Module.from_text` already turns text into bytes with `wasmtime_wat2wasm`, then compiles them, but only the compiled module comes out. The extern is in `wasmtime.raw`, which the entry doesn't re-export.

## What to add

A public function that returns those bytes, without compiling:

```
// Suggested API (lugha-wasmtime's to decide)
public fun wat_to_wasm(text: string): Result<u8[], CompileError>
```

- `Err(CompileError.Text(message))` when the text doesn't parse, as `from_text` does today.
- No engine needed: `wasmtime_wat2wasm` takes none.
- `from_text` could then be `Module.new(engine, wat_to_wasm(text)?)`.

## How Rapidkoi will use it

`test/support.la`'s `counting` fixture is written to `test/tmp/counting.wasm` with `write_bytes(path, wat_to_wasm(counting())?)`, then served by `test/saving.la`'s signed sites and packed by `test/counting.la`. Tests only: nothing in `src/` calls it, and no guest can reach it.
