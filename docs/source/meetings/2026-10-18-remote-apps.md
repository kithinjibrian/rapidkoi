# Remote apps over HTTPS (PRP-009) — design in chat, session 43, 2026-10-18

The owner and Claude, one topic at a time; each answered "yes". Feeds PRP-009. The first real target: `rapidkoi://counter.dafifi.net`, served from the owner's VM (`/opt/rapidkoi`, nginx behind Traefik, through Cloudflare).

## 1. Which addresses open — settled

- `rapidkoi://host[:port]` for a remote host means HTTPS: `https://host/.well-known/rapidkoi/…`. Typing `https://host` opens the same app. The default port is 443.
- Remote hosts are HTTPS only. Plain `http://` stays for this machine (localhost, 127.0.0.1, [::1]).
- Host names only: no raw IP addresses (an app's identity is its host; IPs change and are shared; their certificates are rare).
- Remote apps must be signed. No key file means "HOST has no Rapidkoi key, so it can't be verified". The unsigned `app.rkb` fallback stays localhost-only.
- No redirects: a 3xx is refused with the reason, never followed.
- Certificates are checked as std always checks them: the chain against the system's roots and the host name. Nothing turns that off; there's no "proceed anyway".

## 2. Trust, and how it looks — settled

- First use, quietly: HTTPS vouches for the first key file, and Rapidkoi trusts that key from then on (remembered). No prompt. Next time: the same key, or a signed rotation, is ✓; a different key without approval asks, as for localhost.
- Marker: `✓ counter.dafifi.net · verified`. Localhost keeps `· verified (development)`.
- About shows the fingerprint and "Trusted since DATE, first use".
- Hosts are separate apps: `localhost:8000` and `counter.dafifi.net` have separate keys and state.
- A signed `.rkb` whose host is remote is ✓ once that key is trusted, ⚠ not confirmed before (as today).
- The trust file stores the date of first use: format 2. An older Rapidkoi leaves it alone (Newer); this one reads format 1 and writes 2.

## 3. Testing — settled

- Automated, no internet: a throwaway test CA and a `localhost` certificate, made once with `openssl`, in `test/fixtures/tls/` (test-only; the fixture key in the repo is normal). Sites are served with `serve_tls` on 127.0.0.1. Cases: opens ✓ verified; an untrusted CA, a wrong host name, an expired certificate, no key file, a redirect, and an IP address are each refused with the reason.
- The hook: the background load takes an optional extra CA file. Only test code passes one; the app always passes none (no config, environment variable or flag can set it).
- On screen: `rapidkoi://counter.dafifi.net`, then a republished second version (an update, and a stale cache handled).
- Known pause: the DNS lookup and TLS handshake still briefly hold the window (kivuli's note). If noticeable, it goes upstream to Lugha.

## Found while writing the PRP

- std's `request_tls` takes TLS options (the extra CA) but no `Limits`, so it's capped at the default 16 MB; `request_with` takes limits but no TLS options. The app's fetch (system roots, 32 MB, 30 s) and the tests' fetch (the test CA) would be two different calls. A small std addition, `request_tls_with(method, url, headers, body, limits, opts)`, lets both use one. Recorded as DECISION-024; a prompt for the Lugha session was given to the owner.
