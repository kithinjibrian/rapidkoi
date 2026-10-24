## FEATURE: Signed apps are kept on this device, open from there at once and offline, and update in the background

## OBJECTIVE
Opening a signed app you've opened before is instant and works without a connection: Rapidkoi runs the copy it kept and checks the site in the background. A newer version is used from the next open, with a notice: "Notes updated to 0.2.0". A version its publisher withdrew never runs while Rapidkoi can see the withdrawal. Offline, the kept copy runs, and the app's Signed app page says when it was last checked. ⚙'s Data page shows what the kept copy takes.

## CONTEXT
- Agreed with the owner in chat (2026-10-24), question by question:
  - every signed app opened is kept, and pinned apps are never evicted;
  - a newer version is used from the next open, with a notice;
  - online, a withdrawn build never runs; offline, the kept one runs;
  - 500 MB in all, one build per app;
  - a link naming a version opens the kept build when it's that version; otherwise, online, the named version is fetched and run but not kept; offline, the kept one opens with a notice;
  - nothing shows while it works, only the Signed app page's "Checked …", the notices, and the size in ⚙ → Data.
- Today (`net/loads.begin` → `net/fetching.fetch_app` → `trust/verify.check_site_at` → `loads.arrive` → `bundles/loader.from_fetched`): every open fetches the key, the index and the build, then checks and compiles them. Only trust (host → key, highest sequence) is kept (`data/remembered.la`).
- **The key idea:** `check_site_at(known, host, get, wanted)` takes a `get(path)`. Given a `get` that reads the kept `key`, `index` and build from disk, it re-checks the kept copy exactly as it checks a site: signatures, host, sequence never lower than seen, withdrawn, size and hash. Nothing on disk is trusted because it's on disk.
- Files to read first: `src/net/loads.la`, `src/net/fetching.la`, `src/trust/verify.la`, `src/bundles/loader.la`, `src/data/store.la`, `src/data/databases.la` (a per-app file named by hash; size; Clear), `src/window/pins.la` (Forget), `src/ui/settings.la` (⚙ → Data), `src/ui/shield.la` (the Signed app page), `src/guest/limits.la`.
- Creates `src/data/kept.la`, `test/kept.la`. Changes the files above.
- Decisions: core decisions 5 and 26 (storage), DECISION-021 (loading in the background), PRP-036 (links). None open. Follow-up, not here: partial versions in links (`@0.1`), its own PRP.

## REQUIREMENTS

### Must Do
- **What's kept** (`data/kept.la`), for each signed app opened from a remote or development *signed* site, in `kept/<sha256 of the host>/` in the data folder:
  - `key` and `index`: the site's files as last fetched and checked, unchanged;
  - `<sha256 hex>.rkb`: the build the index names, one per app;
  - `meta`: when it was last checked, when it was last opened, and a notice waiting for the next open. A small CBOR file, written safely.
  - Files are written build first, then `index` and `key`, then `meta`, so a half-written copy fails its check and is ignored. Everything is read with bounds (`MAX_BUNDLE` for the build).
- **Opening** (`begin` and `begin_restoring`): when a kept copy exists for the host, the task re-checks it with `check_site_at` over the kept files, with the link's version as `wanted`.
  - If it passes and gives the kept build, the app opens from it, with no network.
  - It doesn't pass (missing, damaged, a sequence older than one seen, or a link naming another version): the load goes to the network as today.
  - Either way, a background check follows (below).
- **The background check:** after an app opens from its kept copy, the same `fetch_app` runs as a task.
  - **Same build:** only `meta`'s "checked" time is updated.
  - **A newer current build:** the site's files and the new build replace the kept copy, the old build is removed, and `meta` holds "{name} updated to {version}" for the next open. The running window keeps its version.
  - **The running version withdrawn:** the window stops and shows "{name} {version} was withdrawn: {reason}", with Reload, which opens the newest version that isn't withdrawn. The kept copy is replaced by that version, or removed if there's none. *(Flagged for the owner: this stops a window mid-use, which "online, a withdrawn build never runs" implies.)*
  - **Can't reach the site** (offline, DNS, timeout): nothing changes. The Signed app page says "Not checked since {date}".
  - **Checked and refused** (a changed key, an older sequence, a tampered file): the kept copy isn't touched or used again. The next open goes to the network, where the usual flow handles it, including asking about a changed key.
- **A load from the network** of a signed app (first open, or a fall-back) keeps what it fetched, through the same writes. A link's non-current version is run but not kept.
- **Never kept:** unsigned apps, local files, development servers serving an unsigned bundle, and Rapidkoi's built-in apps.
- **Eviction:** after each write, if all kept copies together pass `KEPT_BYTES` (500 MB, in `guest/limits.la`), unpinned apps go, least recently opened first, until under it. Pinned apps never go, even if they alone are over it.
- **Forget** removes the app's kept copy with everything else of it.
- **Notices:** "{name} updated to {version}" on the open after an update; "{name} {version} was withdrawn: {reason}"; offline, "{wanted} isn't available offline; opened {version}" for a link naming a version not kept.
- **The Signed app page** (shield card) gets one line: "Checked for updates {time} ago", or "Not checked since {date}".
- **⚙ → Data** shows the kept copy's size beside the database's.
- All disk work runs in `Cmd.task`, except Forget's removal (a data-folder change the person asked for, like Clear).

