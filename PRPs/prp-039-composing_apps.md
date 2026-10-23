## FEATURE: Composing apps (contract 5, part 3): link fields can be paths into nested state (`editor.task=42`), nested state is kept as named fields, and each part of a Rust app has its own typed messages, wrapped with `.map` on the way up; contract 5 closes.

## OBJECTIVE
From chat with the owner (sessions 197–198 and 203), part 3 of contract 5 (DECISION-049). An app can be split into parts (a list, an editor, pages), each owning its branch of the state tree and its own messages, without one giant `handle` and without fighting links.

Done looks like this:
- A link can carry `?page=task&editor.task=42`, and opening it sets `42` inside `editor`.
- In Rust, `#[derive(State)]` on any struct keeps it as named fields, and `App::editor.then(Editor::task)` is a typed link path.
- `#[derive(Message)]` enums replace string messages: `button("Add").on_press(Msg::Add)`, `self.list.view().map(AppMsg::List)`. A misspelt or missing message is a compile error.

## WHY (the owner's decisions, recorded)
- **All three in one PRP** (the owner): they solve one problem together, and the tasks app needs all three. Only link paths touch the contract.
- **Paths through named fields only,** never list positions (the owner: "a"). Positions shift as items come and go, so old links would point at the wrong item. An item is reached by its ID in a field, as a database row holds another's key.
- **Typed messages** in the shape shown in chat (the owner: "yes"):
  - `type Msg` per app;
  - `Event::Message { msg, value }` typed;
  - `.map(Wrapper)` for parts;
  - `String` still a message type, so a small app can keep strings.
- **Nested link fields chained** (the owner: "a"): `App::editor.then(Editor::task)`. Plain Rust, typed at each step.
- **State is a tree with one owner per piece; other parts hold IDs** (decision 25), so there are no cycles.

## CONTEXT

- Starting state:
  - Contract 4's links name top-level fields only (`src/guest/links.la`, `src/net/address.la`: names `[a-z0-9_]{1,64}`).
  - The Rust `#[derive(State)]` encodes the top-level struct as named fields; its fields use minicbor, so a nested struct is a numbered list.
  - Messages are strings everywhere: `on_press(&str)` and `Event::Message { msg: String }`.
- Ending state:
  - **Contract 5 (`docs/specs/contract-5.md`):** a link field's name is a path: 1 to 4 segments of `[a-z0-9_]`, joined by `.`, at most 64 bytes. It's read and set through nested maps, the same simple-value and same-kind rules applying at the end of the path. A path through anything but a map (a list, a value) reaches nothing: it's left out of the address, and ignored when opening. **Contract 5 closes** with this PRP. After it, any change is a DECISIONS.md entry and a version bump.
  - **Rapidkoi:**
    - `src/net/address.la` (`fields_of`'s names), `src/guest/links.la` (`read_link`'s names; `fields_in` and `with_link` walk paths), `src/data/remembered.la` (an `open` file's link names: read through `read_link`, unchanged).
    - The address box shows `editor.task=42` as written.
  - **The Rust library:**
    - `#[derive(State)]` also writes minicbor `Encode`/`Decode` as named fields, so a `State` struct nested in another is a map.
    - `Field::then` chains link fields (`Field<A, B>` then `Field<B, C>` is `Field<A, C>`, its name `"b.c"`, at most 4 deep).
    - **`Message`:** a trait plus `#[derive(Message)]` (in `rapidkoi-derive`) for enums.
      - A unit variant is written as its name (`"Add"`).
      - A variant with fields is a list, `["Toggle", 3]`.
      - Fields must be `Message` too: `String`, integers and `bool` are.
      - `String` is a `Message`.
    - **`View<M>`** carries its message type:
      - `on_press(M)`, `on_change(M)`, `on_submit(M)`, `on_toggle(M)`;
      - `.map(f)` re-wraps a part's messages;
      - `tabs` takes a `fn(&str) -> M`.
    - **`App`:**
      - `type Msg: Message`;
      - `view(&self) -> View<Self::Msg>`;
      - `handle(self, Event<Self::Msg>)`, where `Event::Message { msg: M, value }` is the decoded message;
      - a message that doesn't decode is an unknown event (nothing changes).
  - **The apps:** counter, quote, the new tab, styled and Settings move to typed messages. `newtab.rkb` and `settings.rkb` are repacked, and `./main` rebuilt.
  - **Docs:**
    - `docs/specs/contract-5.md` (paths, closed);
    - DECISIONS.md: DECISION-049's note that contract 5 is closed;
    - CLAUDE.md (stack: contract 5 closed);
    - MEMORY.md (decision 25);
    - CHANGELOG.md.
- Related existing code: the files above; `guest/rust/rapidkoi/src/{raw,view,state,abi,lib}.rs`, `guest/rust/rapidkoi-derive/src/lib.rs`.
- Lugha spec sections relied on: string `split`, recursion over `Value`. Confirmed while building.
- kivuli API used: none new.
- Open decisions that must be resolved first: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **A path is hostile input,** in a link and in an app's `link` list: segments, depth and length checked; a bad one refused with why (the address box's notice, or the app stopped, as today).
- **Setting a path on a fresh state** follows contract 4's rule at the end of the path (same name, same kind), and changes nothing on the way. A missing branch is never created.
- **Contracts 2–4 are unchanged.** Contract 4 apps' link names stay single segments (a `.` in one is refused, as before).
- **Every app in the repo compiles and behaves as before,** now with typed messages.

### Must NOT Do
- List positions in paths.
- Any other contract change.
- Merging into `redesign` or `main`.

## MODEL AND MESSAGES
- No shell `Model` or `Msg` changes.

## ERROR HANDLING REQUIREMENTS
- `AddressError.BadLink` gains: "A link's field names are lowercase letters, digits and _, joined by ." and "A link's field paths are at most 4 deep".
- `read_link`'s `ContractError.Invalid` gains the same for an app's `link`.

## SECURITY CONSIDERATIONS
- **The guest contract changes (contract 5's links): owner review.** No new host function.
- **Paths are bounded** (4 deep, 64 bytes, 16 fields), and walking them is bounded by the state's existing decode limits.
- No new dependencies. The derive uses the same syn and quote.

## TESTS TO WRITE

`test/address.la`:
- [ ] `?editor.task=42&page=task` parses.
- [ ] A bad segment, a fifth level, `..`, a leading or trailing `.`: each refused with why.
- [ ] `link_of` writes paths as given.

`test/links.la`:
- [ ] `read_link` takes paths for contract 5 and refuses them for contract 4.
- [ ] `fields_in` reads `editor.task` from a nested map, and leaves out a path through a list or a missing branch.
- [ ] `with_link` sets `editor.task` where the fresh state has it with the same kind, and ignores it through a list, a missing branch, or another kind.
- [ ] A contract 5 fixture opened with a path link starts with the nested value set.

`guest/rust` (`cargo test`):
- [ ] A nested `State` struct encodes as a named map and round-trips.
- [ ] `then` builds `"editor.task"`; a list field can't be linked (compile-fail doc test).
- [ ] `#[derive(Message)]`: unit and field variants encode as specified and decode back; a nested enum through `.map` round-trips.
- [ ] `Event::Message` decodes typed; an unknown message is ignored.
- [ ] The apps' tests pass with typed messages.

## ROLLBACK PLAN
- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits. Links go back to top-level fields, and messages to strings.

## ACCEPTANCE CRITERIA
- [ ] A link with `editor.task=42` opens with it set (a fixture, and by eye once the tasks app exists).
- [ ] The counter, quote, the new tab, styled and Settings behave as before.
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines.
- [ ] Docs updated; contract 5 marked closed.

## VALIDATION
- `lughac fmt --check src test`, `lughac test test/`, `lughac build src/main.la -o main`, `cargo test --manifest-path guest/rust/Cargo.toml`
- For the user:
  1. Open the counter, Add, and Settings' controls: each works as before.
  2. The counter's link still opens at its count.
