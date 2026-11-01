# The new tab: target design

**Status:** a target, not a decision. Nothing here is built or approved; each part that needs a new feature becomes its own PRP (and, where it changes the contract or the shell's architecture, a DECISIONS.md entry) before any code.

**Drawn:** 2026-11-01, with the owner, on a design canvas (https://claude.ai/artifact/JHuEyqkgAQMcxHS9kjYHr2, boards A5 and A5b). Standalone copies: `newtab-target/a5.html` (an app gave its place a picture and summary) and `newtab-target/a5b.html` (it gave none). Open them in a browser; the content in them is sample content.

**Today** (`guest/rust/newtab/src/lib.rs`): the wordmark, Restore previous session (N windows), Where you left off with Continue, and Pinned as text buttons.

---

## The page, top to bottom

1. **The wordmark and Restore.** "Rapidkoi" at the start, in the brand gradient (`accent` → `accent_end`, `#ff7700 → #ff2e63`). At the end, only while there's a session to restore, a pill: "Restore previous session", the icons of the apps it brings back, and Restore (primary).
2. **Notifications**, only while something is unread: the heading with an accent count, See all at its end, then the newest four in two equal columns.
3. **Recent**: places, not apps. The newest one large in the left half; the next four as small cards, two by two, in the right half.
4. **Pinned**: the pinned apps as large icons with their names.

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

## What it needs that doesn't exist

- **A record of recent places** with when each was last open (today there's one "where you left off").
- **A place's card, from the app:** beside `title` on the view's root, an optional `summary` (one line, about 200 bytes) and `picture` (`asset:` or `blob:` only: a server's picture would be fetched each time the new tab opens, and fail offline). Rapidkoi keeps the latest with the place. A value that breaks a rule is ignored and the card falls back; it never stops the app. A contract change: a new contract version and a DECISIONS.md entry.
- **"Show in Recent"**, a switch per app in its ⚙ card.
- **Notifications**: the manifest's `notifications` permission (in the vision), the asks and their limits, the bell and its card, the notifications page (a built-in app, core decision 19, DECISION-027), the dock's dot.
- **Invites and shared links** wait on live sessions and sharing (build path steps 5 onwards); **updates** on PRP-044's upgrade.
- **App icons on the new tab** (contract 9 can draw pictures; the comment in `newtab/src/lib.rs` saying otherwise is out of date).

## Not decided

- Whether the bell's card and the new tab show the same list, or the bell only what arrived since you last looked.
- The first run: an empty page, or pins Rapidkoi ships with (Notes isn't built in; it would be a pin to its site).
