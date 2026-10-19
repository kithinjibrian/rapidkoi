## FEATURE: The address box: a pill in the header bar, and a card of local suggestions under it while typing.

## OBJECTIVE
The second redesign step (DECISION-027), from the owner's sketch (sessions 74–76). The header bar holds one thing besides − □ ×: a centred pill-shaped field. At rest it says "Enter Address", or shows the current app's address. Typing opens a rounded card under it, over the page: first what was typed, then "Rapidkoi suggest" with the open tabs, pins and where-you-left-off that match. Enter, → or a click on a suggestion opens it, and the app shows in the content area. Escape or a click on the page closes the card.

## CONTEXT

- Starting state: PRP-014's bare frame (`src/layout.la`): empty header, sidebar and content. The old bar (`frame.bar`), home's search (`home.results`, `home.matches`) and the old page drawing (`legacy.app_view`) are only in the old view.
- Ending state:
  - `src/suggestions.la` (new, pure): `Suggestion` (a label, an optional detail such as a screen title, the `Msg` it sends) and `suggest(query, tabs, pinned, card): Suggestion[]`. Home's matching rules moved here: open tabs, then pins, then a closed where-you-left-off, matched by name or title, ignoring case.
  - `src/addressbox.la` (new, pure): `field(m)` (the pill for the header) and `card(m)` (the suggestions card, or nothing).
  - `src/content.la` (new, pure): `app_view`, moved unchanged from `legacy.la`. `legacy.la` imports it from here.
  - `src/layout.la`: the header's title is `addressbox.field`; the content area shows the current app (home stays empty until the new tab app); the body is an overlay holding the clear layer and the card while it's open.
  - `src/actions.la`: `Msg.Dismiss`. `src/shell.la`: handles it.
  - `src/themes.la`: looks `Address` (the pill), `Suggestions` (the card), `Clear` (the layer that catches clicks). `src/theme.la`: `ADDRESS_HEIGHT`, `ADDRESS_WIDTH`.
  - `src/home.la`: `matches` moves out (home keeps calling it from `suggestions`).
  - Tests: `test/suggestions.la`, `test/addressbox.la` (new); `test/layout.la` updated.
  - `docs/DESIGN.md`, CLAUDE.md's tree, CHANGELOG.md.
- Related existing code: `src/frame.la` (`bar`, `marker`, `marker_icon`, `marker_note`), `src/home.la` (`results`, `matches`), `src/navigate.la` (`go`, the notices), `src/shell.la` (`Draft`, `Go`, `EditAddress`, `FocusSearch`), `src/legacy.la` (the panel's overlay and scrim), `../kivuli/README.md` (`overlay`, `.place`, `input`, `.on_submit`, `.placeholder`, `.name`, `.autofocus`, `.shortcut("Escape")`, `button_with`, `Cmd.focus`, per-corner `radius`).
- Related source files: the owner's sketches (sessions 72 and 74); this discovery (CONTEXT.md sessions 74–76).
- Lugha spec sections relied on: none new.
- kivuli API used: all in kivuli 0.10, listed above.
- Open decisions that must be resolved first: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **The pill:** `ADDRESS_HEIGHT` tall, `ADDRESS_WIDTH` wide at most, centred in the header bar, corners rounded by `ADDRESS_HEIGHT / 2` so its ends are half circles (`Look.Address`).
  - On home: a field with the hint "Enter Address".
  - On an app at rest: the app's address with its trust marker (`frame.marker`, `marker_icon`, `marker_note`: Rapidkoi draws it, never the app); a click or Ctrl+K turns it into the field (today's `EditAddress`).
  - While typing: the field and a → button at its end ("Go"), which sends what Enter sends.
