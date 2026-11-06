## FEATURE: Backup words: write down who you are once, and get it back on a new computer

**Status:** approved (the owner, 2026-11-06). Decisions: DECISION-022 (no Rapidkoi account), DECISION-101 (backup words). Design, the target for every screen: the Settings canvas's "Backup words (DECISION-101)" row (https://claude.ai/artifact/GYQFzuoDVE4xvKx6bsZGQv, 2026-11-06): the whole flow, then 1 the reminder, 2 the words, 3 three typed back, 4 backed up, 5 use backup words with a word wrong, 6 replacing.

## OBJECTIVE

- **Settings › You** gains a **Backup** group under your name, two rows:
  - **Backup words**, "Not backed up" in `caution` at its end; pressed, it opens Rapidkoi's card (board 2). Once backed up it says "Backed up", dim, and opens nothing (board 4).
  - **Use backup words**: opens Rapidkoi's card for typing them (board 5).
- **The words card** (board 2), Rapidkoi's own, over Settings' area: "Backup words", the 12 words in a numbered grid, three across, Cancel and Next (primary).
- **The check card** (board 3): "Backup words", three rows, "Word N" and a field, for three positions picked at random; Back and Done (primary). Done with all three right: backed up, the card closes, and the words are never shown again. A wrong one: the field's edge in `danger`, the card stays.
- **The restore card** (board 5): "Use backup words", 12 numbered fields, three across; Cancel and Use (primary). Words that aren't 12 words of the list, or whose check doesn't add up: "These words aren't right" in `danger`, the card stays.
- **Replacing** (board 6): on a computer that already is someone, Use asks "Replace who you are on this computer?" over "Apps that know you here will see a stranger.", Cancel and Replace (danger). Replace makes the words you here and starts every app afresh, as Forget does for one. On a computer that has never been anyone, Use makes the words you at once.
- **The reminder** (board 1): "Write down your backup words" in the notice bar, with ×, the first time any app is given your key while you aren't backed up. × dismisses it for good; backing up ends it too.
- **The secret is 16 bytes**, and the 12 words are it. The old 32-byte identity is set aside once (DESIGN).

## CONTEXT

