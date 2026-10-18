## FEATURE: Apps reaching the network: a signed app can make HTTPS requests to its own host and the servers it declares, in the background, with switches on the Access page

## OBJECTIVE
- **Requests:** an app asks Rapidkoi to fetch `https://api.notes.acme.com/notes` and gets the response as an event, while the window stays responsive.
- **Only declared servers:** it can reach its own host and the servers it listed when it was published. Anything else is refused before leaving the computer.
- **The Access page** shows the list, marks servers new after an update, and switches each off.
- **Signing in to the app's own server** becomes possible: fetch a challenge, ask for a signature, send it (PRP-011's keys).

Designed in chat, session 62: `docs/source/meetings/2026-10-19-network.md` (topics 1–4, settled). **This is the first capability given to untrusted code: the owner's security review before merge.**

## CONTEXT

- **Starting state:**
  - Apps have no network.
  - Effects are `identity` and `sign`; answers carry bytes or null.
  - The Access page is a placeholder ("None").
  - Manifests have no servers.
- **Ending state:**
  - **Manifest:**
    - `servers: string[]` (in format 1 and 2 bundles; at most 32 names, each passing `check_host` with a dot; `localhost:PORT` allowed)
    - `project.la` reads `[network] servers`
    - `pack` gets `--server NAME` (repeatable), and `publish` writes them
  - **Contract 2:**
    - a third effect: `{kind: "fetch", msg, method, url, headers?: {text: text}, body?: bytes}`
    - its answer `value` is `{status, headers, body}` or `{error: text}` (answers already carry `msg`)
  - **`src/network.la` (new, pure):**
    - checks a request against the app's allowed servers and switches: the URL's scheme and host, the method, headers (no `host`, `cookie`, `content-length`, `connection`, `transfer-encoding`, `upgrade`; names are tokens, values have no control characters), the body ≤ 1 MB
    - returns the request to send, or the refusal sentence
  - **`src/requests.la` (new):**
    - runs each allowed request in a `Cmd.task` (`std.http.request_tls_with`, 30 s, body ≤ 8 MB, no redirects followed)
    - keeps at most 4 in flight per app; more are answered "Too many requests at once; try again"
    - the result arrives as `Msg.Responded(tab, load, id, Response)` and is answered to the guest as an answer event
  - **`src/effects.la`:** a fetch effect goes to `network` then `requests`; the other effects are as before.
  - **`src/people.la`:**
    - `Person.off: string[]` (switched-off servers) and `Person.seen: string[]` (servers shown before)
    - the people file goes to format 2
  - **`src/account.la`** (or a new `src/access.la`):
    - the Access page: the own host, then each declared server, each with a switch, and "new" until seen
    - the summary on the main page ("2 servers", "None")
    - on open, newly declared servers give the notice "Counter can now also reach X"
  - **`guest/rust/rapidkoi`:**
    - `Ask::fetch(msg, Request { method, url, headers, body })`
    - the answer reaches `handle` as `Event::Response { msg, status, headers, body }` or `Event::Failed { msg, error }`
    - `Event::Answer` keeps bytes or `None` (key, signature)
    - a small `sign_in` helper for the challenge → sign → post steps
  - **`docs/server-sign-in.md` (new):** exactly what an app's server checks.
  - **Docs:** the contract doc's fetch section, DESIGN.md (the Access page), CLAUDE.md (rule 3: requests run in tasks, never in `update`), CHANGELOG.md.
- **Related code:** `src/fetching.la` (the same HTTPS client and limits), `src/loads.la` (`Cmd.task`, results by tab and number), `src/effects.la`, `src/account.la`, `src/people.la`.
- **Lugha spec relied on:**
  - `std.http.request_tls_with`, with `Limits { body, timeout_ms }`
  - `std.tls.Options`
- **kivuli API used:** `Cmd.task`, `checkbox`, `button_with` (existing).
- **Open decisions:** none.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Which servers (topic 1)**
- **Allowed for a page** = its own host (for a signed or local-address app; none for a bare file) + its manifest's `servers` − its switched-off servers.
- **HTTPS only,** by host name. Plain `http://` is only for `localhost`/`127.0.0.1`/`[::1]`, and only when the app itself came from this machine: a local file, or a local address. A remote app's request to localhost is refused even if declared: "Apps from the internet can't reach this computer".
- **Unsigned apps from a file** may only use `localhost` servers they declare (development). They get no remote network.
- **Refusals** are answered at once, never sent:
  - "X isn't one of Counter's servers"
  - "You turned off X for Counter"
  - "Apps from the internet can't reach this computer"
  - "Requests use HTTPS"
  - and bad method, header or size reasons

**Requests (topic 2)**
- **Methods:** GET, POST, PUT, PATCH, DELETE.
- **Headers:** the app's own, minus the forbidden list above; `content-length` is set by std.
- **Sending:**
  - there's no cookie jar; `Set-Cookie` is passed on to the app like any header, and stored nowhere
  - redirects are never followed; the app gets the 3xx and its `location` header
  - 30 s per request; the response body is at most 8 MB, else `{error: "The response from X was larger than 8 MB"}`
