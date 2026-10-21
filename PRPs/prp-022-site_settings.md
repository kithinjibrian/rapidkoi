## FEATURE: Site settings: ⚙ opens a card with the app, its account, its servers, its saved state, and Forget.

## OBJECTIVE
Designed in chat with the owner (session 139, from the owner's sketch). The ⚙ in the address box (PRP-020, disabled until now) opens a card hanging from it, in the shield's pattern: the app's icon, name and version at the top with its description under them; then sections, each a heading over a rounded group of rows that open pages, with back:
- **Account** (signed apps only): **Public key ›** (who you are to this app: "User 4f9a" and the key's fingerprint, read-only) and **Identity ›** (your name in this app, sign out / sign in, sign in automatically, start over as someone new).
- **App settings:** **Servers ›** (each server it may reach, a switch each, "New" after an update added one). Permissions (notifications, camera…) join here when they exist; no row until then.
- **App data:** **State ›** (what it saved, its size, "Clear and start over"). The database joins later (build step 4).
- At the bottom, in red: **Forget this app…**

Start over, start over as someone new, and Forget each ask first in a dialog over the window.

## CONTEXT

- Starting state: ⚙ is a disabled button (`src/ui/addressbox.la`, PRP-020). The model and logic of the old info panel survive: `Model.panel: Option<PanelPage>`, `Model.confirm: Option<Confirm>`; `Msg.TogglePanel`, `PanelGo`, `PanelBack`, `Ask`, `Cancel`, `StartOver`, `Forget`, `ToggleServer`, `SetName`, `ToggleAutoSignIn`, `SignOut`, `SignIn`, `StartOverIdentity`; `people/account.la` (`account_of`, `on_account`), `people/access.la` (`access_of`, `on_access`, `looked`, `news`). Their screens were deleted in PRP-017.
- Ending state:
  - `src/ui/settings.la` (new, pure): the card on a page, and the confirm dialog.
  - `src/window/actions.la`: `PanelPage` gains `Key` (the Public key page); `Account` is Identity, `Access` is Servers, `State` is State; `Notifications`, `Data` and `About` go (nothing draws them; `TabAction.About` opens the shield's card instead).
  - `src/ui/addressbox.la`: ⚙ enabled ("Site settings"), in a popover open while `m.panel` is `Some`.
  - `src/ui/layout.la`: the root's `.dialog(…)` while `m.confirm` is `Some`.
  - `src/shell.la`: opening the shield's card closes the settings card and the other way round; leaving the page closes both (as PRP-021's card).
  - `src/ui/tokens.la`, `src/ui/themes.la`: a section's group look (the control grey on the card's frame colour), tokens as needed.
  - Tests: `test/settings.la` (new); `test/shell.la`, `test/account.la`, `test/access.la` follow `PanelPage`'s change.
  - DESIGN.md, CLAUDE.md's tree, CHANGELOG.md.
- Related existing code: `src/ui/shield.la` (the card pattern: identity, rows, pages with back, `CARD_WIDTH`), the old panel's screens in git (`src/panel.la`, `src/people/account.la` and `access.la` before `80ed95c`) for wording and the state size words.
- kivuli API used: `ui.popover`, `ui.dialog`, `dialog_header`, `dialog_footer`, the root's `.dialog(…)`, `ui.switch`, `ui.input`, `button_with`, `kivuli.ui`'s `Button` looks (0.51).
- Open decisions: none.

## THE PAGES

| Page | Shows | Sends |
|---|---|---|
| Main | icon, name, version; description; the sections; Forget this app… | `PanelGo(Key/Account/Access/State)`, `Ask(Forget)` |
| Public key | "User 4f9a" (once the app has asked; else "Not signed in yet"), the key's fingerprint, a sentence: what the app sees of you | back |
| Identity | first name, last name (this app only); Signed in as… / Signed out with Sign out or Sign in; a "Sign in automatically" switch; "Start over as someone new…" | `SetName`, `SignOut`, `SignIn`, `ToggleAutoSignIn`, `Ask(StartOverIdentity)` |
| Servers | its own server first ("Its own server"), then the declared ones, each a switch; "New" under ones an update added; an unsigned app's are plain text with a line saying it can only reach this computer; none: "Notes can't reach anything outside itself." | `ToggleServer(s)` |
| State | "What Notes saved on this device", its size ("Nothing saved", "12 KB"), "Clear and start over…" | `Ask(StartOver)` |

The rows' summaries: Public key "User 4f9a"; Identity "Signed in" / "Signed out"; Servers "2 servers" / "2 servers, 1 off" / "None"; State its size.

Dialogs: "Start Notes over? What it saved on this device will be cleared." [Cancel] [Start over]; "Become someone new to Notes? It will see a new key." [Cancel] [Start over]; "Forget Notes? Its tab, pin, saved state and account here will be removed." [Cancel] [Forget]. The danger button is `ButtonLook.Danger`'s look; ✕, Escape and a click outside send `Cancel`.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- The card: `CARD_WIDTH`, the shield's row look (padding, pill hover, arrow-right), back as arrow-left; section headings dim; each section's rows in one rounded group.
- Unsigned apps: no Account section.
- Only one of the two cards open at a time.
- Every colour from `themes.la`, every size a token.

### Must NOT Do
- Show rows for things that don't exist yet (permissions, notifications, database).
- Change what the messages do (the logic is PRP-011 and PRP-012's), only draw them.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `PanelPage { Main, Key, Account, Access, State }` (Key added; Notifications, Data, About removed).
- No new messages. `TogglePanel` now opens the settings card (and closes the shield's); `Shield(Some(_))` closes the settings card.

## ERROR HANDLING REQUIREMENTS

- None new. The account and access logic already handle an unsigned app and a missing master secret.

## SECURITY CONSIDERATIONS

- Trust screens (DECISION-027): drawn only by Rapidkoi, in the header's card and a native dialog; apps can't open, change or imitate them.
- The public key is public; the master secret is never shown or read here (account labels come from the cached key, as PRP-011 does).
- Destructive actions ask first, in a dialog that keeps the page inert.

## TESTS TO WRITE

`test/settings.la`:
- [ ] ⚙ is enabled on every app page ("Site settings") and sends `TogglePanel`; not on home.
- [ ] Main lists Account (signed only), App settings, App data and Forget; an unsigned app has no Account rows.
- [ ] Each row sends `PanelGo(page)`; back sends `PanelBack`.
- [ ] Identity: typing a name sends `SetName(ThisApp, …)`; Sign out / Sign in; the switch sends `ToggleAutoSignIn`; "Start over as someone new…" sends `Ask(StartOverIdentity)`.
- [ ] Servers: a signed app's switch sends `ToggleServer(s)`; an unsigned app has no switches.
- [ ] State: "Clear and start over…" sends `Ask(StartOver)`.
- [ ] Forget this app… sends `Ask(Forget)`; the dialog's Cancel sends `Cancel`, its danger button `Forget`.
- [ ] Opening the shield's card closes this one and the other way round.

`test/shell.la`, `test/account.la`, `test/access.la`: follow `PanelPage`.

## ROLLBACK PLAN

- `git revert` the PRP's commits on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] ⚙ opens the card as sketched; each page works; the dialogs ask before Start over, start over as someone new and Forget
- [ ] The tests above and `lughac test test/` pass (the owner runs them)
- [ ] `lughac build src/main.la -o main`; `lughac fmt --check src test` clean; no file over 500 lines
- [ ] DESIGN.md, CLAUDE.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main rapidkoi://quote.dafifi.net`: ⚙ opens the card; open Public key, Identity (type a name, sign out and in), Servers (switch one off), State; Forget asks in a dialog; Cancel. Then a local file: no Account section.
