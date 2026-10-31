# Blobs: an app's large bytes beside its database (draft)

DECISION-087, open. A draft for the owner's one review: every answer is filled in. **Agreed** marks what the owner has said; everything else is Claude's recommendation and stands unless the owner objects. Nothing here is built, and building it waits on an approved PRP.

The gap (the owner: "gap noticed in the platform"): an app can keep bytes in a `BLOB` column, 4 MB a value, but it can't show a picture it stored (pictures come only from the bundle or a URL, DECISION-056 Q1); large values crowd SQLite's shared memory and the 1 s call limit; and once sync logs changes with the session extension (DECISION-070), every edit of a large value is copied whole into the log's 100 MB. No app is waiting on it.

The spec is in three parts, by what a choice costs to change later:

1. **Settled now:** what goes into the contract and into sync's design.
2. **Defaults:** numbers and screens, changed any time without a contract change.
3. **Designed, not built:** how later pieces fit, so part 1 doesn't block them.

---

## Part 1 — Settled now

### 1.1 Where the bytes live (agreed)

- Each blob is a file named by the SHA-256 of its bytes, in lowercase hex, which Rapidkoi computes. Nothing the app sends becomes part of a path.
- The files sit beside the app's database: `databases/<sha256 hex>.blobs/<hash>`, next to `databases/<sha256 hex>.db`. One folder per app, flat.
- The same bytes under two keys are one file.
- Small values can still go in `BLOB` columns (SQLite reads them faster below about 100 KB, [sqlite.org/intern-v-extern-blob](https://sqlite.org/intern-v-extern-blob.html)); a blob is for bytes the app treats as a whole.

### 1.2 What a blob is (agreed)

- Any bytes, up to the limit. Rapidkoi never records a type, and never takes one from the app.
- Rapidkoi looks at the bytes only to show a picture (1.6), by their header.
- A blob is never run, never handed to the system (no opening in another program, no saving to the person's files, no path), and never shared with another app.
- The person's own files, shared between apps, are DECISION-078's shared data, not blobs. Secrets wait on their own decision.

### 1.3 Keys, committed with the rows (agreed)

The app names its blobs with keys. Rapidkoi keeps which key holds which bytes in a reserved table in the app's own database:

```sql
CREATE TABLE rapidkoi_blobs (
  key         TEXT PRIMARY KEY,
  hash        BLOB NOT NULL,   -- 32 bytes
  size        INTEGER NOT NULL,
  owner_table TEXT,            -- null: no owner (1.4)
  owner_key                    -- the owner row's primary key, any SQLite value
) WITHOUT ROWID;
```

- Made at the app's first `blob_set`; an app that never uses blobs never has it.
- Only Rapidkoi touches it. The app's SQL is refused any statement naming it (dependency 1, below). The app sees it exists (`sqlite_master`); the contract tells apps to skip names starting `rapidkoi_`.
- About 100 bytes a row, inside the database's 100 MB. The app's `user_version` and migrations are its own, untouched.
- A key is text, 1–256 bytes of UTF-8, no control characters. It's a name, never a path.

**Writing.** `blob_set` and `blob_delete` are steps of `sql_batch`, beside the app's own:

```
sql_batch [
  { blob_set: "photo-17", bytes, owner: { table: "notes", key: "n-8f3a…" } },
  { sql: "INSERT INTO notes (id, body, photo) VALUES (?, ?, ?)", params: ["n-8f3a…", "…", "photo-17"] }
]
```

Rapidkoi, for each `blob_set`, before the transaction:

1. checks the bytes against the limits (part 2);
2. hashes them;
3. writes the file unless one with that hash is there: a temporary file, `fs.sync`, `fs.rename`, `fs.sync` of the folder.

Then one transaction: Rapidkoi's own rows in `rapidkoi_blobs` (insert, replace or delete) and the app's steps, in the order given. Commit; then any file no key names any more is removed.

**A crash or a failed step** leaves at most a file no key names. On opening the app's database, Rapidkoi lists the folder and removes every file whose hash no key names, and every leftover temporary file. A row never names a key whose bytes are missing.

`sql_batch`'s answer is unchanged: `{changes, last_rowid}`, counting the app's own steps.

### 1.4 Owners (agreed)

A blob may belong to a row: `owner: {table, key}`, the table's name and the row's primary key. **Deleting the row deletes its blobs, in the same transaction.**

- The primary key, never the rowid: `VACUUM` can renumber rowids, and synced rows are known by their own ids (DECISION-071).
- The table must exist and have a single-column primary key; anything else fails the batch (`on_fail`).
- How: for each table that owns blobs, Rapidkoi makes a `TEMP` trigger on its own connection:
  ```sql
  CREATE TEMP TRIGGER rapidkoi_owner_notes AFTER DELETE ON main.notes BEGIN
    DELETE FROM rapidkoi_blobs WHERE owner_table = 'notes' AND owner_key = old.id;
  END;
  ```
  and one `AFTER UPDATE OF id` that moves `owner_key` when the key itself changes. A `TEMP` trigger lives in the connection, never in the app's file or its `sqlite_master`. Rapidkoi makes them on open for every table named in `owner_table`, and checks they're there after each batch (an app dropping and remaking a table drops them with it).
- The trigger's SQL names the app's table and column, so it's built from text the app chose: **only names read back from the app's own schema (`PRAGMA table_info`) and matching `[A-Za-z_][A-Za-z0-9_]*` are used**, quoted as identifiers. Anything else fails the batch. This is for the owner's security review.
- Owners cover deleting the row. Changing a row's reference (`photo-17` to `photo-18`) without deleting it needs `blob_set` or `blob_delete` on the key. A blob two rows share is two keys, one file.
- `owner` is optional: a blob with none lasts until `blob_delete`, Clear or Forget (a profile picture, a setting).

### 1.5 The asks

| Kind | What | Answered |
|---|---|---|
| `sql_batch` step `{ blob_set, bytes, owner? }` | store or replace a key's bytes | the batch's `on_answer` / `on_fail` |
| `sql_batch` step `{ blob_delete }` | remove a key (no such key: nothing happens) | the batch's |
| `blob_get` `{ msg, key, from?, len?, hash? }` | read a key's bytes, whole or a range | `on_answer {bytes, from, size, hash}` or `on_fail` |
| `blob_list` `{ msg, prefix? }` | the app's keys | `on_answer {blobs: [{key, size, owner?}], cut}` or `on_fail` |

- **Where bytes come from (agreed: A and D now):** what the app sends (A), and nothing else yet; a fetch saving straight into a blob (B) and the file picker (C) are part 3.
- **Reading (agreed: ranged):** `from` defaults to 0, `len` to the rest; an answer is at most 4 MB, like an SQL answer, so a larger blob is read in pieces. `size` is the whole blob's.
- **Reading in pieces is checked (agreed):** every answer carries the blob's `hash`. A later piece passes it back (`{key, from, hash}`); if the key now holds other bytes, the answer is `on_fail` "This blob changed while it was being read", and the app starts over. Replaced bytes are never kept around for a reader.
- **Read after the write's answer:** an app's asks run at once, side by side, so a `blob_get` sent with the `sql_batch` that writes the blob can run first. The contract says so.
- **Read, use, drop:** an app keeps nothing between calls (DECISION-045), so bytes it holds go in its window state (16 MB, saved with the window). The contract's notes: handle each piece as it arrives and keep the result, not the bytes. Formats read line by line (JSONL, CSV, logs) suit this; a format needing the whole file should stay well under the state cap, or be split into several blobs.
- **Writing is whole:** one `blob_set` carries all the bytes, so a blob is all there or not at all. A guest's result must grow to carry one (`MAX_UPDATE` in `src/guest/limits.la`).
- Each runs in a `Cmd.task`, like `sql`. Other windows of the app get `on_change` with the owner tables a batch changed, as today.
- `blob_list` gives at most 10,000 keys, in key order; `cut` counts those left out.

### 1.6 Showing a picture

- `image("blob:<key>")`: Rapidkoi reads the bytes and shows them. They never pass through the app.
- Under every rule a server picture has (contract 7, Pictures): PNG or JPEG, checked from its header (lugha-imagesize) before anything decodes it, at most 4,096 px a side and 16 MP; WebP waits on kivuli (DECISION-058). SVG stays bundle-only.
- A key with no blob, or bytes that aren't a picture: Rapidkoi's broken-picture mark, and `on_fail` with the sentence, as for a server picture. The app runs on.
- Rapidkoi knows each key's hash (it wrote them), so a picture is cached by hash: replacing a key's bytes shows the new picture, and the same bytes under two keys decode once. Reading the mapping happens in a task, never in `view`; Rapidkoi reloads an app's mapping after any batch that touched blobs or an owner table, and on `on_change`.

### 1.7 Sending a blob (D, agreed)

`fetch` gains `body_blob: <key>` in place of `body`: Rapidkoi sends the blob's bytes as the request body. Every other `fetch` rule is unchanged: the same servers, the same methods, the headers the app sets (its `Content-Type` included). The cap is the blob's, not `fetch`'s 1 MB. A missing key is `on_fail`.

### 1.8 Clear and Forget

They remove the blobs folder with the database (each file, then the folder: `std.fs` deletes nothing recursively).

### 1.9 The contract

A new contract version (the only one, as now: older apps are refused). Blobs and sync's changes (DECISION-071) both need one. **Whichever is built first is contract 8**; the other takes the next number. The contract text gets: the asks (1.5), `blob:` pictures (1.6), `body_blob` (1.7), the reserved `rapidkoi_` names, and the sentences in part 2.

---

## Part 2 — Defaults

| What | Default | Why |
|---|---|---|
| A blob | 16 MB | the state's cap and kivuli's picture cap; one call carries it |
| An app's blobs | 1 GB, apart from its database's 100 MB | photos for a notes app; the person sees it in ⚙ |
| How many | 10,000 an app | `blob_list` in one answer; the folder stays small |
| A `blob_get` answer | 4 MB | as an SQL answer |
| A batch | its blobs plus 4 MB of everything else | one 16 MB blob per batch, room for its rows |

**When one is hit:** the batch fails whole and the app gets `on_fail` with a sentence: "This blob is over 16 MB", "This app's files are full (1 GB)", "This app has 10,000 blobs already". Nothing is half-written.

**Which apps:** signed apps and local development apps get blobs as they get a database (by host and origin key, or by address or file). Built-in apps get none, as they get no database.

**What the person sees:** ⚙'s Data row shows one total, database and blobs together ("Data: 340 MB"); Clear removes both. No new screen.

---

## Part 3 — Designed, not built

Each is a capability for the owner's review when it comes, and fits without changing part 1.

### 3.1 The file picker saving into a blob (C)

The picker is its own feature (DECISION-081 lists files among the missing basics). One of its options: the person's chosen file goes straight into a blob, and the app gets `{key, size}`, never the bytes unless it asks with `blob_get`. **It must answer what 1.4 doesn't need yet:** a blob made outside `sql_batch` has no transaction with rows, so its owner row may not exist. Likely answer: such a blob is ownerless until the app's next batch names it with an owner (a `blob_own` step).

### 3.2 A fetch saving into a blob (B)

`fetch {…, into: {key}}`: Rapidkoi stores the response only on a 2xx status and answers `{status, headers, key, size}`. For large downloads the app doesn't change (maps, media). Same servers as `fetch`; the same ownerless rule as 3.1.

### 3.3 Blobs over 16 MB: write sessions

```
blob_open {msg}                       → on_answer {upload: "u1"}
blob_write {msg, upload: "u1", bytes}   (each ≤ 4 MB, repeated)
sql_batch [ { blob_set: "video-3", upload: "u1", owner }, … ]
```

The pieces go to a temporary file the app can't see; `blob_set` with `upload` hashes, renames and commits it as in 1.3. An abandoned upload is swept on open or after an hour. A few open uploads an app at most: it's state outliving a call, which DECISION-046 keeps rare. `blob_set` taking `upload` instead of `bytes` is an addition: apps using `bytes` keep working.

**Needs lughac first:** file handles (open, write, read at an offset, close). Today `std.fs` and the intrinsics read and write whole files only, so past 16 MB Rapidkoi would hold whole blobs in memory. Also to check: whether `std.http` can send a body from a file.

### 3.4 Sync

- The log carries `rapidkoi_blobs`' rows with the app's rows (it's a table in the app's database); bytes never go in the log.
- A device lacking a hash fetches it when it's first shown or read (on demand), from another device or the Pond, end-to-end encrypted like everything else there; it checks the bytes against the hash.
- **What the app sees while the bytes aren't here:** a `blob:` picture shows Rapidkoi's loading mark, then the picture; offline, the broken-picture mark and `on_fail` "This picture isn't on this device yet". `blob_get` fails with the same sentence. This is why it's designed now: apps written for part 1 already handle `on_fail`.
- Whether `rapidkoi_blobs` follows DECISION-072 (`local_` tables don't sync) through owner tables: a blob owned by a `local_` row stays local. Decided with sync.

### 3.5 Other ways to show bytes

Audio and video, once they exist, take `blob:` sources the same way pictures do.

---

## Dependencies

Requests to other repositories, under their own rules; never worked around here.

1. **lugha-sqlite: reserved names in `Guard`** (`reserved: "rapidkoi_"`): untrusted statements are refused any table or trigger with the prefix, except inside a trigger that has it. Its authorizer (`decide` in `../lugha-sqlite/src/guards.la`) is one rule for the whole connection and ignores the trigger a statement runs in, so today an app could read and write `rapidkoi_blobs` and drop Rapidkoi's triggers. **Needed for part 1.**
2. **lugha-sqlite: the program's own steps in a batch** (`Own(sql, params)`): past the reserved-name rule, every other guard still applying, in the same transaction as the untrusted steps. Also its own statements outside a batch (making the table and triggers). **Needed for part 1.**
3. **lughac: file handles** (open, write, read at an offset, close), and perhaps `std.http` sending a body from a file. **Needed for 3.3 only.**

## For the owner's security review

Every one is a host function untrusted code can call, or SQL built from what it chose:

- the blob asks (1.5) and their limits;
- `blob:` pictures (1.6): bytes the app wrote, decoded by Rapidkoi;
- `body_blob` (1.7): up to 16 MB sent to the app's servers;
- the owner triggers (1.4): SQL naming the app's table and column, from an allowlist read back from its schema;
- the reserved table: the app can't read or change it, but can see it exists.

## Order of work

1. The owner reviews this draft; DECISION-087 is resolved from it.
2. lugha-sqlite gets requests 1 and 2 (`docs/specs/`), built and tagged there.
3. Discovery, then a PRP: part 1 and part 2, with Notes attaching a photo as the app that proves it (in `../notes`, under its own rules).
4. rapidkoi-rust gets the contract's asks and two helpers, tagged first; then the shell; then the built-ins are repacked:
   - writing a row and its blob in one batch;
   - **`blob_lines(key)` (agreed):** reads a blob piece by piece, carries an unfinished last line into the next piece, checks the hash, and delivers `Event::Lines { lines, done }`, so an app reads JSONL, CSV or a log of any size without holding it whole. On a changed blob it fails, and the app starts over.
