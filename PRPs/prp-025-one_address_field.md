## FEATURE: One typing box for the address on every tab, with share (and pin on apps) at its end.

## OBJECTIVE
The owner (session 154): "make sure you're using the same address bar with app opened or new tab"; "yes one typing box for both". The address box holds the same text field on an empty tab and on an app: empty, it says "Enter Address"; on an app it shows the app's address (its host, or its file), and you click in and type straight away, no button swapping into a field. At the box's end: share (the owner's new `export-circle2.svg`), then pin on an app; an empty tab has share only. Share needs links (build step 2), so it's greyed, "Share (comes with links)", like ← ⌂ →.

## CONTEXT

- Starting state: `src/ui/addressbox.la`: on an app at rest the middle is a button showing the trust marker (`frame.marker`), with Ctrl+K and a click sending `EditAddress`; on an empty tab or while editing, `group_input`. End: ☆ on apps.
- Ending state:
  - `assets/icons/export-circle2.svg` (the owner's) as `Icon.Share` in `src/ui/icons.la`.
  - `src/ui/addressbox.la`: the middle is always `group_input`, showing `m.draft` while editing, else the app's address (`shield.facts(p).place`), hint "Enter Address"; its hover text the trust marker's words (`frame.marker_note`); it takes focus by itself only on an empty tab. End: share (disabled), then ☆ on apps.
  - `src/shell.la`: `Draft` on an app page starts editing (`editing: true`), so what's typed shows; Enter goes, `Dismiss` puts the address back.
  - Tests follow (below). DESIGN.md, CHANGELOG.md.
- Lost: Ctrl+K. kivuli gives shortcuts to buttons only, and the address is no longer a button; noted as a known issue (a window-level shortcut is a kivuli request if the owner wants Ctrl+K back).
- Open decisions: none.

## TESTS TO WRITE

- [x] `test/icons.la`: `Icon.Share` is its file (the existing every-icon test covers it once it's in `ALL`).
- [x] `test/addressbox.la`: an app page's field shows nothing typed until typed in; typing sends `Draft`; after `Draft` the box is editing; `Dismiss` ends it.
- [x] `test/header.la`: share is there and disabled ("Share (comes with links)") on an empty tab and on an app; pin only on an app.
- [x] `test/loading.la`, `opening.la`, `saving.la`, `remote.la`: their trust-marker checks read `frame.marker_note` of the current page instead of clicking the old address button.

## ROLLBACK PLAN

- `git revert` the commit on `prp-017-kivuli_0_17`.

## ACCEPTANCE CRITERIA
- [ ] One field on every tab; share (greyed) at the end; pin after it on apps
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean
