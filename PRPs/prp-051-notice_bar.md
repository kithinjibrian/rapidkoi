## FEATURE: The notice bar: Rapidkoi's notices shown at last, one short sentence at the top of the page, the full message on hover

## OBJECTIVE
When something happens that you should hear about but that doesn't stop the window, a one-line bar appears between the header and the page: a short sentence ("Rapidkoi couldn't save your pins", "Notes started over"), the full message when you hover it, and × to close it. It takes no room when there's nothing to say. A notice about an app stays with that app's tab; a notice about Rapidkoi itself follows you to whatever tab is in front. Nothing is lost unseen.

## CONTEXT
- From chat with the owner (2026-10-30), with the comparison page https://claude.ai/artifact/GNPPQNmF2KSdJD42Xsw8EB (a dialog, a bottom bar and this bar; the owner chose this bar, one sentence with the full message on hover).
- **Starting state:** `Model.notice: Option<string>`, one for the whole window, set in about 20 places and drawn nowhere since the address box's suggestions card went (session 108: "the reason is in `m.notice`, shown nowhere"). So every notice below is silent today, among them a failed save of your pins or trust, and a failed upgrade (PRP-048).
- **Discovery answers (the owner):**
  - Q1 (a): a notice about an app stays with its tab; one about Rapidkoi follows you.
  - Q2 (a): one quiet look for every notice: the controls' grey, the text's own colour, ×. No colour by seriousness.
  - Q3 (a): Rapidkoi's notice shows first; a tab's waits behind it. A newer notice of the same kind replaces the older.
  - Q4 (a): besides ×, a tab's notice goes when the tab closes or loads something new (another app, a reload, a successful upgrade); one on an empty tab goes when something opens there; Rapidkoi's go only on ×.
  - Q5 (a): the short sentences below.
- Files read first: `src/window/window.la` (`Model.notice`), `src/ui/layout.la` (where the page is drawn), every place `notice` is set (listed under DESIGN), `docs/DESIGN.md`.
- Files changed: new `src/window/notices.la` (the notices and their rules) and `src/ui/notices.la` (the bar); `src/window/window.la` (the model), `src/ui/layout.la` (the bar above the page), `src/window/actions.la` and `src/shell.la` (closing it), `src/ui/tokens.la` if a size is needed; and each place a notice is set: `src/net/loads.la`, `src/window/upgrades.la`, `src/window/navigate.la`, `src/data/saving.la`, `src/data/sessions.la`, `src/people/access.la`, `src/guest/effects.la`, `src/shell.la`. Tests: new `test/notices.la`; the 27 lines in 8 test files that read `m.notice` follow the new shape.
- Decisions: DECISION-031 (colours from the theme). None open that this depends on.

## REQUIREMENTS

### Must Do
- **A notice has two parts:** the short sentence for the bar, and the full message for its hover.
- **Where it shows:** a bar between the header and the page, only while there's a notice to show: the short sentence, ellipsized if the window is too narrow, the full message as its tooltip, and × at its end ("Close"). It pushes the page down; it covers nothing.
- **Whose it is:**
  - **A tab's notice** (about an app, or typed on an empty tab) shows only while that tab is in front, and stays there when you switch away and back.
  - **Rapidkoi's notice** (its own files, your identity, its data folder, Settings) shows on whichever tab is in front, an empty one too.
