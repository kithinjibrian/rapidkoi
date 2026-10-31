## FEATURE: Single files: an app opens one of the person's files (a copy, or to modify), saves it back, and saves as (contract 9)

**Status:** draft; waits on lughac's `fs.size` (the owner chose (b): "wait for file size query"; `docs/specs/lugha-fs-size.md`).

## OBJECTIVE
An app asks for a file; Rapidkoi's own chooser opens, titled with the app's name; the person picks one, or cancels. The app gets the file's bytes as a blob, its name and size, and, when it asked to modify it, a handle to save back to that one file. Saving back is safe and never overwrites a change made elsewhere. Save as writes a new file the person chooses. The app never sees a path. With it, Notes can attach a photo (M5) and an editor can open, change and save a file (M6).

## CONTEXT
- **Spec:** `docs/specs/files.md` Part 2 (DECISION-089, resolved: the owner approved it, its words and its security review). This PRP builds 2.1–2.9.
- **Decisions:** DECISION-089, DECISION-087 (blobs: the bytes land there, owners), DECISION-015 (no file-size query in lughac: open question below), DECISION-056 (one contract). None open.
- **Built on:** kivuli `v0.66.0` (`Cmd.open_file`, `Cmd.save_file`, `FileFilter`; `kivuli.testing`'s `chooser`, `answer_chooser`; a second chooser while one is open answers `None`); rapidkoi-rust `v0.3.0-pre.4` (the asks and events; the built-ins are already on it).
- **Files to read first:** `src/guest/contract.la`, `src/guest/blobasks.la`, `src/guest/nodes.la`, `src/guest/effects.la` (497 lines), `src/data/databases.la` (458), `src/data/blobs.la`, `src/data/blobmap.la`, `src/data/queries.la`, `src/window/window.la` (497), `src/window/actions.la`, `src/shell.la` (480).
- **Creates:** `src/guest/fileasks.la` (reading the asks), `src/window/files.la` (the open dialog and the handles, per window), `src/data/files.la` (reading and writing the person's file, in tasks), `src/guest/storing.la` (the storage and file arms moved out of `effects.la`); tests `test/fileasks.la`, `test/files.la`, `test/storing.la`.
- **Changes:** `nodes.la` (`Effect.FileOpen`, `FileSave`, `FileSaveAs`; `BatchStep.BlobOwn`), `contract.la`, `sqlasks.la`/`blobasks.la` (`blob_own`), `databases.la` (`blob_own` in `write`), `effects.la` (split, below), `window.la` (one field), `actions.la` (`FileChosen`, `FileDone`), `shell.la` (two lines), `docs/specs/contract-9.md`, CHANGELOG.md.

## REQUIREMENTS

### Must Do
- **`file_open {msg, key, accept?, edit?}`**: Rapidkoi's chooser (`Cmd.open_file`) titled "Open a file for {app}", or "Open a file for {app} to modify" with `edit`; `accept`'s extensions become one filter (`*.png`…). Cancelled: `on_answer` `null`. Chosen: in a task, the file is read, checked (at most 16 MB, the app's blob quota and count) and stored as an ownerless blob under `key`, the same path `blob_set` takes; answered `{key, name, size}`, plus `handle` with `edit`.
- **`file_save {msg, handle, key}`**: in a task, blob `key`'s bytes written over the handle's file: a temporary file beside it, `fs.sync`, `fs.rename`, `fs.sync` of the folder; refused with "This file changed since it was opened" when the file's SHA-256 differs from the one kept when it was read or last saved. Answered `{size}`; the kept hash moves on.
- **`file_save_as {msg, key, name?, accept?}`**: Rapidkoi's save dialog (`Cmd.save_file`) titled "Save a file from {app}", `name` suggested; the blob written safely to the chosen path; answered `{name, size, handle}`, or `null` when cancelled.
- **`blob_own {key, owner}`** as an `sql_batch` step: sets an existing blob's owner under blobs' owner rules (the table exists before the batch, one-column key); a missing key fails the batch with "This app has no blob with that key".
- **Handles** are opaque (`h` and a counter), kept in the model per tab with the path and the hash, never in the app's state or on disk. They end when the tab closes or loads again; a stale one answers "This file is no longer open here".
- **One chooser at a time per window**: an ask while one is open answers `on_fail` "A file dialog is already open" (kivuli would answer `None`; Rapidkoi says why).
- **Names, never paths**: the app gets the path's last part only.
- **The sentences** of `docs/specs/files.md` 2.8 and `docs/specs/rapidkoi-rust-files.md` 6; the system's message never reaches the app.
- **Contract 9's text** gains the asks and the step (an addition, as the owner decided).

### Must NOT Do
- Several files, folders, change notices, remembered grants, "Open with…" (spec Part 3).
- Read or write any file but the one the person chose, or tell the app where it is.
- Read or write a file in `update`: always a task (opening a file the person chose is already one of CLAUDE.md's exceptions for apps, not for this).
- An Open… item for local apps in ☰ (DECISION-020's use of the same dialog): its own small PRP.

## DESIGN

**Decided in discovery (Claude; the owner reviews them here):**
1. **The flow.** The ask is checked in `update`; if no chooser is open, the window records it (`window/files.la`: the tab, its load, `msg`, which ask) and returns kivuli's command, mapped to `Msg.FileChosen(Option<string>)`. On the answer: `None` answers the app `null`; `Some(path)` starts a task (`data/files.la`) whose result comes back as `Msg.FileDone(tab, load, msg, Result)`, answered to the app (or dropped for a closed tab or another load, as every task's result is).
2. **The bytes go through blobs' own write path** (`databases.call` with a `BlobSet` step), so the quota, count, file placement, sweep and the `rapidkoi_blobs` signal (pictures refresh) all apply unchanged.
3. **The model:** one field on `Model`, `files: Files` (`window/files.la`: the chooser waiting, if any, and the handles), so `window.la` stays at 499 lines. `AppPage` (`tabs.la`, 500 lines) is untouched.
4. **`effects.la` split** (497 lines; three new arms take it past 500): the storage and file arms (`Sql`, `SqlBatch`, `BlobGet`, `BlobList`, and the three file asks) move to `guest/storing.la`, one function `effects` calls. The owner approves the split with this PRP.
5. **Conflict check by hash**, not by modification time: lughac has no file metadata (DECISION-015), and a hash can't be fooled by a clock.

**The open question (the owner's):** lughac has no file-size query (DECISION-015, deferred: "acceptable: the file is local and chosen by the person"). So the 16 MB cap is checked **after** the file is read, and a huge file chosen by mistake (a 4 GB video) is read whole before it's refused, holding memory and, as any blocking read does in Lugha today, the window, for as long as that takes.
- (a) Build now under DECISION-015's precedent, checking after reading until the query ships.
- **(b) Wait** for lughac's `fs.size` before building. **Chosen by the owner.** So `data/files.la` checks `fs.size(path)` before `read_bytes`, and a file over 16 MB is refused without a byte read.

**Security** (approved with the spec; restated): every ask's shape checked before anything opens; the chooser is the only way a path enters, and it never leaves the model; writes go only to the handle's own path, atomically, after the conflict check; a handle dies with its tab; names only reach the app; the system's messages never do.

## TESTS
- [ ] `test/fileasks.la`: each ask and `blob_own` read whole; each malformed one (a bad key, more than 16 extensions, a handle that isn't text) stops with its sentence.
- [ ] `test/files.la` (files under `/tmp`): a file read into a blob; one over 16 MB refused; save back writes atomically; a file changed on disk is refused and left as it was; save as writes a new file.
- [ ] `test/storing.la`, through `update` and `kivuli.testing`: `file_open` opens a chooser titled "Open a file for Notes" (and "… to modify" with `edit`) with the app's filter; `answer_chooser(None)` answers `null`; a chosen path lands as a blob with its name and size and, with `edit`, a handle; a second ask while one is open fails; save back through the handle; a handle after the tab reloaded fails; save as gives a handle; `blob_own` gives a picked blob an owner, and deleting the row removes it.
- [ ] The moved storage tests still pass from `test/storing.la` / `test/queries.la`.

## ACCEPTANCE
- [ ] Every test above passes; no file over 500 lines.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye, with the SDK's `examples/editor` or Notes (M5, M6): the chooser shows the app's name and the right words; cancelling does nothing; a file opens, is modified and saved back; editing it elsewhere first makes the save say so; save as writes a new file.
