## FEATURE: An app's windows all run one version: upgrading or reloading one window moves every window of that app, all or nothing

## OBJECTIVE
Every open window of an app runs the same version, because they share one database (PRP-041) and two versions disagreeing about its shape can break it. Clicking the upgrade arrow, or Reload, in any window of an app moves all of that app's windows to the new version together, each keeping what it had. If the new version can't be loaded, no window changes: each keeps running what it ran, and the person sees why. A link that opens a new window of an app already open runs the version already open.

## CONTEXT
- From chat with the owner (2026-10-30), after asking what happens to an app's database when its schema changes. The problem, stripped down: an older version touching data a newer one changed. This PRP fixes the case where two versions run at once. The case where an older version runs after a newer one (going back, a withdrawal) is separate and not in scope.
- Discovery answers (the owner):
  - Q1 (a): upgrading one window upgrades every window of the app, each keeping its state.
  - Q2 (a): a new window runs the version the app's open windows run, and the arrow shows.
  - Q3 (a): all or nothing; one load, every window moved only once it's ready.
  - Q4 (a): Reload too, for every kind of app: signed, local development (file or `localhost`).
  - Q5: the edge cases below, and built-in apps get the same treatment ("the only distinction that really matters is that we trust them natively").
- What's already true (read in `src/window/navigate.la`, `src/window/instances.la`):
  - + starts another window from the running program (`instances.another`): same version.
  - Opening an app already open shows its tab (PRP-026); Restore skips apps open now (PRP-035).
  - **The gap:** a link with fields always opens a new window and loads on its own (`navigate.open_file` → `loads.begin_link`), so it can run another version, even an older one it names.
