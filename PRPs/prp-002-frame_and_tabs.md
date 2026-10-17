**Status:** approved by the owner, 2026-10-17 (session 9)

## FEATURE: Rapidkoi's frame and tabs: the bar, the tab rail, home, and the app info panel

## OBJECTIVE
Rapidkoi stops being "one app fills the window" and becomes the frame the owner sketched (`docs/source/meetings/2026-10-17-ui-design.md`, sketches in `2026-10-17-ui/`). A bar runs across the top (← ⌂ →, the address/search box, ⓘ, ⤴), a tab rail runs down the right (tabs, +, divider, settings), and apps live only in the area between. Several apps run at once in tabs, each in its own store. A new tab is home: the logo, "Where you left off", and Pinned tiles. The info panel shows what the app is and holds Restart, Pin and Close. A crash stays inside its tab. Where kivuli lacks a feature, an explicit placeholder stands in (DECISION-018).

```
./main                     → home, in one tab
./main counter.wasm        → the counter in one tab
type a path in the box ↵   → opens that app here
+                          → a new home tab
```

## CONTEXT

- Starting state: PRP-001 on `main`. `shell.la` runs one guest and handles `boot`, `update` and `view`; `render.la` has `Msg { Event, Restart }`; `main.la` uses `sandbox`.
- Ending state:

```
src/main.la      — kivuli `application` (Cmd.focus is needed for Ctrl+K)
src/shell.la     — Model (tabs, current, draft, panel, pinned, left_off), Msg, update, view: composition only
src/tabs.la      — Tab and Page types; opening a file into a page; start/step/show/halt (moved from shell.la);
                   restoring a page from saved state
src/frame.la     — pure: the bar and the rail
src/home.la      — pure: the home page (logo, Where you left off, Pinned rows) and the in-session search filter
src/panel.la     — pure: the app info panel
src/contract.la  — read_tree returns the root's optional `title` too (contract addition)
src/theme.la     — the frame tokens from docs/DESIGN.md
test/tabs.la test/frame.la test/home.la test/panel.la test/shell.la (rewritten) test/contract.la (+title)
guest/rust/      — `View::title`; the counter sets "Counter"
CLAUDE.md        — rule 3 (opening a local file in `update`), file tree, KNOWN ISSUES (placeholders)
```

- Related existing code: `src/shell.la`, `src/render.la`, `src/guest.la`; kivuli's `application`, `Cmd.focus`, `View.map`, `.icon`, `.tooltip`, `.on_submit`, `.autofocus`, `.name`, `.width`, `.height`, `.expand`, `separator`, `spacer`, `scroll`.
- Related source files: `docs/source/meetings/2026-10-17-ui-design.md`, `docs/DESIGN.md` (The frame).
- Open decisions: none block. DECISION-016 (disabled inputs) and DECISION-019 (state across restarts) stay as they are.

## PLACEHOLDERS (DECISION-018)

Each is listed in CLAUDE.md KNOWN ISSUES and replaced when kivuli ships the feature:

