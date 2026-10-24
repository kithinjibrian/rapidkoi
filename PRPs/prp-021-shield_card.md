## FEATURE: The shield always shows, in its state, and clicking it opens a card: the app, its connection and its signature.

## OBJECTIVE
Designed in chat with the owner (sessions 128–131). On every app page the 🛡 sits at the start of the address box in one of three looks: verified, caution or unsigned. Clicking it opens a small card under it (a kivuli 0.51 popover), after the owner's sketch: the app's icon and name at the top, then two rows, "Connection secure ›" and "Signed app ›", whose words change with the state; each opens its own page in the card, with ‹ back. The shield answers "can I trust this?"; what the app may do stays in ⚙ site settings.

## CONTEXT

- Starting state: `src/ui/addressbox.la` shows the shield (a picture, no action) only for verified or trusted signed apps; site settings (⚙, disabled) after it; kivuli 0.50.
- Ending state:
  - `lugha.toml`: kivuli 0.51 (`lughac update`).
  - `src/ui/shield.la` (new, pure): `Shield` facts worked out from an `AppPage` (`facts(p)`), its `Look` (Verified, Caution, Unsigned), and `card(m, page)`, the popover's content.
  - `src/window/actions.la`: `Msg.Shield(Option<ShieldPage>)` (open on a page, or close); `src/window/window.la`: `Model.shield: Option<ShieldPage>`, cleared when the page changes (another tab, home, a new app).
  - `src/shell.la`: handles it.
  - `src/ui/addressbox.la`: the shield is a button on every app page, in its look, wrapped in `popover(…)`.
  - `src/ui/themes.la`: the three looks' colours.
  - Tests: `test/shield.la` (new), `test/header.la` updated.
  - DESIGN.md, CLAUDE.md's tree, CHANGELOG.md.
- Related existing code: `src/ui/frame.la` (`marker`, `marker_note`), `src/trust/verify.la` (`Trust`, `Standing`), `src/trust/keys.la` (`fingerprint`), `src/net/address.la` (`parse`: a local development address or a remote one), the old panel's `first_trusted` date words (git: `src/panel.la` before `80ed95c`).
- kivuli API used: `popover(trigger, open, content, on_close)`, `kivuli.ui`'s `ui.popover` (288 px panel), `button_with`, `Part.Popover` (0.51).
- Open decisions: none.

## WHAT THE CARD SAYS

The card: the app's icon (its bundle icon, else its initial) and name; then the two rows.

| Row | Verified | Caution | Unsigned |
|---|---|---|---|
| Connection | Connection secure (a remote app, HTTPS) | This computer (a development address) | Local file |
| Signed | Signed app | Not confirmed yet (a signed file whose key isn't trusted) · New publisher (opened as a separate app after its key changed) | Not signed |

The shield's look: Verified when the signature checks out (a verified site, or a signed file whose key Rapidkoi trusts); Caution for a development address, a key not confirmed yet, or a new publisher; Unsigned when nothing is signed. A trusted signed file is Verified though its connection reads "Local file": a file can't have an insecure connection, and its signature is what vouches for it.

**Connection page:** a sentence for the case ("Rapidkoi reached HOST over a secure connection: nobody between you can read or change it." / "Loaded from this computer for development." / "Opened from a file on this computer."), then the host or file.

**Signed page:** a sentence ("HOST's publisher signed this app: you're running exactly what they published." / the caution's reason / "Nothing vouches for this app."), then Publisher (host · key fingerprint), Known since ("Same publisher since 18 Oct 2026", "first use" date words as the old panel had them; or "New publisher"), Version (`1.2.0 · verified build`, when there's a manifest).

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Facts are pure (`shield.facts(p)`) and tested apart from the view.
- The shield button: `Icon.Shield` at `ADDRESS_ICON`, coloured by look (verified `on_frame`'s white, caution the accent orange, unsigned dim), tooltip "Site information".
- The card: `ui.popover`, open while `m.shield` is `Some`; a click outside or Escape sends `Shield(None)`; the rows send `Shield(Some(page))`; ‹ sends `Shield(Some(Main))`.
- Every colour from `themes.la`, every size a token.

### Must NOT Do
- Put permissions, servers or data in the card (⚙ site settings, a later PRP).
- Change what's verified or how (only show it).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- `Model.shield: Option<ShieldPage>`, `ShieldPage { Main, Connection, Signed }`: the card's page while open.
- `Msg.Shield(Option<ShieldPage>)`: open the card on a page, go to a page, or close it (`None`). Selecting a tab, going home or opening an app closes it.

## ERROR HANDLING REQUIREMENTS

- None new: every fact comes from a page Rapidkoi already checked.

## SECURITY CONSIDERATIONS

- A trust screen (DECISION-027): drawn only by Rapidkoi, from its own checks, in the header where apps can't paint. An app can't open, change or imitate it.
- It shows a key's fingerprint, never anything secret.

## TESTS TO WRITE

`test/shield.la`:
- [x] A remote signed app whose site checked out: Connection secure, Signed app, look Verified, host, fingerprint, version.
- [x] A signed file whose key Rapidkoi trusts: Local file, Signed app, look Verified (its signature checks out against a trusted key; where it came from is shown in words).
- [x] A signed file not confirmed: Not confirmed yet, look Caution.
- [x] A new publisher: New publisher, look Caution.
- [x] A development address: This computer, look Caution.
- [x] A bare module or unsigned bundle: Local file, Not signed, look Unsigned.
- [x] "Known since" words for a date and for 0 (not recorded).
- [x] `update`: `Shield(Some(Main))` opens, `Shield(Some(Signed))` turns the page, `Shield(None)` closes; selecting a tab or going home closes it.

`test/header.la`:
- [x] The shield is a button on every app page ("Site information"), not on home; clicking it sends `Shield(Some(Main))`; in the card, "Connection secure" sends `Shield(Some(Connection))` and ‹ sends `Shield(Some(Main))`.

## ROLLBACK PLAN

- `git revert` the PRP's commits on `prp-017-kivuli_0_17`; kivuli back to 0.50.

## ACCEPTANCE CRITERIA
- [ ] The shield on every app page, in its look; clicking it opens the card under it; each row opens its page; ‹, Escape and a click outside behave
- [ ] The tests above pass, and `lughac test test/` (the owner runs them)
- [x] `lughac build src/main.la -o main`; `lughac fmt --check src test` clean
- [x] DESIGN.md, CLAUDE.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main rapidkoi://quote.dafifi.net`: a white shield; click it: the card with the app's name, "Connection secure ›", "Signed app ›"; open each, go back, press Escape. Then a local file (`./main counter.rkb`): a dim shield, "Local file", "Not signed".
