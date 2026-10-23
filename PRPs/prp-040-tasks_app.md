## FEATURE: The tasks app (build step 3): a real app built only on Rapidkoi and contract 5. A list with All · Open · Done in its link, adding with Enter, a task's own screen at `editor.task=42`, and Delete asked first.

## OBJECTIVE
From chat with the owner (sessions 197 and 205): step 3 of the vision, "a notes or tasks app built only on Rapidkoi, to shape the component library". It proves contract 5 and the Rust library on something that isn't a demo.

Done looks like this. Open the tasks app:
- Type "Buy milk" and press Enter: it's on the list.
- Tick it: its title dims.
- Pick Open: the address shows `?page=list&list.filter=open`, and only open tasks show.
- Press a task: its screen opens at `?page=task&editor.task=2`, with its title, notes and Done.
- Delete asks first.
- Copy the address, open it in a new tab: the same screen, the same task.
- Reload and Restore keep everything.

## WHY (the owner's decisions, recorded)
- **Tasks rather than notes or chat** (session 197): its pieces (lists, rows, forms, tabs, confirming) serve most apps. Notes would need rich text first; chat needs servers to reach apps.
- **Scope** (session 205):
  - **In:** the three screens; one list.
  - **Out:** due dates, reordering, search.
  - **Where:** `guest/rust/tasks`, run locally. Publishing it is a later step, asked first.
- **Its look** (session 205): only Rapidkoi's names, so it follows light/dark, contrast and text size; the sketch below. A done task shows by its tick and dimmed title: contract 5 has no strikethrough, and it's closed.

## CONTEXT

- Starting state:
  - Contract 5 is complete and closed (PRP-037 to PRP-039).
  - The library has `tabs`, `checkbox`, `input` with `on_submit`, `text_area`, `button`, `spacer`, `row`, `column`, `.style`, typed messages and `.map`, nested `State`, and `then` link paths.
- Ending state:
  - **`guest/rust/tasks` (new),** contract 5:
    - **State:** `page` (`"list"`, `"task"` or `"delete"`), `list: TaskList { filter, draft, tasks: Vec<Task> }`, `editor: Editor { task: u32 }`, `next_id`. A `Task` is `{ id, title, notes, done }`. Each struct derives `State`.
    - **Messages:** `AppMsg { List(ListMsg), Editor(EditorMsg) }`.
      - `ListMsg { Filter(String), Draft, Add, Toggle(u32), Open(u32) }`.
      - `EditorMsg { Back, Title, Notes, Done, Delete, Confirm, Cancel }`.
      - The list's and the editor's views are their own functions, joined with `.map`.
    - **Links:**
      - the list names `page` and `list.filter`;
      - a task or its delete screen names `page` and `editor.task`.
      - The app checks a link in `on_open`: a task that doesn't exist goes back to the list.
    - **Rules:**
      - an empty or all-space title isn't added;
      - a title is at most 200 characters, and notes at most 10,000;
      - IDs only grow, never reused.
  - **`test/tasks.la` (new):** end to end through the window, like `test/counter.la`, skipped until the app is built: adding, ticking, filtering, opening a task, Delete asked first, and opening its link in a new window.
  - **Docs:**
    - CLAUDE.md (tree, commands: build, pack and serve the tasks app);
    - MEMORY.md (project state: step 3);
    - CHANGELOG.md;
    - `docs/source/` nothing new;
    - any library gaps found go in DECISIONS.md, not worked around.
- Related existing code: `guest/rust/{rapidkoi,counter,settings}`, `test/counter.la`, `docs/specs/contract-5.md`.
- Open decisions that must be resolved first: none.

