## FEATURE: Sessions and app windows: launch lands on the new tab with "Restore previous session"; an instance is a window onto its app and lives exactly as long as it's open; an app reopens where you last were.

## OBJECTIVE
From chat with the owner (session 183), from the basics up, replacing PRP-034's automatic reopening. The flowchart agreed in chat:

- **Launch always lands on the new tab.** If the last session had instances open, the new tab offers **"Restore previous session"**: one click brings every one of them back as it was (same app, same name, same screen). Opening anything else first drops that session.
- **An instance is a window onto its app** (the owner: "a, a window onto the app"):
  - It lives exactly as long as it's open. Closing it ends it; nothing lingers.
  - Closing an app's last instance closes the app.
- **An app's data outlives every window.** Reopening a closed app gives one window, **where you last were** (the owner). Until storage (build step 4), an app's data is its first instance's saved state.
- **A quit and a crash leave the same thing behind.** The session is saved as it changes, so the next launch offers the same Restore.

## CONTEXT

- Starting state (PRP-033, PRP-034, session 182):
  - Launch reopens every saved instance in the dock and lands on the empty tab.
  - The `open` file is saved as tabs change, and every open signed instance's state as it changes.
  - Closing an instance closes its tab; its saved state stays.
  - Continue switches to a reopened instance.
- Ending state:
  - **Launch** (`shell.start`): restore Rapidkoi's data, read the `open` file as **the previous session** (`Model.previous: Option<Open>`), and land on the empty tab. Nothing is reopened on its own.
  - **The new tab** (built-in app) gets a new read and action, behind a new built-in permission `restore_session` (PRP-030's model; contract 3, still open for step B):
    - `session` answers `{count, apps: [names]}`, or null when there's nothing to restore;
    - `restore_session` restores it.
    - The Rust new tab shows **"Restore previous session"** (with the count) when there's one; `scripts/newtab.sh` declares the permission.
  - **Restore** (`navigate.reopen_all`, PRP-034) **adds** the previous session's instances to what's open; an instance already open just stays. `previous` is then cleared.
  - **The previous session is dropped** (`previous` cleared, and the saved state of its extra instances, "2", "3"…, deleted) as soon as something else is opened: the address box, a pin, Continue, +. A crash before you choose leaves it for the next launch.
  - **Closing an instance ends it:** an extra instance's ("2", "3"…) saved state is deleted when it closes. The first instance's state is the app's data and stays.
  - **Where you left off is the app:** its first instance, where you last were. Closing the app's last instance, whichever it was, leaves the app in "where you left off". Continue, a pin or its address reopens one window from the app's data.
  - **`window.la` split (FILE SIZE RULE: at 499 lines, it can't take `Model.previous`):** `toggle_pin`, `pin`, `is_pinned` and `forget` move to `src/window/pins.la`, unchanged; callers' imports follow.
  - DESIGN.md (the flowchart, in words), CLAUDE.md, MEMORY.md (decision 23 amended), CHANGELOG.md; PRP-034 marked superseded on reopening.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Saving stays continuous (PRP-034): the `open` file and every open signed instance's state are saved as they change, so a crash is a quit.
- Restore brings back each instance with its name, its screen (a signed one's saved state) and the one in front, in order.
- Dropping the previous session never touches an app's data (first instances' state), pins, trust or permission settings.
- The new tab's `session` read and `restore_session` action are answered only for a page holding `restore_session`, which only the built-in new tab can (PRP-030).

### Must NOT Do
- Reopen anything at launch without the person's click.
- Keep a closed extra instance anywhere.
- A list of instances in the app's settings (not needed: what's open is all there is).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `Model.previous: Option<Open>`: the last session, until restored or dropped.
- No new `Msg`: `restore_session` arrives as the new tab's ask, and restoring is `navigate.reopen_all` from the answer.

## ERROR HANDLING REQUIREMENTS

- A previous session's instance that can't come back shows why in its tab, as in PRP-034.
- A failure deleting a dropped or closed instance's state is a notice, never a crash.

## SECURITY CONSIDERATIONS

- **A new built-in permission and two asks exposed to a guest: needs the owner's review (SECURITY RULE).** It reads only the previous session's app names and count, and restores only what the person had open. Built-in only, Rapidkoi's own new tab.
- The `open` file is still read as hostile input (PRP-034).

## TESTS TO WRITE

`test/restore.la`:
- [x] Launch with a saved session: no instance reopened, the empty tab current, `previous` holding it.
- [x] The new tab's `restore_session` reopens it all (names, order, the one in front); `previous` is cleared.
- [x] Opening something else first drops it: `previous` cleared, its extra instances' state files deleted, first instances' kept.
- [x] Restore adds: an app opened before Restore stays, with no second window for an instance already open.

`test/instances.la`:
- [x] Closing an extra instance deletes its saved state; closing the first keeps the app's data.
- [x] Closing the app's last instance leaves the app in where you left off; reopening it gives one window from the app's data.

`test/effects.la`, `test/permissions.la`:
- [x] `session` and `restore_session` need `restore_session`; the answer's count and names; null with no previous session.

`guest/rust` (`cargo test`):
- [x] The new tab shows "Restore previous session" only when there's a session; the button asks to restore.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits (back to PRP-034's reopening).

