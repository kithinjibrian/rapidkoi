## FEATURE: Time and name parts (contract 8): apps show times and your name that Rapidkoi fills in, without learning your time zone or your name

## OBJECTIVE
An app can show "Edited 5 minutes ago", "Thu 31 Oct, 14:32" or "Hello, Brian" without ever knowing your time zone or your name: it hands Rapidkoi a text made of parts, plain strings and placeholders, and Rapidkoi draws the words in your zone just before showing them. "5 minutes ago" moves on by itself. An app can ask what time it is, and gets UTC. Every app moves to contract 8; contract 7 apps are refused, as older ones are now.

## CONTEXT
- From chat with the owner (2026-10-31): core decision 30 and its amendments (the show pattern; "anything worked out from the person's values may only become text"), DECISION-083 (name tokens become parts, in the contract that brings time), DECISION-086 (time zones needed from Lugha; shipped in lughac's milestone 145, PRP-176). The owner: "let's do time plus name parts".
- **Starting state:** names are `{FIRST_NAME}` and `{LAST_NAME}` scanned out of every text (`src/guest/placeholders.la`); apps have no clock; contract 7 is the only one (DECISION-056).
- **Discovery answers (the owner):**
  - Q1: parts on `text` nodes; `{ "name": "first" | "last" }`; `{ "time": <ms UTC>, "show": "time" | "date" | "date_time" | "relative" | "weekday" }`; a `now` ask; 24-hour clock and day before month; the picker, the time zone permission and schedules later. ("yes")
  - Q2: offsets worked out in `update`, never in `view`, kept on the model; added to CLAUDE.md's `update` exceptions; a `Parts` node; `now` answered through `on_answer`; no new messages. ("yes")
  - Q3: bad parts stop the app with its sentence; a failed local offset shows that moment in UTC with " UTC". ("yes")
  - Q4: the relative table, other years, a 60-second tick only while the front tab shows a time, the screen key, parts only on `text`. ("yes")
  - Q5: the files below; offsets on `Model`, since `window/tabs.la` is at 500 lines. ("yes")
  - Q6: what it must not touch, below. ("nothing to add")
  - Q7: the order of work below. ("yes")
  - Q8: `now` to the second, the clock-error leak recorded; the owner's review before merging. ("yes")
  - Q9: the tests and checks below. ("yes")
- **Files read first:** `docs/specs/contract-7.md`, `src/guest/limits.la`, `src/guest/nodes.la`, `src/guest/tree.la`, `src/guest/placeholders.la`, `src/guest/effects.la`, `src/ui/content.la` (`screen_key`, where names are filled), `src/window/window.la`, `src/shell.la` (`update`, `subscriptions`), `test/support.la`, `docs/DESIGN.md`.
- **Files created:** `src/guest/times.la`, `src/window/zones.la`, `docs/specs/rapidkoi-rust-contract-8.md`, `test/times.la`, `test/zones.la`; `docs/specs/contract-8.md` (renamed from `contract-7.md` with `git mv`).
- **Files changed:** `src/guest/limits.la`, `src/guest/nodes.la`, `src/guest/tree.la`, `src/guest/placeholders.la`, `src/guest/effects.la`, `src/window/window.la`, `src/ui/content.la`, `src/shell.la`; `test/support.la` and the tests whose fixtures say contract 7 (`access`, `bundle`, `events`, `links`, `verify`, `loader`, `signing`, `shell`, `frame`), and the tree, contract, placeholders and effects tests; `CLAUDE.md`, `DECISIONS.md`, `CHANGELOG.md`; then `guest/rust` and `assets/apps/*.rkb` (step 3 below).
- **Relied on:** `std.time` (`lughac spec`, milestones 115 and 145): `now_ms()`, `civil(ms, offset_s): Result<Civil, TimeError>`, `local_offset(ms): Result<i64, TimeError>` (seconds east of UTC, `TZ` else `/etc/localtime` else UTC), `Civil { year, month, day, weekday (1 Monday – 7 Sunday), hour, minute, second, ms }`. kivuli's `Sub.every` (already used in `shell.la`).
- **Decisions:** DECISION-083 and DECISION-086 resolved by this PRP; DECISION-085 (open) untouched. None open blocks it.

## REQUIREMENTS

### Must Do
- **Parts:** a `text` node has `text` (a plain string, as now) or `parts`, never both, never neither. A part is a string, `{ "name": "first" | "last" }`, or `{ "time": ms, "show": … }`. At most `MAX_PARTS` (64) parts a text; plain strings count against the existing text and screen limits. `time` a whole number of milliseconds within years 1 to 9999 (UTC); `show` one of `time`, `date`, `date_time`, `relative`, `weekday`. Anything else is a decode error: the app stops with its sentence.
- **Drawn in your zone, 24-hour clock, day before month:**
  - `time`: "14:32"; `date`: "Thu 31 Oct" (with the year when not this year: "Thu 31 Oct 2023"); `date_time`: "Thu 31 Oct, 14:32"; `weekday`: "Thursday".
  - `relative`, on your local calendar: under a minute either way "just now"; 1–59 minutes "1 minute ago" / "5 minutes ago" / "in 5 minutes"; same day, an hour or more, "3 hours ago" / "in 3 hours"; the day before or after "yesterday" / "tomorrow"; within six days "Thu" / "next Thu"; further this year "Thu 24 Oct"; another year "24 Oct 2023".
  - A moment whose local offset the system can't give is drawn in UTC with " UTC" after it; no notice; tried again on the next tick.
- **Names:** a `name` part is the name this app may show (`people.name_in`), "" when unset. The `{FIRST_NAME}` and `{LAST_NAME}` tokens go: a plain string's braces are shown as typed.
- **Where parts fill:** `text` nodes only; never typing boxes, titles or `label`s.
- **Offsets:** `Model.offsets: Map<i64, i64>` (moment → seconds east of UTC). At the end of `update`, `zones.settle` looks up each moment the front tab's screen shows that has no offset yet; the tick refreshes them all. `time.local_offset` is called in `window/zones.la` only, never in `view`.
- **The tick:** `Sub.every(60_000, …)` only while the front tab's screen shows a time part; it sets `m.now` and refreshes offsets, and never calls the app.
- **Redrawing:** `screen_key` includes the current minute and the screen's offsets when its screen has time parts, and the name when it has name parts.
- **`now`:** a new ask, `{ "kind": "now", "msg" }`, answered with `on_answer` and `value` the UTC time in milliseconds, rounded down to a whole second.
- **Contract 8:** `CONTRACT_VERSION` 8; contract 7 apps refused with the existing sentence; `docs/specs/contract-8.md` describes parts and `now`, the rest as contract 7.
- **Records:** CLAUDE.md: "The contract is version 8", and the `update` exception "reading the system's time zone offset (`time.local_offset`, in `window/zones.la`), which the C library caches". DECISIONS.md: a contract 8 entry, DECISION-083 and DECISION-086 resolved, the clock-error leak (core decision 30's accepted leaks). CHANGELOG.
- **The request for `rapidkoi-rust`** (`docs/specs/rapidkoi-rust-contract-8.md`): `text` from parts (`text(["Hello, ", first_name(), "!"])`), `first_name()`, `last_name()`, `time(ms)` with `.time()`, `.date()`, `.date_time()`, `.relative()`, `.weekday()`, and `Effects::now(msg)`.

### Must NOT Do
- Let any filled-in value, offset or name reach an app: no event, answer, link, saved state or anything else carries one (core decision 30).
- Run anything an app sends: parts are data.
- Keep contract 7: one contract (DECISION-056).
- Change the rest of the contract (links, storage, styles, pictures, roles, events).
- Add the person's clock or date-order settings, the date picker, the time zone permission or schedules (later PRPs).
- Call the app from the tick.
- Touch `src/trust/`, keys, identity, sync, `src/window/tabs.la`, or a sibling repository (`rapidkoi-rust` changes in its own, from the request).
- Add a dependency or `extern`.

## DESIGN
- **`guest/nodes.la`:** `Node.Parts(Part[], Look, Facts)` beside `Words`; `enum Part { Plain(string), Name(Which), Time(i64, Show) }`, `enum Which { First, Last }`, `enum Show { Time, Date, DateTime, Relative, Weekday }`. `Effect.Now(Value)`.
- **`guest/limits.la`:** `CONTRACT_VERSION: i32 = 8`, `MAX_PARTS = 64`, `FIRST_MOMENT` (0001-01-01T00:00Z, -62135596800000) and `LAST_MOMENT` (9999-12-31T23:59:59.999Z, 253402300799999).
- **`guest/times.la`** (pure): `words(at: i64, show: Show, offset: Option<i64>, now: i64, now_offset: i64): string`, built on `time.civil`; `None` offset draws UTC with " UTC". Month and weekday names in English, from Rapidkoi's own tables.
- **`window/zones.la`:** `moments(n: Node): i64[]` (distinct, in order); `settle(m: Model, lookup: fun(i64): Result<i64, TimeError>): Model` adds missing offsets for the front tab; `refresh(m, lookup)` redoes all and drops moments no longer shown; production passes `time.local_offset`, tests a fixed function. A failed lookup leaves the moment out (drawn in UTC).
- **`guest/placeholders.la`:** `fill(n: Node, first: string, last: string, offsets: Map<i64, i64>, now: i64): Node` turns `Parts` into `Words`; the token scanning goes.
- **Security (the owner's review before merging):** the `parts` decoder (untrusted input, every bound above); the `now` ask, a new capability given to apps: UTC to the second, nothing about place; known leak, recorded: a device's clock error is visible to an app's server that compares `now` with its own clock. The time zone is read-only from the system through `std.time`; no path from an app.
- **Order of work:**
  1. This branch: the shell's side, tested on WebAssembly text fixtures.
  2. `rapidkoi-rust`, in its repository: the request, tagged (suggested `v0.3.0-pre.1`).
  3. Here: `guest/rust` on that tag; `./scripts/newtab.sh && ./scripts/settings.sh`; `./main` rebuilt.
  4. Notes (`../notes`) and the counter rebuilt on the tag and republished on the VM: the owner's, or Claude's with the exact commands shown first.
  5. Merge into `main` last.

## TESTS
- [ ] `times.la`: each `show` at offsets 0, +3 h and -5 h; `date` and `date_time` with the year in another year; every `relative` row both ways, singular "1 minute ago" and "1 hour ago", "yesterday" across local midnight; a `None` offset gives "14:32 UTC"
- [ ] `zones.la`: `moments` finds each distinct moment once; `settle` looks up only missing ones; `refresh` redoes and drops; a failed lookup leaves the moment out; the system lookup passed in, never the machine's zone
- [ ] decoding: valid `parts`; refused with its sentence: `text` and `parts` both, neither, 65 parts, an unknown kind, `name` "middle", a time before year 1 and after 9999, a fractional time, `show` "soon"
- [ ] filling: name parts fill and an unset name is ""; time parts fill from offsets and `now`; a name typed as "{FIRST_NAME}" shows as typed; typing boxes and titles never fill
- [ ] privacy: after drawing a screen with name and time parts, the app's next event and its saved state hold none of the drawn words
- [ ] `now`: answered with `on_answer`, UTC milliseconds rounded down to a second
- [ ] the tick: subscribed only while the front tab shows a time; it changes the screen key and calls no app
- [ ] contract 8: a contract 7 app is refused with the existing sentence; every fixture runs as 8

## ACCEPTANCE
- [ ] Every test above passes, written first
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] Every file at or under 500 lines
- [ ] `rapidkoi-rust` tagged; the built-in apps repacked on it; the new tab and Settings open
- [ ] The owner's security review of the diff, before merging
- [ ] For the owner, by eye, with Notes rebuilt on the tag: its list shows times like "5 minutes ago" or "Thu 31 Oct" in your zone, moving on by themselves; "Hello, Brian" shows your Rapidkoi name; the new tab and Settings open
