# Refusing names that point to the local network (DECISION-026, PRP-013) — design in chat, session 66, 2026-10-19

The owner and Claude, one question at a time; each answered as below. Follows PRP-012's security review, which recorded the gap: std looks a name up and connects wherever it points, so a declared `evil.example` pointing at `127.0.0.1` or a router was guarded only by the certificate check.

## 1. Which addresses — settled ("yes, block all of those")

- This computer: 127.0.0.0/8, ::1, 0.0.0.0, ::
- Private networks: 10/8, 172.16/12, 192.168/16, fc00::/7
- Link-local: 169.254/16 (cloud metadata at 169.254.169.254), fe80::/10
- Shared ISP space (CGNAT, Tailscale and other VPNs): 100.64/10
- Multicast, broadcast and reserved ranges
- IPv4 inside IPv6 (`::ffff:a.b.c.d`) checked as the IPv4 address it is

## 2. Where — settled ("both")

- An app's fetches (PRP-012), and opening an app at a `rapidkoi://` address (PRP-009).
- Development is unchanged: `localhost` typed as such is never looked up.

## 3. What std gains — settled ("A")

- `net.resolve(name)`: every address a name points to.
- An option on std's TLS and HTTP calls to connect to one given address, while the URL's name is still used for `Host`, SNI and the certificate (like curl's `--resolve`).
- Rapidkoi looks the name up, checks every address with its own pure, tested code, and connects to exactly the address it checked, so a second DNS answer can't differ.
- Built in `../lugha` by its own session; Rapidkoi waits for it.

## 4. A mixed answer — settled ("A")

- A name with any local address is refused whole: "api.acme.com points to this computer or your local network, so Rapidkoi won't connect to it".
- All public: tried in order until one connects.
