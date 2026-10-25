# Request: lughac's stack frames, and kivuli's `resolve` (a crash) — fixed: lughac PRP-167 (DECISION-059 resolved)

From Rapidkoi, DECISION-059 (2026-10-24). For sessions in `../lugha` and `../kivuli`, each under its own rules.

## What happens

Rapidkoi segfaults drawing Notes 0.3.0 (`./main rapidkoi://notes.dafifi.net`): the main thread's 8 MB stack runs out in kivuli's `view/lazies/resolve`, which walks a screen's tree recursively before each diff. The backtrace is about 50 frames of `resolve` / `resolve_children`; `#0` is at `resolve`'s entry.

## Why

Each `resolve` frame is enormous. Measured in Rapidkoi's `./main` with `objdump` (the `sub …, %rsp` at the function's start):

| Build | `resolve<Msg>` | `resolve_children<Msg>` |
|---|---|---|
| kivuli 0.54, `-O 0` (the default) | 0x43688 = 276 KB | 0x27e8 = 10 KB |
| kivuli 0.58, `-O 0` | 0x482d8 = 289 KB | 0x2828 = 10 KB |
| kivuli 0.58, `-O 2` | 0x1de28 = 122 KB | 0x1d98 = 7.6 KB |

Each level of a screen costs two or three frames (`Styled` → `Column` → `resolve_children`), so about 13 levels of nesting exhaust 8 MB at `-O 0`, and about 30 at `-O 2`. Rapidkoi lets an app nest 64 deep (contract 7), so an app can crash the shell by nesting boxes.

`resolve` is one `match` over `View` with about twenty arms, each building `View` values; `View.Styled` carries a `Style` (with its `Sheet`) by value, and every arm's temporaries seem to get their own stack slots.

## What would fix it

- **lughac (the root):** a function's frame shouldn't grow with every arm's temporaries: share slots between arms of a `match` (they're never live together), and keep large aggregates (a struct of several KB) on the heap or behind one pointer, so a frame stays in the hundreds of bytes. Possibly also a guard page / clear error for stack exhaustion instead of a segfault.
- **kivuli (meanwhile, or as well):** `resolve` and the diff walk trees recursively, by value. Boxing `Style` inside `View.Styled` (or passing views by reference) keeps each frame small; an explicit stack would make depth not matter.

## How Rapidkoi will use it

Nothing changes in Rapidkoi: its screens and apps' screens render as they are, at any depth up to `MAX_NODE_DEPTH` (64). A test in `test/` will draw a 64-deep app screen once a fixed lughac or kivuli is out (rendering needs a display; until then it's checked by hand).
