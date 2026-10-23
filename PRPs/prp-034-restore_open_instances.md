## FEATURE: Rapidkoi reopens the instances you had open when it was closed: each app's instances, their names, their state, and which one was in front.

## OBJECTIVE
From the owner (session 179): "if i close and open browser my previous unclosed instances are lost". After this, quitting with Notes, Notes › Work and Mail open, and starting again, shows Notes, Notes › Work and Mail again. Each instance comes back:
- with its name;
- for a signed app, with its saved state;
- with the one you were on in front.

The dock and the strip look as you left them.

## CONTEXT

- Starting state:
  - Open tabs aren't saved (CLAUDE.md KNOWN ISSUES: "Open tabs aren't restored on launch yet").
  - The data folder keeps trust, pins, where you left off, people and each signed app's state. Only the app used last has its state saved on every change (`saving.persist`, `used_last`).
  - Launch order (session 170): restore, build the window, start the apps, answer them.
  - Instances (PRP-033): `AppPage.instance` (stable key), `.label` (name), `.seen` (last used); signed apps' state is per instance (`states.keep_in`).
- Ending state:
  - `src/data/remembered.la`: an `open` file, `{format, kind: "rapidkoi open", tabs: [{path, instance, label?, seen}], current: index or null}`, at most `MAX_OPEN` (64) tabs and `MAX_OPEN_FILE` (1 MB). `open_file(…)` and `read_open(bytes)` check every field as hostile input:
    - a path at most 4096 bytes;
    - an instance "" or a number from 2;
    - a label 1–64 characters;
    - no repeated (app, instance) pair.
  - `src/data/saving.la`: `OPEN` (a `Part`, like `PINS`). `persist` saves the open tabs whenever they change: opened, closed, renamed, reordered, or a different one in front. Saving also keeps **every open signed instance's state** whenever it changes, not only the one used last, so each comes back as it was.
  - `src/shell.la` `start`, launch step 2 (build the window): the restored tabs are made before any app starts.
    - A file opens as `open_path` opens it.
    - An address begins its background load (`loads.begin`), each in its own tab.
    - Each page is then given its `instance`, `label` and `seen`, and a signed app resumes its instance's state (`loads.resumed`).
    - The one that was in front is current. With nothing restored, the empty tab, as today.
  - `./main path/to/app` at launch: the restored tabs first, then the named app opens as it does today (it switches to its open instance if it's among them).
  - CLAUDE.md KNOWN ISSUES (the open-tabs line goes), DESIGN.md (launching), CHANGELOG.md.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- A tab that can't come back says why in its tab, as any failed open does, and the others still open:
  - a file gone;
  - a server down;
  - a key that changed (that one asks, as today).
- An unsigned app's instance comes back fresh. Its state was only ever for the session (PRP-007 amendment 2; CLAUDE.md KNOWN ISSUES), and that stays.
- The empty tab isn't saved; the window still always has a tab.
- `open` is written safely (`store.save`), as every file is, and a broken or newer one is set aside with a notice, as `pins` is.
- Launch order holds: no app starts before its tab, and the new tab, are built from what's restored (session 170).

### Must NOT Do
- Restore unsigned apps' state (unchanged rule).
- Ask on launch whether to restore (decided below, unless the owner says otherwise).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new: restoring builds the same `Model` opening each one would.

## ERROR HANDLING REQUIREMENTS

- `read_open` returns `Unreadable` (`Broken` with the field, or `Newer`), and `saving.restore` sets the file aside with its notice, as for pins.
- A tab that fails to reopen is a failed page in its tab, never a panic.

## SECURITY CONSIDERATIONS

- `open` is Rapidkoi's own file in its data folder, but it's read as hostile input: every field is bounded and checked before use. A path from it is opened only as the person's own earlier choice was: a file Rapidkoi opened before, or an address, which still goes through every trust check.
- Nothing new reaches a guest.

## TESTS TO WRITE

`test/remembered.la`:
- [x] The open file round-trips: paths, instances, labels, seen, current.
- [x] A bad path, instance, label, more than 64 tabs, a repeated (app, instance), and a newer format are each refused.

`test/saving.la`:
- [x] Opening, closing, renaming and switching instances saves the open file; nothing else changing writes nothing.
- [x] Every open signed instance's state is saved when it changes.
- [x] A broken open file is set aside with a notice.

`test/shell.la` (or `test/restore.la`):
- [x] Starting on a data folder with an open file reopens its tabs: the same instances, names and current one, in order.
- [x] A signed instance comes back with its state; an unsigned one comes back fresh.
- [x] A file that's gone reopens as a failed page; the others still open.
- [x] With `./main app` and restored tabs: the restored tabs, then the app (switched to if it's already open).

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits; the `open` file is then just ignored.

## ACCEPTANCE CRITERIA
- [ ] Quit with several instances open; start again: they're all back, named, the same one in front
- [ ] Signed instances keep their state; a missing file shows why in its tab
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean; no file over 500 lines
- [x] CLAUDE.md, DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user:
  1. `./main`, open the counter at `rapidkoi://counter.dafifi.net`, add 3.
  2. **+** in the strip; rename it "Work"; add 1.
  3. Open the quote too. Quit.
  4. `./main`: Counter (3), Work (1) and Quote are back, the same one in front.

## CONFIRMED ON APPROVAL (the owner, session 181: "commit them, then build it")
- **Always restore**, with no "restore your tabs?" question and no setting yet: closing the window keeps your place, as Continue does. A setting can come with the menu.
- **Save every open signed instance's state when it changes.** That's more writes than today, which saves only the one used last. Each is a small file in the data folder, written safely.

## AMENDMENTS (session 181, while building)

- **Reading the open file is `saving.restore_open`, called by `start`, not a `Model` field:** `window.la` is at 499 lines, and what's restored is only needed at launch.
- **`navigate.reopen_all`** rebuilds the tabs: a file through `open_path` and `resumed`, an address through `begin`, which keeps the instance (PRP-033). The one saved as in front is current.
- **Continue switches** to the left-off instance when it's open again (`navigate.instance_open`), since reopening at launch can bring it back: still one tab per app. `test/restore.la` `continue_to_an_app_already_reopened_shows_it`.
- **Tests:** `test/restore.la` (new) for launching and saving; the open file's format in `test/remembered.la`. Saving tests that restart in the same folder now see their tabs reopened, which they allow.
- **Session 182 (the owner: "i open the app and it goes directly to the counter app not new tab"):** launch lands on the empty tab; the instance that was in front becomes `last`, the one to go back to.
