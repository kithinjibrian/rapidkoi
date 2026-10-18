## FEATURE: Remote apps over HTTPS: `rapidkoi://counter.dafifi.net` opens a signed app from a real host, verified, in the background

## OBJECTIVE
Typing `rapidkoi://counter.dafifi.net` (or `https://counter.dafifi.net`) opens the counter from the internet:
- **The first time:** the key file arrives over HTTPS and its key is trusted from then on. The page is marked `✓ counter.dafifi.net · verified`.
- **Every later time:** the same checks as localhost. A swapped key asks first; a withdrawn version is refused with the publisher's reason; an older index is refused.
- **Remote apps must be signed, HTTPS is required,** certificates are always checked, and there's no way past a failed check.

Designed in chat, session 43: `docs/source/meetings/2026-10-18-remote-apps.md` (topics 1–3, all settled).

## CONTEXT

- **Starting state:**
  - `address.parse` accepts local hosts only (`NotLocal`, `NotYet` for https).
  - `fetching.fetch_app` builds `http://` URLs.
  - The keyring's `Identity` has no date.
  - The trust file is format 1.
- **Ending state:**
  - `src/address.la`:
    - `Address` gains `remote: bool`
    - `parse` accepts a remote host name for `rapidkoi://` and `https://` (port optional, 443 by default)
    - it refuses remote `http://` (`NotSecure`) and IP addresses (`IpAddress`)
    - `shown` drops `:443`
    - `site_url(a, path)` gives `https://…` or `http://…`
  - `src/fetching.la`:
    - fetches through `site_url`
    - refuses a remote host with no key file ("can't be verified")
    - refuses a redirect
    - the unsigned fallback is localhost only
    - `fetch_app` takes an optional extra CA file (`Network { ca_file: Option<string> }`), which the app always passes as none
  - `src/verify.la`: `Identity.since` (ms of first use). A first use sets it; a rotation or a higher sequence keeps it.
  - `src/remembered.la`: trust format 2 (`since`). It reads format 1 (since unknown, 0), writes 2, and treats 3+ as newer.
  - `src/frame.la`: `· verified` for remote hosts, `· verified (development)` for local.
  - `src/panel.la`: About shows "Trusted since DATE, first use" (or "first use, date not recorded" for 0).
  - `src/navigate.la`: `go` accepts remote addresses.
  - `test/fixtures/tls/`: a test CA, a `localhost` certificate and key, a wrong-host certificate and an expired one, made once with `openssl`, with a README saying how and that they're test-only.
  - Tests: `test/address.la`, `test/fetching.la` (TLS), `test/remembered.la`, `test/views.la`/`test/panel.la`.
  - Docs: CLAUDE.md (known issues: remote hosts open; rule 3 unchanged), CHANGELOG.md, DESIGN.md (the marker).
- **Related code:**
  - `src/address.la`, `src/fetching.la`, `src/loads.la`, `src/verify.la`, `src/remembered.la`
  - the VM site: memory `test-vm-rapidkoi-sites`
- **Lugha spec relied on:**
  - `std.http` `request_with(…, limits)` for `https://`, verified against the system roots
  - `std.http.serve_tls` (tests)
  - `std.tls.Options { ca_file, timeout_ms }`
  - **`request_tls_with(…, limits, opts)` (DECISION-024: not in std yet)**
