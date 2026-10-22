## FEATURE: The new tab reads your pins and where you left off, and opens them; every app hears when it comes back to the front.

## OBJECTIVE
From chat with the owner (sessions 165–166), step A of the new tab's redesign (the owner's mockup: the wordmark, "Where you left off" with Continue, a row of pinned apps). The built-in new-tab app asks Rapidkoi for your pins and the app you left off, and asks it to open one. It asks again each time it comes to the front. With only text and buttons until contract 3's images (step B), the new tab shows the wordmark, the app you left off with a **Continue** button, and **Pinned** with one button per pin.

## CONTEXT

- Starting state: contract 2 (`init`, `opened`, `message`, `answer`; asks `identity`, `sign`, `fetch`). The new-tab app (PRP-027) runs once per window in `Model.newtab`, outside the tabs, and is never answered (`answer_all` walks tabs only). Nothing tells an app its tab came back to the front.
- Ending state:
  - **Contract 3** (DECISION-039): wire event names `on_init`, `on_open`, `on_resume`, `on_message`, `on_answer`, `on_response`, `on_fail` (`docs/specs/app-events.md`); new asks `pins`, `left_off`, `open_pin`, `continue`. `on_pause` isn't sent yet.
  - `src/guest/contract.la`, `nodes.la`, `running.la`, `limits.la`: contract 3 encoded and decoded beside 2 (see DECISION-039 for whether 2 keeps running).
  - `src/guest/home.la` (new, pure): what the new tab's reads return, built from the model: `pins_of(m)`, `left_off_of(m)`, as the answer values below.
  - `src/guest/effects.la`: answers the new asks, **for the built-in new tab only**; `answer_newtab(m)` beside `answer_all`.
  - `src/shell.la`: sends `on_resume` to an app whose tab becomes current, and to the new tab whenever the empty tab becomes current; `open_pin` and `continue` run what `OpenPinned` and `Continue` do.
  - `guest/rust/rapidkoi`: contract 3 (`Event::Resume`, `Ask::pins`, `Ask::left_off`, `Ask::open_pin`, `Ask::resume_left_off`, and `Place`, `Places` decoding the answers).
  - `guest/rust/newtab`: asks on `on_open` and `on_resume`; draws the wordmark, where you left off with Continue, and the pins; repacked into `assets/apps/newtab.rkb` (contract 3).
  - `guest/rust/counter`, `guest/rust/quote`: rebuilt for contract 3 only if DECISION-039 says contract 2 stops running.
  - DECISIONS.md (DECISION-039), `docs/specs/app-events.md` (final), CLAUDE.md (stack: contract 3), DESIGN.md, CHANGELOG.md.
- Related existing code: `src/guest/effects.la` (how asks are answered), `src/window/navigate.la` (`open_pin`, `reopen`), `src/window/leftoff.la`, `src/ui/frame.la` (`label`, `icon_of`).
- Open decisions that must be resolved first: **DECISION-039**.

## THE READS

| Ask (wire `kind`) | Answer `value` (CBOR) |
|---|---|
| `pins` | an array of up to `MAX_PLACES` (64) maps `{name, place, icon?}`, in the sidebar's order |
| `left_off` | one such map with `title?` added, or null when there's none |
| `open_pin` `{index}` | none: Rapidkoi opens pin `index` of the last `pins` answer in the empty tab, or does nothing if it's gone |
| `continue` | none: Rapidkoi opens where you left off in the empty tab, or does nothing if there's none |

- `name`: the app's name. `place`: a site's host (`quote.dafifi.net`), or a local app's file name alone (`counter.rkb`), never a path. `icon`: the bundle's icon bytes (PNG), when it has one. `title`: the screen title it was showing.
- Never in an answer: a full path, saved state, keys, trust details, which tabs are open.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- The new asks are answered only for the page in `Model.newtab`, which `window/newtab.la` made from the bundle built into `./main`. Not by its name, manifest or path: any other app asking gets null, and `open_pin`/`continue` from it do nothing.
- `open_pin` checks the index against the pins as they are now, not as they were read: a pin removed since opens nothing, a pin moved opens what's at that index now.
- `on_resume` goes to an app whose tab becomes current after being in the background: selected, switched to, the tab in front closed, opened while already open. Never right after `on_open`, never to the tab that was already current.
- Each `on_resume` is one guest call under the usual fuel; a failure stops that app as any call does.
- `MAX_PLACES` and the existing icon cap bound every answer.

### Must NOT Do
- Give any app other than the built-in new tab a way to read pins or where you left off, or to open apps.
- Send `on_pause` (named, not built).
- Images, wrapping rows, colours: contract 3's drawing additions are step B.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new in the window: `open_pin` and `continue` reuse `navigate.open_pin` and `navigate.reopen`.
- None: `open_pin(i)` is checked against the pins as they are when it arrives.

## ERROR HANDLING REQUIREMENTS

- A malformed ask (`open_pin` without an index, a negative index) is a contract error: the app stops, as for any malformed effect. For the new tab, that leaves the empty area (PRP-027).
- Nothing to open is not an error: nothing happens.

## SECURITY CONSIDERATIONS

- **Host capability exposed to a guest: needs the owner's review before it's built (CLAUDE.md SECURITY RULE).** Two reads of the person's own data and two actions opening apps they already chose. Bounded to the bundle compiled into `./main`; every other app is refused.
- No paths leave Rapidkoi: places are hosts or bare file names.
- `open_pin` opens only an existing pin and `continue` only the left-off app: the new tab can't name an address or file of its own.

## TESTS TO WRITE

`test/home.la` (new, pure):
- [ ] `pins_of`: names, places (a host for an address, a bare file name for a path with folders), icons; at most 64; the sidebar's order.
- [ ] `left_off_of`: with a title, without one, none.

`test/effects.la`:
- [ ] The new tab's `pins` and `left_off` asks are answered with the values above.
- [ ] Another app asking `pins` or `left_off` gets null; its `open_pin` and `continue` open nothing.
- [ ] `open_pin(i)` opens pin `i` in the empty tab; an index past the end opens nothing; `continue` opens where you left off; with none, nothing.

`test/contract.la`:
- [ ] Contract 3's events encode with the `on_` names; contract 3's new asks decode; a malformed `open_pin` is a contract error.
- [ ] Contract 2 as DECISION-039 says (still runs with its old names, or is refused with a sentence).

`test/shell.la`:
- [ ] Selecting a background app's tab sends it `on_resume` (an echo fixture records it); selecting the current tab doesn't; opening doesn't (only `on_open`).
- [ ] Showing the empty tab sends the new tab `on_resume`.

`guest/rust` (`cargo test`):
- [ ] The new tab asks for pins and where you left off on `on_open` and `on_resume`; draws Continue only with a left-off app; one button per pin sending `open_pin(i)`.
- [ ] `Places` decodes Rapidkoi's answers; a malformed one is none.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits; repack the new tab at contract 2.

## ACCEPTANCE CRITERIA
- [ ] The new tab shows where you left off with Continue, and your pins; each opens its app in the empty tab
- [ ] Pinning an app, then going back to the new tab, shows it there (`on_resume`)
- [ ] No other app can read pins or where you left off, or open apps
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines
- [ ] DECISIONS.md, `docs/specs/app-events.md`, CLAUDE.md, DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`, open the counter and quote, pin the quote, close the counter. + : the new tab shows the counter under where you left off with Continue, and the quote under Pinned. Continue opens the counter in that tab; + again, then the quote's button opens the quote.
