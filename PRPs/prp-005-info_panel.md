## FEATURE: The info panel redesigned (the owner's sketch), and ⟳ Reload and ☆ Pin in the bar

## OBJECTIVE
The info panel becomes the owner's sketch (`docs/source/meetings/2026-10-17-ui/sketch-3-info-panel.png`, with the names agreed in `docs/source/meetings/2026-10-18-signing-and-panel.md`):
- a header with the app's icon, name, version and host marker
- its description
- **Your settings** (Account · Notifications · Access) and **On this device** (Where you left off · Data), each row with a summary that opens its own page
- **Forget this app**

The bar gains **⟳ Reload**, which keeps the app's state, and **☆ Pin**. Features that don't exist yet (accounts, notifications, permissions, app data) get honest placeholder pages.

## CONTEXT

- Starting state: `panel.view` is a flat column of texts and buttons (Restart, Pin/Unpin, Close tab, Reload for development). The bar is ‹ ⌂ › [address] ⓘ ⤴. `Model.showing_panel: bool`.
- Ending state:

```
src/panel.la     — the panel: header, sections and rows, and each page (Main, Account, Notifications, Access, State, Data, About)
src/panel_rows.la — (if panel.la passes 500 lines) the row and section builders
src/window.la    — Model.panel: Option<PanelPage>, Model.confirm: Option<Confirm>; forget(m)
src/navigate.la  — reload(m) keeps state (same state_version) or starts over with a notice
src/tabs.la      — reopen(engine, path, saved): load again and resume with the saved state
src/actions.la   — PanelGo, PanelBack, Ask(Confirm), Cancel, StartOver, Forget, TogglePin; Pin/Unpin folded into TogglePin
src/frame.la     — the bar: ‹ ⌂ › ⟳ [address ☆] ⓘ ⤴
src/themes.la    — Look.Section (the rows' rounded cards)
test/panel.la (new), test/views.la, test/shell.la, test/loader.la
docs/DESIGN.md, CLAUDE.md (known issues), CHANGELOG.md
```

- Related source files: the two meeting notes above, MEMORY.md decisions 15–16, DECISION-022 (accounts).
- kivuli API used: `image(Image.Bytes…)`, `.style`, `.button_style`, `.icon`, `.tooltip`, `separator`, `spacer`, `row`, `column`, `.wrap`.
- Open decisions: none block. DECISION-022 (accounts) is shown as a placeholder.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**The bar (`frame.la`)**
- `‹ ⌂ › ⟳` at the start; the title is `row([address.expand(), ☆])`; `ⓘ ⤴` at the end.
- **⟳** (`view-refresh-symbolic`, tooltip "Reload") only on app pages.
- **☆** (`non-starred-symbolic` / `starred-symbolic` when pinned, tooltip "Pin" / "Unpin") only on app pages, sends `TogglePin`.

**⟳ Reload keeps state (`navigate.la`, `tabs.la`)**
- It reloads the app from where it came from (the file is read again; a development server is fetched again), then:
  - **same `state_version`** as the running page: it resumes with the current state (`view(state)`, no `init`), exactly like Continue
  - **a different `state_version`**: it starts fresh, and the notice says "Counter's saved state changed shape, so it started over". `migrate` comes with PRP-006.
  - **a page that had stopped**: it starts fresh, as today's Restart did.
- This replaces today's development-only Reload. Restart (fresh state) becomes **Start over**, in the panel.

**The panel (`panel.la`)**
- `Model.panel: Option<PanelPage>`, with `PanelPage { Main, Account, Notifications, Access, State, Data, About }`. ⓘ toggles `None` ↔ `Main`, `PanelGo(page)` opens a page, and `PanelBack` returns to `Main`.
- **Main:**
  - **Header:** the icon (`Image.Bytes`, 48 px; the initial without one), the name (title style), `Version x.y.z` (dim; "No version" for a bare `.wasm`), and the **marker** as a subtle button that opens **About**.
  - **The description** (if any).
  - **YOUR SETTINGS** (a small dim heading), then a `Look.Section` card of rows:
    - Account · "Not signed in"
    - Notifications · "Off"
    - Access · "None" (or a count once permissions exist)
  - **ON THIS DEVICE**, then a card:
    - Where you left off · its size (B, KB or MB) or "Nothing saved"
    - Data · "None"
  - **Forget this app** (danger).
