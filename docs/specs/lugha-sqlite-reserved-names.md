# lugha-sqlite request: reserved names, and the program's own statements

From Rapidkoi, DECISION-087 (`docs/specs/blobs.md`). For a session in `../lugha-sqlite`, under its own rules; Rapidkoi doesn't build it and doesn't work around its absence. Against `v0.1.0`.

## Why

A program hosting SQL it didn't write (Rapidkoi runs each app's SQL under `Guard.untrusted()`) sometimes keeps a little bookkeeping of its own in the same database, so that its rows and the untrusted rows commit in one transaction. Rapidkoi's case: a table `rapidkoi_blobs` mapping an app's blob keys to files, and `TEMP` triggers that delete an owner row's blobs when the app deletes the row.

Today `guards.decide` is one rule for the whole connection, and it ignores the trigger a statement runs in. So:

- the untrusted SQL can read and write the program's table, and drop the program's triggers;
- refusing them would also refuse the program's own statements, and the bodies of its triggers, since the authorizer can't tell them apart.

Both features are generic: any program mixing its own bookkeeping with untrusted SQL wants them.

## 1. Reserved names in `Guard`

`Guard` gains `reserved: string`, a name prefix (`""`, the default in `Guard.untrusted()`, reserves nothing).

When `deny` is on and `reserved` isn't empty, the authorizer refuses an untrusted statement any action naming, case-insensitively, a table, index or trigger whose name starts with the prefix:

- reading or writing it: `SQLITE_READ`, `SQLITE_INSERT`, `SQLITE_UPDATE`, `SQLITE_DELETE` (the table is the first detail);
- creating, dropping or altering it: `SQLITE_CREATE_TABLE`, `SQLITE_CREATE_TEMP_TABLE`, `SQLITE_DROP_TABLE`, `SQLITE_DROP_TEMP_TABLE`, `SQLITE_ALTER_TABLE` (the table is the second detail), and the index and trigger codes (an index or trigger *on* a reserved table, or *named* with the prefix: a reserved trigger name can't be taken, so untrusted SQL can't make a trigger that looks like the program's);
- **except inside a trigger whose name starts with the prefix** (the authorizer's fifth argument, today `_trigger`): the program's own triggers do their work when untrusted statements fire them. A trigger the untrusted SQL made, under any other name, is refused the reserved table like any statement.

Reading the schema stays as it is: the names are visible in `sqlite_master` and `PRAGMA table_list`; only their contents and shape are guarded. (Rapidkoi documents the prefix for apps.)

To check while building: how SQLite authorizes the triggers dropped implicitly when a table is dropped (`DROP TABLE notes` drops a `TEMP` trigger on `notes`). Either answer works for Rapidkoi, which checks its triggers are there after every batch; the package's docs should say which it is.

## 2. The program's own statements

The program's own SQL passes the reserved-name rule; every other guard (the time limit, size and length limits, `ATTACH`, extensions, unsafe PRAGMAs) still applies.

- **In a batch:** `batch` takes steps that are either untrusted or the program's own, run in the order given, in one transaction, all or nothing. For example:
  ```
  public enum Step { Untrusted(string, Value[]), Own(string, Value[]) }
  public fun batch_steps(self, steps: Step[]): Result<Ran, SqliteError>
  ```
  `batch(steps)` keeps its meaning (every step untrusted). `Ran` counts changes as now; whether `Own` steps count is the package's choice, documented. (Rapidkoi reports only the app's changes, so a count of the untrusted steps alone, or both counts, would help.)
- **Outside a batch:** `run_own(sql, params)` and `query_own(sql, params, max_rows)`, for making the table and triggers on open, reading the mapping, and sweeping.

One way to build it: a `Ref<bool>` the authorizer closure reads, set only while the package prepares an `Own` statement. The authorizer runs when a statement is prepared, so the flag covers exactly that statement, and never the triggers an untrusted statement fires later (those are covered by rule 1's trigger exception).

## Tests the package would want

- With `reserved: "rapidkoi_"`: untrusted `SELECT`, `INSERT`, `UPDATE`, `DELETE`, `DROP TABLE`, `ALTER TABLE`, `CREATE INDEX … ON rapidkoi_blobs` and `CREATE TEMP TRIGGER rapidkoi_x …` are each refused (`Kind` for an authorization error), and `RAPIDKOI_BLOBS` too (case).
- The same statements as `Own` steps or `run_own` succeed.
- A batch mixing `Own` and untrusted steps commits all or nothing: an untrusted step failing rolls back the `Own` ones before it.
- An `Own` `CREATE TEMP TRIGGER rapidkoi_t AFTER DELETE ON t BEGIN DELETE FROM rapidkoi_m …; END`, then an untrusted `DELETE FROM t`: the trigger's delete runs.
- An untrusted `CREATE TEMP TRIGGER mine AFTER DELETE ON t BEGIN DELETE FROM rapidkoi_m; END`, then an untrusted `DELETE FROM t`: refused.
- `reserved: ""` behaves exactly as `v0.1.0`.
- The time limit still stops a long `Own` statement.

## How Rapidkoi will use it

Rapidkoi moves to the tag (`lugha.toml`, then `lughac update`), sets `reserved: "rapidkoi_"` on every app's connection, makes `rapidkoi_blobs` and its owner triggers with `run_own`, and writes each `blob_set`/`blob_delete` as an `Own` step beside the app's untrusted ones (`docs/specs/blobs.md`, 1.3 and 1.4). Publishing the tag is asked of the owner first.
