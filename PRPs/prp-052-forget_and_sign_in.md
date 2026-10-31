## FEATURE: Forget and Sign in: an app's settings come down to one Sign in switch and Forget, and a forgotten app never sees an earlier you again

## OBJECTIVE
In an app's ⚙, the Account section shows **Sign in** as a switch and **Name** as a row; there's no Identity page with buttons, no Clear…, no Start over. Switching Sign in off makes the app a guest: it gets no key and no signature, every run, and keeps its data here; on again, it's the same you. **Forget this app…** removes its windows, pin, data and settings here, and the next time you open it, it sees someone new: never the person it knew before, not even the very first one.

## CONTEXT
- From chat with the owner (2026-10-31): DECISION-084 ("can this be simplified to a single action", "how many actions do we actually need by neccessity", "yes record it"), which replaces DECISION-079; core decision 17 (identity) and 30 (privacy, rule 6) amended to match.
- **Starting state:**
  - Five controls: Start over (the app's own, `Confirm.StartOver`), Clear data (`ClearData`, the Data page), Start over as someone new (`StartOverIdentity`), Sign out and Sign in, and Sign in automatically (`ToggleAutoSignIn`; off, each run starts signed out, `src/data/saving.la:166`).
  - **A bug:** Forget removes the app's `Person`, generation and all (`forget_person`, `src/people/people.la:161`), so reopening it derives generation 0: the app gets the very first key back, undoing every Start over as someone new.
- **Discovery answers (the owner):**
  - Q1: the card's Account section: Public key ›, **Sign in** (a switch, on by default), **Name** › (the Identity page keeps only the name fields). App data's Data row becomes a plain line (its page and Clear… go). Forget's dialog: **"Forget Notes?"** / **"Its windows, pin, data and settings here will be removed. Next time you open it, it won't know you."**, buttons Cancel and Forget. Off, the Public key row says **"Not shared"**. ("yes sign in and name", "keep that wording")
  - Q2: `Person.signs_in` replaces `auto` and `signed_out`; `People.forgotten`; the people file's format 5; `SetSignIn(bool)`; `Confirm` keeps only `Forget`; `PanelPage.State` goes, `Account` becomes `Name`.
  - Q3: Forget's identity part comes first and always completes; failures to remove files are the existing notice; a full `forgotten` drops its oldest.
  - Q4: the edge cases below.
  - Q5: the files below, and the split of `forgotten.la` out of `people.la` (489 lines).
  - Q6: what it must not touch, below.
  - Q7: no open decision blocks it.
  - Q8: a keyed tag from the master secret; the owner's security review before merging.
  - Q9: the tests and checks below.
- **Files read first:** `src/people/people.la`, `src/people/account.la`, `src/people/identity.la` (how keys and HKDF are used), `src/ui/settings.la`, `src/window/actions.la`, `src/shell.la` (`StartOver`, `ClearData`, `Forget`), `src/guest/effects.la` (`respond`), `src/data/saving.la` (startup), `docs/DESIGN.md`.
- **Files changed:** new `src/people/forgotten.la` and `test/forgotten.la`; `src/people/people.la`, `src/people/account.la`, `src/window/actions.la`, `src/ui/settings.la`, `src/shell.la`, `src/guest/effects.la`, `src/data/saving.la`; tests `test/people.la`, `test/account.la`, `test/settings.la`, `test/shell.la`, `test/effects.la`.
- **Relied on:** `std.crypto.hkdf_sha256` and `equal` (`lughac spec`, milestone 120; already used in `src/people/identity.la:168`); kivuli's `switch` (already used on this card).
- **Decisions:** DECISION-084 (resolved); DECISION-044 (open) untouched: Forget keeps dropping "where you left off" as today; DECISION-022 (deferred): this changes sign-in's controls, so the owner's review.

## REQUIREMENTS

### Must Do
- **The card** (`ui/settings.la`, `front`): Account section: Public key ›; **Sign in** with a switch (`SetSignIn`); **Name** › showing the name this app sees (its own, else the one for every app, else "Not set"; ellipsized). Off, Public key's value is "Not shared".
- **The Name page** (was Identity): the first and last name fields only.
- **App data:** the Data row is a line with the size, not a link; its page and Clear… go.
- **Forget's dialog:** the agreed words, Cancel and Forget.
- **Sign in:** on, `identity` and `sign` are answered as today; off, `null`. No event to the app, no restart; the switch governs later asks only. Toggled quickly, the last wins.
- **Forget,** in this order:
  1. records its key's generation (above) + 1 in `forgotten` under its tag, and removes its `Person` (signed apps only: unsigned and local development apps record nothing);
  2. closes its tab, unpins it, drops its "where you left off", its windows' saved state, its database and its kept copy, as today; a failure is the existing notice and Forget still finishes;
  3. keeps its trusted key.
- **Reopening a forgotten app:** its key's generation is its `Person.generation` plus what `forgotten` holds under its tag (amended in implementation, the owner: "yes": a new `Person` is first saved in `people/access.la`, which has no master secret, so the number can't move into it; the entry stays as the base). Forgetting again records that total + 1.
- **The people file, format 5:** `signs_in` per app; `forgotten` entries `{ tag: 32 bytes, generation: ≥ 0 }`, at most `MAX_PEOPLE`, the oldest dropped when full. Format 4 is read and converted: `signs_in = auto && !signed_out`.
- **Removed:** `StartOver`, `ClearData`, `StartOverIdentity`, `SignIn`, `SignOut`, `ToggleAutoSignIn`, `Confirm.StartOver`, `Confirm.ClearData`, `Confirm.StartOverIdentity`, `PanelPage.State`; the startup rule that starts non-auto apps signed out.

