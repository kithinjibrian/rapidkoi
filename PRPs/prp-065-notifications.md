## FEATURE: Apps send you notifications, if you let them; the new tab lists them

**Status:** approved (the owner, 2026-11-03: "approved, go ahead"). The core of three (the owner, 2026-11-03: "yes for the three"); the bell, the dock dot and the See all page come next, and desktop alerts after lugha-gtk has them.

## OBJECTIVE

- **An app that declares `notifications`** can send you a notification while it runs. It's one sentence and a place in that app.
- **The first time it sends one,** Rapidkoi asks over that app: "Counter wants to send you notifications", with Don't allow, Allow once and Always allow. This is the same question every declared permission asks today (core decision 21, PRP-031). You can change the answer in the ⚙ card's Permissions page. A blocked app's notifications are dropped, and the app keeps running.
- **The new tab shows Notifications** only while something is unread: the heading with an accent count, then the four newest in two equal columns (board A5), under Restore and above Recent, where A5 draws it. Each one has the app's icon and its sentence. Opening one opens the app at its place and marks it read. Dismissing one (×) removes it.
- **An app can take a notification back** once it's no longer true. Sending again with the same key replaces the old one.
- **Everything works offline:** a local app file or `./main serve` on 127.0.0.1 is enough.

## CONTEXT

- **Where it comes from:** the owner, 2026-11-03, "there should be a permission for gating which apps can send notifications … a notification structure that should handle any kind of notification we will design now and in the future. we test by having apps send notifications locally". The owner said yes to three things:
  - the question on the first send;
  - only while the app runs, with scheduled reminders as a later PRP;
  - the split into the core, then the bell and the dot, then See all, then desktop alerts.
- **The design:** `docs/designs/newtab-target.md`, "Notifications" and its contract item 4.
- **What exists:**
  - `bundles/permissions.la` has only built-in permissions. `Tier.Any` (default `Ask`) has no permission in it yet, but its dialog (`ui/grants.la`) and its setting (`people.setting_of`) are built.
  - `guest/effects.la` checks each ask's permission (`needed`, `holds`) and answers it (`answer_head`).
  - The new tab's asks are answered by `guest/home.la`.
  - Opening a place is `Msg.OpenPlace(address, origin)`, as Recent's cards do.
  - Rapidkoi's own small files are `data/recentfile.la` and `data/pinfiles.la`, saved by `data/saving.la`.
  - Forget is `people/forgotten.la`.
- **Files at the limit:**
  - `src/shell.la` is at 500 lines, so this PRP adds no shell message (opening reuses `OpenPlace`; dismissing is an ask).
  - `src/guest/effects.la` and `src/data/saving.la` are at 486 lines, so new code goes in new modules.
  - `guest/rust/newtab/src/lib.rs` is at **503 lines** after the 1970 fix. The section's placeholders (their faces, marks and three pictures) go with it, which brings it under the limit; the section is its own `notices.rs`.
- **Contract:** additions to contract 11 with no new version (DECISION-056, as PRP-055 and PRP-062 were): a permission name and four asks. Older readers never see them.
- **SDK:** rapidkoi-rust `v0.5.0-pre.4` adds `Ask::notify`, `Ask::withdraw` and the new tab's three asks, plus a "Notify me" button in the counter example (asked before tagging).
- **Decisions:** none open touch this PRP. A new entry, DECISION-093 "Notifications: one record for every kind", records the record and the permission once approved.
- **Security:** `notify` and `withdraw` are new host functions that any app can call. Under CLAUDE.md, they need **the owner's review**; the checks are under DESIGN.

## REQUIREMENTS

### Must Do

- **The permission:**
  - `notifications`, `Tier.Any`, default Needs approval;
  - words: "sending you notifications" and "send you notifications";
  - label: "Sending you notifications".
  - A second permission, `read_notifications`, is `Tier.BuiltIn`, for the new tab: reading, opening and dismissing them.
- **The asks, from any app holding `notifications`:**
  - `notify` + `text` (1–200 characters), `fields`? (its place: state fields, as a link carries them), and `key`? (1–64 bytes, `a–z 0–9 _ -`). Not answered.
  - `withdraw` + `key`: removes that app's notification with that key, if there is one. Not answered.
  - A malformed ask (no text, too long, a bad key or bad fields) stops the app with its sentence, as other bad asks do.
- **The new tab's asks (`read_notifications`):**
  - `notifications`: `{ "unread": n, "items": [ … ] }`, unread first and newest first, at most 64;
  - `open_notification` + `index`: opens its place and marks it read;
  - `dismiss_notification` + `index`: removes it.
  - `on_change` fires when the list changes, as it does for pins.
- **Each item as the new tab reads it:**
  - `{ "kind": "app", "name", "icon": "icon:PLACE", "text", "address", "sent", "read" }`;
  - `kind` is always `"app"` now. Later kinds (`"invite"`, `"share"`, `"update"`) add an `action` and a `from`, each with its own PRP.
- **Keeping:**
  - Rapidkoi keeps notifications in a `notifications` file in the data folder, in the same shape as the record below;
  - at most 20 per app (the oldest goes first) and 200 in all;
  - kept across restarts.
  - A file that can't be read starts empty, as the recent file does.
  - Forget removes that app's notifications.
