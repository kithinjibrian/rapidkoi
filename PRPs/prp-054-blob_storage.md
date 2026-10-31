## FEATURE: Blob storage: an app keeps, reads, lists and deletes large bytes beside its database (contract 8)

## OBJECTIVE
An app can store bytes under a key it names, in the same transaction as its rows; read them back whole or in checked pieces; list its keys; and have a blob go when the row that owns it is deleted. The person sees one total for the app's data in ⚙, and Clear and Forget remove the blobs with the database. Showing a blob as a picture and sending one as a request body are PRP-055, on the same contract.

## CONTEXT
- **Spec:** `docs/specs/blobs.md` (DECISION-087, resolved: the owner approved it whole, its defaults and its security review). This PRP builds its part 1 (1.1–1.5, 1.8, 1.9) and part 2; 1.6 and 1.7 are PRP-055.
- **Decisions:** DECISION-087, DECISION-046/052 (storage), DECISION-045 (an app keeps nothing between calls), DECISION-056 (one contract), DECISION-071 (synced ids). None open.
- **Built on:** lugha-sqlite `v0.2.0` (commit `5ae3d34`): `Guard.reserved`, `Step.Untrusted`/`Step.Own`, `batch_steps`, `run_own`, `query_own`. Its answer to the request lists four differences, handled below (design 9–12).
- **Files to read first:** `src/data/databases.la` (270 lines: `call`, `write`, `size_of`, `clear`), `src/data/queries.la` (asks in tasks, `MAX_IN_FLIGHT`), `src/guest/sqlasks.la`, `src/guest/contract.la` (effects dispatch), `src/guest/nodes.la` (`Effect`, `Query`), `src/guest/effects.la` (484 lines), `src/guest/limits.la`, `src/shell.la` (Clear and the size at 440–455), `test/databases.la`, `test/queries.la`, `test/sqlasks.la`.
- **Creates:** `src/data/blobs.la` (files: path, hash, write, read a range, sweep, remove), `src/guest/blobasks.la` (reading `blob_set`/`blob_delete` steps, `blob_get`, `blob_list`), `test/blobs.la`, `test/blobasks.la`.
- **Changes:** `src/guest/limits.la`, `src/guest/nodes.la`, `src/guest/contract.la`, `src/guest/sqlasks.la`, `src/data/databases.la`, `src/data/queries.la`, `src/guest/effects.la` (split, below), `src/shell.la`, `lugha.toml` (the lugha-sqlite tag), `docs/specs/contract-7.md` → `contract-8.md`, `guest/rust/*` (the SDK tag), `assets/apps/*.rkb` (repacked), CHANGELOG.md.
- **Relied on:** `lughac spec`: `std.crypto.sha256`, `std.fs` (`rename`, `sync`, `list_dir`, `remove_file`, `remove_dir`, `make_dirs`, `exists`), `read_bytes`/`write_bytes`. lugha-sqlite `v0.2.0`: `Guard.reserved`, `Step`, `batch_steps`, `run_own`, `query_own`, `Kind.Denied`.

## REQUIREMENTS