- **kivuli API used:** none new.
- **Open decisions:** **DECISION-024** (std can't take both `Limits` and `tls.Options` in one request). Building waits for it, or for the owner's choice of the fallback below.

## IMPLEMENTATION REQUIREMENTS

### Must Do

**Addresses (topic 1)**
- **Remote `rapidkoi://host[:port][/route]` and `https://host[:port]/…`:**
  - `host` passes `rules.check_host`: lowercase, a–z 0–9 . -, 1–253 bytes, at least one dot (so `localhost` stays local).
  - The port is optional (443 by default).
- **Refused with a plain reason:**
  - remote `http://`: "Remote apps open over HTTPS only: rapidkoi://HOST"
  - an IP address (IPv4 dotted, or bracketed IPv6) that isn't local: "Open apps by their host name, not an IP address"
- **Local addresses** behave exactly as today.

**Fetching**
- **Remote:** every request goes to `https://HOST[:PORT]/.well-known/rapidkoi/…`. The limits are as today: 30 s per request, `MAX_BUNDLE` for a build, key and index caps as in `verify`.
- **No key file** (a 404) on a remote host: refused, "HOST has no Rapidkoi key, so it can't be verified". No `app.rkb` fallback.
- **Any 3xx answer:** refused, "HOST answered with a redirect (STATUS); Rapidkoi doesn't follow redirects". Never followed.
- **A certificate failure:** refused with std's reason, "HOST's certificate couldn't be verified: …". There's no way past, and ⟳ tries again.
- **`Network { ca_file: Option<string> }`:**
  - `loads.begin` always passes `Network { ca_file: None }`.
  - Only tests construct one with a file.
  - It's never read from a file, the environment or a flag.

**Trust (topic 2)**
- **`Identity.since`:**
  - a first use sets it to `time.now_ms()` (in the task's check)
  - a rotation, a higher sequence or a separate app keeps the `from` identity's
  - a separate app sets its own
- **The trust file:**
  - written as format 2, with `since` per entry
  - format 1 read with `since: 0`
  - format 3+ is Newer, untouched
- **Marker:** `✓ HOST · verified` when the address is remote; `✓ HOST · verified (development)` when local. ⚠ and the rest are unchanged.
- **About:** "Trusted since 18 Oct 2026, first use"; for `since: 0`, "Trusted on first use (date not recorded)". The fingerprint is as today.

### Must NOT Do
- No remote plain HTTP, no IP-address hosts, no redirects followed, no unsigned remote apps, no certificate bypass.
- No way for the app (config, environment, flag, file) to add a trusted CA. The extra CA is for tests only.
- No change to the bundle, index or key formats, or the checks themselves.
- No tab restore (its own PRP).

## MODEL AND MESSAGES
- **Model fields:** none added.
- **Messages:** none added (remote loads use PRP-008's `Fetched`/`Tick`).
- **`Address.remote: bool`**, `Identity.since: i64`, `fetching.Network`.

## ERROR HANDLING REQUIREMENTS
- **`AddressError` additions:** `NotSecure(host)` and `IpAddress(host)`, with the sentences above.
- **Fetch refusals:** as `Refusal.Failed` or `Unverified`, with the sentences above. A certificate failure is `Failed` (nothing was verified), with std's reason.
- **Nothing panics** on any response, certificate or redirect.

## SECURITY CONSIDERATIONS (the owner reviews before merge: it opens the network)
- **The first remote fetch of a key** rests on HTTPS: std verifies the chain and the host name, always.
- **First-use trust** is remembered (PRP-007), so later visits are checked against it. A swapped key asks.
- **Hosts:** names only. Redirects are never followed, and a 3xx is refused.
- **No unsigned remote code ever runs.**
- **The extra CA hook is test-only by construction:** one parameter, passed as none by the only production caller (`loads.begin`); a test asserts that.
- **Limits** are unchanged (30 s, 32 MB), so a hostile server can't make Rapidkoi hold more.
- **The fixture keys** in `test/fixtures/tls/` are throwaway, trusted only by the tests, and documented as such.

## TESTS TO WRITE
- [ ] **address:**
  - `rapidkoi://counter.dafifi.net` and `https://counter.dafifi.net` parse as remote, port 443, `shown` "counter.dafifi.net"
  - `:8443` is kept
  - remote `http://` gives `NotSecure`; `rapidkoi://174.138.43.152` and `[2001:db8::1]` give `IpAddress`
  - a bad host is `NoHost`
  - local addresses are unchanged
- [ ] **fetching over TLS** (serve_tls on 127.0.0.1 with the fixtures; the test CA through `Network`):
  - a signed site opens, with its record
  - an untrusted CA (`Network { ca_file: None }`) is refused with the certificate sentence
  - a wrong-host certificate is refused
  - an expired certificate is refused
  - a 404 key file on a remote address gives "can't be verified" (localhost's fallback is unchanged)
  - a 301 is refused
  - `loads.begin` passes no CA
- [ ] **verify:** a first use sets `since`; a higher sequence and a rotation keep it.
- [ ] **remembered:** trust format 2 round trips; format 1 reads with `since: 0`; format 3 is Newer.
- [ ] **views:** the remote marker `✓ HOST · verified`; About's "Trusted since …" and the 0 case.
- [ ] **end to end over TLS:** `start(["./main", "rapidkoi://localhost:PORT"])`, with a test CA network injected in the test, opens ✓, and trust is saved with `since`. (Remote host names can't be tested offline; `localhost` with TLS exercises the HTTPS path.)

## ROLLBACK PLAN
- Branch: `prp-009-remote_apps`, from main after PRP-008.
- Local addresses are untouched, so abandoning it leaves today's behaviour.

## ACCEPTANCE CRITERIA
- [ ] `./main rapidkoi://counter.dafifi.net` shows the loading page, then the counter, `✓ counter.dafifi.net · verified` *(the owner)*
- [ ] After a restart it opens against the remembered key, and About shows "Trusted since …" *(the owner)*
- [ ] A republished 0.2.1 on the VM is picked up on ⟳ *(the owner)*
- [ ] Every test above
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a doc comment
- [ ] No file over 500 lines
- [ ] CHANGELOG.md updated
- [ ] The owner's security review before merge

## VALIDATION
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner:
  1. `./main rapidkoi://counter.dafifi.net`
  2. Then republish (`~/rapidkoi-sites/counter.dafifi.net`, `version = "0.2.1"`, `./main publish`, copy `site/.well-known` to the VM) and press ⟳.