- Starting state:
  - The arrow and Reload (`Msg.Reload`, the tab menu's Reload) call `navigate.reload`, which reloads only the current window (PRP-044: "upgrades this window").
  - A signed or development address reloads in the background (`loads.begin` with the running page kept); a file reloads at once (`restarts.load_again`); Settings is opened again (`settings_app.open`).
  - A reload that fails (`loads.arrive`, `Err(r)`) closes the running page and shows the refusal in its place.
- Files read first: `src/window/navigate.la` (`reload`), `src/net/loads.la` (`begin`, `arrive`, `offered`), `src/window/restarts.la` (`with_saved`, `load_again`), `src/window/instances.la` (`another`), `src/window/window.la` (`same_app`, `with_tab`).
- Files changed:
  - **New `src/window/upgrades.la`**: `loads.la`, `shell.la` and `tabs.la` are at 492 lines, so the new logic goes in its own module, imported by `net.loads` and `window.navigate`.
  - `src/window/navigate.la` (`reload`), `src/net/loads.la` (`arrive`: follow on success, keep the windows on failure, a link's window takes the running program).
  - Tests: new `test/upgrades.la`.
  - Docs: DECISIONS.md (core decision 28 amended), CHANGELOG.md.
- Decisions: core decision 28 (amended by PRP-044, amended again here). DECISION-044 is open but unrelated (what Continue means). None open that this depends on.

## REQUIREMENTS

### Must Do
- **Upgrading or reloading moves every window of the app** (`same_app` on its path): the arrow, Reload, and the tab menu's Reload, for signed apps, local development apps (file or address) and built-in apps.
  - The new version is loaded **once**, in the window it was asked from.
  - When it's ready, every window of the app moves onto it, each keeping its own state, instance, name and fields. One whose state's shape changed starts over.
  - One notice at most for the shape change: "{name}'s saved state changed shape, so it started over" (the existing sentence), however many windows started over.
  - The arrow goes from every window once they run the newest version.
- **All or nothing:** if the load fails (unreachable with nothing kept, refused, won't start), every window keeps running the version it ran, the arrow stays, and the notice gives the refusal's existing sentence. The running page is no longer replaced by the refusal.
- **A changed key** is asked once, in the window it was asked from; the other windows keep running meanwhile. "Keep using my version" leaves them all as they were. "Open as a separate app" is a separate app (its own key, its own database): the other windows stay.
- **A link's new window, for an app already open,** starts from the program the open windows run (as + does), with the link's fields. When the link named another version, the existing sentence says so ("This link was made with {name} {v}; opened {version}").
- **Edge cases:**
  - A second arrow or Reload click while the app's upgrade is loading does nothing.
  - A window closed during the upgrade is left out.
  - A window opened during the upgrade starts on the old version and moves with the others.
  - A window stopped because its version was withdrawn moves too.
  - A window still loading when the upgrade lands takes the new program.
- Core decision 28 amended in DECISIONS.md.

### Must NOT Do
- No automatic upgrade: only the arrow or Reload moves windows (PRP-044).
- No new notice, dialog or question: "Upgrade all windows?" was declined (Q1).
- Nothing about an older version opening a database a newer one changed: separate, later.
- No change to the contract, bundles or the database code.

## DESIGN
- **`src/window/upgrades.la`:**
  - `upgrading(m, path) -> bool`: some window of the app is loading with a running page kept (an upgrade under way). `navigate.reload` does nothing when it's true.
  - `follow(m, id) -> (Model, bool)`: after tab `id` runs its new page, every other tab of the same app (running, stopped or loading) is moved onto that page's program, with its own state through `restarts.with_saved` (fresh when there's none or its shape changed), its instance, name, `seen` and fields kept, and its old page closed. Returns whether any window's state started over.
  - `adopted(m, page) -> Option<AppPage>`: for a link's window, the program an open window of the same app runs, started fresh with the link's fields.
- **`loads.arrive`:**
  - On success with a running page kept: `follow`, and one notice.
  - On `Err(r)` with a running page kept: the tab gets its kept page back, and the notice is the refusal's sentence. (A tab with no running page still shows the refusal as now.)
  - On success for a tab with no running page kept, when another window of the app runs: `adopted` instead, the fetched build offered through `offered` when it's newer.
- **`navigate.reload`:** unchanged for the window clicked; files and Settings, which reload at once, call `follow` straight after.
- No new message, model field or error enum. `follow` closes old pages before starting new ones, and calls guests only to start them (bounded by fuel as now).
- **Security:** no new capability, no host function, no new input. Guests are started the way + already starts them.

## TESTS
- [x] Upgrading a signed app with two windows moves both to the new version, each keeping its own state; the arrow goes from both
- [x] A window whose state's shape changed starts over; one notice for two such windows
- [x] Reloading a local app file with two windows moves both
- [ ] Reloading a development address with two windows moves both (not written: the same `loads.begin` path as the signed tests)
- [ ] Reloading Settings with two windows moves both (see AMENDMENTS)
- [x] A failed upgrade leaves every window running its version, the arrow still showing, with the refusal's sentence
- [x] A changed key: "Keep using my version" leaves both windows as they were
- [x] A second Reload while the upgrade loads starts nothing
- [x] A window closed during the upgrade is left out; one opened during it moves with the others
- [x] A window stopped by a withdrawal moves too
- [x] A link with fields to an app already open runs the open windows' version, with its fields; a link naming another version says so
- [x] A link to an app that isn't open loads as before

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] For the owner, by eye: open Notes in two windows (+), deploy a new Notes version, wait for the arrow, click it in one window: both run the new version, each with its own note open, and the arrow goes from both

## AMENDMENTS

- **Notices aren't drawn (the owner, Q6 (a)):** a failed upgrade's reason goes to `m.notice`, which nothing shows since session 108. Kept as is: every window keeps running, and the gap is fixed for every notice at once, separately.
- **A stopped window shows the new failure:** keeping the old page matters only when it runs. A window already stopped whose new version fails too shows the new failure, as before (`test/loader.la`'s `reload_fetches_the_new_build`, `test/opening.la`'s `a_changed_byte_is_refused_with_no_way_past`).
- **A changed key has two answers, not three:** "Keep using my version" and "Open as a separate app"; there's no accepting the new key in place. Keeping mine leaves every window as it was (tested).
- **Found, not fixed (outside this PRP):** "Open as a separate app" loads from the kept copy first, which is the old publisher's, so the "separate app" runs the old publisher's build unmarked. On `main` too. Its own fix.
- **Settings has no test of its own:** it goes through the same `landed`, and no built-in changes between reloads in a test.
- **Test helpers:** the site helpers moved from `test/keeping.la` to `test/sites.la`, shared with `test/upgrades.la`. `test/shell.la`'s reload test now expects the window kept running.