### Must Do
- **Contract 8** (`CONTRACT_VERSION` 8; 7 refused with the rebuild sentence, as today). The spec moves to `docs/specs/contract-8.md` with blobs' asks, the reserved `rapidkoi_` names, the sentences, and the notes: read after the write's answer; handle pieces as they arrive.
- **`sql_batch` steps** may be `{blob_set, bytes, owner?}` or `{blob_delete}` beside `{sql, params}`. A key is 1–256 bytes of UTF-8 without control characters; bytes at most 16 MB; an owner is `{table, key}`.
- **Writing** (in the task): check limits; SHA-256; write `databases/<app>.blobs/<hash>` unless present (temporary file, `fs.sync`, `fs.rename`, `fs.sync` of the folder); then one transaction: `Own` steps on `rapidkoi_blobs` and the app's steps in order; commit; remove files no key names any more.
- **Owners:** the table must exist with a single-column primary key, both names read back with `PRAGMA table_info` and matching `[A-Za-z_][A-Za-z0-9_]*`. As the last `Own` step of every `sql_batch`, in its transaction: for each table `rapidkoi_blobs` names, delete the mappings whose owner key no longer exists (`NOT EXISTS (SELECT 1 FROM main.<t> WHERE <pk> = owner_key)`), and every mapping of a table that's gone (spec 1.4, amended).
- **`blob_get {msg, key, from?, len?, hash?}`** → `on_answer {bytes, from, size, hash}`; `len` capped at 4 MB; a `hash` that no longer matches fails with "This blob changed while it was being read".
- **`blob_list {msg, prefix?}`** → `on_answer {blobs: [{key, size, owner?}], cut}`, key order, at most 10,000.
- **Limits** (part 2): 16 MB a blob, 1 GB an app's blobs, 10,000 blobs; a batch fails whole, with "This blob is over 16 MB", "This app's files are full (1 GB)", "This app has 10,000 blobs already".
- **The sweep:** once per app per run, before its first write: files no key names, and leftover temporary files, are removed.
- **⚙'s Data row:** the database file plus `SELECT sum(size) FROM rapidkoi_blobs`, one total. **Clear and Forget** remove every file in the blobs folder, then the folder, with the database.
- **Other windows' `on_change`** lists the app's changed tables, never a `rapidkoi_` one.
- **rapidkoi-rust**, tagged first (a request in `docs/specs/rapidkoi-rust-contract-8.md`): version 8, the steps and asks, `Event` variants for the answers, a helper writing a row and its blob in one batch, and `blob_lines(key)`.

### Must NOT Do
- `blob:` pictures and `body_blob` (PRP-055); a fetch into a blob, the picker, write sessions, sync (spec part 3).
- Any blob for built-in apps (they have no database).
- Read the app's rows: Rapidkoi reads only `rapidkoi_blobs`, the owner tables' schema, and whether an owner key exists (inside SQLite, never returned).
- Work around a missing lugha-sqlite or lughac feature: recorded and stopped.
- A new screen or new words in ⚙ beyond the existing Data row's number.

## DESIGN