| Wanted | Placeholder in PRP-002 | Replaced by |
|---|---|---|
| A file dialog for "Open a file…" | Type a path into the address box and press ↵ | kivuli file dialog |
| The info panel *over* the app | A column beside the app (the app narrows) | kivuli overlay |
| Right-click menu on tabs | The same actions in the info panel only | kivuli context menu |
| A wrapping grid | Fixed rows of `TILES_PER_ROW` tiles | kivuli FlowBox |
| App art and banners | Tiles and tabs show the app's initial; the banner has no picture | PRP-003 (generated art, `banner`) |
| History and search | In-session only: open tabs, pinned apps and the last left app; plain substring match | PRP-003 (on-device store, loose search) |
| Pins and "where you left off" kept across restarts | In memory; gone when Rapidkoi quits | PRP-003 |
| Back, forward, share; addresses | Shown, disabled, with tooltips "Comes with links" | Links step |
| Settings and more | Shown, disabled, tooltip "Coming soon" | Later |

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Model (`shell.la`, `tabs.la`)**
- `Model { engine, tabs: Tab[], current: i64, next: i64, draft: string, editing: bool, panel: bool, pinned: Pin[], left_off: Option<LeftOff> }`.
- `Tab { id: i64, page: Page }`. `Page` is `Home` or `App(AppPage)`.
- `AppPage { name, path, title: Option<string>, program, screen }`, where `screen` is PRP-001's `Running | Stopped`.
- `Pin { name, path }`. `LeftOff { name, path, title, state: u8[] }`.
- Tab ids are never reused, so a guest event for a closed tab is ignored.
- One `Engine` for the program; every app page has its own store (PRP-001's `open`).

**Messages**
- `Tab(id, render.Msg)` is a guest event or Restart, routed to that tab only.
- `Select(id)`, `NewTab`, `CloseTab(id)`, `Home`, `Draft(text)`, `Go`, `EditAddress`, `OpenPinned(path)`, `Continue`, `TogglePanel`, `Pin`, `Unpin(path)`, `FocusSearch`.

**Behaviour**
- `./main` opens one home tab; `./main path` opens that app in one tab.
- **+**: a new home tab, made current, with the search box focused.
- **Selecting a tab** shows it. Background apps keep their state and get no calls.
- **Closing** a tab closes its store. The tab to its right becomes current (or the left one); closing the last tab leaves one home tab. A closed app page becomes `left_off`.
- **⌂ on an app page**: the page becomes home, the app's store closes, and it becomes `left_off` with its state bytes.
- **Continue**: reopens `left_off` in the current tab **with its saved state** (open → `view(state)`, skipping `init`). If the file is gone or won't load, show the error page with the reason.
- **The address box**:
  - On home it's an input (`.name("address")`, `.autofocus()`, `.placeholder("Search or type a path")`). Typing filters home (below).
  - ↵ (`Go`) on a path to a file opens that app in this tab. On `rapidkoi://…` or `https://…` it shows "Links come in a later step". Otherwise it shows "No app or file called “…”".
  - On an app page, the box is a subtle button showing `📄 {file name} · local file`. Clicking it (`EditAddress`) turns it back into the input, holding the path.
- **Ctrl+K** anywhere: a hidden-shortcut button (`FocusSearch`) shows home's input or the address editor, with `Cmd.focus("address")`.
- **Home search** (in session):
  - an empty box shows "Where you left off" and Pinned
  - text filters Pinned, open tabs and `left_off` by a case-insensitive substring of name or title, shown in the same tile rows
  - no matches: "Nothing matches “…”"
- **Pin / Unpin** live in the info panel. Pinned tiles open the app here; a missing file shows the error page.
- **The info panel** (ⓘ toggles it, only on app pages):
  - name (title style), title, `📄 path · local file`, "Contract v1"
  - "Permissions: none. This app can't reach anything outside itself."
  - Restart, Pin/Unpin, Close tab, and ✕ to close the panel
- **Window title**: "Rapidkoi" on home, "Rapidkoi — {title or name}" on an app.
- **A crash** shows PRP-001's error page inside that tab. Restart there or in the panel. The other tabs are untouched.

**Views (pure, from `theme.la` tokens and DESIGN.md icons)**
- **Bar**: back, home and forward icon buttons (back and forward disabled, tooltip "Comes with links"), the address box (`.expand()`), ⓘ (app pages only), and ⤴ (disabled, "Comes with links").
- **Rail** (`.width(RAIL_WIDTH)`):
  - one button per tab: the app's initial, or the home icon for home. The current tab is `Primary`, the others `Subtle`; tooltip "name · title".
  - then **+**, a separator, a spacer, settings and more (both disabled, "Coming soon")
- **Home**:
  - the logo (text "Rapidkoi", title style; the circle is the art placeholder)
  - a "Where you left off" card (`.height(BANNER_HEIGHT)`) with "name · title" and Continue, shown only when `left_off` exists
  - "Pinned": rows of `TILES_PER_ROW` tiles, each a `TILE_SIZE` square button with the initial and the name under it; shown only when something is pinned
- **App area**: render's view mapped with `.map(fun(m) = Msg.Tab(id, m))`. With the panel open: `row([app.expand(), panel.width(PANEL_WIDTH)])`.

**Contract addition (additive, still v1)**
- The root node may carry `props.title` (text). `read_tree` returns `Described { root: Node, title: Option<string> }`. A non-text `title` is a contract error, as for other props. Older shells ignore it.
- In the Rust crate, `View::title(&str)` sets it, and the counter uses `.title("Counter")`.

**Rule 3 (part of approving this PRP).** Extend the exception: "*…and opening a local app file the person chose (≤ 32 MB, PRP-001's cap).*" Why: kivuli has no file commands, and the read is bounded and local.

### Must NOT Do
- No links, addresses, network, manifests, banners or generated art. No disk storage of history, pins or state (PRP-003, DECISION-019).
- No guest calls from `view`. Background tabs get no calls.
- No changes to kivuli, lugha-gtk or lugha-wasmtime. Placeholders only, as listed.
- No contract change other than the optional root `title`.
- No colours, custom CSS or bare numbers (DESIGN.md).

## ERROR HANDLING REQUIREMENTS
- Opening a path: unreadable, too large or not a module gives that tab an error page (PRP-001's `reason`), with no Restart (the module never loaded). Never a crash.
- Continue or a pinned tile whose file is gone: the error page, same rules.
- Guest failures stay as in PRP-001, per tab.
- A `Tab(id, …)` for a missing id is ignored. That isn't an error: the tab was closed.

## SECURITY CONSIDERATIONS
- Only Rapidkoi draws the bar and rail. The app's view is mapped into the app area and can't reach them. The local-file marker is drawn by Rapidkoi only.
- Paths come from the person (the argument, the box, pins). They are read only, never executed or written.
- Each tab keeps its own store, fuel and limits. A tab can't see another's state.
- `left_off` state stays in memory only in this PRP.

## TESTS TO WRITE
Pure (no display), with kivuli.testing:
- [ ] Frame: back, forward and share disabled; ⓘ only on app pages; rail tooltips and the current tab's style; + and Ctrl+K messages.
- [ ] Home: logo text; no card or Pinned when empty; the card with `left_off`; Pinned rows of 4 (5 pins give 2 rows); filtering by name and title, case-insensitive; "Nothing matches".
- [ ] Panel: texts, Pin vs Unpin, Restart, Close, and ✕.
- [ ] Contract: root `title` is read; a non-text title is a contract error; `title` on a non-root node is ignored.
Runtime (fixture guests from `test/support.la`):
- [ ] Two tabs of the echo fixture: an event in tab A changes only A's state.
- [ ] A crash in tab A: B still runs; Restart A brings it back.
- [ ] Closing the current tab picks its neighbour; closing the last leaves one home tab; an event for a closed tab id is ignored.
- [ ] ⌂ turns the page into home and records `left_off` with its state; Continue restores that state (`view` gets it, `init` isn't called: the fixture's init and saved states differ).
- [ ] Go: a file path opens; a missing path, `rapidkoi://x` and plain words give their messages.
- [ ] Pin and Unpin; a pinned tile opens its app; a pin whose file was deleted gives the error page.
- [ ] Window title on home and on an app with and without `title`.
- [ ] `test/counter.la`: the counter's tab shows "Counter" as its title.

## ROLLBACK PLAN
- Branch: `prp-002-frame`; `main` stays at PRP-001 until merge.
- Abandoning it means deleting the branch. No data, no migrations.

## ACCEPTANCE CRITERIA
- [ ] `./main` shows home in the frame; typing a path and ↵ opens the counter in that tab *(the owner checks on screen)*
- [ ] + opens a second tab; both apps keep their own state; ✕ in the panel and Close tab behave as above *(on screen)*
- [ ] ⌂ then Continue brings the counter back with its count *(on screen)*
- [ ] Every placeholder is listed in CLAUDE.md KNOWN ISSUES
- [ ] `lughac test test/` passes; `lughac fmt --check src test` is clean; `lughac build src/main.la -o main` succeeds
- [ ] `cargo test` and the wasm build pass in `guest/rust/`
- [ ] Every `public` item has a doc comment; no file over 300 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- `cargo test --manifest-path guest/rust/Cargo.toml && cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown`
- For the owner: `./main`, type `guest/rust/target/wasm32-unknown-unknown/release/counter.wasm` ↵, click Add twice, then ⌂, then Continue (the count should still be 2). Then + for a second tab, ⓘ, Pin, ⌂, and the Pinned tile.

## AMENDMENT 1 — 2026-10-17 (session 10, the owner after trying it)

Approved by the owner in chat:
1. **Split** `src/shell.la` (305 lines): the Model and its helpers move to `src/window.la`, and `shell.la` keeps `boot`, `update`, `view` and re-exports what moved.
2. **Home is not a tab.**
   - The rail lists only apps. + and ⌂ show home, with no rail item highlighted.
   - Opening an app from home opens a new tab. Opening one from an app page's address box replaces that tab's app.
   - ⌂ keeps the app open in its tab. "Where you left off" is the last app visited: an open tab is selected again; a closed one is reopened with its saved state in a new tab.
   - Closing the last tab shows home.
3. **Themes, starting with dark.** Rapidkoi's own parts (the bar and rail, and home's background) take colours from a Rapidkoi theme. Dark comes first: bars `#242424`, home `#161616`. Apps still can't set colours.
   - kivuli can't apply colours yet, so the theme is defined, turned into a stylesheet and tested, but not applied: a placeholder (DECISION-018) until kivuli has app stylesheets and CSS classes (DECISION-020).
4. **Start maximized** (the owner: "start in full screen mode because it is a browser"). This means maximized: filling the screen with the system title bar kept, as browsers open. Not F11-style full screen, which would hide the window controls the owner keeps. kivuli can't maximize a window yet, so it's a placeholder: the default size stays until kivuli adds it (DECISION-020).
