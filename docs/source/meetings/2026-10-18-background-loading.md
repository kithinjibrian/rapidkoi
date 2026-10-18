# Background loading (PRP-008) — design in chat, session 40, 2026-10-18

The owner and Claude, one topic at a time; each answered "yes". Feeds PRP-008. kivuli 0.6.0 brought `Cmd.task` (a green thread, one message back) and `Sub.every`. Downloads run in the background; a file read, a DNS lookup, crypto and compiling a module still hold the one thread, task or not.

## 1. What you see while an app loads — settled

- The tab opens at once, on a loading page: the address and "Loading…" with a spinner.
- Everything else stays usable: other tabs, home, other loads at the same time.
- Closing a loading tab drops its result when it arrives (tasks can't be cancelled). ⟳ on a loading tab starts again, and the earlier result is ignored.
- It ends as the app, or as the refused or stopped page with the reason.
- Local files stay instant (reading a file blocks anyway).
- The marker shows the address with "loading…" until every check has passed. Nothing shows ✓ early.

## 2. Where the checks run — settled

- The task gets a snapshot of the trusted keys as plain data. It fetches and checks the key file, the index and the build, and returns the verified build plus what trust to record.
- Trust is recorded only when the result arrives in the window: the one place the keyring changes.
- Two loads of one site at once: if that site's trust changed since the snapshot, the result isn't used as it stands, and the tab quietly loads again against the new trust. Two tabs can't each "first use" a different key.
- Compiling happens when the result arrives, on the window's thread, and only after every check passed.

## 3. How long a load may take — settled

- 30 seconds per request (key file, index, build); the size caps are unchanged.
- No progress bar (std.http can't report one). After 3 seconds the page shows the time so far: "Loading… 7 s".
- A timeout shows the reason ("localhost:8000 didn't answer in 30 seconds"); ⟳ tries again.
- Still local addresses only. Remote https hosts (counter.dafifi.net) are the next PRP, built on this one.
- CLAUDE.md rule 3 loses its "fetches from a local development address" exception: `update` no longer waits on the network.