- **A row:** `row([text(label).expand(), text(summary).dim(), button("", PanelGo(page)).icon("go-next-symbolic").tooltip(label)])`. kivuli buttons can't hold a whole row, so the arrow is the click target: a placeholder until kivuli has clickable rows (DECISION-020, a new item).
- **Pages:** a header row (`‹` back button, tooltip "Back", plus the page's title), then:
  - **Account:** "How {name} knows you". "Not signed in". "Sign-in comes in a later step: each app will get its own ID for you, and the name you choose." (DECISION-022)
  - **Notifications:** "{name} hasn't asked to send notifications."
  - **Access:** "None: {name} can't reach anything outside itself." (Format 1 manifests declare no permissions.)
  - **Where you left off:** "{size} saved · state v{n}", and **Start over** (danger), which asks first: "Start {name} over? What it saved on this device will be cleared." [Cancel] [Start over]. Start over restarts the guest fresh (today's Restart).
  - **Data:** "No data yet." (App databases come later.)
  - **About:** the publisher ("Not signed: a local bundle", "Development server, unsigned", or "Not signed: a bare module"), where it came from (the path or address), the version, `Contract v1 · state v{n}`, and the bundle format (1, unsigned).
- **Forget this app** asks first: "Forget {name}? Everything it stored on this device will be deleted." [Cancel] [Forget]. Forget (`window.forget`):
  - closes the tab **without** making it "where you left off"
  - unpins it
  - drops a "where you left off" with the same path
  - closes the panel

  All in this session (PRP-006 adds the on-disk store; Forget then deletes from it too).
- `Model.confirm: Option<Confirm>` with `Confirm { StartOver, Forget }`: `Ask(c)` shows the confirm row in place of the button, `Cancel` clears it, and changing page or tab clears it.
- **Looks:** the panel keeps its place beside the page (overlay placeholder, DECISION-018). Its section cards are `Look.Section` (the frame's `#242424`, radius `PAGE_RADIUS`) on the panel's page colour. No new colours.

### Must NOT Do
- No real accounts, notifications, permission prompts, databases or signing: those pages are placeholders, saying so plainly.
- No change to the contract, bundles or the guest.
- Start over and Forget never act without the confirm.
- No colours outside `themes.la`.

## ERROR HANDLING REQUIREMENTS
- Reload failures (file gone, server down, bad bundle) show the error page for that tab, as opening does. The old state is lost only if the app can't load at all. (PRP-006 keeps it on disk.)
- A message for a page or tab that's gone is ignored.

## SECURITY CONSIDERATIONS
- The marker and the About page are drawn by Rapidkoi only, and never claim ✓ for an unsigned app.
- Forget removes what Rapidkoi holds in memory now. The on-disk part comes with the store (PRP-006).

## TESTS TO WRITE
- [ ] bar: ⟳ and ☆ only on app pages; ☆ shows Pin or Unpin and sends TogglePin; TogglePin pins, then unpins.
- [ ] reload keeps state: the echo fixture after an event, then Reload, keeps its state. A rebuilt bundle with another state_version starts fresh with the notice. A stopped page starts fresh. A development server's new build is picked up, with state kept when state_version matches.
- [ ] panel Main: the header texts (name, version, marker); both headings; each row's summary; the size formats (0 → "Nothing saved", 900 B, 12 KB, 3.4 MB); Forget is danger-styled.
- [ ] pages: each opens by its arrow and comes back with Back; each placeholder's text.
- [ ] confirms: Start over asks, Cancel clears, confirming restarts fresh; Forget asks, then removes the tab, the pin and the left-off, and closes the panel. Changing tab clears a pending confirm.
- [ ] About shows the right publisher line for a bundle, a development server and a bare module.

## ROLLBACK PLAN
- Branch: `prp-005-panel`. No data, no migrations.

## ACCEPTANCE CRITERIA
- [ ] The panel looks like the sketch: header, description, two section cards with rows, Forget *(the owner checks on screen)*
- [ ] ⟳ keeps the counter's count; ☆ pins and unpins *(on screen)*
- [ ] Start over and Forget each ask first *(on screen)*
- [ ] `lughac test test/` passes; `lughac fmt --check src test` is clean; `lughac build src/main.la -o main` succeeds
- [ ] Every `public` item has a doc comment; no file over 500 lines; CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner: `./main counter.rkb`, Add a few times, ⟳ (the count stays), ☆, ⓘ: open each row, Start over (Cancel, then for real), and Forget.