### Must NOT Do
- Change the contract: no new ask, event or version; `identity` and `sign` behave as now (no app needs rebuilding).
- Touch how keys are made: the master secret, `people/identity.la`'s derivation from host, origin and generation.
- Touch `src/trust/` or trusted keys.
- Touch other apps' `Person`, database or kept copy, or the name for every app.
- Remove Restart after a crash (recovery, not Start over).
- Add a dependency, `extern` or library; repack `assets/apps/`.
- Store a forgotten app's host or key in readable form.

## DESIGN
- **`src/people/forgotten.la`** (new, depends on std.crypto, cbor):
  - `struct Forgotten { tag: u8[], generation: i64 }`.
  - `tag_of(secret: u8[], host: string, origin: u8[]): Result<u8[], ForgottenError>`: `hkdf_sha256(secret, [], "rapidkoi forgotten 1\0" + host + "\0" + origin, 32)`. Keyed by the master secret, so the file alone says nothing about which apps were forgotten; a different secret gives different tags.
  - `remember(list, tag, generation)`: replaces an entry with the same tag, appends otherwise, drops the oldest past `MAX_PEOPLE`.
  - `recall(list, tag): Option<i64>` and `drop_tag(list, tag)`; tags compared with `equal`.
  - Encoding and decoding its part of the people file; decoding checks each tag is 32 bytes and each generation ≥ 0, else the file is unreadable (as any bad field).
  - `enum ForgottenError { Crypto(string) }`, shown through the existing identity notice.
- **`Person`:** `signs_in: bool` (default true) replaces `auto` and `signed_out`. **`People`:** `forgotten: Forgotten[]`.
- **`generation_for(people, secret, host, origin)`** (in `forgotten.la`'s caller, `people.la`): `person(…).generation` + `recall(forgotten, tag).unwrap_or(0)`. Used wherever a key is made: `guest/effects.la` (`respond`), `people/account.la` (the Public key label) and `ui/settings.la` (the Key page), each of which already has the master secret. New `Person`s stay at generation 0: nothing raises it any more; only format 4 records carry one.
- **Messages:** `SetSignIn(bool)` (`window/actions.la`), handled in `people/account.la`. `Confirm { Forget }`; `PanelPage { Main, Key, Name, Access, Permissions }`.
- **Security (the owner's review before merging):** sign-in controls change (authentication); a keyed tag from the master secret; the people file read as hostile input. No network, process, SQL, path from app input or new dependency.

## TESTS
- [ ] `forgotten.la`: one app, one tag, every time; two apps, two tags; another secret, another tag; `remember` then `recall`; `remember` twice keeps one entry; full drops the oldest; encode then decode round-trips; a 31-byte tag and a generation of -1 each make the file unreadable
- [ ] `people.la`: format 4 with auto on and signed in → `signs_in` on; signed out → off; auto off → off; format 5 round-trips; a new `Person` signs in
- [ ] `account.la`: `SetSignIn(false)` then `(true)`; Forget of a signed app at generation 2 records 3; reopened, its key's generation is 3 and the entry stays; forgetting again records 4; Forget of an unsigned app records nothing
- [ ] `effects.la`: off, `identity` and `sign` answer `null`; on, the key; on again, the same key; after Forget and reopening, the key differs from those of generations 0, 1 and 2
- [ ] `settings.la` (`kivuli.testing`): the card shows "Sign in" with a switch and "Name"; no "Identity", "Clear…" or "Start over"; pressing the switch sends `SetSignIn`; off, Public key reads "Not shared"; the Name page has the two name fields only; the Name row reads the app's name, else everyone's, else "Not set"; the Forget dialog reads "Forget Notes?" and the agreed sentence
- [ ] `shell.la`: Forget removes the database and kept copy and records the generation; a failed removal is the notice and the generation is still recorded

## ACCEPTANCE
- [ ] Every test above passes, written first
- [ ] `lughac test test/` (skips only for "no display"), `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] Every file at or under 500 lines
- [ ] The owner's security review of the diff, before merging
- [ ] For the owner, by eye, with Notes at notes.dafifi.net: ⚙ shows Sign in as a switch and Name as a row, with no Clear… or Identity page; Sign in off, Public key says "Not shared", on again, the same key; Forget Notes shows the agreed words; reopened, Notes' Public key is different from before; the Name page has only the two name fields
