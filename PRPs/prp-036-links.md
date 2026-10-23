## FEATURE: Links (build path step 2): a window's state is named fields Rapidkoi can read (contract 4); after each event the app names which fields go in the address; opening such a link opens a new window with those fields set.

## OBJECTIVE
From chat with the owner (sessions 190–191), from the basics. A link is an app's address plus the part of a window's state the app chooses to show, as text:

```
rapidkoi://counter.dafifi.net/?count=3
```

Press Add three times and the address box shows that. Open it, and a new counter window starts at 3. The developer decides, in the same line that returns the new state, which fields go out; their values are always read from the state, so the address can't drift from it. Only simple values (numbers, text, true/false) can go in a link.

## WHY (the owner's decisions, recorded)

- **A link is preserved state in text** (the owner), limited to what the app chooses, so private data and secrets never leave the window.
- **Chosen when the new state is returned,** not declared at the struct's definition: the link can depend on what just happened. Fields are named, not valued (the owner: "two places for things to diverge").
- **State only changes when the app answers an event** (or Rapidkoi puts a whole state in place), so the address is read off at exactly those moments, with no separate "update the link" call.
- **A link opens a new window** (the owner).
- **Simple values only** (the owner), enforced by the Rust library at compile time and by Rapidkoi at run time.
- **State as named fields, contract 4** (the owner): Rapidkoi has to read the fields to build the address. Contract 3 stays sealed bytes, so published contract 3 apps keep working (rule 6).

## CONTEXT

- Starting state:
  - Contract 3 (PRP-030): state is opaque bytes the app encodes (`handle(state, event) → {state, effects}`).
  - Addresses name only an app.
  - Opening an address of an app already open switches to its last-used window (PRP-026, PRP-033).
  - The share button is disabled ("comes with links").
- Ending state:
  - **Contract 4** (`docs/specs/app-events.md` and a new `docs/specs/contract-4.md`): DECISIONS.md entry and bump.
    - A window's state is a CBOR map with text keys at the top level; values may be anything.
    - `handle`'s result may carry `link`: a list of top-level field names, or null to clear. Absent means unchanged.
    - `on_open` comes after Rapidkoi has set the link's fields in the fresh state. The app checks them there (its "not found" is its own).
    - Contracts 2 and 3 run unchanged, with no links.
  - **Rapidkoi** (`src/guest/`, `src/net/address.la`, `src/window/`):
    - Parses `?name=value&…` after an address, within limits (16 fields, a 64-byte name, a 1024-byte value, 4 KB in all).
    - Builds the address after each event from the named fields' current values, checking each is a simple value (number, text, bool, null).
    - Shows `host/?count=3` in the address box.
    - Opens a link in a **new window**: a link with fields always opens a new window. A bare address of an open app still switches to it.
    - Sets fields from the link only where the fresh state (after `on_init`) has that name with a value of the same kind. Anything else is ignored.
    - `AppPage.link: string[]`: the window's current link fields, kept with the window (and in the `open` file, so Restore keeps its address).
  - **The Rust library** (`guest/rust/rapidkoi`, a new proc-macro crate `guest/rust/rapidkoi-derive`, with the reason in SECURITY below):
    - `#[derive(State)]` encodes the struct as named fields and generates its link fields (only simple-typed fields).
    - `Next::state(s)`, `.link(&[Counter::count])`, `.no_link()` and `.ask(…)` replace the `(state, asks)` pair. The old form keeps working for contract 3 apps.
    - The library speaks contract 4.
  - **The apps:** the counter uses links (`?count=3`); quote and the new tab move to contract 4. Republishing the counter and quote on the VM is a separate step, asked first (outward-facing).
  - **The share button** stays disabled: kivuli has no clipboard command. That's DECISION-047, with a request at `docs/specs/kivuli-clipboard.md`. The address box's text can be selected and copied meanwhile.
  - Docs: DESIGN.md (the address shows the link), CLAUDE.md (stack: contract 4), MEMORY.md (a links decision), CHANGELOG.md.
- Related existing code: `src/guest/contract.la`, `events.la`, `running.la`, `src/net/address.la`, `src/window/navigate.la`, `src/ui/addressbox.la`, `guest/rust/rapidkoi/src/{abi,lib}.rs`.
- Open decisions: none. DECISION-047 (clipboard) only keeps share disabled.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- The address never holds a field the app didn't name, and never a value that isn't the state's own.
- An incoming link is hostile input. Bound it before reading; set only fields the fresh state has with the same kind of value; ignore the rest. A bad link never stops the app.
- Link fields are simple values only: Rapidkoi refuses anything else in a name list (the app stops with its sentence: "This app put a list in its address").
- A link opens a new window (an instance, PRP-033), never replacing the current one.
- The link fields survive Reload and Restore with the window.

### Must NOT Do
- Cross-app links: an app opening another's link, the `open_apps` permission for external apps (always ask), the "‹ Mail" return. Those are the next PRP (proposed: PRP-037).
- Copying to the clipboard (DECISION-047).
- The web form (`https://open.rapidkoi.app/…`), `@version` links.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `AppPage.link: string[]`: the current link fields.
- No new `Msg`: opening a link is the address box's `Go`, or `./main <link>`.

## ERROR HANDLING REQUIREMENTS

- `ContractError` gains the link rules: names not text, too many, a non-simple value.
- `AddressError` gains the query rules: too long, too many fields, a bad name.
- Each is a sentence on the app's error screen, or the address box's notice, never a crash.

## SECURITY CONSIDERATIONS

- **The guest contract changes (contract 4): owner review.** No new host function: links ride in `handle`'s existing result and `on_open`.
- Incoming links are untrusted. They're bounded and type-matched against the app's own fresh state; meaning is the app's to check.
- New dependency: `rapidkoi-derive`, a proc-macro crate inside the workspace, needed because Rust derives must live in their own crate. It runs at the developer's build, not in Rapidkoi.

## TESTS TO WRITE

`test/address.la`:
- [ ] `?count=3&filter=work` parses; limits; a bad name.

`test/links.la` (new):
- [ ] The address after an event shows the named fields' current values.
- [ ] Absent keeps the list; null clears it; a non-simple field stops the app.
- [ ] Opening a link: a new window; fields set when the fresh state has them with the same kind; others ignored; a bare address of an open app still switches.
- [ ] Reload and Restore keep the window's link.
- [ ] Contract 3 apps: no links, unchanged.

`test/contract.la`, `test/events.la`:
- [ ] Contract 4's state as a map; `link` in the result.

`guest/rust` (`cargo test`):
- [ ] `#[derive(State)]` round-trips; only simple fields get link names (a `Vec` field doesn't compile as a link: a compile-fail test).
- [ ] `Next` builds the result; the counter's link follows its count.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits (contract 4 is then unknown, and its bundles are refused as "needs a newer version").

## ACCEPTANCE CRITERIA
- [ ] Add three times: the address box shows `counter…/?count=3`; opening that address gives a new window at 3
- [ ] Contract 2 and 3 apps run as before
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines
- [ ] DECISIONS.md (contract 4, DECISION-047), docs, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`, open the local counter (rebuilt, contract 4), Add ×3: the address shows `?count=3`. Copy it from the box, then **+** for a new tab and paste it: a second window at 3.
