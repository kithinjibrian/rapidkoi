## FEATURE: Local development: `./main serve` a bundle, and open it at `rapidkoi://localhost:PORT`

## OBJECTIVE
A developer serves their `.rkb` from their own machine with `./main serve counter.rkb`, and Rapidkoi opens it at `rapidkoi://localhost:8000`, typed in the address box or given on the command line. Repacking the bundle and pressing **Reload** in the info panel loads the new build: the start of the vision's development loop. Only local addresses open for now; the real internet waits for HTTPS and signing (DECISION-009). Uses Lugha's new `std.http` (milestone 111).

```
$ ./main serve counter.rkb --port 8000
serving Counter 0.1.0 at rapidkoi://localhost:8000 (Ctrl+C to stop)
$ ./main rapidkoi://localhost:8000
```

## CONTEXT

- Starting state: PRP-003 (bundles) on `main`. `loader.load(engine, path)` opens `.rkb` and `.wasm` files. The address box opens files and answers `rapidkoi://…` with "Links come in a later step".
- Ending state:

```
src/address.la  — pure: parse `rapidkoi://host:port[/route]` and `http://host:port/…`; local hosts only; Address, AddressError
src/serve.la    — the `serve` command: std.http server on 127.0.0.1, one fixed route for the bundle; its handler is a pure function
src/loader.la   — load(): a local address fetches the bundle (std.http, 32 MB, 3 s) and loads it like a file
src/main.la     — `./main serve …` runs serve.run (no window)
src/window.la   — go(): local addresses open; other addresses give a notice
src/frame.la, src/panel.la — `🛠 localhost:8000 · development`; the panel's Reload
src/tabs.la     — reload(page): fetch and start again, fresh state (kept under 300 lines; see below)
test/address.la, test/serve.la, test/loader.la (new), test/shell.la, test/views.la
CLAUDE.md       — rule 3 (a localhost fetch in update), COMMANDS (serve), FILE ORGANIZATION, KNOWN ISSUES
```

- Lugha spec relied on: `std.http` (`serve_with`, `request_with`, `Limits`, `bytes`, `text`, `Request.path`, `Response.status`/`body`), `std.net` addresses, `spawn` (tests).
- Open decisions: none block. DECISION-009 (signing) and fetching from the internet stay out.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Addresses (`address.la`, pure)**
- `parse(text) → Result<Address, AddressError>` with `Address { host, port, route, http: bool }`.
  - `rapidkoi://HOST:PORT[/route…]`, and `http://HOST:PORT/anything.rkb` (for a quick test).
  - HOST must be `localhost`, `127.0.0.1` or `[::1]`, with a port from 1 to 65535.
- Any other host is `NotLocal(host)`: "Only local development addresses (localhost) can be opened yet". `https://` is `NotYet`: "HTTPS comes with signing". Malformed input is `Bad`.
- `url(a)` gives the fetch URL. For `rapidkoi://`, that's `http://HOST:PORT/.well-known/rapidkoi/app.rkb`, the well-known folder from the packaging design. For `http://`, it's the URL as given.
- The route is parsed and kept on the page but not passed to the app (the links step).

**Fetching (`loader.la`)**
- `load(engine, text)`: when `address.parse` succeeds, `std.http.request_with("GET", url, #{}, [], limits)` with `body = MAX_BUNDLE` and `timeout_ms = FETCH_TIMEOUT_MS` (3000).
  - A status other than 200 gives "The development server answered {status}".
  - A network error gives "Couldn't reach {address}: {reason}".
  - Otherwise the body goes through exactly the same `bundle.read` path as a file. Bare `.wasm` over HTTP isn't accepted: bundles only.
- A `.rkb` file path still works exactly as now.

**Rule 3 (part of approving this PRP).** The shell's `update` may also **fetch a bundle from a local development address**, bounded by `MAX_BUNDLE` and `FETCH_TIMEOUT_MS`. Why: kivuli has no background tasks yet. A local fetch takes milliseconds, and the timeout caps the worst case. Fetching from the internet needs background loading first (DECISION-021, new).

**Serving (`serve.la`)**
- Usage: `./main serve BUNDLE.rkb [--port N]` (default 8000).
  - It reads and `bundle.read`s the file at start: an invalid bundle is refused, with `rapidkoi serve: <why>` and exit 1.
  - It listens on **127.0.0.1 only**, never all interfaces, and prints `serving {name} {version} at rapidkoi://localhost:{port} (Ctrl+C to stop)`.
