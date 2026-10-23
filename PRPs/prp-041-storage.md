## FEATURE: Storage (build step 4, part 1): every app gets its own SQLite database on this device. It sends SQL with parameters and gets rows back as events; a write is a batch in one transaction; its other windows hear which tables changed. Contract 6.

## OBJECTIVE
From chat with the owner (session 207, DECISION-046, DECISION-051), with research.

Done looks like this:
- An app asks `SELECT … WHERE id = ?` and gets its rows as an event.
- It writes with a batch that's all or nothing.
- Its data outlives every window and every restart.
- Its other windows hear `on_change` with the tables that changed, and ask again.
- The tasks app keeps its tasks in its database, so a task's link opens that task in any window.
- Each app's ⚙ card shows how much it stores, with Clear. Forget removes it.

## WHY (the owner's decisions, recorded)
- **SQL-first** (the owner: a real database is a selling point for offline apps).
  - WebSQL's end was a standards rule that doesn't bind Rapidkoi.
  - SQLite promises its SQL, C API and file format through 2050.
  - The guardrails are SQLite's own guidance.
- **The split:** window state is what you're looking at (per window, in the link); the database is what the app knows (per app, shared, kept). A window copies the rows it asked for into its state.
- **The six choices:**
  1. values only as parameters;
  2. a read is one statement, a write a batch in one transaction, nothing held open between events;
  3. answers as events, run in the background;
  4. results capped (10,000 rows or 4 MB);
  5. `on_change` with table names;
  6. SQLite's own types, carried as CBOR.
- **Who and how long:**
  - signed apps: by host and origin key, until Forget or Clear;
  - local development apps: by address or file, kept;
  - built-in apps: none.
- **Limits:** 100 MB, 1 s per call. No permission dialog.
- **Through `lugha-sqlite`** (DECISION-051), published and tagged `v0.1.0`.
- **Later:** the change log (history, undo, sync).

## CONTEXT

- Starting state:
  - Contract 5 (closed) has no storage.
  - Signed apps' window state is saved per instance (`src/data/states.la`).
  - `lugha-sqlite` 0.1 exists (open, `run`, `query`, `batch`, `on_change`, `Guard.untrusted()`, `heap_limit`; 16 tests).
