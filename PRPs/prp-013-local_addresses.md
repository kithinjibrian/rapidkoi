## FEATURE: Refusing server names that point to this computer or the local network, for app fetches and for opening apps

## OBJECTIVE
- A name an app fetches, or a `rapidkoi://` address being opened, is looked up by Rapidkoi first. If any address it points to is on this computer or the local network, it's refused with a plain reason, and nothing is sent there.
- Rapidkoi connects to exactly the address it checked, so a second DNS answer can't differ (no DNS rebinding).

Designed in chat, session 66: `docs/source/meetings/2026-10-19-local-addresses.md` (questions 1–4, settled). Resolves DECISION-026. **Network capability given to untrusted code: the owner's security review before merge.**

## CONTEXT

- **Starting state:**
  - `requests.send` (app fetches) and `fetching.get` (opening apps) hand the URL to `std.http.request_tls_with`, which resolves the name and connects to whatever it gets.
  - The only guard is HTTPS with a checked certificate (DECISION-026).
- **Ending state:**
  - **`src/network.la`:**
    - `local_address(ip: string): bool`, pure: IPv4 and IPv6 text, every range below; IPv4 inside IPv6 judged as IPv4
    - `check_addresses(host, addresses): Result<string[], NetworkError>`: all public → the addresses in order; any local → `PointsLocal(host)`; none → `NoAddress(host)`
  - **A shared helper** (`src/dialing.la`, new) used by both `requests.send` and `fetching.get`:
    - for a remote name: `net.resolve` → `check_addresses` → try each address in order with std's connect-to-address option, the name kept for `Host`, SNI and the certificate
    - stop at the first that connects; a refusal is never sent anywhere
    - `localhost` development addresses are not looked up (they're typed as such) and behave as today
  - **Sentences:**
    - "X points to this computer or your local network, so Rapidkoi won't connect to it"
    - "X has no address" (the lookup gave nothing)
    - "Couldn't look up X: …" (the resolver's failure)
  - **Opening an app:** the same sentences, as `Refusal.Failed(host, why)`.
- **Related code:** `src/requests.la`, `src/fetching.la`, `src/network.la`, `src/address.la`.
- **Lugha std relied on (not yet in std: a Lugha session adds it first):**
  - `std.net.resolve(name: string): Result<string[], string>`: every address the system resolver gives (IPv4 and IPv6 as text, in its order, no duplicates); an IP literal gives itself
  - a connect-to-address option: `tls.Options { address: string, … }` (`""` = look the name up as today). When set, `tls.connect_with(addr, opts)` and `http.request_tls_with(…, opts)` dial `address` at `addr`'s port, and keep `addr`'s host for SNI, the certificate check and `Host`. Plain `http://` honours it too.
- **kivuli API used:** none new.
- **Open decisions:** none (DECISION-026 resolved). **Blocked on the std addition**; nothing is built until the installed `lughac` has it.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- **Local ranges** (the owner: "block all of those"):
  - this computer: 127.0.0.0/8, ::1, 0.0.0.0/8, ::
  - private: 10/8, 172.16/12, 192.168/16, fc00::/7
  - link-local: 169.254/16, fe80::/10
  - shared ISP space: 100.64/10
  - multicast: 224/4, ff00::/8; broadcast: 255.255.255.255
  - reserved and special: 192.0.0/24, 192.0.2/24, 198.18/15, 198.51.100/24, 203.0.113/24, 240/4, 2001:db8::/32, 64:ff9b::/96 (NAT64, judged as the IPv4 inside), and the IPv4-mapped `::ffff:a.b.c.d` and IPv4-compatible forms judged as IPv4
  - anything that doesn't parse as an address is refused, not guessed
- **Both places:** app fetches (`requests.send`) and opening an app (`fetching.get`, every request of a load: key, index, build).
- **A mixed answer is refused whole** (owner: "A").
- **Pinned:** the connection goes to an address from the checked list, never through a second lookup.
- **Timeouts:** one deadline for the whole request (lookup and every attempt): 30 s for a fetch, `FETCH_TIMEOUT_MS` for a load.

### Must NOT Do
- No lookups for `localhost`/`127.0.0.1`/`[::1]` development addresses: they keep today's rules (local apps only).
- No setting to allow a range (not asked for; a VPN user can't reach a VPN host from an app).
- No caching of lookups.
- No working around a missing std option in rapidkoi (CLAUDE.md scope rule).

## MODEL AND MESSAGES
- No new messages or model fields. `Network` gains a **test-only** `hosts: (string, string[])[]` (a name → the addresses a test's lookup returns, like `ca_file`). Rapidkoi itself always has `[]` and never reads it from a file, the environment or a flag.

## ERROR HANDLING REQUIREMENTS
- `NetworkError` gains `PointsLocal(host)` and `NoAddress(host)`, with the sentences above.
- A resolver failure is a `Reply.Failed` / `Refusal.Failed` sentence, never a panic.

## SECURITY CONSIDERATIONS (the owner reviews before merge)
- The check runs on every address before any connection, and the connection is pinned to one of them.
- No range can be allowed back by an app or a manifest.
- IPv4 hidden in IPv6 can't slip past.
- Development addresses are unaffected, and only apps from this computer reach them (PRP-012).

## TESTS TO WRITE
- [x] **network** (pure): `local_address` at each range's edges (172.15.255.255 public, 172.16.0.0 and 172.31.255.255 local, 172.32.0.0 public; 100.63.255.255 / 100.64.0.0 / 100.127.255.255 / 100.128.0.0; 169.254.169.254; 0.0.0.0; 255.255.255.255; 224.0.0.1; 240.0.0.1), IPv6 (::1, ::, fe80::1, fc00::1, fd12::1, ff02::1, 2001:db8::1, 2606:4700::1 public), mapped (`::ffff:10.0.0.1`, `::ffff:8.8.8.8`, `64:ff9b::7f00:1`), garbage refused; `check_addresses`: all public, mixed, none.
- [x] **requests:** with `Network.hosts`, a name pointing at 127.0.0.1 and one at [93.184.215.14, 10.0.0.1] are refused with the sentence, and the local test server receives nothing; the pinned connect: the TLS fixture on 127.0.0.1 reached as `localhost` through the pin option succeeds, and as `wrong.example` fails its certificate.
- [x] **fetching:** opening `rapidkoi://x.example` with `hosts` pointing at 192.168.1.1 is refused before any request.
- [x] **dialing:** the first address that refuses connection falls through to the next; one deadline across attempts.

## ROLLBACK PLAN
- Branch `prp-013-local_addresses`, from main once `lughac` has the std addition.
- Without it, things are as today: the certificate check is the guard.

## ACCEPTANCE CRITERIA
- [ ] Every test above
- [ ] `lughac test test/` and `cargo test` pass; `lughac build src/main.la -o main` succeeds; `lughac fmt --check src test examples` is clean; no file over 500 lines
- [ ] The quote demo still works at `rapidkoi://quote.dafifi.net` *(the owner)*
- [ ] CHANGELOG.md, CLAUDE.md (known issues), MEMORY.md (decision 18) updated
- [ ] **The owner's security review before merge**

## VALIDATION
- `lughac fmt --check src test examples`
- `lughac test test/`
- `lughac build src/main.la -o main`
- For the owner: `./main rapidkoi://quote.dafifi.net` still shows the quote, signed in.