- **The card** shows while the box has text, under the pill, the same width, over the page (`Look.Suggestions`: the page colour, a thin edge, rounded):
  1. What was typed. Opens it (`Msg.Go`).
  2. "Rapidkoi suggest", then each match: open tabs (`Select`), pins (`OpenPinned`), where-you-left-off when closed (`Continue`). The heading is left out when nothing matches.
  3. When opening fails, the reason (today's notice) shows dim in the card, and the card stays open so it can be fixed.
- **Closing:** Enter, → or a suggestion opens it and closes the card. Escape or a click outside the card (a `Look.Clear` layer over the page and sidebar, under the card) sends `Msg.Dismiss`: the box is emptied, editing ends, the notice clears, so an app page shows its address again.
- **Lining up:** the overlay covers the content area and the sidebar together, so the card's centre is the window's centre, under the header's.
- **The content area** shows the current app as before (`content.app_view`): running, loading, stopped, refused, or the changed-key question. Home shows nothing.
- Suggestions are local only: nothing typed leaves the computer.

### Must NOT Do
- Arrow keys through the suggestions: kivuli has no key events in a text field. Tab and Enter work. Later, with a kivuli change.
- A popover holding the field (the sketch exactly): later, with a kivuli change (session 75).
- Back, forward, reload, ☆, ⓘ, share, tabs or the rail: later redesign PRPs.
- Suggest an app's screens (`/about`): they come with links.
- Any online suggestion service.

## MODEL AND MESSAGES

- Model fields added or changed: none (`draft`, `editing`, `notice` already exist).
- Messages added: `Dismiss`: Escape or a click outside the card. Empties the box, ends editing, clears the notice.

## ERROR HANDLING REQUIREMENTS

- No new failures. `navigate.go`'s existing notices ("No app or file called “x”", an address's reason) show in the card instead of above the page.
- A failed open in a tab (refused, stopped) shows in the content area, as today.

## SECURITY CONSIDERATIONS

- What's typed is matched in memory against what's already on this computer; nothing is sent, logged or stored.
- The trust marker stays Rapidkoi's: the pill sits in the header bar, where apps can't draw.
- No new dependency, capability or file.

## TESTS TO WRITE

`test/suggestions.la`, pure:
- [ ] An empty query suggests nothing.
- [ ] A query matches an open tab by name and by screen title, ignoring case, and sends `Select(id)`.
- [ ] A query matches a pin, sending `OpenPinned(path)`.
- [ ] A closed where-you-left-off matches, sending `Continue`; an open one isn't listed twice.
- [ ] The order is tabs, pins, left-off.

`test/addressbox.la`, through `kivuli.testing`:
- [ ] Home at rest: a field with the hint "Enter Address", and no card.
- [ ] Typing sends `Draft`; with text, the card shows what was typed first, and clicking it sends `Go`.
- [ ] → ("Go") sends `Go`; Enter in the field sends `Go`.
- [ ] Matches show under "Rapidkoi suggest"; with none, the heading is gone.
- [ ] Clicking a suggestion sends its message; after `update`, the card is gone and the app is current.
- [ ] Escape (`press(v, "Escape")`) and the clear layer send `Dismiss`; after `update`, the box is empty and the card is gone; on an app page its address shows again.
- [ ] Opening "nope" shows "No app or file called “nope”" in the card, and the card stays.
- [ ] On an app page at rest, clicking the address sends `EditAddress`; Ctrl+K does too.

`test/layout.la`:
- [ ] The content area shows the current app's screen (the echo fixture's text).
- [ ] Home's content area is empty.

## ROLLBACK PLAN

- Branch to return to: `redesign`, with PRP-014 merged (the PRP's branch, `prp-015-address_box`, is deleted unmerged).
- State: the bare frame.

## ACCEPTANCE CRITERIA
- [ ] The pill sits centred in the header, ends round; typing opens the card under it; Enter, → and suggestions open apps in the content area; Escape and a click outside close it
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/suggestions.la test/addressbox.la test/layout.la`, then `lughac test test/`
- `lughac build src/main.la -o main`
- For the user: `./main`. A centred pill saying "Enter Address" in the header, with round ends. Type part of a pinned app's name: a rounded card drops under the pill with what you typed, then "Rapidkoi suggest" and the app. Click it: the app shows in the content area and the pill shows its address. Click the address, type "nope", press Enter: the reason shows in the card. Press Escape: the card closes.
