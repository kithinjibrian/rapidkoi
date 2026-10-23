# Contract 6: storage

PRP-041, DECISION-052. Contract 5 (`contract-5.md`) plus storage: every app gets its own SQLite database on this device. Nodes, styles, roles, events, state and links are contract 5's. Any change is a DECISIONS.md entry and a version bump. Contracts 2–5 run unchanged, without storage.

## Whose database

- A signed app: one database for its host and origin key.
- A local development app: one for its address, or its file's full path.
- Built-in apps: none. Their asks are answered `on_fail`: "The new tab keeps no data", or "This app can't keep data here" for Settings.

The file is `databases/<sha256 hex>.db` in Rapidkoi's data folder, so no app's name or address becomes a path. It lasts until the person clears it (⚙ → Data → Clear…) or forgets the app. No permission is asked: like the web's storage, it stays on this device and no other app can see it.

## Asks

Both are effects, like `fetch`: the app names a `msg`, and the answer comes later as an event carrying it.

### `sql`

```
{ "kind": "sql", "msg": "tasks", "sql": "SELECT id, title FROM tasks WHERE done = ?", "params": [0], "max_rows": 100 }
```

One statement. `max_rows` is optional: at most, and by default, 10,000. Answered by:

```
on_answer { "msg": "tasks", "value": { "columns": ["id", "title"], "rows": [[1, "Buy milk"]], "cut": false } }
```

`cut` is true when rows were left out, past `max_rows` or 4 MB.

### `sql_batch`

```
{ "kind": "sql_batch", "msg": "saved", "steps": [ { "sql": "INSERT INTO tasks (title) VALUES (?)", "params": ["Pay rent"] } ] }
```

Every step in one transaction: all or nothing. Answered by:

```
on_answer { "msg": "saved", "value": { "changes": 1, "last_rowid": 2 } }
```

### Values

Parameters and answers are SQLite's own types as CBOR: integers, floats, text, bytes and null. A boolean parameter is stored as 1 or 0, as SQLite does. Values only ever go in as parameters, never into the SQL's text.

### When it fails

`on_fail { "msg", "value": { "error" } }`, and the app runs on:

| Why | `error` |
|---|---|
| The database is full | This app has no room left (100 MB) |
| Over a second | This app's query took longer than a second |
| Something apps can't do (`ATTACH`, extensions, a dangerous `PRAGMA`) | This app's SQL asked for something apps can't do |
| SQL or a value over SQLite's limits | This app's SQL or a value is too long |
| Busy | This app's data is busy; try again |
| A constraint, a mistake in the SQL | SQLite's own message |
| No database for this app (no data folder) | This app can't keep data here |
| Too many calls at once | This app asked for too much at once |

## `on_change`

When a write in one window changes tables, the same app's other running windows get `on_change` with `"tables": ["tasks"]`. No other app hears of it. `on_change` for anything else carries no `tables`.

## Bounds

A bad shape stops the app with its sentence, as any contract error:

| What | At most |
|---|---|
| SQL | 100 KB |
| Parameters | 999 a statement |
| A value | 4 MB |
| Steps in a batch | 100 |

And for every call: 10,000 rows or 4 MB an answer (cut past it), 1 second, 100 MB a database, and 256 MB of SQLite's memory for all apps together. Every connection is guarded by lugha-sqlite's `Guard.untrusted()`: an authorizer, a progress handler, `SQLITE_DBCONFIG_DEFENSIVE`, `trusted_schema` off, length limits and `max_page_count`.

## Window state and the database

A window's state is what you're looking at; the database is what the app knows. A window copies the rows it asked for into its state and asks again on `on_change`. A link carries an ID (`editor.task=2`), and the window that opens it reads that row.
