## FEATURE: Named instances: an app can be open more than once, each instance named and keeping its own state, switched from a strip in the header; the sidebar stays one entry per app.

## OBJECTIVE
From chat with the owner (sessions 175–176). People need the same app on different screens. The bedrock:
- **Instances.** An app can be open several times. Each instance has a name ("Notes", "Notes 2", renamed to "Work") and its own state, a namespace that's easy to restore.
- **The sidebar is a dock:** one entry per app, however many instances it has, with a small count when there's more than one.
- **The header's start**, where ← ⌂ → were (PRP-032), shows the current app's instances as a strip:
  - one button per instance, the current one marked;
  - **+** for a new instance of the same app;
  - ⋯ when they don't fit.

Panes and several windows come later, on top of this (the owner: "laying the bedrock").

## WHY (the owner's discussion, recorded)

- Several tabs per app would crowd the dock, and mixes one app's screens with everything else. A strip scoped to the current app keeps the dock short and the switching local.
- Names, not titles: an app's screen title changes as its screen changes, so it can't name an instance. Rapidkoi names them by number, and the person can rename them.
- Panes (two instances side by side in one window) were weighed and deferred. People can use several windows for that, which needs kivuli (DECISION-042, below).

## CONTEXT

- Starting state:
  - One tab per app (PRP-026): `window.open_tab_of`, `same_app`. The sidebar shows pinned apps, then the other open apps (PRP-023/028). The header's start is empty (PRP-032).
  - A signed app's state is saved per app, under its identity (`data/states.la`, `state_name(host, key)`).
  - "Where you left off" (`LeftOff`) and pins hold a path.
