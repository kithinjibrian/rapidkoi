## FEATURE: Showing a stored blob as a picture, and sending one as a request body (contract 9)

**Status:** approved (the owner, 2026-10-31: "approved").

## OBJECTIVE
An app shows a picture it stored with `image("blob:<key>")`, and uploads a blob to its own servers with `fetch`'s `body_blob`, without the bytes passing through the app. A blob that isn't there, or isn't a picture, shows Rapidkoi's broken-picture mark and tells the app why; the app runs on. Replacing a blob shows the new picture in every window of the app. With PRP-054 this completes blobs' part 1 (`docs/specs/blobs.md` 1.6 and 1.7). Notes attaching a photo can then be built, in `../notes`, under its own rules.

## CONTEXT
- **Spec:** `docs/specs/blobs.md` 1.6, 1.7 (DECISION-087, resolved; approved whole with its security review: `blob:` pictures and `body_blob`).
- **Decisions:** DECISION-087, DECISION-056 Q1/Q3 (pictures: sources, limits, `on_fail`), DECISION-058 (WebP and size limits wait on kivuli). None open.
- **Contract:** stays 9. PRP-054 shipped contract 9 without these two; they're additions an app built on `v0.3.0-pre.2` never sends, so nothing it does changes. The SDK gains them in `v0.3.0-pre.3`.
- **Files to read first:** `src/guest/pictures.la` (`good_source`, `asset_of`), `src/net/downloads.la` (`fetch_page`, `download`, `arrive`, `checked`), `src/net/requests.la` (`ask`), `src/guest/contract.la` (`fetch_of`), `src/guest/nodes.la` (`Effect.Fetch`), `src/data/queries.la` (`arrive`), `src/data/databases.la` (`call`, `write`), `src/data/blobmap.la` (`check_owners`).
- **Changes:** those files, `src/guest/effects.la` (one line), `docs/specs/contract-9.md`, `docs/specs/rapidkoi-rust-contract-9.md` (pre.3), CHANGELOG.md; tests `test/pictures.la`, `test/downloads.la`, `test/contract.la`, `test/requests.la`, `test/queries.la`.
- **Relied on:** everything PRP-054 built; `lugha-imagesize` (already used by `downloads.checked`).

## REQUIREMENTS

### Must Do
- **`blob:<key>` is a picture source** (`pictures.good_source`): the key 1–256 bytes, no control characters, as a blob key. No `color` on it (only bundle SVGs take one).
- **Fetching one** (`downloads.fetch_page`): a `blob:` source not yet kept is read in a task, like a download, at most `MAX_DOWNLOADS` at a time with the others: the app's database path, `Call.BlobGet` for the whole blob up to `MAX_PICTURE` (10 MB), answered as `Msg.Pictured` with a `Reply`, so `arrive` and `checked` treat it exactly as a server picture (PNG or JPEG by its header, 4,096 px a side, 16 MP, the 256 MB decoded budget, `on_load`/`on_fail`). No network, so no server rule applies.
- **Failures, as sentences, the app running on:** no blob store (the new tab, built-ins): "This app keeps no files here"; no such key: "This app has no blob with that key"; over 10 MB: "This picture is larger than 10 MB"; not a PNG or JPEG: as today.
- **Fresh after a change:** a write that stored, replaced or removed blobs (or whose owner check removed some) reports it; when it arrives, every window of that app drops its kept `blob:` pictures, so the next check reads them again. Writes that touch no blob drop nothing.
- **`fetch` takes `body_blob: <key>`** instead of `body` (both is a bad ask: the app stops). The request is checked as today (an empty body); in the task, the blob is read whole (at most `MAX_BLOB`, 16 MB) and becomes the body, then the request is sent. No such key, or no blob store: `on_fail` with the sentence, nothing sent.
- **Contract 9's text** and the SDK request gain both, with the sentences.

### Must NOT Do
- A fetch saving into a blob, the picker, write sessions, sync (spec part 3).
- SVG, or `color`, from a blob: only the bundle's SVGs are drawn as SVG.
- Change any picture limit, or a request's checks beyond the body's source.
- Read a blob in `update` or `view`: always in a task.

## DESIGN

**Decided in discovery (Claude; the owner reviews them here):**
1. **A blob picture rides the download path.** `downloads.download` gets a sibling, `from_blob(id, load, source, path)`, whose task answers `Msg.Pictured(id, load, source, Reply)`: `Got(200, [], bytes)` or `Failed(sentence)`. Nothing after it changes, so every picture rule applies unchanged.
2. **Freshness by a signal, not by hashes in the page.** `AppPage` (`window/tabs.la`) is at 500 lines and `Kept` is shared by every picture, so the page doesn't learn hashes. Instead `databases.write` adds `rapidkoi_blobs` to the changed tables when it stored, replaced or removed a blob, or its owner check removed one (`blobmap.check_owners` returns whether it did; SQLite's update hook never reports `WITHOUT ROWID` tables, so the table can't come from there). `queries.arrive` drops kept `blob:` pictures in every window with the same database path when it sees it, and still tells apps only their own tables. A plain `sql` that deletes owner rows doesn't signal: its pictures were of rows the app deleted.
3. **`Effect.Fetch(Value, Request, Option<string>)`**, the third the blob's key; `requests.ask` takes it. `Request` keeps its shape (20 literals use it).
4. **The blob body is read in the request's task**, after `check_request` passed: an app can't make Rapidkoi read a blob for a server it may not reach.

**Security** (approved with the spec; restated): `blob:` keys checked like `blob_set`'s; the bytes are decoded by Rapidkoi only after their header passes, as a server's are; `body_blob` sends only to servers `fetch` may reach, at most 16 MB, the app's own blob only (its database path); nothing is read in `update`.

## TESTS
- [ ] `test/pictures.la`: `blob:photo-1` is a good source; an empty key, a control character, `color` on a blob source are refused with their sentences.
- [ ] `test/downloads.la`: a `blob:` picture stored as a PNG is fetched in a task and kept (`on_load`); a missing key fails with its sentence (`on_fail`), the app running on; a blob that isn't a picture fails as "This picture isn't a PNG or JPEG"; a page with no database (the new tab) fails with "This app keeps no files here".
- [ ] `test/queries.la`: a write storing a blob drops two windows' kept `blob:` pictures; a write touching no blob keeps them; other windows' `on_change` never names `rapidkoi_blobs`.
- [ ] `test/contract.la`: `fetch` with `body_blob` reads the key; with `body` and `body_blob` both, refused.
- [ ] `test/requests.la`: a request with `body_blob` sends the blob's bytes to a local test server; a missing key answers `on_fail` and sends nothing.

## ACCEPTANCE
- [ ] Every test above passes; no file over 500 lines.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye, once Notes can attach a photo: the photo shows in the note, replacing it shows the new one, deleting the note removes it.

## BUILD NOTES

- Built as designed. `requests.ask` refuses a `body_blob` at once when the app has no data folder ("This app keeps no files here"), before anything is counted in flight.
- Tests keep their blob databases under `/tmp`, as `test/blobmap.la` does and for the same reason.
- `lughac fmt --check`, the full suite (698 passed, exit 0) and the `-O 2` build pass. The SDK's `v0.3.0-pre.3` (`docs/specs/rapidkoi-rust-contract-9.md`) isn't needed to merge: the built-ins use neither addition.
