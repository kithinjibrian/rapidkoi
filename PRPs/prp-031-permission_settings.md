## FEATURE: Each permission is Always allowed, Needs approval or Blocked, set in the app's settings card; built-in apps choose their frame controls in their manifest; apps hear `on_change` when what they read changes.

## OBJECTIVE
From chat with the owner (session 168), after PRP-030. You decide what an app may do with each permission it declares:
- **Always allowed:** answered at once.
- **Needs approval:** Rapidkoi asks you first, and the app waits for your answer.
- **Blocked:** answered with nothing, and the app keeps running.

You set this on a **Permissions** page reached from the settings card's **App settings** group, next to Servers. A built-in app's manifest chooses which address-box icons and which settings rows it shows (`controls`), so ⚙ appears on the new tab and opens just its Permissions. An app reading your pins or where you left off hears `on_change` when they change while it's in front, so the new tab never goes stale.

## CONTEXT

- Starting state (PRP-030): the manifest's `permissions`, with built-in ones for `Origin.BuiltIn` only. An undeclared ask stops the app; a declared one is always answered. The settings card (`ui/settings.la`) has Account (Public key, Identity), App settings (Servers), App data (State) and Forget; it shows on app pages only. The new tab has no ⚙. Apps read only on `on_open` and `on_resume`.
- Ending state:
  - **Settings per app** (`people/people.la`, the people file): `Person.permissions: (string, Setting)[]`, the permissions set away from their default. Built-in apps are keyed `builtin:<file>` (`builtin:newtab`), since they have no host or key. The people file format moves to 2; format 1 reads with every permission at its default.
  - `bundles/permissions.la`: `Setting` (`Allowed`, `Ask`, `Blocked`), `default_of(p)` (`Allowed` for every built-in permission), words for each setting.
  - **Answering** (`guest/effects.la`):
    - Allowed: as PRP-030.
    - Blocked: pins as `[]`, where you left off as null, `open_pin` and `continue` do nothing.
    - Needs approval: the ask waits, and later asks from the same page wait behind it, until you answer the dialog.
  - **The approval dialog** (`ui/settings.la` beside the confirm dialog, `Model.approval`): "{app} wants to {words}". The buttons are Don't allow, Allow once and Always allow.
    - Always allow sets the permission to Allowed.
    - Don't allow, or closing the dialog, answers as Blocked does, this once.
    - It's Rapidkoi's own dialog, drawn by the shell.
  - **The card:** App settings holds Servers (›) and Permissions (›, "2 allowed · 1 needs approval"). The Permissions page has one drop-down per declared permission, in words, plus Back.
  - **`controls`** (the manifest, `bundles/bundle.la` and `rules.la`): `{address: [...], settings: [...]}`.
    - Address icons: `settings`, `share`.
    - Settings rows: `account.public_key`, `account.identity`, `app_settings.servers`, `app_settings.permissions`, `app_data.state`, `forget`. A group's name alone (`app_settings`) means all its rows.
    - The loader refuses an external app that lists it: "This app asks to change Rapidkoi's own controls". External apps always get the full card and the shield. Signed in format 2.
  - `ui/addressbox.la`, `ui/settings.la`: the empty tab's address box shows what the new tab's `controls.address` names. The card shows only the rows its `controls.settings` names. An app without `controls` shows what it does today.
  - **`on_change`** (contract 3, `guest/events.la`): sent to the page in front when something it may read changes:
    - your pins (`read_pins`);
    - where you left off (`read_left_off`);
    - one of its own permission settings.
    Not sent to contract 2 apps, or for a permission that's Blocked.
  - `commands/pack.la`: `--control address=settings`, `--control settings=app_settings.permissions` (repeatable).
  - `guest/rust/rapidkoi`: `Event::Change`. `guest/rust/newtab`: reads on `on_change` too; repacked with `--control address=settings --control address=share --control settings=app_settings.permissions`.
  - `docs/specs/app-events.md` (`on_change`), DESIGN.md, CHANGELOG.md, CLAUDE.md (tree, pack flags).
- Related existing code: `src/ui/settings.la` (`front`, `servers_page`, `dialog`), `src/people/access.la` (server switches: the pattern for settings per app), `src/people/people.la`, `src/guest/effects.la`, `src/bundles/loader.la` (`Origin`), `../kivuli` `ui.dropdown`.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- A setting is the person's; no app can read or change it. An app learns only what a Blocked or refused ask leaves it with.
- **Allow once** answers that one ask; the next one asks again (the owner, session 168).
- **Always allowed** is the default for every built-in permission (the owner, session 168).
- Setting a permission back to Allowed, or answering Always allow, sends the app `on_change` if it's in front.
- One approval dialog at a time. A second app's ask waits for the first dialog to close. Switching tabs closes a dialog, and that counts as Don't allow.
- `controls` changes what the frame shows, never what Rapidkoi does: ⚙ still opens Rapidkoi's own card, and a row still does what it does for every app.