### Must NOT Do
- Never cache compiled machine code: wasmtime's serialized modules would mean running native code read from disk. Compiling stays per open.
- Never run a kept build without re-checking it.
- Never keep more than one build per app, or a link's older version.
- Never swap the code under a running window, except stopping it when its version is withdrawn.
- No partial versions in links (the follow-up PRP). No "clear cache" button: Forget covers it.

## DESIGN
- `data/kept.la`:
  - `Kept { dir, host }`, `kept_for(store, host)`;
  - `read_site(k): fun(string): Result<(i64, u8[]), string>`, which is `check_site_at`'s `get` over the kept files (404 for a missing one);
  - `keep(k, files, build, meta): Result<(), KeptError>`, `remove(k)`, `size_of(k)`;
  - `evict(store, pinned_hosts, limit): Result<string[], KeptError>`, which returns the hosts removed;
  - `Meta { checked, opened, notice: Option<string> }`.
- `fetching.la`: `Fetched.Signed` gains where it came from, `From.Kept | From.Site`, so `arrive` knows to start the background check. A new `refresh(...)` task returns a `Refreshed`: `Same`, `Newer(Verified, Record, version)`, `Withdrawn(version, reason, Option<Verified>)`, `Unreachable`, or `Refused(why)`.
- Messages: `Msg.Refreshed(id, number, Refreshed)`, and `Msg.Kept(Result<(), string>)` for a write's outcome (a failed write is only logged to the page's ⓘ, never shown as an error).
- `AppPage` gains `checked: Option<i64>` and `offline: bool` for the Signed app page.
- `test/support.la`: a signed site served from `test/tmp/` as `saving.la` does, with `counting()` as its build, and a way to publish a second version and withdraw one.

## TESTS
- [x] `kept.la`: keep, then read back through `check_site_at`: the same build passes
- [x] `kept.la`: a changed byte in the build, the index or the key fails the check, and the open falls back to the network
- [x] `kept.la`: a kept index older than the sequence seen is refused (rollback)
- [x] `kept.la`: eviction removes the least recently opened unpinned app first, never a pinned one, and stops under the limit
- [x] Opening a kept app with its site unreachable runs it, and the Signed app page says "Not checked since …"
- [x] Opening a kept app with the site up: it runs from the kept copy, and the check updates "checked"
- [x] A newer version published: the running window keeps its version; the next open runs the new one with "Counting updated to 0.2.0"
- [x] The running version withdrawn: the window stops with the reason; Reload opens the newest valid version
- [x] Every version withdrawn: the kept copy is removed, and the next open shows the site's refusal
- [x] Offline, a withdrawn kept version still runs (nothing can say otherwise)
- [x] A link naming the kept version opens from it; one naming another version, online, runs it without keeping it; offline, opens the kept one with the notice
- [x] A development server's unsigned bundle, a local file and the built-in apps are never kept
- [x] Forget removes the kept copy; ⚙ → Data shows its size
- [x] Restore (a session's windows) opens from kept copies

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [x] No file over 500 lines
- [x] CHANGELOG.md updated; core decision 26 or a new one records the cache
- [ ] For the owner, by eye: open `rapidkoi://notes.dafifi.net`, quit, turn the network off, and open it again: it opens. Deploy Notes 0.1.1, open it (still 0.1.0), and open it again: "Notes updated to 0.1.1"

## AMENDMENTS

- **The withdrawn window (owner, at approval):** stopped, as written. It shows the stopped-app screen with "{name} {version} was withdrawn: {reason}"; ⟳ (Reload) opens the newest version that may run.
- **Where it lives:** `data/kept.la` (the files) and `net/keeping.la` (the two tasks: `open_app`, `refresh`); `fetching.check_signed` hands back the files it checked (`Source`).
- **⟳ asks the site first,** falling back to the kept copy only when the site can't be reached: otherwise reloading a development site would show the stale copy.
- **A changed key:** the open runs the kept copy, still verified under the old key; its check is refused, the copy is dropped, and the next open asks about the new key. `test/saving.la`'s two tests that assumed every open goes to the site now open once more, with a comment.
- **The offline link notice** is for a version the kept index lists but whose build isn't kept; a version the index doesn't list keeps PRP-036's "This link was made with …".
- **Not covered by a test:** `sqlasks.bad_shapes_say_why` already takes about 5 s of its 10 s limit alone (the same on `main`); in the full suite with this PRP's tests it sometimes times out. Reported to the owner, not changed here.
