# rapidkoi-rust request: contract 8, time and name parts (`v0.3.0-pre.1`)

From Rapidkoi, PRP-053 (core decision 30, DECISION-083, DECISION-086). For a session in `../rapidkoi-rust`, under its own rules. Rapidkoi's side is built and tested on its branch `prp-053-time_and_name_parts`; it waits on this tag to move its built-in apps (the new tab, Settings) and merge. Notes and the counter on the VM need rebuilding on it too, or they stop opening once Rapidkoi moves.

## What changed in the contract

Contract 8 is contract 7 with two additions and one removal, and Rapidkoi runs **only** contract 8 (`rapidkoi_version` 8; 7 and older are refused with "This app needs to be rebuilt for this Rapidkoi"). The full text: `docs/specs/contract-8.md` in Rapidkoi.

1. **Texts made of parts.** A `text` node carries `text` (a string, as before) **or** `parts`, never both. `parts` is a list of at most 64, each one of:
   - a string, shown as written;
   - `{ "name": "first" }` or `{ "name": "last" }`: the person's name for this app, filled in by Rapidkoi;
   - `{ "time": ms, "show": "time" | "date" | "date_time" | "relative" | "weekday" }`: a moment (whole milliseconds since 1970-01-01 00:00 UTC, an integer, within years 1 to 9999), drawn by Rapidkoi in the person's zone ("14:32", "Thu 31 Oct", "Thu 31 Oct, 14:32", "5 minutes ago", "Thursday").

   The app never learns what's drawn. Anything else stops the app with its sentence.
2. **The `now` ask.** `{ "kind": "now", "msg" }`, any app, no permission: answered with `on_answer`, `value` the UTC time in milliseconds rounded down to a second.
3. **No more `{FIRST_NAME}` and `{LAST_NAME}`:** braces in a string are only braces.

## What the SDK needs

1. **`__CONTRACT_VERSION` = 8** (`abi.rs`, and its test).
2. **Texts from parts.** Something like:
   ```rust
   text_of(["Hello, ".into(), first_name(), "!".into()])
   text_of(["Edited ".into(), time(note.updated).relative()])
   ```
   - `first_name()`, `last_name()` and `time(ms: i64)` with `.time()`, `.date()`, `.date_time()`, `.relative()`, `.weekday()`, each a `Part`; `&str` and `String` into `Part`.
   - `text_of(parts)` writes `parts`; `text(s)` stays a plain string. At most 64 parts: more is a mistake the SDK can catch in a debug assertion.
   - `time` takes milliseconds since 1970 UTC: the shape `now` answers with and SQLite's `unixepoch() * 1000` gives.
3. **`Ask::now(msg)`**, read back as an `Event::Data` with an integer (or a small `Now::read`).
4. **Docs:** the crate's description and `lib.rs`'s top say contract 8; an example with "Hello, …" and an "Edited … ago" line; that times drawn this way never reach the app.
5. **Tag** `v0.3.0-pre.1` (a breaking change: every app rebuilds).

## Then, in Rapidkoi

`guest/rust` moves to the tag; `./scripts/newtab.sh && ./scripts/settings.sh` repack the built-ins; `./main` is rebuilt; Notes (`../notes`) and the counter are rebuilt on the tag and republished.
