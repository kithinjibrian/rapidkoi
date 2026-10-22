## FEATURE: Permissions in the manifest; the new tab, holding built-in ones, reads your pins and where you left off and opens them; every app hears when it comes back to the front.

## OBJECTIVE
From chat with the owner (sessions 165–166), step A of the new tab's redesign (the owner's mockup: the wordmark, "Where you left off" with Continue, a row of pinned apps). An app declares what it needs in its manifest's `permissions` (the owner, session 166). Some permissions are **built-in**: only an app compiled into `./main` may hold them, and an external app declaring one doesn't run, with a screen saying why. The new tab declares `read_pins`, `read_left_off` and `open_apps`, asks Rapidkoi for your pins and the app you left off, and asks it to open one. It asks again each time it comes to the front. With only text and buttons until contract 3's images (step B), the new tab shows the wordmark, the app you left off with a **Continue** button, and **Pinned** with one button per pin.

## CONTEXT

- Starting state: contract 2 (`init`, `opened`, `message`, `answer`; asks `identity`, `sign`, `fetch`). The new-tab app (PRP-027) runs once per window in `Model.newtab`, outside the tabs, and is never answered (`answer_all` walks tabs only). Nothing tells an app its tab came back to the front.
- Ending state:
  - **Contract 3** (DECISION-039): wire event names `on_init`, `on_open`, `on_resume`, `on_message`, `on_answer`, `on_response`, `on_fail` (`docs/specs/app-events.md`); new asks `pins`, `left_off`, `open_pin`, `continue`. `on_pause` isn't sent yet.
  - `src/guest/contract.la`, `nodes.la`, `running.la`, `limits.la`: contract 3 encoded and decoded beside 2 (see DECISION-039 for whether 2 keeps running).
  - `src/bundles/permissions.la` (new, pure): `Permission` (`ReadPins`, `ReadLeftOff`, `OpenApps`), its wire name, its tier (`BuiltIn`; `Any` exists for later, holding none yet), parsing a manifest's list.
  - `src/bundles/bundle.la`, `rules.la`: `Manifest.permissions: string[]` instead of "must be empty": each a name of 1–32 lowercase letters and `_`, at most 16, no repeats. A signed bundle (format 2) signs them with the rest of the manifest.
  - `src/bundles/loader.la`: `Loaded.permissions`; an `Origin` (`BuiltIn` for `window/newtab.la`'s bytes, `External` for every file and address) decides the check at load: an unknown name refuses the app ("This app needs a newer version of Rapidkoi"), a built-in one from an external app refuses it ("This app asks for something only Rapidkoi's own apps can have: {what}").
  - `src/window/tabs.la`: `AppPage.permissions`, from the bundle (empty for a bare `.wasm`).
  - `src/commands/pack.la`: `--permission NAME`, repeatable, so `scripts/newtab.sh` can declare them.
  - `src/guest/home.la` (new, pure): what the new tab's reads return, built from the model: `pins_of(m)`, `left_off_of(m)`, as the answer values below.
  - `src/guest/effects.la`: answers the new asks for a page holding their permission; a page asking for one it doesn't hold is stopped ("This app asked for {what} without declaring it"). `answer_newtab(m)` beside `answer_all`.
  - `src/shell.la`: sends `on_resume` to an app whose tab becomes current, and to the new tab whenever the empty tab becomes current; `open_pin` and `continue` run what `OpenPinned` and `Continue` do.
  - `guest/rust/rapidkoi`: contract 3 (`Event::Resume`, `Ask::pins`, `Ask::left_off`, `Ask::open_pin`, `Ask::resume_left_off`, and `Place`, `Places` decoding the answers).
  - `guest/rust/newtab`: declares `read_pins`, `read_left_off`, `open_apps` (`scripts/newtab.sh`); asks on `on_open` and `on_resume`; draws the wordmark, where you left off with Continue, and the pins; repacked into `assets/apps/newtab.rkb` (contract 3).
  - `guest/rust/counter`, `guest/rust/quote`: rebuilt for contract 3 only if DECISION-039 says contract 2 stops running.
  - DECISIONS.md (DECISION-039), `docs/specs/app-events.md` (final), CLAUDE.md (stack: contract 3), DESIGN.md, CHANGELOG.md.
- Related existing code: `src/bundles/bundle.la` (`permissions` reserved, must be empty, PRP-003), `src/bundles/rules.la`, `src/bundles/signing.la`, `src/guest/effects.la` (how asks are answered), `src/window/navigate.la` (`open_pin`, `reopen`), `src/window/leftoff.la`, `src/ui/frame.la` (`label`, `icon_of`).
- Open decisions that must be resolved first: **DECISION-039**.

## THE READS

| Ask (wire `kind`) | Needs | Answer `value` (CBOR) |
|---|---|---|
| `pins` | `read_pins` | an array of up to `MAX_PLACES` (64) maps `{name, place, icon?}`, in the sidebar's order |
| `left_off` | `read_left_off` | one such map with `title?` added, or null when there's none |
| `open_pin` `{index}` | `open_apps` | none: Rapidkoi opens pin `index` of your pins as they are now, in the empty tab, or does nothing if there's no such pin |
| `continue` | `open_apps` | none: Rapidkoi opens where you left off in the empty tab, or does nothing if there's none |

All three permissions are built-in.

- `name`: the app's name. `place`: a site's host (`quote.dafifi.net`), or a local app's file name alone (`counter.rkb`), never a path. `icon`: the bundle's icon bytes (PNG), when it has one. `title`: the screen title it was showing.
- Never in an answer: a full path, saved state, keys, trust details, which tabs are open.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Built-in is decided by where the bytes came from (`Origin.BuiltIn`: only `window/newtab.la`), never by anything in the manifest: a manifest can't vouch for itself.
- An external app declaring a built-in permission is refused at load, before it runs: the error screen says which permission, in words ("reading your pins", "reading where you left off", "opening your apps").
- An ask is answered only if the page declared its permission; otherwise the app is stopped with an error screen (the owner, session 166), as for any contract violation.
- Fine-grained permissions (the owner, session 166): one per thing an app can reach.
- `open_pin` checks the index against the pins as they are now, not as they were read: a pin removed since opens nothing, a pin moved opens what's at that index now.
- `on_resume` goes to an app whose tab becomes current after being in the background: selected, switched to, the tab in front closed, opened while already open. Never right after `on_open`, never to the tab that was already current.
- Each `on_resume` is one guest call under the usual fuel; a failure stops that app as any call does.
- `MAX_PLACES` and the existing icon cap bound every answer.

### Must NOT Do
- Let an external app hold a built-in permission, whatever its manifest says.
- Add `Any` permissions or a screen asking the person to approve one (none exist yet).
- Send `on_pause` (named, not built).
- Images, wrapping rows, colours: contract 3's drawing additions are step B.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None new in the window: `open_pin` and `continue` reuse `navigate.open_pin` and `navigate.reopen`.
- None: `open_pin(i)` is checked against the pins as they are when it arrives.

## ERROR HANDLING REQUIREMENTS

- A malformed ask (`open_pin` without an index, a negative index) is a contract error: the app stops, as for any malformed effect. For the new tab, that leaves the empty area (PRP-027).
- `BundleError` gains the permission rules (a bad name, too many, a repeat); `Refusal.Failed` carries the two load refusals above, shown on the error screen.
- An undeclared ask stops the page (`Screen.Stopped`) with its sentence; Restart runs it again, and it stops again if it asks again.
- Nothing to open is not an error: nothing happens.

## SECURITY CONSIDERATIONS

- **Host capability exposed to a guest: needs the owner's review before it's built (CLAUDE.md SECURITY RULE).** Two reads of the person's own data and two actions opening apps they already chose, each behind a built-in permission.
- Only bytes compiled into `./main` can hold a built-in permission; an external bundle declaring one never runs, and one that didn't declare it is stopped the first time it asks.
- A signed bundle's permissions are signed: changing them breaks its signature.
- No paths leave Rapidkoi: places are hosts or bare file names.
- `open_pin` opens only an existing pin and `continue` only the left-off app: the new tab can't name an address or file of its own.

## TESTS TO WRITE

`test/permissions.la` (new, pure):
- [x] Each permission's name and tier; parsing a list; an unknown name.

`test/bundle.la`, `test/signing.la`:
- [x] Permissions round-trip; a bad name, more than 16, a repeat are refused; old bundles with `[]` still read; a changed permission breaks a signed bundle.

`test/loader.la` (or `test/newtab.la`):
- [x] The built-in new tab loads with its three permissions.
- [x] The same bytes loaded as an external file are refused, naming the permission; an unknown permission is refused as "needs a newer version".

`test/pack.la`:
- [x] `--permission` writes them; a bad name is a plain error.

`test/home.la` (new, pure):
- [x] `pins_of`: names, places (a host for an address, a bare file name for a path with folders), icons; at most 64; the sidebar's order.
- [x] `left_off_of`: with a title, without one, none.

`test/effects.la`:
- [x] The new tab's `pins` and `left_off` asks are answered with the values above.
- [x] An app without the permission asking `pins`, `left_off`, `open_pin` or `continue` is stopped with its sentence; nothing is answered or opened.
- [x] `open_pin(i)` opens pin `i` in the empty tab; an index past the end opens nothing; `continue` opens where you left off; with none, nothing.

`test/contract.la`:
- [x] Contract 3's events encode with the `on_` names; contract 3's new asks decode; a malformed `open_pin` is a contract error.
- [x] Contract 2 as DECISION-039 says (still runs with its old names, or is refused with a sentence).

`test/shell.la`:
- [x] Selecting a background app's tab sends it `on_resume` (an echo fixture records it); selecting the current tab doesn't; opening doesn't (only `on_open`).
- [x] Showing the empty tab sends the new tab `on_resume`.

`guest/rust` (`cargo test`):
- [x] The new tab asks for pins and where you left off on `on_open` and `on_resume`; draws Continue only with a left-off app; one button per pin sending `open_pin(i)`.
- [x] `Places` decodes Rapidkoi's answers; a malformed one is none.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits; repack the new tab at contract 2.

## ACCEPTANCE CRITERIA
- [ ] The new tab shows where you left off with Continue, and your pins; each opens its app in the empty tab
- [ ] Pinning an app, then going back to the new tab, shows it there (`on_resume`)
- [x] An external app declaring a built-in permission shows why it didn't run; one asking without declaring is stopped
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines
- [x] DECISIONS.md, `docs/specs/app-events.md`, CLAUDE.md, DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`, open the counter and quote, pin the quote, close the counter. + : the new tab shows the counter under where you left off with Continue, and the quote under Pinned. Continue opens the counter in that tab; + again, then the quote's button opens the quote.

## AMENDMENTS (session 167, while building)

- **No `AppPage.permissions`:** a page holds a permission when its manifest lists it. The loader refused every external bundle declaring a built-in one, so the manifest is already vetted (`effects.holds`). Same rule, without touching every page literal.
- **The events live in `src/guest/events.la`** (`Happened`, `event_for(h, version)`), not in `contract.la`, which keeps contract 2's encoders. Their tests are `test/events.la`, which keeps `test/contract.la` under 500 lines.
- **Opening is the window's own message:** `open_pin` and `continue` send `OpenPinned(path)`, or the "where you left off" card's own message (`Select(id)` or `Continue`), through `Cmd.after(0, …)`. One tab per app and the empty tab apply as for a click. "Where you left off" is the card's app: the last one visited while it's open, else the last closed.
- **The new asks decode for any contract;** the permission is the gate. A contract 2 app asking for one is stopped like any other.
- **The new tab asking to fetch** is answered at once with a failure: it has no server.
- **Rust:** a structured answer (pins, where you left off) arrives as `Event::Data { msg, cbor }`, read with `Place::list` and `Place::one`. Continue is `Ask::continue_left_off` (`continue` is a Rust keyword).
- **Icons in answers** are at most `MAX_PLACE_ICON` (64 KB); a larger one is left out.