**Decided in discovery (Claude; the owner reviews them here):**
1. **Two PRPs, one contract.** This one is storage; PRP-055 is showing and sending. Both before the SDK's release, as contract 7's three PRPs were (`pre` tags between).
2. **The owner check runs in every write's transaction.** `SELECT DISTINCT owner_table FROM rapidkoi_blobs` (an index on `owner_table`), then one `Own` delete per table, appended after the app's steps. Since `batch_steps` takes its steps up front, the owner tables are read with `query_own` on the same connection just before; a batch that names a new owner adds its table. No triggers, so nothing to remake per connection.
3. **An app's writes run one at a time.** Today an app's asks run side by side (up to `MAX_IN_FLIGHT`). Removing unnamed files after a commit, and the sweep, would race another write that just placed the same file and hasn't committed. So each app gets a write lane: `sql_batch` calls queue behind the one running (in the model, per app); reads (`sql`, `blob_get`, `blob_list`) still run beside them. A queued write still counts against `MAX_IN_FLIGHT`. This also ends `SQLITE_BUSY` between an app's own writes.
4. **A read racing a write:** a `blob_get` whose file vanishes between lookup and read answers "This blob changed while it was being read", the same as a hash mismatch.
5. **Limits in `limits.la`:** `MAX_BLOB` 16 MB, `BLOBS_BYTES` 1 GB, `MAX_BLOBS` 10,000, `MAX_BLOB_PIECE` 4 MB, `MAX_KEY` 256. `MAX_UPDATE` grows by one `MAX_BLOB` (a result may carry one blob step), so a guest's whole result is at most about 33 MB, within its 256 MB memory.
6. **Errors:** `src/data/blobs.la` has its own enum, `BlobError { TooBig, Full, TooMany, NoSuchKey, Changed, BadOwner(string), Disk(string) }`, each mapped to one sentence for the app; a disk error's system message is never shown to the app ("This app's files couldn't be written").
7. **`effects.la` split** (484 lines; the two new asks take it past 500): its storage arms (`Sql`, `SqlBatch`, and the new `BlobGet`, `BlobList`) move into `data.queries` as one function `effects` calls, with their tests moving to `test/queries.la`. The owner approves the split with this PRP.
8. **The tag:** `lugha.toml` moves lugha-sqlite to `v0.2.0`; `lughac update` rewrites `lugha.lock` (never edited by hand).
9. **`changes` is the app's steps' sum.** Every write goes through `batch_steps`, whose `changes` sums the untrusted steps only (Rapidkoi's own steps aren't counted). Contract 7 answered `batch`'s v0.1.0 count, which was the last step's alone (a lugha-sqlite bug, fixed in v0.2.0); contract 8 says `changes` is the sum over the app's steps. `last_rowid` is unaffected by Rapidkoi's own inserts (`rapidkoi_blobs` is `WITHOUT ROWID`): tested.
10. **Rapidkoi's objects are made, and checked, before any app SQL.** SQLite never shows the authorizer the new name in `ALTER TABLE x RENAME TO rapidkoi_y`, so an app could rename its own table into the prefix. Each call, first, with `run_own`: if `rapidkoi_blobs` exists, its columns must match (`PRAGMA table_info`); if they don't, every blob ask and blob step fails with "This app's database has a table Rapidkoi uses for its own", and nothing is changed. 
11. **Views are reserved too** (lugha-sqlite's addition): nothing for Rapidkoi to do; the contract lists tables, indexes, triggers and views named `rapidkoi_` as Rapidkoi's.
12. **No triggers** (the owner: "go with B"): lugha-sqlite refuses dropping a table that carries a reserved trigger, which would block apps' migrations, so owners are kept by the check in 2 instead. Changing a row's primary key drops its blobs; the contract says so.

**Security** (approved with the spec; restated for the build): every key, length, offset, owner and step is checked before a task starts, and a bad one stops the app with its sentence; nothing the app sends becomes a path (hashes are Rapidkoi's); the owner check's SQL only from names read back from the schema and the allowlist; `rapidkoi_` is refused to the app's SQL by lugha-sqlite's `Guard.reserved`; Rapidkoi's own statements go through `Own`/`run_own` with every other guard on.

## TESTS
- [ ] `test/blobasks.la`: each step and ask read whole; each malformed one (key empty, 257 bytes, a control character; bytes over 16 MB; an owner without a table or key; `from` or `len` negative; a `blob_get` without a key) stops with its sentence.
- [ ] `test/blobs.la` (files, in a temporary folder): a write lands under its hash; the same bytes twice are one file; a range read; a read past the end; a sweep removes unnamed files and temporary files and keeps named ones.
- [ ] `test/databases.la`: a batch with a `blob_set` and an `INSERT` commits both; a failing step keeps neither, and the file is swept later; `blob_delete` removes the key and, with no other key on the hash, the file; deleting an owner row removes its blob in the same transaction; a migration in one batch (new table, copy, drop, rename) keeps its blobs; dropping an owner table removes its blobs; changing a row's primary key drops its blob; an owner table without a single-column primary key fails the batch; the app's own `SELECT * FROM rapidkoi_blobs` and `DROP TABLE rapidkoi_blobs` are refused; each limit's sentence; `blob_get` with a stale `hash`; `blob_list` with a prefix and with `cut`; the size includes blobs; Clear removes the folder.
- [ ] `test/queries.la`: two batches from one app run one after the other; a read runs beside a running write; other windows' `on_change` never names `rapidkoi_blobs`.
- [ ] `test/databases.la`: `changes` sums the app's steps and leaves out Rapidkoi's; `last_rowid` is the app's last insert with a `blob_set` after it; a table renamed into `rapidkoi_blobs` with another shape fails blob asks with its sentence and keeps the app's data; a view `rapidkoi_v` over it is refused.
- [ ] Contract: a contract-7 stand-in is refused; `test/support.la`'s stand-ins move to 8.

## ACCEPTANCE
- [ ] Every test above passes; no file over 500 lines.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] The built-ins (new tab, Settings) rebuilt on the SDK's contract-8 tag, repacked, and open.
- [ ] For the owner, by eye: ⚙'s Data row for an app with blobs shows the total; Clear empties it. (Notes attaching a photo proves the pair after PRP-055.)
