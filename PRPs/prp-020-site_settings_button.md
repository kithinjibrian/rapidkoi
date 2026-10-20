## FEATURE: A site settings button in the address box, right of the shield, disabled until the settings exist.

## OBJECTIVE
The owner wants site settings reachable from the address box (session 127). On every app page, signed or not, a settings button (the Iconsax `settings` icon, `ADDRESS_ICON`) sits inside the address box right after the 🛡 (first in the box when there's no shield). For now it's greyed out, "Site settings (coming soon)": the settings themselves (app info, Account, Access, Data, likely a `ui.sheet` from the right) are the next PRP, which makes this button open them. Hidden on home, like ☆.

## CONTEXT

- Starting state: `src/ui/addressbox.la`'s `field` puts the shield at the group's start and ☆ at its end; `Icon.Settings` exists in `src/ui/icons.la`.
- Ending state: `field` adds the settings button after the shield on app pages; `test/header.la` checks it. DESIGN.md, CHANGELOG.md.
- kivuli API used: `kivuli.ui`'s `group_button_with`, `.disabled`, `.tooltip` (all in 0.50).
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- `group_button_with([icon(Icon.Settings, t.text) at ADDRESS_ICON, centred])`, disabled, tooltip "Site settings (coming soon)".
- On app pages only; after the shield when it shows.

### Must NOT Do
- Build the settings sheet or wire the button to a message (the next PRP).
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None.

## ERROR HANDLING REQUIREMENTS

- None.

## SECURITY CONSIDERATIONS

- None: the button does nothing yet. The settings it will open are trust screens, drawn natively by Rapidkoi (DECISION-027).

## TESTS TO WRITE

`test/header.la`:
- [x] On an app page (signed or not), "Site settings (coming soon)" is there and disabled.
- [x] On home, it isn't there.

## ROLLBACK PLAN

- `git revert` the commit on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] The settings icon shows right of the shield on an app page, greyed out, with its tooltip; not on home
- [ ] `lughac test test/header.la`, then `lughac test test/` pass (the owner runs them)
- [x] `lughac build src/main.la -o main` succeeds; `lughac fmt --check src test` is clean

## VALIDATION
- For the user: `./main rapidkoi://quote.dafifi.net`: 🛡 then the greyed settings icon at the box's start, ☆ at its end; home shows neither.