- **In the background:** each request is a `Cmd.task` capturing only plain data (the request, and a test-only CA as in PRP-009's `Network`).
- **Arrival:** a result for a closed tab, or an older load, is dropped (like PRP-008).
- **Concurrency:** at most 4 in flight per tab.
- **Answers** go through `handle` as `{event: "answer", msg, value: {status, headers, body} | {error}}`.

**Signing in (topic 3)**
- Nothing new in Rapidkoi beyond fetch. `docs/server-sign-in.md` states:
  - the signed message: CBOR `{kind: "rapidkoi sign-in", host, challenge}`
  - the public key (32 bytes, Ed25519)
  - challenges must be fresh and single-use
  - the host must be the server's own
- The Rust helper composes `Ask::fetch` (challenge) → `Ask::sign` → `Ask::fetch` (post the key and signature), and reports the server's reply.

**Updates and switches (topic 4)**
- On each open:
  - declared servers not in `Person.seen` give one notice ("Counter can now also reach X, Y") and are marked new on the Access page until the page is viewed; then they're added to `seen`
  - on the very first open, everything declared is seen silently (no notice: nothing changed)
- Switches: `Person.off`, toggled per server (own host included). They're saved with the people file, and Forget removes them.

### Must NOT Do
- **No network for anything else in Rapidkoi apps:** no sockets, no WebSockets (later), no DNS lookups for the app.
- **No ambient credentials:**
  - no cookies stored
  - no `Authorization` added by Rapidkoi
  - no identity sent without the app's `sign` effect
- **No redirects followed. No plain HTTP to remote hosts. No raw IPs.**
- **No prompt at first use or on update** (declaration-only, the owner's rule).

## MODEL AND MESSAGES
- **Messages:**
  - `Responded(i64, i64, i64, Reply)`: a request's result, with its tab, load and request number
  - `ToggleServer(string)`: the Access page's switch for this app
  - `Seen`: the Access page was viewed, so "new" marks clear
- **Model:**
  - requests in flight per tab: a count on `AppPage`
  - `requests: i64`: the next request number

## ERROR HANDLING REQUIREMENTS
- **`NetworkError`:**
  - `NotListed(host)`, `Off(host)`, `Local`, `NotHttps`, `BadUrl`
  - `BadMethod(m)`, `BadHeader(name)`, `TooLarge(n)`, `TooMany`
  - each with its sentence, answered to the app as `{error}`
- **Transport errors** are mapped as `fetching.network` does: timeout, can't reach, the certificate. A bad response is `{error}`, never a panic.

## SECURITY CONSIDERATIONS (the owner reviews before merge)
- **The allowed list:**
  - comes from the signed manifest (format 2) or the person's own local file
  - is checked on every request, before anything is sent
  - the URL is parsed by Rapidkoi, never passed through raw
- **No localhost from the internet:** a remote app can't reach services on this computer.
- **No ambient authority:**
  - no cookie jar or stored credentials
  - Rapidkoi adds nothing that identifies the person; identity only travels when the app asks to `sign`, and only as a host-bound signature
- **Caps:** request body 1 MB, response 8 MB, 30 s, 4 in flight per app, and answers through `handle` bounded by fuel.
- **Headers are filtered,** so an app can't smuggle `Host` or request-splitting.
- **The off switches** are enforced before sending.
- **Results** are dropped for closed tabs and stale loads.

## TESTS TO WRITE
- [ ] **network** (pure):
  - own host allowed; declared allowed; others refused
  - a switched-off server refused
  - http refused except localhost, and localhost only for local apps
  - forbidden headers, a bad method and an oversized body each refused
  - an IP refused
- [ ] **manifest:**
  - `servers` round-trips in both formats
  - more than 32, or a bad name, is refused
  - `rapidkoi.toml [network] servers`, and `pack --server`
- [ ] **requests** (live sockets, local HTTP and TLS with the test CA):
  - a GET and a POST with a body arrive; the answer has status, headers and body
  - a 301 is returned, not followed
  - a timeout and a too-large response give errors
  - 5 at once: the fifth refused
  - a result for a closed tab is dropped
- [ ] **effects:** a v2 echo app's fetch answer carries the response map; a refused fetch carries `{error}`.
- [ ] **Access page:**
  - lists the own host and declared servers
  - a switch sends `ToggleServer` and the next request is refused
  - after an update adds a server: the notice, "new", then seen
  - Forget clears the switches
- [ ] **sign-in end to end** (a local test server that issues a challenge and checks the signature with the public key): the helper's three steps succeed; signed out, it fails.
- [ ] **Rust:** `Ask::fetch` encodes; `Event::Response`/`Failed` decode; the helper's steps.

## ROLLBACK PLAN
- Branch: `prp-012-network`, from main after PRP-011.
- Without it, apps simply have no network as today.

## ACCEPTANCE CRITERIA
- [ ] A demo app (a small "quote of the day" from its own server, published to the VM) shows a fetched result, signed in with its key *(the owner)*
- [ ] Switching its server off on the Access page makes it show the refusal *(the owner)*
- [ ] Every test above
- [ ] `lughac test test/` and `cargo test` pass
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated
- [ ] **The owner's security review before merge**

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `cargo test --manifest-path guest/rust/Cargo.toml`
- `lughac build src/main.la -o main`
- For the owner: the demo app on `counter.dafifi.net`'s VM (or `localhost` first). Open it, see the fetched quote and "signed in", then switch the server off on the Access page.