- Ending state:
  - `src/window/tabs.la`: `Tab.instance: string`, the instance's name. A `Tab` is now one instance of an app; tabs of the same app (`same_app`) are its instances.
  - `src/window/window.la`: `instances_of(m, id)` (the current tab's app's instances, in creation order); `new_instance(m)`; `rename(m, id, name)`; `next_name(m, app)` (the app's name, then "{name} 2", "{name} 3": the lowest number free).
  - `src/window/navigate.la`: opening an app that's open switches to the instance used last, as today (PRP-026). Only the strip's **+** makes a second instance.
  - `src/ui/sidebar.la`: one entry per app (its last-used instance's), with a count when it has more than one instance. Clicking it shows that app's last-used instance. Its × and Close close the app's every instance.
  - `src/ui/instances.la` (new, pure): the header's strip.
    - One button per instance of the current app, its name, the current one marked.
    - **+** ("New instance").
    - Each instance's right-click menu: Rename…, Close.
    - The ones that don't fit, behind ⋯.
    - Nothing on the empty tab, and nothing while the app has a single instance, beyond **+**.
  - `src/ui/header.la`: the bar's start is the strip.
  - **Rename** is a small dialog (Rapidkoi's own), an input pre-filled with the name, then Rename and Cancel. A name is 1–64 characters, unique within the app. An empty or taken name isn't accepted, and the dialog says why.
  - **State per instance** (`data/states.la`): a signed app's state is saved per instance: `state_name(host, key, instance)`. The first instance keeps today's file name, so today's saved state is the first instance's and nothing moves.
  - `LeftOff.instance`: where you left off is an instance ("Notes › Work"); Continue reopens it by name, with its state.
  - Pins stay per app: a pin opens the app's last-used instance, or the first ("Notes") when none is open.
  - `src/window/actions.la`: `Msg.NewInstance`, `Msg.RenameInstance(i64)` (open the dialog), `Msg.InstanceName(string)` (typing), `Msg.ConfirmRename`, `Msg.SelectInstance(i64)`.
  - DECISIONS.md: DECISION-042 (deferred, kivuli): several Rapidkoi windows, since a second `./main` only raises the first (DECISION-014). Request: `docs/specs/kivuli-several-windows.md`.
  - MEMORY.md (a decision: instances), DESIGN.md (the strip, the dock count), CLAUDE.md (tree), CHANGELOG.md.
- Related existing code: `src/window/window.la` (`open_tab_of`, `same_app`, `close`), `src/ui/sidebar.la`, `src/data/states.la`, `src/window/leftoff.la`, `src/ui/settings.la` (the dialog pattern).
- Open decisions: none. DECISION-042 doesn't block this.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Instances don't share data. Each is the app with its own whole state, until storage (build step 4) moves data out of the instance. Say so in DESIGN.md: "Notes › Work" and "Notes › Personal" have different notes today.
- Each instance is its own guest (its own store, memory and fuel), under every existing limit.
- One tab per app still holds for **opening**: the address box, a pin or Continue switch to an open instance, never start another. Only **+** in the strip makes a second.
- A name is the person's: Rapidkoi gives numbered defaults and never renames an instance itself.
- Closing an app's last instance closes the app (it becomes where you left off, as today).

### Must NOT Do
- Panes or several windows (deferred).
- Name instances from screen titles.
- Restore which instances were open at launch: open tabs aren't restored yet (CLAUDE.md KNOWN ISSUES; its own PRP).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `Tab.instance: string`.
- `Model.renaming: Option<(i64, string)>`: the instance being renamed and the name typed so far.
- `LeftOff.instance: string`.
- Messages: `NewInstance`, `SelectInstance(i64)`, `RenameInstance(i64)`, `InstanceName(string)`, `ConfirmRename`, and `Cancel` (existing) closes the dialog.

## ERROR HANDLING REQUIREMENTS

- A rename that's empty, longer than 64 characters or already taken in the app isn't applied. The dialog shows the reason under the input, and Rename is disabled.
- A saved state for an instance that's gone is removed like any other (`states`' cap, Forget).
- Forgetting an app drops every instance's state.

## SECURITY CONSIDERATIONS

- Instance names are the person's text, shown in the frame. They're never given to the app (no new host function), and they're stored in Rapidkoi's data folder.
- The state file name hashes the identity and the instance name, so a name is never used in a path.

## TESTS TO WRITE

`test/instances.la` (new):
- [x] `next_name`: "Notes", then "Notes 2", "Notes 3"; a closed "Notes 2" is reused.
- [x] **+** makes a new instance of the current app, current, with the next name; its guest is its own (an event reaches only it).
- [x] The strip lists the current app's instances in creation order, the current one marked; **+**; nothing on the empty tab.
- [x] Rename: the dialog opens pre-filled; a new name applies; an empty, too-long or taken name doesn't, with its reason; Cancel closes it.
- [x] Opening an open app (the address box, a pin, Continue) switches to its last-used instance; only **+** makes another.
- [x] Closing the last instance closes the app (left off); closing one of several shows a neighbour of the same app.

`test/sidebar.la`:
- [x] One entry per app with several instances, its count shown; clicking it shows the last-used instance; its × closes every instance.

`test/states.la`:
- [x] Each instance's state round-trips under its own name. The first instance's file name is today's.

`test/leftoff.la` (or `test/saving.la`):
- [x] Where you left off keeps the instance's name; Continue reopens that instance with its state.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits.

## ACCEPTANCE CRITERIA
- [ ] **+** in the header opens another instance of the current app; the strip switches between them; each keeps its own state
- [ ] Instances can be renamed; names and states survive closing and Continue
- [ ] The sidebar shows one entry per app, with a count
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean; no file over 500 lines
- [x] DECISIONS.md (DECISION-042), MEMORY.md, DESIGN.md, CLAUDE.md, CHANGELOG.md updated

## VALIDATION
- For the user:
  1. `./main`, open the counter, add 3.
  2. **+** in the header: "Counter 2" starts at 0.
  3. Switch between them in the strip: each keeps its count.
  4. Right-click "Counter 2" → Rename… → "Work". The sidebar shows one Counter with "2".

## CONFIRMED ON APPROVAL (the owner, session 177: "a. yes")
- A pin is the app (it opens the last-used instance); where you left off is an instance.
- Default names are numbered ("Notes", "Notes 2"), renamed by the person; Rapidkoi doesn't ask for a name on **+**.

## AMENDMENTS (session 177, while building)

- **A stable key, separate from the name:** `AppPage.instance` ("" for the first, then "2", "3"…) keys the instance's saved state, and `AppPage.label` is the name the person gave. That way renaming never moves a file. `display_name` shows the label, or the app's name and number. The fields sit on `AppPage`, not `Tab`, so leaving, saving and background loads carry them. `AppPage.seen` marks the last-used instance.
- **A new instance starts from the app's compiled program** (`instances.another`), as Restart does: no reload from disk or the network.
- **The UI module is `src/ui/strip.la`** (`strip.view`, `strip.rename_view`). `test/instances.la` imports `window.instances` too, and two imports can't share a name.
- **The dock's ×:** the current entry's × (Ctrl+W) closes the instance in front, so one Ctrl+W meaning holds. Another entry's ×, and the menu's "Close all" (for more than one instance), close every instance. There's no `SelectInstance`: the strip sends `Select(id)`.
- **States:** `instance_name`, `keep_in`, `saved_state_in`, `drop_app` (Forget drops every instance), and `move_state` moves every instance on a rotation. The first instance's file is unchanged.
- **The left-off file** stores `instance` (when not "") and `label`, and a read checks them.
