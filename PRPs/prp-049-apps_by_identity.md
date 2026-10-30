## FEATURE: Apps by identity: an app is its host and its first trusted key wherever Rapidkoi remembers it, so a changed key keeps your version and a separate app is a real second app, named by you

## OBJECTIVE
When a site's key changes with no signed hand-over, the app you had keeps working: it opens from its kept copy, with your data, online or offline, and stops updating. "Open as a separate app" asks you to name the new app, then opens it as a second app at the same address, with its own kept copy, data and tab. Both can be open side by side. Typing an address where you have several apps asks which one. A publisher who comes back with its key, or a signed rotation from it, makes its app update again.

This is steps 1–7, 9 (without a default), 10, 11 and 12 of the journey (https://claude.ai/artifact/Nx55CYNApso8E7GjWYGyvN). **PRP-050** follows with the chip and its dropdown, the default, the colours (dock marks, the naming screen's dot) and renaming.

## CONTEXT
- From chat with the owner (2026-10-30): DECISION-064 records every choice. It builds on DECISION-009 ("identity = host + key"), the signing meeting (`docs/source/meetings/2026-10-18-signing-and-panel.md`) and PRP-011 (`Identity.origin`).
- **Depends on PRP-048** (`src/window/upgrades.la`, `same_app` in `follow`), not yet merged into `main`: this PRP starts once it is, and its branch is rebased on it.
- Starting state (read on `main`):
  - Kept copies are per host: `kept/<sha256 of host>` (`src/data/kept.la`). The background check removes the copy on any refusal, a changed key included (`src/net/keeping.la`, `Refreshed.Refused`).
  - `window.same_app` compares host and port (tabs, one tab per app, instances, PRP-048's upgrades). `open_tab_of` finds an app's tab by address.
  - Pins (`Pin.path`), "where you left off" (`LeftOff.path`) and the saved session (`OpenTab.path`) name an app by its address; Forget filters pins and left-off by `path` (`src/window/pins.la`).
  - Data and saved state are already per host and origin key (`src/data/databases.la`, `src/data/states.la`).
  - "Open as a separate app" trusts the new key for the host (`verify.separate`, saved in the trust file with `separate: true`), then loads the address, kept copy first (`navigate.separate_app` → `loads.begin`).
  - The address box shows "{host} · new publisher" for a separate app (`src/ui/frame.la:71`).
- Files changed (beyond tests): `src/data/kept.la`, `src/net/keeping.la`, `src/net/loads.la`, `src/window/window.la`, `src/window/navigate.la`, `src/window/pins.la`, `src/window/instances.la`, `src/window/upgrades.la`, `src/window/tabs.la`, `src/data/remembered.la` (split, below), `src/data/sessions.la`, `src/trust/verify.la`, `src/ui/frame.la`, `src/ui/content.la`, `src/shell.la`, `src/window/actions.la`. New: `src/window/identity.la`, and the module split out of `remembered.la`.
- **File size (needs approval):** `src/data/remembered.la` is at 496 lines and gains the origin key in pins, left-off and the session, plus each identity's name. Proposed split: pins and left-off's reading and writing move to `src/data/pinned.la`, unchanged but for the new field; `remembered.la` keeps trust and the session.
- Decisions: DECISION-064 (resolved), DECISION-009, core decision 28 (amended again: a changed key no longer removes the kept copy). None open that this depends on.

## REQUIREMENTS

### Must Do
- **An app's identity** is its host and origin key when signed, else its path (`identity.la`). Everything below that says "app" means identity.
- **Kept copies per identity:** `kept/<sha256 of host and origin key>`. A copy in the old per-host folder is moved to its identity's folder the next time it's opened (its key file names the key; the trust ring gives the origin), or dropped when no identity matches.
- **A changed key keeps your version:**
  - The background check that finds the site signed by a key with no trail from this app's keeps the copy and marks it moved on (`Meta.moved`). Other refusals (a bad build, an older index) remove it as now.
  - A moved-on app opens from its kept copy, offline or not, and is never offered an upgrade. Reload opens the kept copy too.
  - Its shield takes the caution look, and its card says "{host} is now signed by another key", with **Open as a separate app**.
- **Opening an address** (typing it, `./main ADDRESS`, a link):
  - No app known there: as now.
  - One app, and its site hasn't moved on: as now.
  - One app whose site moved on: the question in the tab, "{host} is now signed by a different key", with **Open my version** and **Open as a separate app**. Today's "Keep using my version" and "Don't open it" become "Open my version".
  - Several apps there: the list in the tab, "Which app?", one row per app (its name and key fingerprint); picking one opens it. (PRP-050 adds the default.)
- **Naming a separate app:** "Open as a separate app" shows "Name the new app" in the tab: an empty field, **Open** (off until the name can be used) and **Cancel**. A name can't be empty ("Give it a name"), longer than 64 characters ("At most 64 characters"), or another app's at this address ("Another app here is called that"). Open trusts the new key as a separate app under that name, then loads it from its site, never from another app's kept copy, and keeps it under its own identity. Cancel goes back to the question.
- **An app's name** is the one the person gave it, else its manifest's. It's saved with its identity (the trust file, format 4) and shown wherever the app's name is: its tab, the dock, Continue, pins.
- **Tabs by identity:** one tab per app, instances, upgrades and "where you left off" match by identity, so both apps can be open side by side.
- **Pins, Continue and the session remember the identity:** each saves the app's origin key beside its address (the pins, left-off and session files' format 2). One saved before this opens the only app at its address, or asks when there are several.
- **Forget and Clear** act on one app: its pin, left-off, kept copy, saved state and database; another app at the same address is untouched.
- **Whoever signs the site decides which app is live:** a site signed by an app's key, or by a key its signed trail leads to, is that app's; checks of the other apps at the address find it moved on.
- The address box no longer shows "· new publisher".
- DECISIONS.md: core decision 28 amended (a changed key keeps the copy).

### Must NOT Do
- No chip, dropdown, default, colours or renaming: PRP-050.
- No moving data between apps (DECISION-064).
- No change to the contract, bundles, signing or how a key change is detected (`verify.check_site_at`).
- Files and development addresses keep their path as their identity: nothing changes for them.

## DESIGN
- **`src/window/identity.la`:** `AppId` (`Signed(host, origin)` or `Local(path)`), `id_of(page)`, `same(a, b)`; `window.same_app` callers move to it. Pages carry their identity through `Trust` (host, and the origin looked up in the ring on load).
- **`kept.la`:** `kept_for(store, host, origin)`; `moved_from(store, host)` for the one-time move. `Meta` gains `moved: bool`; a meta written before reads as not moved.
- **`keeping.la`:** `refresh` returns `Refreshed.Moved` for an unapproved key (the copy kept, `moved` set); `open_app` takes the identity it opens, never reads another identity's copy, and keeps only under the identity of what it fetched.
- **`verify.la`:** `Identity` gains `name: Option<string>`; `separate(ring, change, name)`. Trust file format 4 (3 reads with no names).
- **`navigate.la`:** opening an address looks up the identities known at its host first (local, no network). The question and the list are new `Screen` variants (`Choosing`, `Naming`), so the tab shows them before anything loads.
- **New messages:** `OpenMine(origin)`, `NameSeparate(string)`, `ConfirmSeparate`, `CancelSeparate`, `Choose(origin)`.
- **Security (owner review, SECURITY RULE: trust and keys):**
  - The kept copy's folder is a hash of host and key: no name or address becomes a path.
  - A name is the person's input: trimmed, checked as above, stored and shown as text only.
  - Nothing here trusts a key: separate apps are trusted exactly as now (`verify.separate`), now with a name.
  - The moved-on copy is still checked exactly as a site on every open (`check_site_at` over the kept files); only its update path stops.

## TESTS
- [ ] An app's identity: signed by host and origin, a rotation keeps it, a file by its path
- [ ] A changed key keeps the copy, marks it moved on, and the app opens from it online and offline, with its state; no upgrade is offered
- [ ] Other refusals still remove the copy
- [ ] The moved-on app's shield card offers Open as a separate app
- [ ] A copy in the old per-host folder moves to its identity's folder on open; one that matches no identity is dropped
- [ ] Typing the address of a moved-on app: the question; Open my version opens it from its copy
- [ ] Open as a separate app: the naming screen; each rule's sentence; Open off until valid; Cancel goes back
- [ ] The named separate app loads from its site (not the other's copy), is kept under its own identity, and starts with empty data
- [ ] Both apps open side by side in their own tabs; + and PRP-048's upgrade stay within one app
- [ ] Typing an address with two apps: the list; picking one opens it
- [ ] Pins, Continue and the session reopen the exact app; older files without the key open the only app or ask
- [ ] Forget and Clear touch only one app
- [ ] The publisher comes back with its key, or a rotation from it: its app updates again; the other app is moved on
- [ ] A new key with no trail: a third app
- [ ] Files and development addresses behave as before
- [ ] The address box shows no "· new publisher"

## ACCEPTANCE
- [ ] Every test above passes
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass
- [ ] For the owner, by eye: publish counter at a local site, open it and count; re-key the site and publish; reopen: your count, the caution shield and its card; Open as a separate app, name it: a second tab at count 0; close both, type the address: "Which app?"; Forget one: the other stays

## APPROVAL
- The owner (2026-10-30): the words as written; the split of `remembered.la` into `remembered.la` and `pinned.la`; one PRP.