- `handle(file, request) → Response`, pure apart from reading `file`:
  - `GET /.well-known/rapidkoi/app.rkb` → 200, the file's **current** bytes, `content-type: application/vnd.rapidkoi.bundle`. Repacking is picked up with no restart.
  - `GET /` → 200, a one-line text: "Rapidkoi development server: open rapidkoi://localhost:{port}".
  - Anything else → 404. Other methods → 405.
  - It never serves any other file and never takes a path from the request, so there's no path traversal.
  - Each request re-reads the bundle and checks it again; a bundle broken by a bad repack → 500 with the reason as text.
- `std.http.serve_with` with its default limits (the requests are tiny).

**Showing it**
- **Address on a page from a server:** `🛠 localhost:8000 · development`, drawn by Rapidkoi only.
- **Info panel:**
  - "Served from rapidkoi://localhost:8000 (development, unsigned)"
  - a **Reload** button: fetch again, compile, start with a fresh `init` (the new build may have a new state shape: DECISION-019 handles state across builds properly later)
  - Restart, as now: the same build, fresh state
- **Pins, "where you left off", search:** a served app's `path` is its address text, so pinning and reopening work as for files.

**Tabs size.** `tabs.la` is 295 lines. Reload's logic (fetch, then start) lives in `loader.la`, and `tabs.la` gains only `reload(page)`, a few lines. If it would pass 300 anyway, stop and propose a split.

### Must NOT Do
- No hosts other than localhost, 127.0.0.1 and [::1]. No HTTPS. No index, signatures or hash checks (DECISION-009).
- No routes passed to apps (links). No live reload or file watching yet (Reload is manual).
- The server never listens beyond 127.0.0.1 and never serves anything but the one bundle and its index text.
- No background threads in the window (DECISION-021).

## ERROR HANDLING REQUIREMENTS
- `AddressError { NotLocal(host), NotYet, Bad(why) }`, each with a plain message, shown as the address box's notice.
- Fetch failures and bad bundles go to the error page, with Reload offered for development pages (the server may come back).
- `serve`: a missing or invalid bundle, a bad port, or a port in use (`serve_with`'s `Err`) gives `rapidkoi serve: <why>` and exit 1.
- Nothing panics on a server's response or a client's request.

## SECURITY CONSIDERATIONS
- Local only, on both sides. The client refuses other hosts before connecting, and the server binds 127.0.0.1.
- The response is hostile, like any bundle: `std.http` caps the body at `MAX_BUNDLE` and times out, then `bundle.read` checks everything.
- The server takes no paths from requests and serves one file, re-checked each time.
- "Development, unsigned" is shown wherever the address is.
- Not reviewed for the internet: this is a development tool. DECISION-009 and DECISION-021 come before any remote host.

## TESTS TO WRITE
- [ ] address:
  - each accepted form, with its port and route
  - other hosts refused; https refused
  - malformed input: no port, port 0, port 70000, `rapidkoi://`, letters in the port
  - `url()` for both schemes
- [ ] serve (pure handler):
  - the bundle route gives 200 and the current bytes; after the file changes, the new bytes
  - `/` gives the hint; another path gives 404; POST gives 405
  - a broken bundle on disk gives 500
- [ ] serve (command): a bad bundle or a bad port gives exit 1 and a message
- [ ] loader, with a real server on a spawned green thread at a fixed test port:
  - it fetches and loads a bundle, the manifest's name kept
  - a server that 404s or is missing (nothing listening) gives the error page
  - a response too large is refused
- [ ] shell/window: Go on `rapidkoi://localhost:PORT` opens a tab; `rapidkoi://example.com` gives the notice; Reload refetches (the server's bundle swapped between, so the version changes)
- [ ] views: the 🛠 marker in the address; the panel's "Served from …" and Reload

## ROLLBACK PLAN
- Branch: `prp-004-dev-server`. `main` stays at bundles and icons until merge.

## ACCEPTANCE CRITERIA
- [ ] `./main serve counter.rkb` in one terminal, `./main rapidkoi://localhost:8000` in another: the counter opens, marked development *(the owner checks on screen)*
- [ ] Repack with another `--version`, press Reload in ⓘ: the panel shows the new version *(on screen)*
- [ ] `rapidkoi://example.com` in the box gives "Only local development addresses (localhost) can be opened yet"
- [ ] `lughac test test/` passes; `lughac fmt --check src test` is clean; `lughac build src/main.la -o main` succeeds
- [ ] Every `public` item has a doc comment; no file over 300 lines; CHANGELOG.md updated

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner: the two commands above; then repack, and Reload.
