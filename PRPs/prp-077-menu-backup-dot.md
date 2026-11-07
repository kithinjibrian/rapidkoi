## FEATURE: ☰ carries a caution dot while you aren't backed up, and its card offers "Back up your words".

Status: approved

## OBJECTIVE
Once an app has been given your key (so there's a you to lose) and you haven't written down your backup words, ☰ in the header has a small dot in `caution`, as the bell has its accent dot while anything is unread. Opening ☰ shows "Back up your words" above Settings; it opens Settings with Rapidkoi's backup-words card over it (PRP-076), at the words. Once you've backed up (or used words to become someone), the dot and the item go. Closing the reminder notice doesn't remove them: they stay until you back up.

## CONTEXT
- Read first: `src/ui/header.la` (`end`, `menu_card`), `src/ui/bell.la` (the dot over an icon: `Icon.Dot`, `UPDATE_DOT`, `overlay`), `src/window/backup.la` (`backed_up`, `card`, `written`, `become`), `src/window/ownapps.la` (`open_settings`), `src/data/restoring.la` (the identity read at start).
- Changes: `src/window/window.la` (one field), `src/window/backup.la`, `src/window/actions.la` (one message), `src/shell.la`, `src/ui/header.la`, `src/data/restoring.la`, `src/guest/effects.la`; tests in `test/header.la`, `test/backup.la`.
- Decisions: DECISION-101 (backup words, shown once), DECISION-022 (no account; 👤 removed). None open.

## REQUIREMENTS

### Must Do
- `Model.unbacked: bool`: true only when an identity file exists and says not backed up. Set where the identity is read or changes: at start (the identity restorer), when an app is first given your key (where `reminded` is called), on `written` (false) and `become` (false). `view` reads only this field.
- ☰ shows `Icon.Dot` in `t.caution` over its top end while `unbacked`, placed as the bell's dot is.
- ☰'s card, while `unbacked`: "Back up your words" first, then the items as now. It sends `Msg.BackUp`: Settings opens (or comes to the front) and `backup.card(m, its tab, false)` opens over it.
- Backed up, the card is never shown again (PRP-076): with `unbacked` false, nothing offers it.

### Must NOT Do
- No dot or item before any identity exists: there's nothing to lose yet.
- No new copy beyond "Back up your words".
- No contract change: Settings isn't told anything new (closing the card already sends it `on_change`, PRP-076).

## DESIGN
- `unbacked` is a cache of the identity file's `backed_up`, refreshed by `backup.refreshed(m)` (reads `backup_of`, in `update` only: Rapidkoi's own small file, CLAUDE.md architecture 3). A failed read is `false`: the dot never shows from an error.
- `Msg.BackUp` in `shell.la`: `menu: false`, `ownapps.open_settings`, then `backup.card` over the Settings tab's id. If Settings can't start, its notice shows as now and no card opens.
- Screen readers: ☰'s label stays "Menu"; while `unbacked` it's "Menu, back up your words".

## TESTS
- [ ] A fresh window: no dot, card is Open App…, Settings, About Rapidkoi, Quit.
- [ ] An identity made and not backed up: `unbacked`, the dot shows, "Back up your words" is first.
- [ ] Pressing it opens Settings with the backup card at the words.
- [ ] Backed up through the card: `unbacked` false, no dot, no item.
- [ ] Restored from words on a fresh computer: `unbacked` false.
- [ ] Closing the reminder notice leaves the dot.

## ACCEPTANCE
- [ ] The tests above pass.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] For the owner, by eye: with an identity not backed up, ☰ has the amber dot; ☰ › Back up your words opens the card over Settings; after backing up, the dot and the item are gone.
