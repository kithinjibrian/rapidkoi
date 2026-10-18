# Remembering (PRP-007) — design in chat, session 35, 2026-10-18

The owner and Claude, one topic at a time. Feeds PRP-007.

## 1. What's remembered — settled

- **Now (PRP-007):** trusted keys (host → key, highest index sequence seen), pins, and where you left off (the app, its title, its state and the state version that wrote it).
- **Next (its own PRP):** open tabs restored on launch, each with its state ("yeah 1-3 now, tabs next").
- **Later:** copies of apps for offline use, with remote hosts and background loading (DECISION-021).

## 2. Where and in what shape — settled

- `$XDG_DATA_HOME/rapidkoi/`, else `~/.local/share/rapidkoi/`; the folder 0700, the files 0600.
- Three files, so one broken file loses only its own part: `trust`, `pins`, `left-off`.
- CBOR, each file starting with its own format number; read as untrusted input, with size limits.
- Written safely: a temporary file, `fs.sync`, `fs.rename`, `fs.sync` on the folder (std m115, m116). A crash leaves the old file or the new one.
- Not SQLite: that's apps' own data (step 3 or 4).

## 3. When it's saved — settled ("yes save")

- On every change, on the window's thread: pins when pinned or unpinned; trust when a site verifies with a new key or a higher sequence; left-off when an app is left. Loaded once at start.
- Not in `Cmd.task` (kivuli 0.6): file I/O still holds Lugha's one thread inside a task, so it would pause the window just the same. Moves to a task when std's file I/O stops blocking.
- Needs a CLAUDE.md rule 3 exception: `update` saves Rapidkoi's own small files in its data folder, written safely.

## 4. A file that can't be read — proposed, not yet answered

- Missing: a first run; start empty, silently.
- Broken: renamed aside (`pins.broken`), that part starts empty, a notice says where it went. For trust, the notice says sites will be trusted on first use again. Not fail-closed: a broken trust file is an accident; anything that can write the data folder could write a valid one.
- From a newer Rapidkoi (a higher format number): never read or overwritten; that part works in memory this session, with a notice.
- A left-off state whose version no longer matches starts over, as Reload does.

## Aside: kivuli 0.6.0

`Cmd.task` and `Sub.receive` unblock DECISION-021 (fetching in the background). Its own PRP, after tabs: every open gains a loading state.