- **Rate:** at most 10 `notify` asks a minute per app. Over that, the extra ones are dropped and the app keeps running.
- **Only while it runs:** an ask comes only from a running app. Nothing runs in the background.
- **The new tab:** the Notifications section from A5, where its placeholder was (under Restore, above Recent):
  - only while `unread > 0`;
  - the heading "Notifications" with the count in `accent`;
  - the newest four, unread first;
  - each one is the app's icon, its sentence in `text`, and its app and time in `dim`;
  - the row opens it, and × dismisses it.
- **The spec:** in `docs/specs/contract-11.md`, the permission, the asks and their limits.

### Must NOT Do

- **Invites, shares or updates:** their senders aren't built yet. The record leaves room for them, but no code makes them.
- **The bell, the dock dot, the See all page, desktop alerts or scheduled reminders:** each is its own PRP.
- **Letting an app give a look, a button, a face or a picture:** it sends a sentence and a place only. Rapidkoi draws everything else.
- **Notifications from apps that aren't running.**
- **A new contract version.**

## DESIGN

**The record** (`window/notifications.la`, pure, like `window.recent`):

```
Notice {
    id: i64,           // Rapidkoi's, rising
    kind: Kind,        // App only for now; Invite, Share, Update join with their PRPs
    path: string,      // the app (as Place.path)
    origin: Option<u8[]>,
    name: string,      // the app's name and icon when it was sent (a Face)
    text: string,
    address: string,   // its place, as recent's address
    key: Option<string>,
    sent: i64,         // ms
    read: bool,
}
Notices { items: Notice[], next: i64, sent_lately: (path+origin, ms)[] }
```

An app is identified by its path plus origin, as a recent place is (PRP-062), so two apps at one address keep their notifications apart.

- **Functions:**
  - `notify(n, who, page, text, fields, key, now)`: with a key, it replaces that app's notification with the same key; it drops an app's 21st and the 201st overall; over the rate, it returns `n` unchanged;
  - `withdraw(n, page, key)`;
  - `open_at(n, i)`, which marks it read and returns its place;
  - `dismiss(n, i)`;
  - `forget_app(n, path, origin)`;
  - `listed(n)`: unread first, newest first.
- **Where the code goes:**
  - the file is `data/noticesfile.la`, read at launch and saved by `saving.persist` when it changes;
  - the asks are decoded in `guest/contract.la` (`Effect.Notify`, `Withdraw`, `Notifications`, `OpenNotification`, `DismissNotification`) and answered in a new `guest/noticeasks.la`, which `effects.answer_head` calls, as it does for `storing`;
  - the answers' shape lives in `guest/home.la`.
- **The permission's question:** `notify` needs `Permission.Notify` through `needed`, so the existing check and dialog apply unchanged.
  - Blocked answers it by dropping the notification. Allow once keeps this one.
  - For a page behind, the existing rule holds: it waits until it's in front.
- **Security** (for the owner's review):
  - `text`, `key` and `fields` are checked before anything is kept, with the existing caps (`MAX_STRING`, `MAX_PLACE`) and the new ones above;
  - an app can't name another app, a kind, an action, an icon or a time: Rapidkoi fills in each of those from the page that asked;
  - the shell never holds more than 200 notifications;
  - only `read_notifications`, a built-in permission, sees any of them;
  - no file path is ever answered (a file app's address is its name, as in Recent).

**The new tab:** a new `notices.rs` draws the section; the placeholders it replaces leave `lib.rs`, under 500 lines.

## TESTS

- [ ] `test/permissions.la`: `notifications` parses as `Any` and asks by default; `read_notifications` is built-in, and an external app declaring it is refused.
- [ ] `test/notifications.la`:
  - [ ] notify keeps a notification, and the same key replaces it;
  - [ ] withdraw removes it, and another app's key is left alone;
  - [ ] the 21st from one app drops its oldest, and the 201st overall drops the oldest;
  - [ ] the 11th in a minute is dropped;
  - [ ] the list is unread first, then newest first;
  - [ ] open marks it read and gives its place; dismiss removes it;
  - [ ] Forget removes only that app's.
- [ ] `test/noticesfile.la`: a round trip, a missing file is empty, and a broken file is empty.
- [ ] `test/contract.la`: each ask decodes; a missing text, 201 characters, a bad key or bad fields stops the app with its sentence.
- [ ] `test/effects.la` / `test/noticeasks.la`, using a stand-in app in `test/support.la` that declares `notifications`:
  - [ ] the first notify opens the question;
  - [ ] Always allow keeps it, and the next one isn't asked;
  - [ ] Blocked drops it and the app runs on;
  - [ ] not declaring the permission stops the app;
  - [ ] the new tab's `notifications` lists it, and its `open_notification` opens the place.
- [ ] `test/home.la`: an item's shape, with `icon:PLACE` and no file path.
- [ ] guest/rust: the section shows only while something is unread, with its count, four at most, and dismiss and open sending their asks.

## ACCEPTANCE

- [ ] A local app with `notifications` asks once, and once allowed, its notification shows on the new tab and opens its place.
- [ ] Blocked in ⚙ means nothing shows and the app runs on.
- [ ] Notifications survive a restart; Forget clears the app's.
- [ ] `lughac test test/`, `lughac fmt --check src test`, `lughac build -O 2 src/main.la -o main` and guest/rust's tests pass.
- [ ] For the owner, by eye: pack the counter example with `notifications`, open it from its file with no network, press "Notify me" and allow it. Then open a new tab: Notifications shows one, with the counter's icon. Clicking it opens the counter at that count, and the section goes.
