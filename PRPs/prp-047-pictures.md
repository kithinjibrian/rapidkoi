## FEATURE: Pictures in an app's screen: from its bundle (icons, SVG) or from its servers (photos), loaded by Rapidkoi

## OBJECTIVE
An app shows pictures: Notes its trash and search icons, drawn in the theme's colour; a gallery its photos, from its own server. The app names a picture in its view, `image("asset:trash.svg")` or `image("https://notes.dafifi.net/files/7.jpg")`, and Rapidkoi finds it: from the bundle at once, or downloaded in the background as a browser loads an `<img>`. While a picture loads, its box keeps its size; when one can't be shown, Rapidkoi's broken-picture mark takes its place and the app is told why, and runs on. Nothing is fetched but a picture a view names, from a server the app may reach.

PRP-047, the last of three for contract 7 (DECISION-056 Q1–Q3): after the base (PRP-045) and scrolling (PRP-046). The SDK's `v0.2.0` follows it.

## CONTEXT
- Agreed with the owner (DECISION-056): Q1 where pictures come from (the bundle, or a URL in the view on allowed servers, Elm's model, `on_load`/`on_fail`); Q2 sizing (`Sizing` styles, `fit` `contain`/`cover`, the box's corners and borders, `color` on a bundle's one-colour SVG, `label`); Q3 limits (10 MB a picture, 8,000 px a side and 40 MP, 256 MB decoded a window, 6 downloads at once, 30 s, PNG/JPEG/WebP from servers, SVG ≤ 1 MB from the bundle only; a failed picture shows the mark and sends `on_fail`; a broken view stops the app).
- **What exists:**
  - Bundles carry assets by name (`[assets]` in `rapidkoi.toml`, `--asset NAME=FILE`), but only PNGs: `bundles/rules.la` reads a PNG's header (`png_size`) and refuses anything else.
  - Requests go through `net/network.la` (`check_request`: the app's host and declared servers, HTTPS; `http://localhost` from this machine), `net/dialing.la` (refuses local addresses) and `net/requests.la` (a `Cmd.task` each, 30 s).
  - `ui/icons.la` recolours Rapidkoi's own SVGs by replacing one colour in their text.
- **kivuli 0.54** (`../kivuli/README.md`): `image(Image.Bytes(key, bytes))` shows a PNG, JPEG or SVG, keyed so the same key isn't decoded twice; `.fit(Fit.Contain | Cover | …)`; an SVG reads nothing outside itself (`<image>` refused). It shows **nothing**, and says nothing, for a picture it can't show: not PNG/JPEG/SVG (so **no WebP**), corrupt, or over **4,096 × 4,096 px** or 16 MB.
- Files: new `src/guest/pictures.la` (pure: the pictures a tree names, checked), `src/net/pictures.la` (downloads, in tasks), `src/bundles/images.la` (pure: which pictures a bundle may hold, by header); changed `guest/nodes.la`, `guest/tree.la`, `guest/limits.la`, `bundles/rules.la`, `window/tabs.la` (a page's pictures), `ui/render.la`, `docs/specs/contract-7.md`; tests to match.
- Decisions: DECISION-056 (resolved). Q1–Q3 below are where kivuli falls short of it.

## SECURITY (needs the owner's review)
Downloading a picture is Rapidkoi fetching for an untrusted app: a new capability, so this PRP needs your explicit review.
- Only URLs a view names; only through `check_request`'s rules (the app's host and declared servers, HTTPS, local addresses refused, no redirects): never a server `fetch` couldn't reach.
- No cookies, no headers from the app, no credentials: a plain GET. (So a private picture needs a URL the server signs; asking with the app's headers is a later question.)
- Each download is capped (10 MB, 30 s) and read as bytes; the size is checked from the header before kivuli decodes anything. Pictures are handed to kivuli as bytes, never as paths.
- A bundle's SVG is text the app wrote: kivuli refuses `<image>` and reads nothing outside it; recolouring replaces `currentColor` in its text and nothing else.

## Q1: kivuli can't show what DECISION-056 agreed (needs the owner)
kivuli shows PNG, JPEG and SVG up to 4,096 px a side, and gives no sign when a picture fails to decode. Agreed was WebP too, 8,000 px and 40 MP, and `on_fail` for every failure.
- **(a) Recommended:** build to what kivuli can show today and ask kivuli for the rest (kivuli requests, DECISION-058): WebP is refused with its sentence ("This picture is WebP, which Rapidkoi can't show yet"); the limit is 4,096 px a side and 16 MP until kivuli lifts it; a picture that passes the header check but fails to decode shows nothing and sends no `on_fail` until kivuli reports it. Each is recorded, not worked around; raising the limits later is not a contract change (only refusing less).
- (b) Wait for kivuli before building pictures at all. Notes keeps having no icons.

## Q2: where reading a picture's header lives (needs the owner)
Checking 8,000 px and 40 MP before decoding means reading PNG, JPEG and WebP headers. `png_size` is in Rapidkoi today; JPEG's (walking its segments to the SOF marker) and WebP's would join it.
- **(a) Recommended:** a new Lugha package, **`lugha-imagesize`** (`../lugha-imagesize`, its own repository): `size_of(bytes): Result<ImageSize, ImageError>` with the format, width and height, for PNG, JPEG, GIF, WebP and SVG's `width`/`height`/`viewBox`. Generic, so per CLAUDE.md it isn't Rapidkoi's; written in its own session, tagged `v0.1.0`, then this PRP uses it and `png_size` goes.
- (b) Keep them in Rapidkoi (`bundles/images.la`): faster, against the rule.

## Q3: how many pictures a view may name (needs the owner)
6 downloads at once is agreed, but not how many a view may name: 10,000 nodes could name 10,000 URLs, all queued.
- **(a) Recommended:** at most **256 pictures** in a view (by distinct source); more stops the app ("This app's screen names too many pictures (300; at most 256)"). A gallery shows a page at a time.
- (b) No cap but the queue.

## REQUIREMENTS

### Must Do
- **The `image` node** (contract 7): props `src` (required: `"asset:NAME"` or an `https://` URL, ≤ 2 KB), `fit` (`"contain"`, the default, or `"cover"`), `color` (a colour, as in styles; only on an `asset:` SVG), `label` (1–200 bytes), `on_load`, `on_fail` (messages), `style` (sizes, corners, borders, shadow, opacity, `align`, `grow`). No children. A bad prop stops the app with its sentence; so does an `asset:` the bundle doesn't have, a URL `check_request` refuses, or `color` on anything but a bundle SVG.
- **From the bundle:** shown at once. A bundle may hold PNG, JPEG and SVG assets (SVG ≤ 1 MB, the rest ≤ `MAX_ASSET`), each checked by its header at load (format, size); a bad one refuses the bundle as today. `color` replaces `currentColor` in the SVG with the colour, resolved from the theme like any colour.
- **From a server:** each source named by the current view is downloaded once in a `Cmd.task` (`net/pictures.la`), at most 6 at a time a window, the rest queued in the order the view names them; 30 s each; at most 10 MB, a body over it is cut off and fails. The header is checked (format, a side, megapixels) before the bytes are kept. Kept in memory with the page by source, never on disk; a source no longer in the view stays kept while the window's decoded total (width × height × 4) is under 256 MB, dropped least recently shown first.
- **While loading:** the box shows nothing, at the size its style gives.
- **When it can't be shown** (refused by the network rules at fetch time, a failed request, a status other than 200, over a limit, not PNG/JPEG): Rapidkoi's broken-picture mark (an Iconsax icon, in the theme's dim text colour), and `on_fail` with the app's message and `{ "error": sentence }`. When it's ready: `on_load` with the app's message as an `on_message`. Neither is sent if the app gave no message. Each is sent once per source while it stays in the view.
- **The limits** go in `guest/limits.la`: `MAX_PICTURE` 10 MB, `MAX_PICTURE_PX`, `MAX_PICTURE_PIXELS`, `MAX_SVG` 1 MB, `MAX_DECODED` 256 MB, `MAX_DOWNLOADS` 6, `MAX_PICTURES` (Q3).
- **Drawing it** (`ui/render.la`): kivuli `image(Image.Bytes(key, bytes))` with `.fit`, its style, named by its `label` for tests; the key is the source (and colour, for a recoloured SVG), so a picture is decoded once.
- **The SDK (`../rapidkoi-rust`, at the owner's word, as before):** `image(src)`, `.fit(Fit::Cover)`, `.color(Colour)`, `.label(…)`, `.on_load(msg)`, `.on_fail(msg)`, `Event` for both; tagged `v0.2.0-pre.3`, then `v0.2.0` once Rapidkoi's built-ins and the examples run on it.
- `docs/specs/contract-7.md` (the `image` node, the limits), kivuli requests and DECISIONS.md entries for Q1's gaps, CHANGELOG.

### Must NOT Do
- No fetching a picture no view names (no preloading by the app), and no server `fetch` couldn't reach.
- No disk cache: pictures last as long as the window.
- No SVG from a server, no GIF animation, no video.
- No headers or cookies on a picture's request.
- No pictures in Rapidkoi's own frame from apps: the frame stays Rapidkoi's.

## DESIGN
- `nodes.Node` gains `Picture(Picture)`: `Picture { source: Source, fit: Fit, colour: Option<Colour>, label: Option<string>, on_load: Option<Value>, on_fail: Option<Value>, look: Look }`; `enum Source { Asset(string), Url(string) }`.
- `guest/pictures.la` (pure): `sources(root): Source[]` in view order, distinct; the checks on a source against the bundle and `check_request`.
- `window/tabs.la`: an `AppPage` gains `pictures: Pictures` (source → `Loading`, `Ready(bytes, w, h, shown_at)`, `Failed(sentence)`, and the queue). After each event that changes the tree, `wanted` diffs the view's sources against it, starts downloads up to 6 (`Msg.PictureLoaded(tab, source, Result<u8[], string>)`), and evicts. A result for a page that has gone, or a source no longer wanted, is dropped.
- `ui/render.la`: `Picture` → loading (an empty box of its size), ready (`image`), failed (the mark).
- The contract stays version 7: contract 7 isn't released until this lands.

## TESTS
- [x] `image` reads each prop; a missing or bad `src`, a bad `fit`, `color` on a URL or a PNG, a bad `label` each stop the app with their sentence
- [x] An `asset:` the bundle lacks, and a URL to a server the app may not reach, stop the app
- [x] More than `MAX_PICTURES` sources stops the app (Q3)
- [x] A bundle with a PNG, a JPEG and an SVG loads; an SVG over 1 MB, a picture over the size limits, and a file that's none of them are refused with their sentences
- [x] A bundle picture is drawn at once; `color` recolours `currentColor` and nothing else
- [x] A view naming 8 URLs starts 6 downloads; each result starts the next
- [x] A download's result: ready → drawn and `on_load` sent once; a failed request, a 404, too large, too many pixels, WebP (Q1) → the mark and `on_fail` with its sentence; no message given → nothing sent
- [x] A result for a closed page or a source no longer in the view is dropped
- [x] Past 256 MB decoded, the least recently shown pictures not in the view are dropped first
- [x] Header reading: PNG, JPEG (baseline and progressive), WebP (lossy, lossless, extended): their sizes; truncated or garbage input is an error, never a crash (in `lugha-imagesize` under Q2 (a))
- [x] The new tab and Settings still pass

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [ ] For the owner, by eye: Notes with its icons from its bundle, recoloured in light and dark; a test page of photos from a local server, one broken, loading in the background without holding the window

## AMENDMENTS

- **Approved by the owner, 2026-10-24, "a for all"**, the security section reviewed with it. Q1 (a): built to what kivuli shows today (PNG, JPEG, SVG; 4,096 px a side, 16 MP; WebP refused with its sentence; a decode failure unreported), the gaps asked of kivuli (DECISION-058). Q2 (a): headers read by a new package, `lugha-imagesize`. Q3 (a): at most 256 pictures a view.
- **A picture's messages arrive as `on_message`** (the owner, during the build): `on_load` with no value, `on_fail` with Rapidkoi's sentence as `value`, as a press and an input's text do, so a typed message (`Msg::PhotoFailed(3)`) arrives whole in Rust. `on_fail` the event stays for asks.
- **`Kept` moved to `src/window/downloaded.la`** (the owner approved the split at 501 lines; named `downloaded` because `kept` clashes with locals).
- **The broken-picture mark** is Iconsax's gallery-slash (`assets/icons/gallery-slash.svg`, from the owner), in the theme's dim text colour.
- **Pages keep their bundle's pictures in `Program`** (engine, module, assets), so every way a page opens or restarts has them.
- **A switched-off server** (⚙ → Access) fails the picture with its sentence; the app runs on. Any other refusal stops the app.
- **The SDK:** `image`, `Fit`, `.color`, `.on_load`, `.on_fail` tagged and pushed as `rapidkoi-rust` `v0.2.0-pre.3`; the built-ins moved to it and were repacked; the full suite passed (568). `v0.2.0` follows once Notes and the examples run on it.
