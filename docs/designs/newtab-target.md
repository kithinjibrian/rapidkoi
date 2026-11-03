# The new tab: target design

**Status:** a target, not a decision. Nothing here is built or approved; each part that needs a new feature becomes its own PRP (and, where it changes the contract or the shell's architecture, a DECISIONS.md entry) before any code.

**Drawn:** 2026-11-01, with the owner, on a design canvas (https://claude.ai/artifact/JHuEyqkgAQMcxHS9kjYHr2, boards A5 and A5b). Standalone copies: `newtab-target/a5.html` (an app gave its place a picture and summary) and `newtab-target/a5b.html` (it gave none). Open them in a browser; the content in them is sample content.

**Today** (`guest/rust/newtab/src/lib.rs`): the wordmark, Restore previous session (N windows), Where you left off with Continue, and Pinned as text buttons.

---

## The page, top to bottom

1. **The wordmark and Restore.** "Rapidkoi" at the start, in the brand gradient (`accent` → `accent_end`, `#ff7700 → #ff2e63`). At the end, only while there's a session to restore, a pill: "Restore previous session", the icons of the apps it brings back, and Restore (primary).
2. **Notifications**, only while something is unread: the heading with an accent count, See all at its end, then the newest four in two equal columns.
3. **Recent** (real since PRP-062, without an app's own picture or summary yet): places, not apps. The newest one large in the left half; the next four as small cards, two by two, in the right half.
4. **Pinned** (real since PRP-061): the pinned apps as large icons with their names, one line of them, and See all at the heading's end (up to 256 pins; a line never wraps, as contract 9's rows don't and the app doesn't know the window's width).

The page has a faint glow of the brand colours at its top-left corner. Sections are `SPACE_XL` apart; headings are small, bold, spaced capitals in `dim`.

## Restore

- One icon per app, not per window (restoring brings every window of that app back). At most three, overlapping, then "+N".
- An app without an icon shows its initial in a circle of its colour from the theme's `publishers`, as the dock does.
- Hovering lists every app and its windows; a screen reader hears "Restore previous session: Notes, Trip, Counter and 2 more".

## Notifications

- **One list, three senders,** told apart by what's drawn, never by what an app says:

  | Kind | From | Drawn as | Button |
  |---|---|---|---|
  | An app's own | the app | its icon; its sentence | none: the row opens the place |
  | Invited / shared with you | a person, through Rapidkoi | their face, the app's icon on its corner | Join (primary) / Open |
  | An update | Rapidkoi | the app's icon with Rapidkoi's mark on its corner | Update |

- Only Rapidkoi makes invites, shares and updates. An app can send only its own kind: no button, no face, no mark.
- Items with a button come first, then the rest; newest first within each.
- An item goes when it's no longer true: an invite when its session ends or expires, an update once installed (a newer one replaces it).
- Every notification is a sentence and a place: opening it opens the app there.
- Rapidkoi draws them; an app gives values, never a look.
- Elsewhere: a bell in the header (before 👤 and ☰) with an accent dot while anything is unread, opening a card of the latest; an accent dot on each dock entry with something unread (the count badge stays the windows' count).
- Desktop alerts (the system's, when Rapidkoi isn't in front) are off until the person turns them on for an app.
- Limits, with the guest's other caps: the length of each, how many are kept per app, how often an app may send, and only while it runs.

## Recent

- One section. The newest place is the bet, drawn large; it isn't a separate "Continue".
- **Every card has the same parts:** a picture area, the place's title (bold), then its details in `dim`: the app's summary when it gave one, the app, its link (large card only), and when you were last there.
- **The picture area** is the picture the app chose for that place, or, without one, a wash of the app's colour from the top-left with its icon on it (`a5b.html`). No screenshots: Rapidkoi never captures an app's screen.
- The whole card opens the place; hovering lifts it and edges it in `accent`.
- Small cards are the large card made small, so they read as places, not as pinned apps.

## What it needs, by repository

Checked 2026-11-01 against kivuli 0.66, lugha-gtk, rapidkoi-rust v0.3.0-pre.4 and contract 9. **Shipped** (2026-11-01): kivuli 1 (kivuli 0.67, its PRP-072) and the contract's 1–4 (contract 10, PRP-059, rapidkoi-rust `v0.4.0-pre.1`); the replica draws with them, and only the wordmark, the beach and the apps' icons stay pictures. Every rapidkoi item is a PRP first; contract items share one new contract version (a DECISIONS.md entry), tagged in rapidkoi-rust before the built-ins move to it.

### kivuli

Nearly everything is there already: `Gradient.Linear`/`Radial`, `overlay`, `wrap` with `.equal()` and `.max_per_line(n)`, `fit_row` (as many as fit, then a "more"), pictures with `cover`.

1. **Shipped (0.67).** **A radial gradient with a shape and a centre**: an ellipse from a corner (`radial-gradient(ellipse 95% 130% at 0% 0%, …)`). Today's `Radial` is a circle from the centre. For the washes and the page's corner glow *(replica: wash-\*.svg)*.

The wordmark's gradient stays a picture: GTK's CSS can't fill text with a gradient.

### lugha-gtk

1. **Desktop alerts**: `GNotification` (new, title, body, icon, buttons) and `g_application_send_notification` / `withdraw_notification`. Nothing else here: the rest is CSS and widgets kivuli already binds.

### rapidkoi: the contract (1–4 shipped in contract 10; 5–7 wait for what reads them)

1. **Gradients** in `background`: linear, and radial with a shape and a centre *(replica: wordmark aside, every wash)*.
2. **Layers**: a box whose children sit on top of each other, for a face with the app on its corner, an icon on its wash, and Restore's overlapping icons *(replica: face-\*.svg, update-notes.svg, restore-apps.svg)*.
3. **Wrapping boxes** (kivuli's `wrap`, `equal`, `max_per_line`): Notifications' two equal columns and the small cards' grid, without the replica's wide filler picture (wash-fill.svg) or fixed minimum widths.
4. **A fitting row** (kivuli's `fit_row`): Pinned's one line, as many as fit, then See all.
5. **A place's card**: `summary` and `picture` (`asset:`/`blob:` only) on the view's root beside `title`; a bad value is ignored, never stops the app.
6. **Shipped (contract 11, PRP-060)** as `icon:PLACE`. **Pictures Rapidkoi hands over**: a picture source for an app's icon from an answer (pins, recent places, the session, notifications). Today an answer carries the icon's PNG bytes, which no `image` can show.
7. **Not needed (PRP-060):** `icon:` draws an app without an icon as its initial on a theme colour. **An app's colour** in those answers (the theme's `publishers`), for the initial of an app without an icon and for its wash.

### rapidkoi: the shell

1. **Recent places**: a record of places with when each was last open, replacing the one "where you left off"; an ask answering the newest five (title, summary, picture, icon, colour, link, time) and one opening a place by its index.
2. **Show in Recent**: a switch per app in its ⚙ card; off forgets its places.
3. **Shipped (PRP-060)**, with names and icons, and the new tab's Restore is real. **The session's apps**: Restore's answer gives each app's name, icon and colour, not only names and a window count.
4. **Shipped (PRP-065)**, an app's own kind; See all comes with the notifications page. **Notifications**: the manifest's `notifications` permission; an ask to send one (a sentence and a place) and its limits (length, how many kept per app, how often, only while running); keeping them; a built-in permission for the new tab to read them; ordering (with a button first, newest first) and removal when no longer true; opening one opens its place.
5. **The bell**: in the header before 👤 and ☰, an accent dot while anything is unread, opening a card of the latest.
6. **The dock's dot**: on each entry with something unread.
7. **The notifications page**: a built-in app (core decision 19, DECISION-027), Notifications' See all.
8. **Desktop alerts**: off per app until turned on in its ⚙ card; sent only when Rapidkoi isn't in front (lugha-gtk 1).
9. **The pins page**: Pinned's See all, every pin (up to 256): a second screen of the new tab, or a built-in app.
10. **The new tab itself**: from the replica to real data, and the shell tests that check the new tab (`test/layout.la`, `test/newtab.la`, `test/restore.la`) back on real behaviour.
11. **Later**: invites and shared links with live sessions and sharing (build path step 5 onwards); updates with PRP-044's upgrade.

### rapidkoi-rust (a tag per contract version)

1. **The contract's new styles and nodes**: `Background` gradients, a layer box, `wrap` (equal, per line), the fitting row with its "more".
2. **A place's card**: setting `summary` and `picture` on the view's root.
3. **Pictures from answers**: an `image` source for an icon Rapidkoi gave, and the colour on `Place`.
4. **Asks**: recent places and opening one; the session's apps with icons and colours (`Session` today has `count` and `apps` as names); reading notifications and opening one (built-in); sending one (`notifications` permission).

## Not decided

- Whether the bell's card and the new tab show the same list, or the bell only what arrived since you last looked.
- ~~The first run: an empty page, or pins Rapidkoi ships with~~ Answered (PRP-064): a first run starts with Settings pinned.
