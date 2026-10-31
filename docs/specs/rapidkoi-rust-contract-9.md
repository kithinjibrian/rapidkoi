# rapidkoi-rust request: contract 9, blob storage (`v0.3.0-pre.2`)

From Rapidkoi, PRP-054 (DECISION-087, `docs/specs/blobs.md`). For a session in `../rapidkoi-rust`, under its own rules, after `v0.3.0-pre.1`. Rapidkoi's side is built on its branch `prp-054`; it waits on this tag to move its built-in apps and merge. Notes and the counter on the VM need rebuilding on it too, or they stop opening once Rapidkoi moves.

## What changed in the contract

Contract 9 is contract 8 with blob storage, and Rapidkoi runs **only** contract 9 (`rapidkoi_version` 9; 8 and older are refused with "This app needs to be rebuilt for this Rapidkoi"). The full text: `docs/specs/contract-9.md` in Rapidkoi. Showing a blob as a picture and sending one as a request body come in PRP-055, a later `pre` tag on the same contract.

1. **Blob steps in `sql_batch`.** A step is `{ "sql", "params" }` as before, or:
   - `{ "blob_set": key, "bytes": <bytes>, "owner"?: { "table": text, "key": value } }`: store or replace a key's bytes; with an owner (a table that exists before the batch, with a single-column primary key), deleting that row deletes the blob in the same write, and changing its primary key drops it;
   - `{ "blob_delete": key }`: remove a key (no such key: nothing happens).

   A key is text, 1–256 bytes of UTF-8, no control characters. Bytes at most 16 MB. The batch answers as before, `{ changes, last_rowid }`, and `changes` is now the **sum over the app's own steps** (it was the last step's alone).
2. **`blob_get`** `{ "kind": "blob_get", "msg", "key", "from"?, "len"?, "hash"? }`: answered `on_answer` with `{ "bytes", "from", "size", "hash" }`: at most 4 MB from `from` (default 0). Pass the first answer's `hash` with later pieces: if the blob changed in between, `on_fail` "This blob changed while it was being read".
3. **`blob_list`** `{ "kind": "blob_list", "msg", "prefix"? }`: answered `on_answer` with `{ "blobs": [{ "key", "size", "owner"? }], "cut" }`, in key order, at most 10,000.
4. **Reserved names:** tables, indexes, triggers and views named `rapidkoi_…` are Rapidkoi's; the app's SQL is refused them.
5. **Limits:** 16 MB a blob, 1 GB an app's blobs, 10,000 blobs; a batch over one fails whole with its sentence.

## What the SDK needs

1. **`__CONTRACT_VERSION` = 9** (`abi.rs`, and its test).
2. **Steps:** `Ask::sql_batch` keeps taking `(String, Vec<Value>)` statements; a new form takes steps, something like:
   ```rust
   Ask::sql_batch_steps("saved", vec![
       Step::blob_set("photo-17", bytes).owner("notes", id),
       Step::sql("INSERT INTO notes (id, photo) VALUES (?, ?)", params![id, "photo-17"]),
   ])
   Step::blob_delete("photo-17")
   ```
3. **Reads:** `Ask::blob_get(msg, key)` with `.from(n)`, `.len(n)`, `.hash(h)`; `Ask::blob_list(msg)` with `.prefix(p)`. Their answers read back as `Event` variants (`Event::Blob { msg, bytes, from, size, hash }`, `Event::Blobs { msg, blobs, cut }`), the way `Rows` and `Done` are today.
4. **Helpers:**
   - writing a row and its blob in one batch (the shape above, named for the common case);
   - **`blob_lines(key)`**: reads a blob piece by piece, carries an unfinished last line into the next piece, passes the hash back, and delivers `Event::Lines { msg, lines, done }`, so an app reads JSONL, CSV or a log of any size without holding it whole. On a changed blob it fails, and the app starts over. Its state between pieces lives in the app's state (an app keeps nothing between calls).
5. **Docs:** contract 9 in the crate's description and `lib.rs`; read after the write's answer (asks run side by side); handle pieces as they arrive; changing a row's primary key drops its owned blobs; never name anything `rapidkoi_`.
6. **Tests:** each step and ask encodes as above; the answers decode; `blob_lines` across pieces that split a line, at the end without a trailing newline, and on a changed hash.
7. **Tag** `v0.3.0-pre.2` (a breaking change: every app rebuilds), asking the owner before pushing it.

## Then, in Rapidkoi

`guest/rust` moves to the tag; `./scripts/newtab.sh && ./scripts/settings.sh` repack the built-ins; `./main` is rebuilt; PRP-054 merges. Notes (`../notes`) and the counter are rebuilt on the tag and republished.