## THE LOOK (the owner, session 205)
```
 Tasks
 [ All ] [ Open ] [ Done ]                    tabs, the selected one in accent
 ┌──────────────────────────────────────────┐
 │ What needs doing?                        │ Enter adds
 └──────────────────────────────────────────┘
 ┌──────────────────────────────────────────┐
 │ ☐  Buy milk                            › │ a row: surface, medium corners, hover a step lighter
 │ ☑  Call the bank                       › │ done: its title text_dim
 └──────────────────────────────────────────┘
 3 open · 1 done                               text_dim, small
 (no tasks: "No tasks yet")
```
```
 ‹ Tasks                                         subtle
 [ Pay rent                                ]     title
 [ Notes…                                  ]     several lines
 ☐ Done
                                   [ Delete ]    danger
```
```
 Delete "Pay rent"?
 [ Cancel ]  [ Delete ]                          danger
```

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **Only contract 5 and the library:** no shell change.
- **Every value from a link is the app's to check:** an unknown filter means All; a missing task, the list.
- **The checkbox and the row are separate presses:** ticking doesn't open the task.

### Must NOT Do
- Due dates, reordering, search, several lists.
- Persisting tasks beyond the window's state (storage is step 4).
- Changing contract 5.
- Publishing to the VM (asked first, separately).

## MODEL AND MESSAGES
- No shell changes. The app's own messages are as above.

## ERROR HANDLING REQUIREMENTS
- The app never panics on input:
  - over-long text is cut to the limit;
  - a link's bad values fall back (unknown filter → All, missing task → the list).

## SECURITY CONSIDERATIONS
- No new capability, no network, no permissions: the app asks Rapidkoi for nothing.
- No new dependencies.

## TESTS TO WRITE

`guest/rust/tasks` (`cargo test`):
- [x] Adding trims and refuses an empty title; IDs grow.
- [x] Toggling flips done.
- [x] The filter shows the right tasks, and an unknown filter means All.
- [x] Opening a task links `page` and `editor.task`; Back links the list's.
- [x] Delete goes to confirm; Cancel returns; Confirm removes the task and returns to the list.
- [x] A link to a missing task lands on the list (`on_open`).
- [x] Title and notes are cut at their limits.

`test/tasks.la` (skips until built):
- [x] Add two tasks with Enter; tick one; pick Open: only the other shows, and the address has `list.filter=open`.
- [x] Open a task: its screen, the address has `editor.task=…`.
- [x] Delete asks; Delete removes it.
- [x] That task's link opened in a new window shows the same task.

## ROLLBACK PLAN
- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits (the app is new and self-contained).

## ACCEPTANCE CRITERIA
- [ ] Everything in OBJECTIVE works by eye, in dark and light, at 100% and 150%.
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); fmt clean; build ok.
- [x] Docs and CHANGELOG.md updated; any gap found recorded in DECISIONS.md.

## VALIDATION
- `cargo test --manifest-path guest/rust/Cargo.toml`, `lughac test test/tasks.la`
- For the user:
  ```
  cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p tasks
  ./main pack guest/rust/target/wasm32-unknown-unknown/release/tasks.wasm --name Tasks --version 0.1.0 -o tasks.rkb
  ./main serve tasks.rkb --port 8001 &
  ./main rapidkoi://localhost:8001
  ```
  Add, tick, filter, open, delete; copy the address into a new tab.

## AMENDMENTS (session 206, while building)

- **A task's link opens that task only in its own window, for now.**
  - A link opens a new window, which starts fresh, and its tasks live in the first window's state until storage (step 4). So in a new window, `editor.task=2` finds no task 2 and the app lands on its list, as its `on_open` rule says. The filter's link (`list.filter=open`) works anywhere.
  - "Copy the address into a new tab: the same task" in OBJECTIVE and VALIDATION waits for storage. `test/tasks.la` checks what's true today.
  - This is the first real evidence for step 4: a link to data needs the data to outlive the window.
- **The tick on the list has no visible words:** the row's title opens the task, so the tick is a bare checkbox labelled for screen readers (`label`).
  - kivuli.testing can't find a button by that label (DECISION-050), so the end-to-end test ticks a task through its own screen's Done.
  - The library's unit tests cover the list's tick.
- **The state's ID field is `last_id`** (the last ID given), so IDs start at 1.
- **No library gaps found** besides strikethrough (contract 5 has none, as agreed) and screen-reader labels (DECISION-050).