### Must NOT Do
- Let an external app hide the shield or any control (`controls` is refused for it).
- Stop an app for a Blocked or refused ask (stopping stays for undeclared asks, PRP-030).
- Add any permission an external app can hold.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `Model.approval: Option<Approval>`: the ask waiting for you (which page, which permission, the app's name).
- `Msg.Approve(Answer)` (`Once`, `Always`, `Deny`): the dialog's buttons.
- `Msg.SetPermission(string, Setting)`: a drop-down on the Permissions page, for the card's app.
- `PanelPage.Permissions`: the card's new page.

## ERROR HANDLING REQUIREMENTS

- `BundleError` gains the `controls` rules: an unknown icon or row name, a malformed map, too many entries.
- `Refusal.Failed` gains "This app asks to change Rapidkoi's own controls" for an external app.
- A people file whose permissions are broken is set aside like any broken file (`saving`), with every setting back at its default.

## SECURITY CONSIDERATIONS

- **This grants and withholds a guest capability: it needs the owner's review before it's built (SECURITY RULE).** The approval dialog and the Permissions page are trust screens, drawn by the shell (MEMORY.md decision 19).
- The settings live in Rapidkoi's data folder. An app never sees them, only their effect on its answers.
- `controls` is built-in only, enforced at load by origin (PRP-030's rule).

## TESTS TO WRITE

- `test/permissions.la`: each setting's words; `default_of` is Allowed for built-in.
- `test/people.la`: settings round-trip; format 1 reads at defaults; `builtin:newtab` is keyed apart from any host.
- `test/bundle.la`, `test/signing.la`, `test/newtab.la`: `controls` round-trips and is checked; signed; refused from an external origin; the built-in new tab loads with it.
- `test/effects.la`:
  - Allowed answers.
  - Blocked answers `[]`, null, nothing opened, and the app runs on.
  - Needs approval sets `Model.approval` and holds the ask and those after it.
  - Once answers it and asks next time; Always answers it and sets Allowed; Deny answers as Blocked; switching tabs counts as Deny.
- `test/settings.la`:
  - App settings shows Servers and Permissions (with its summary).
  - The Permissions page has a drop-down per permission, sending `SetPermission`.
  - `controls.settings` limits the rows.
  - The approval dialog's three buttons.
- `test/addressbox.la`: the empty tab shows ⚙ and share as the new tab's `controls.address` says; without a new tab, share only.
- `test/shell.la`:
  - `on_change` reaches the page in front when pins change, when where you left off changes, and when its setting changes.
  - Not to a contract 2 app; not for a Blocked permission.
- `test/events.la`: `on_change` on the wire; none for contract 2.
- `test/pack.la`: `--control` writes `controls`; a bad one is a plain error.
- `guest/rust` (`cargo test`): `Event::Change` decodes; the new tab reads on it.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the PRP's commits; repack the new tab without `--control`.

## ACCEPTANCE CRITERIA
- [ ] ⚙ shows on the new tab and opens its Permissions; each permission can be set to Always allowed, Needs approval or Blocked, and it sticks across restarts
- [ ] Needs approval shows Rapidkoi's dialog when the new tab reads; Blocked leaves that section out
- [ ] Pinning or closing an app from the sidebar while the new tab is in front updates it at once (`on_change`)
- [ ] An external app with `controls` doesn't run, with its reason
- [ ] `lughac test test/` and `cargo test` pass (the owner runs them); build ok; fmt clean; no file over 500 lines
- [x] DESIGN.md, CHANGELOG.md, CLAUDE.md, `docs/specs/app-events.md` updated

## VALIDATION
- For the user:
  1. `./main`, then ⚙ on the new tab: App settings → Permissions shows three drop-downs, all Always allowed.
  2. Set "Reading your pins" to Needs approval, then go to an app and back with +: the dialog asks. Allow once shows your pins; the next + asks again.
  3. Set it to Blocked: Pinned disappears.
  4. Pin an app from the sidebar's menu while on the new tab: it appears under Pinned at once.

## AMENDMENTS (session 169, while building)

- **Settings are `People.grants`,** a list of `{app, permission, setting}`, not a `Person` field. `Person` is keyed by a host and a 32-byte key, which a built-in app doesn't have. The people file's format is 3; format 2 reads with every permission at its default.
- **An app's key** (`access.key_of`) is a signed app's host, else its path. The new tab's path is `builtin:newtab` (`newtab.NEWTAB_KEY`).
- **Only the page in front asks you.** A page behind with an ask that needs approval waits until it's shown, and leaving the page is Don't allow (`effects.left_behind`).
- **The UI is `src/ui/grants.la`** (the summary, the page, `shows_row`, `shows_icon`, the dialog's content), with tests in `test/grants.la`. `test/permissions.la` was already `bundles/permissions.la`'s.
- **Wording:** the row labels ("Reading your pins") and the dialog's ("read your pins") are `label_of` and `wish_of` in `bundles/permissions.la`.

- **Session 171 fixes (the owner's screenshot: the dialog couldn't be closed):** ⚙'s popover closes with `Msg.ClosePanel`, never the toggle, because a popover can report closing twice and the toggle reopened it, holding every click. An approval also waits while ⚙'s or the shield's card is open, and asks once it closes.

## LATER (parked by the owner, session 168)

- "Where you left off" went empty after the quote tab closed (the owner's screenshots, session 168; steps unknown). Look into it once this is built.
