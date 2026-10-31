# rapidkoi-rust request: single files (`v0.3.0-pre.4`)

From Rapidkoi, DECISION-089 (`docs/specs/files.md`, milestone M2). For a session in `../rapidkoi-rust`, under its own rules, after `v0.3.0-pre.3`. Rapidkoi's side is PRP-056 (milestone M3), built once kivuli has its file dialogs (`docs/specs/kivuli-file-dialogs.md`); this tag can come before it, since it only encodes asks and decodes answers.

## What changes in the contract

Additions to contract 9, nothing removed: `rapidkoi_version` stays 9 and apps on `pre.3` keep working. The full text will be in Rapidkoi's `docs/specs/contract-9.md` once PRP-056 lands; until then, `docs/specs/files.md` Part 2 is the source.

The person chooses every file in Rapidkoi's own dialog, and choosing is the permission. The app never names a file, never sees a path, and learns only a file's **name** (its last part, `receipt.pdf`) and **size**.

1. **`file_open`** `{ "kind": "file_open", "msg", "key", "accept"?, "edit"? }`
   - Opens Rapidkoi's chooser ("Open a file for {app}", or with `edit: true` "Open a file for {app} to modify").
   - The chosen file is stored as a blob under `key` (replacing it, as `blob_set` does), with no owner.
   - `accept`: up to 16 extensions without the dot (`["png", "jpg"]`), only filtering what the chooser shows.
   - Answered `on_answer` with `{ "key", "name", "size" }`, plus `"handle"` (text) when `edit` was true; or with `null` when the person cancelled (`value` absent or null: not an error).
2. **`file_save`** `{ "kind": "file_save", "msg", "handle", "key" }`
   - Writes blob `key`'s bytes over the file `handle` names, safely.
   - Answered `on_answer` `{ "size" }`, or `on_fail` (sentences below), and nothing is written on a failure.
3. **`file_save_as`** `{ "kind": "file_save_as", "msg", "key", "name"?, "accept"? }`
   - Opens Rapidkoi's save dialog ("Save a file from {app}"), `name` suggested.
   - Answered `on_answer` `{ "name", "size", "handle" }`, or `null` when cancelled.
4. **The `blob_own` step** in `sql_batch`: `{ "blob_own": key, "owner": { "table", "key" } }` gives an ownerless blob (one from `file_open`) an owner row, under blobs' owner rules (the table must exist before the batch, with a primary key of one column; deleting the row deletes the blob; changing its key drops it).
5. **Handles** are opaque text, valid only in the window that got them, until it closes or reloads. Keep them in the window's state; they mean nothing elsewhere and never go in a link.
6. **Failures** (`on_fail`, the sentence as `error`):
   - "A file dialog is already open"
   - "This file is over 16 MB"
   - "This app's files are full (1 GB)"
   - "This file changed since it was opened" (`file_save`: changed on disk since it was read or last saved; nothing written; offer to open it again, or save as)
   - "This file is no longer open here" (a handle from a closed or reloaded window)
   - "Rapidkoi couldn't read this file" / "Rapidkoi couldn't save this file"
   - "This app has no blob with that key"
   - "This app keeps no files here" (the new tab, built-in apps)
7. A malformed ask stops the app, as any bad ask does: a key as blob keys are (1–256 bytes, no control characters), `accept` more than 16 or not text, `handle` not text, both missing.

## What the SDK needs

1. **The asks**, following `Ask::blob_get`'s builder style; for example:
   ```rust
   Ask::file_open("attached", "photo-3").accept(["png", "jpg", "jpeg"])
   Ask::file_open("opened", "doc").edit()
   Ask::file_save("saved", &handle, "doc")
   Ask::file_save_as("exported", "doc").name("notes.txt")
   ```
2. **The step:** `Step::blob_own(key).owner(table, key)` (or `Step::blob_own(key, table, key)`: the SDK's choice), beside `Step::blob_set` and `Step::blob_delete`.
3. **The answers as events**, the way `Event::Blob` and `Event::Blobs` are:
   - `Event::FileOpened { msg, key, name, size, handle: Option<String> }`
   - `Event::FileSaved { msg, size }` and `Event::FileSavedAs { msg, name, size, handle }` (or one variant: the SDK's choice)
   - `Event::FileCancelled { msg }` for `null`: the SDK knows which `msg`s were file asks, as it already tracks `blob_lines`' readers, so a cancel never arrives as a bare `Answer`.
   - Failures stay `Event::Failed { msg, error }`.
4. **A helper for the common case:** open a picture and attach it to a row, e.g. a `FileOpened` followed by a batch with `Step::blob_own`, documented as the pattern Notes will use.
5. **Docs:** the contract's rules above in the crate's docs: choosing is the permission; names, never paths; handles last while the window is open and never go in links; `file_save` never overwrites another program's change; cancelling isn't an error; 16 MB a file.
6. **An example**, `examples/editor`: open a text file to modify, edit it in a typing box, save back, save as, and show "This file changed since it was opened". It proves the editing path end to end once Rapidkoi has PRP-056 (milestone M6).
7. **Tests:** each ask and the step encode as above; `accept` and `edit` only when given; each answer decodes to its event; a `null` answer to a file ask is `FileCancelled`, and to any other ask stays what it is today.
8. **Tag** `v0.3.0-pre.4`, asking the owner before pushing it.

## Then, in Rapidkoi

PRP-056 builds the shell's side on kivuli's dialogs; `guest/rust` moves to this tag and the built-ins are repacked (they use none of it); Notes attaches a photo (M5) and the editor example proves saving back (M6).