- Ending state:
  - **Contract 6** (DECISION-052; `docs/specs/contract-6.md`): contract 5 plus storage. Contracts 2–5 run unchanged, without storage.
    - **`sql` effect:** `{kind: "sql", msg, sql, params: [values], max_rows?}`.
      - Answered by `on_answer` with `{columns: [names], rows: [[values]], cut: bool}`, or by `on_fail` with `{error}`.
      - `max_rows` defaults to, and is capped at, 10,000.
    - **`sql_batch` effect:** `{kind: "sql_batch", msg, steps: [{sql, params}]}`, all or nothing.
      - Answered by `on_answer` with `{changes, last_rowid}`, or by `on_fail` with `{error}`.
      - At most 100 steps.
    - **Values:**
      - integers, floats, text, bytes, null, as CBOR;
      - a boolean parameter is stored as 1 or 0, as SQLite does;
      - anything else in `params` is a contract error that stops the app.
    - **`on_change`** gains `tables: [names]` when it's sent because the app's data changed in another window of the same app. Contract 3–5 apps never get it.
    - **Bounds:**
      - SQL at most 100 KB;
      - at most 999 parameters;
      - a value at most 4 MB;
      - an answer at most 4 MB (rows past it are cut, `cut: true`).
  - **Rapidkoi:**
    - **`src/data/databases.la` (new):** where each app's database lives and how one call runs.
      - **Files:** `databases/<sha256 hex>.db` in the data folder, named by a hash so no app's name or address becomes a path.
        - signed: of the host and origin key;
        - local development: of the address, or of the file's full path.
      - **One call:** open, `Guard.untrusted()` (100 MB, 1 s), run, read the update hook's tables, close.
      - **Its size:** for the ⚙ card.
      - **Clear:** removes the file and SQLite's `-wal`/`-journal` beside it.
    - **`src/data/queries.la` (new):** an app's `sql`/`sql_batch` ask, checked (contract 6, a database for it, the bounds), then sent in a `Cmd.task`, never in `update`.
      - The answer arrives as `Msg.Queried(tab, load, msg, result)`, like a fetch's.
      - A write that changed tables sends `on_change` with them to the app's other running windows (the same database).
      - The answer to the window that asked comes as its `on_answer`.
    - **`src/guest/contract.la`** reads the two effects; **`src/guest/effects.la`** routes them to `queries`. The effects module is near its limit: the routing stays a few lines there.
    - **⚙ card (`src/ui/settings.la`):** App data, State becomes Data. It shows the database's size and has "Clear…", asked first ("Clear {app}'s data?"). Forget also removes the database.
    - **`lugha.toml`:** `sqlite = { git = "https://github.com/kithinjibrian/lugha-sqlite", version = "0.1" }`.
    - **`main`:** `sqlite.heap_limit(256 MB)` once at start.
  - **The Rust library:**
    - `Ask::sql(msg, sql, params)` and `Ask::sql_batch(msg, steps)`;
    - `Value` (`Integer`, `Real`, `Text`, `Bytes`, `Null`), with `From` for numbers, text, `bool` and `Option`;
    - a `params![…]` macro;
    - `Event::Rows { msg, columns, rows, cut }`, `Event::Done { msg, changes, last_rowid }`; a refusal is `Event::Failed { msg, error }`, as for a fetch;
    - `Event::Change` carries `tables: Vec<String>` (empty when it isn't about data);
    - `export!` reports 6.
  - **The tasks app** moves to its database.
    - Tables are made in `on_open`: `CREATE TABLE IF NOT EXISTS`, with `PRAGMA user_version` for later changes.
    - It lists with `SELECT`; adds, ticks and deletes with batches; and asks again on `on_change`.
    - Its window state keeps only the page, the filter, the draft, `editor.task` and the copy it read.
    - A task's link opens that task in a new window.
  - **Docs:**
    - DECISIONS.md: DECISION-052 (contract 6);
    - `docs/specs/contract-6.md`, `docs/specs/app-events.md` (`on_change`'s tables);
    - CLAUDE.md: stack (contract 6, lugha-sqlite), tree, and architecture rule 3 (queries run in tasks);
    - MEMORY.md, DESIGN.md (the Data row), CHANGELOG.md.
- Related existing code: `src/net/requests.la` (an app's request, checked and sent in a task: the model to follow), `src/data/states.la`, `src/data/store.la`, `src/ui/settings.la`, `src/people/access.la` (`key_of`), `guest/rust/rapidkoi/src/{abi,lib,fetch}.rs`, `guest/rust/tasks`, `../lugha-sqlite/src/lib.la`.
- Open decisions that must be resolved first: none. DECISION-052 is approved with this PRP.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **Every query runs in a `Cmd.task`.** `update` only checks and describes it (architecture rule 3).
- **Everything from the guest is hostile input:** the SQL's size, the count and size of parameters, `max_rows`, the number of steps, each checked before anything runs. A bad shape stops the app. A refused or failed query is an `on_fail`, and the app runs on.
- **SQLite's guardrails on every connection** (`Guard.untrusted()`). An app reaches only its own database file. No app's input becomes a path.
- **A write's tables reach only the same app's other windows.** No other app ever hears of them.
- **Answers are bounded:** rows and bytes; rows past the bytes are cut, not an error.

### Must NOT Do
- The change log, history, undo, sync.
- Raising an app's size limit, or settings for it (later).
- Storage for built-in apps.
- Changing contracts 2–5.
- Merging into `redesign` or `main`.

## MODEL AND MESSAGES
- `Msg.Queried(i64, i64, Value, Queried)`: a tab, its load, the ask's msg, and the result, ready to answer.
- `Confirm.ClearData` for the ⚙ card's Clear.

## ERROR HANDLING REQUIREMENTS
- `on_fail`'s sentence, by `lugha-sqlite`'s kind:
  - `Full`: "This app has no room left (100 MB)";
  - `Interrupted`: "This app's query took longer than a second";
  - `Denied`: "This app's SQL asked for something apps can't do";
  - `TooBig`: "This app's SQL or a value is too long";
  - `Busy`: "This app's data is busy; try again";
  - `Constraint`, `Misuse`, `Other`: SQLite's own message, which describes the app's own SQL.
- A database that can't be opened (disk, permissions) is an `on_fail` too, never a crash.
- Clear and Forget failing to remove a file is a notice.

## SECURITY CONSIDERATIONS
- **A new capability exposed to every contract 6 guest: owner review (SECURITY RULE).**
  - Each app reaches only its own database, a file Rapidkoi names by hash.
  - SQL is bounded and guarded:
    - an authorizer: no ATTACH, no other database, no extensions, safe PRAGMAs only;
    - a 1 s limit per call;
    - a 100 MB file cap;
    - limits on SQL and value length;
    - `trusted_schema` off;
    - the defensive flag;
    - a process-wide memory cap.
  - Values only as parameters.
- No permission dialog (the owner): like the web's storage, on this device only, invisible to other apps.
- **New package dependency: `lugha-sqlite` 0.1** (DECISION-051), the reason for this PRP. It links the system's `libsqlite3`; building Rapidkoi now needs `libsqlite3-dev` (CLAUDE.md's Needs).
- Untrusted SQL can't escape its file: `ATTACH` and other databases are denied, and `max_page_count` caps its size.

## TESTS TO WRITE

`test/databases.la`:
- [x] A signed app's and a local app's database paths are hashes under `databases/`, never the address.
- [x] One call:
  - creates, writes and reads;
  - each value type round-trips;
  - a write reports its tables;
  - Full, Interrupted and Denied come back as their sentences.
- [x] Clear removes the file and its journal. Size reads it.

`test/queries.la`:
- [x] A contract 6 app's `sql` ask becomes a task, and its result is answered as `on_answer` (rows) or `on_fail` (sentence).
- [x] Bad shapes stop the app: SQL over 100 KB, over 999 parameters, a non-value parameter, over 100 steps.
- [x] A contract 5 app asking for `sql` is stopped ("unknown effect"), as any unknown ask.
- [x] A batch that changes `tasks` sends `on_change` with `["tasks"]` to the same app's other window, and not to another app's.
- [x] A stale result (the tab closed or reloaded) is dropped.

`test/contract.la`, `test/links.la` (or a new `test/contract6.la`; contract.la is at its limit):
- [x] The two effects read, and their bounds hold.

`test/settings.la`:
- [x] The Data row shows the size; Clear asks, then removes it.
- [x] Forget removes the database.

`test/tasks.la` (skips until built):
- [x] Tasks survive closing the window.
- [x] A task's link opened in a new window shows that task.

`guest/rust` (`cargo test`):
- [x] `Ask::sql` and `sql_batch` encode as specified; `params!` and `From` make `Value`s.
- [x] `Event::Rows`, `Done`, `Failed` and `Change { tables }` decode.
- [x] The tasks app's asks for each action.

## ROLLBACK PLAN
- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits. Contract 6 is then unknown ("needs a newer version"), and databases already written stay on disk, unused.

## ACCEPTANCE CRITERIA
- [x] The tasks app keeps its tasks across closing every window and restarting Rapidkoi.
- [x] A task's link opens it in a new window.
- [x] Two windows of the tasks app: ticking in one updates the other.
- [x] ⚙ shows the data's size; Clear empties it.
- [x] Contracts 2–5 apps run as before (their tests pass: counter, quote, the new tab, Settings).
- [x] `lughac test test/` and `cargo test` pass (the owner runs them); build ok (with `libsqlite3-dev`); fmt clean; no file over 500 lines.
- [x] DECISION-052, the contract 6 spec, CLAUDE.md, MEMORY.md, DESIGN.md, CHANGELOG.md updated.

## VALIDATION
- `lughac fmt --check src test`, `lughac test test/`, `lughac build src/main.la -o main`, `cargo test --manifest-path guest/rust/Cargo.toml`
- For the user:
  1. Build, pack and serve the tasks app (CLAUDE.md COMMANDS). Add three tasks and tick one.
  2. Close the tab, open the app again: they're there.
  3. Open a task, copy the address, and paste it into a new tab: the same task.
  4. With two windows side by side in the strip, tick in one and switch: the other shows it.
  5. ⚙ → Data: its size. Clear: the list is empty.

## AMENDMENTS (session 209, while building)

- **Contract tests in `test/sqlasks.la`:** the two effects are read by a new pure module, `src/guest/sqlasks.la` (`sql_of`, `batch_of`), since `contract.la` is near its limit; its tests are there.
- **A call in flight is capped per window** ("This app asked for too much at once"), so an app can't queue unbounded tasks.
- **⚙'s Data size is read in a task** when the card opens (`Msg.DataSized`), not in `view`.
- **Built-in apps:** the new tab is refused "The new tab keeps no data"; Settings, in a tab, "This app can't keep data here" (its path is `builtin:`).
- **The Rust library dropped `Eq`** from `Ask`, `Event` and `Progress`: `Value` holds floats. `PartialEq` stays, so tests compare as before.
- **The tasks app** writes title and notes as you type, one `UPDATE` each, and keeps its copy in step at once; every write's answer reads the list again.
- **Built-in bundles** (`newtab.rkb`, `settings.rkb`) are repacked for contract 6 (their code changed only in `Event::Change { .. }`).
- **A local server's database is named by its host and port** (session 213, the owner: "yes, name it by host and port"), not by the address as typed: `rapidkoi://localhost:8001`, `http://localhost:8001` and `http://localhost:8001/app.rkb` are one app's data. A file stays its full path.