- **One at a time:** Rapidkoi's notice shows first; the tab's shows once it's closed. A newer notice replaces an older of the same kind (a tab keeps one; Rapidkoi keeps one). Two that arrive together (two files failing to save at once) join in one notice.
- **When it goes:** × closes the one showing. A tab's notice also goes when the tab closes, or when the tab loads something new (another app, a reload, a successful upgrade; Continue, a pin, an address typed there); if that load has its own notice, that one shows. Rapidkoi's go only on ×.
- **The look:** one look for every notice: the theme's `control` background, the text's own colour, × as the cards' close buttons are; padded `SPACE_SM` × `SPACE_MD`. No colour by seriousness.
- **The words** (the owner, Q5): the bar's short sentence, then the hover's full message, today's sentence unless noted:

  **A tab's (about an app)**

  | When | Bar | Hover |
  |---|---|---|
  | typed something that isn't an app or file | No app or file called "X" | the same |
  | an address that can't be opened | This address can't be opened | the reason (e.g. "Remote apps open over HTTPS only: rapidkoi://…") |
  | a link's version no longer listed | This link opened Notes 2.0 | This link was made with Notes 1.0; opened 2.0 |
  | a link's version not kept, offline | Opened Notes 1.0 offline | 0.9.0 isn't available offline; opened 1.0.0 |
  | an upgrade or reload failed (PRP-048) | Notes couldn't be reloaded | the refusal (e.g. "Couldn't reach rapidkoi://…: …") |
  | its window state didn't fit the new version | Notes started over | Notes's saved state changed shape, so it started over |
  | its saved state was damaged | Notes started fresh | Rapidkoi couldn't read Notes's saved state, so it set it aside in … |
  | it may now reach more servers | Notes can reach more servers | Notes can now also reach … |
  | Clear or Forget couldn't remove its data | Notes's data couldn't be removed | the file and why |

  **Rapidkoi's**

  | When | Bar | Hover |
  |---|---|---|
  | saving one of its files failed | Rapidkoi couldn't save your pins (trust, session, …) | Rapidkoi couldn't save your pins: … |
  | reading one failed; set aside | Rapidkoi couldn't read your pins | Rapidkoi couldn't read your pins, so it set it aside in … |
  | reading one failed; nothing will be saved | Rapidkoi won't keep your pins this session | Rapidkoi couldn't read your pins (…); changes this session won't be kept |
  | clearing the last session failed | Rapidkoi couldn't clear your last session | Rapidkoi couldn't clear your last session: … |
  | no home folder, or the data folder can't be opened | Rapidkoi can't save anything this session | There's no home folder, so nothing will be saved this session / the folder and why |
  | your identity couldn't be read, kept or used | Rapidkoi couldn't read your identity / keep your identity / work out your key | the full message |
  | Settings couldn't open | Settings couldn't open | why |

### Must NOT Do
- No dialog, toast or bottom bar; nothing disappears on a timer (Q4).
- No colour by seriousness (Q2); no icon.
- No new notices: the same events as today, with their words split in two.
- Escape in the address box (`Dismiss`) no longer clears a notice: only × does.

## DESIGN
- **`src/window/notices.la`:** `Notice { short: string, full: string }`; the model keeps `notice: Option<Notice>` (Rapidkoi's) and `tab_notices: (i64, Notice)[]` (at most one per tab). Functions: `for_tab(m, id, n)`, `for_rapidkoi(m, n)` (each replacing, or joining what arrived in the same update), `shown(m) -> Option<Notice>` (Rapidkoi's first, else the current tab's), `dismiss(m)`, `tab_closed(m, id)`, `tab_loaded(m, id)`. Every place that sets a notice today calls one of the two setters with both parts.
- **Clearing on a new load:** `loads.begin_at`, `navigate.open_file` (a file opened in a tab), Continue and a pin call `tab_loaded` for the tab first; `window.close` calls `tab_closed`.
- **`src/ui/notices.la`:** `bar(m) -> Option<View<Msg>>`; `layout.la` puts it above the page when there is one.
- **Message:** `Msg.DismissNotice`.
- **Security:** no new input; the full messages already exist. A message that names a file path (a save failure) shows only on hover, as today it shows nowhere: no new path is exposed to an app (the bar is Rapidkoi's, never an app's).

## TESTS
- [x] A tab's notice shows only on its tab, and is still there after switching away and back
- [x] Rapidkoi's notice shows on any tab, an empty one too
- [x] Rapidkoi's shows first; closing it shows the tab's behind it
- [x] A newer notice of the same kind replaces the older; two in one update join
- [x] × closes the one showing; a tab's goes when the tab closes or loads something new; Rapidkoi's stays until ×
- [x] The bar shows the short sentence with the full message as its tooltip, and nothing when there's no notice
- [x] Each notice in the tables is set with its short sentence and full message, as the right kind
- [x] Escape in the address box leaves the notice

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] For the owner, by eye: type an address that doesn't exist (the bar on that tab; switch away and back; ×); open a link to an unlisted version; make the data folder read-only and pin an app (the save failure on every tab, over the tab's own notice; hover for the full message)

## AMENDMENTS

- **`Notice` lives in `window/window.la`** beside the model (the model holds it; `window/notices.la` imports the model). `window/notices.la` holds the rules and every notice's words, one function per row of the tables.
- **A notice is no longer wiped by accident:** before, opening something, switching tabs, typing in the address box and every app's arrival (`arrive`) set the one notice line to nothing. Some failures were only ever hidden that way: a failed save of what's open, made while an app loaded, now shows. Three tests that relied on that (`saving`, `shell`, `layout`) now expect the notice.
- **Removing an old session's extra windows** failing shows as "Rapidkoi couldn't clear your last session", the full message naming what.
- **Forget** that couldn't remove an app's data is Rapidkoi's notice (its tab is gone); Clear's is the tab's.
- **Two notices of a kind in one change join** (`with_more`): a save failure on top of an earlier one, two files failing together, what restoring said with what opening said.
- **Tests read the full message** through `support.said(m)`.