- **Read first:** DECISION-022, DECISION-101, DECISION-084 (Forget), DECISION-085 (questions over an app's area), DECISION-100 (Settings stays an app: public facts only), PRP-011 (identity), PRP-052 (Forget), PRP-057 (questions on the app), PRP-075; `src/people/identity.la`, `src/window/appinfo.la` (`forget_info`), `src/ui/questions.la`, `src/window/notices.la`.
- **Changes, the shell:**
  - `src/people/identity.la`: format 2, a 16-byte secret, `backed_up` and `reminded` in the file; the old format set aside.
  - new `src/people/words.la`: secret ↔ 12 words, with BIP-39's checksum (`std.crypto`'s `sha256`, `random_bytes`).
  - new `assets/words/english.txt`: BIP-39's English list, 2,048 words, built in with `include_str` and checked at start-up by a test (2,048 lines, sorted, unique).
  - new `src/window/backup.la` (the cards' state and changes) and `src/ui/backup.la` (the cards), on PRP-057's question layer.
  - `src/bundles/permissions.la`: `ManageIdentity` (`manage_identity`), built in.
  - `src/guest/settingsasks.la`, `src/guest/settingsanswers.la`: `backup_words` and `use_backup_words`, and `backed_up` in `you`.
  - `src/window/notices.la` and `src/guest/effects.la`: the reminder at the first key given.
  - `docs/specs/contract-11.md`, `docs/DESIGN.md`, `CHANGELOG.md`.
- **Changes, the app:** `guest/rust/settings/src/lists.rs` (the Backup group), `lib.rs`, `tests.rs`; `scripts/settings.sh` (`--permission manage_identity`); `assets/apps/settings.rkb` repacked.
- **rapidkoi-rust first,** tagged `v0.5.0-pre.10`: `You::backed_up`, `Ask::backup_words`, `Ask::use_backup_words`.
- **Relied on, checked:** `std.crypto` has `random_bytes`, `hkdf_sha256` and `sha256` (used in `people/identity.la`, `trust/keys.la`). kivuli inputs for the cards' fields. BIP-39's list and checksum: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki and its `english.txt`; the list's licence is checked before it's added.
- **Decisions:** DECISION-022 and DECISION-101 resolved; none open.

## REQUIREMENTS

### Must Do

- **The screens exactly as the canvas's boards,** in Rapidkoi's theme and tokens: the cards are `question_card` (the page's colour, an `edge` line, `PAGE_RADIUS`), `SPACE_LG` inside, at most `APPS_WIDTH`, over the scrim of Settings' area only, as Forget's are (PRP-057); the words in a mono font on `control` cells; buttons in Rapidkoi's looks (normal, primary, danger).
- **The words never reach an app:** Rapidkoi draws them and owns the fields they're typed into. Settings only asks Rapidkoi to open a card (`backup_words`, `use_backup_words`, needing the built-in `manage_identity`) and reads `backed_up` from `you`. No ask returns a word, and nothing Settings can read changes with what's typed.
- **Words:** 16 random bytes; their SHA-256's first 4 bits appended; 132 bits in 12 groups of 11, each an index into the list. Reading back: each word must be in the list (any case, spaces trimmed), and the 4 bits must match, or "These words aren't right".
- **Shown once:** the words card opens only while not backed up. Backed up (`backed_up: true` in the identity file) is written when the check passes, or when restoring succeeds. Cancel or Back before Done shows them again next time: only Done ends them.
- **The check:** three distinct positions from `random_bytes`, in order. Words compared as restoring does (case, spaces).
- **Restoring:**
  - **Never been anyone** (no identity file): the words become the secret, written as format 2 with `backed_up: true`.
  - **Already someone** (an identity file): the replace question; Replace writes the new secret, then forgets every app as PRP-052's Forget does (windows' state, database, kept copy, settings here, a generation past every old key), so no app can link the two; Cancel changes nothing.
  - A failed write leaves the old file and says so in the card, never half a change.
- **The reminder:** shown when an app is first given your key (`identity` or `sign`) and neither `backed_up` nor `reminded`; × writes `reminded: true`. Words: "Write down your backup words".
- **The old 32-byte identity (format 1):** at start, it's kept as `identity-v1` (never deleted) and a new format-2 one is made when first needed; once, the notice bar says "Rapidkoi made you a new identity, with backup words". Apps' data stays (it's the same person who chose this); each app gets a new key. People's settings keyed by app stay; Forget's generations, worked out from the old secret, start again.
- **Typed words held only while a card is open:** in the model, cleared when it closes; never saved, logged, put in a notice or sent anywhere.
- **Every function used outside its module tested,** each error included.

### Must NOT Do

- **No "show again",** no Rapidkoi password, and no restoring at first launch (DECISION-101).
- **No words or secret to any guest,** Settings included (DECISION-100), and none in a notice, a log or an error sentence.
- **No pairing, ponds or sync:** later steps.
- **No silent replacement** of an identity, ever: every replacement asks or tells.
- **No contract version bump:** additions, as PRP-075's.

## DESIGN

- **The identity file, format 2:** `{format: 2, kind: "rapidkoi identity", master: 16 bytes, backed_up: bool, reminded: bool}`; read as hostile, every field checked; 16 bytes exactly, else `Broken`. `master()` keeps its promise: never replaces a broken or newer file.
- **Model:** `backup: Option<Backup>`, `Backup` one of `Showing`, `Checking(positions, typed[3], wrong[3])`, `Restoring(typed[12], bad)`, `Replacing(secret)`; in `window/backup.la` (`window.la` is at 480 lines: one field there).
- **Messages:** `BackupOpen`, `BackupNext`, `BackupBack`, `BackupTyped(i, s)`, `BackupDone`, `RestoreOpen`, `RestoreTyped(i, s)`, `RestoreUse`, `RestoreReplace`, `BackupClose`, `ReminderClose`.
- **Errors** (`people.words`' `WordsError`): `NotAWord(position)`, `Count`, `Checksum`, all shown as the one sentence "These words aren't right". `IdentityError` as today; a failed write: "Rapidkoi couldn't save who you are", the card stays.
- **Security, for the owner's review before building:** the secret's size and how it's made; the word encoding and checksum; the identity file's new fields; Replace's wipe; two new host calls (`backup_words`, `use_backup_words`) and `backed_up` in `you`, built-in only; typed words held in memory while a card is open (Lugha's GC can't promise to wipe them; noted, not solved).

## TESTS

- [ ] `test/words.la`: 16 bytes → 12 words → the same 16 bytes, for BIP-39's published test vectors; a word not in the list, 11 or 13 words, and a bad checksum each refused; case and spaces ignored; the list is 2,048 sorted, unique words.
- [ ] `test/identity.la`: format 2 made with 16 bytes, `backed_up` and `reminded` false; a format-1 file kept as `identity-v1` and a new one made; a broken or newer file still refused; `backed_up` and `reminded` written and read.
- [ ] `test/backup.la` (`kivuli.testing`): opening shows 12 words; Next asks three positions; right words: backed up and closed, opening again does nothing; a wrong one: its field marked, still open; Cancel and Back.
- [ ] `test/backup.la`: restore on a computer with no identity: the words become it, backed up; with one: the replace question; Replace: the new secret, every app forgotten (their databases gone, new keys); Cancel: nothing changed; bad words: "These words aren't right".
- [ ] `test/settingsasks.la`, `test/settingsanswers.la`: `backup_words` and `use_backup_words` need `manage_identity`; `you` carries `backed_up`.
- [ ] `test/notices.la`: the reminder on the first key given, not when backed up or dismissed; × dismisses it for good.
- [ ] Settings (`cargo test`): the Backup group's rows and their states; pressing each sends its ask.

## ACCEPTANCE

- [ ] Every screen matches the canvas's boards 1–6.
- [ ] The words appear once, are checked, and never appear again; restoring them on a fresh data folder gives the same key in an app as before.
- [ ] No ask or answer to any app carries a word.
- [ ] rapidkoi-rust tagged `v0.5.0-pre.10`; the built-ins on it, repacked.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: start with your format-1 identity (see the notice); sign in to Notes (see the reminder); back up, typing the three words; check the row says Backed up and won't open; on a copy of the data folder with no identity, restore the words and see Notes' key match; on one that has an identity, see the replace question, Cancel, then Replace.