## ACCEPTANCE CRITERIA
- [ ] Launch shows the new tab; with a previous session, "Restore previous session" brings it all back; ignoring it starts fresh
- [ ] Closing an extra instance ends it; the app's data survives every window; a reopened app is where you last were
- [ ] A crash, then launch, offers the same Restore
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines
- [x] DESIGN.md, CLAUDE.md, MEMORY.md, CHANGELOG.md updated

## VALIDATION
- For the user:
  1. `./main`, open the counter, add 3, **+** for Counter 2, add 1. Quit.
  2. `./main`: the new tab, with "Restore previous session (2)". Click it: both back, at 3 and 1.
  3. Quit, `./main`, and type an address instead: the session is dropped. Open the counter: one window at 3.

## CONFIRMED ON APPROVAL (the owner, session 184: "yes approved", the security review and the split included)
- Restore is offered until something else is opened (as Firefox does), not for the whole session.
- Restore adds to what's already open; an instance already open just stays.

## AMENDMENTS (session 184, while building)

- **`Msg.RestoreSession`** exists after all: the new tab's `restore_session` ask sends it with `Cmd.after(0, …)`, as `open_pin` sends `OpenPinned`. Restoring is window work, done in `update`.
- **Dropping the session** is `saving.drop_session`, run when a message adds a tab while `previous` is set. Its extra windows' state is found by host (`states.drop_host_instance`), since a path doesn't carry the key.
- **An app named on the command line is part of launch:** it doesn't drop the session. That's the case where "Restore adds" applies; otherwise opening anything drops it first.
- **Where you left off is the app:** `leftoff.as_app` (first instance, no state or program of its own) when an extra window closes or is the one used last, so its state never overwrites the app's data.
- **A closed extra window's state** is deleted in `persist` (`states.drop_state_in`).
- **`src/data/saving.la` is at 500 lines,** at the limit: the next change there needs a split first.
- **Session 186, the owner's model** (replacing "dropped as soon as something else is opened", which would lose sessions by accident):
  - The previous session stays on offer all session.
  - Restore stacks it under the current one, which wins where they overlap.
  - An unrestored previous session is replaced at the next launch by the one after it, and its extra windows' state goes then.
  - The owner's exception: an empty session never replaces one.
  - Built as `src/data/sessions.la` (`begin_session`, `restored`, the `previous` file); `saving.drop_session` removed.
- **Session 187:** "the current wins" is per app, not per window (the owner: an app with 3 windows in the old session and 1 now came back with all 3). An app open now keeps only its current windows; an app that isn't open comes back with all of its own. `navigate.reopen_all`.
- **Session 188:** the first instance is the app, so it can't be closed on its own: no × or Close in the strip, and `instances.close_instance` on it closes every window (the dock's × or Ctrl+W on it close the app).
