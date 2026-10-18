# CHANGELOG — rapidkoi

Follows [Keep a Changelog](https://keepachangelog.com) format.
Updated at the end of every session when something is completed and merged.
Never deleted. Older entries are never modified.

---

## [Unreleased]

### Added
- Project scaffold: a Lugha program with kivuli as its UI dependency.
- The shell: `./main app.wasm` runs a WebAssembly app under UI contract v1 (text, button, input, column, row).
- Apps that crash, loop forever, misbehave or are incompatible show "This app stopped working" with Restart, and never take Rapidkoi down.
- A Rust library for writing Rapidkoi apps, and a counter demo app.
- Rapidkoi's frame: a bar (back, home, forward, an address and search box, app info, share) and a tab rail on the right.
- Tabs: several apps at once, each in its own sandbox; a crash stays in its tab. Home isn't a tab: + and ⌂ show it, and every app stays open.
- Home: "Where you left off" (Continue returns to the app, or reopens a closed one exactly as it was) and Pinned apps, filtered as you type; Ctrl+K jumps to search.
- The app info panel: what the app is, its permissions (none), and Restart, Pin and Close tab.
- Apps can give each screen a title; Rapidkoi shows it in the tab and the window title.
- The bar lives in the title bar, one row with the window controls; Rapidkoi opens maximized, in its dark theme.
- The window is one flat frame colour, the title bar and the rail included, with no shadow; the page sits flush inside it, curving only at its top-right corner.
- Text fields an app gives no handler show greyed out instead of failing.
- Bundles: `./main pack` packs an app (its module, name, version, description, icon, banners and assets) into one `.rkb` file; type its path, or run `./main app.rkb`, to open it. The info panel shows its version and description.
- A bundle's icon shows on its tab and its home tile. Icons and banners are checked for size before anything decodes them.
- Local development: `./main serve app.rkb` serves a bundle on this machine, and `rapidkoi://localhost:8000` opens it, marked development; Reload picks up a repack.
- The info panel, redesigned: the app's icon, name, version and where it's from; Your settings (Account, Notifications, Access); On this device (Where you left off, Data); and Forget this app, each with its own page. Start over and Forget ask first.
- ⟳ Reload and ☆ Pin in the bar. Reload keeps the app's state.
- Text starts at the left, in apps and in Rapidkoi's own screens.
- Signed apps, published locally: describe an app in `rapidkoi.toml`, make a key with `./main keygen`, and `./main publish` writes a static site of signed builds and a signed index. `./main serve site/` serves it, and Rapidkoi verifies the key, the index and the build before running anything, marked ✓ verified (development). A failed check is refused with its reason and no way past it.
- A site whose key changes without its old key's approval asks first: keep using your version, or open it as a separate app. Key rotations signed by the old key are followed silently.
- `./main publish --withdraw VERSION --reason TEXT` withdraws a version; Rapidkoi refuses a withdrawn build with the publisher's reason.
- `./main pack` with no arguments packs from `rapidkoi.toml`, signed when it has a key. Signed `.rkb` files opened by path show ⚠ not confirmed until their key is trusted this session.
- `./main keygen` writes the secret key readable by you only (0600), whole or not at all.
- Rapidkoi remembers across restarts: the sites it trusts (so a swapped key still asks first), your pins, and where you left off. A signed app keeps its state by its host and key: opened again by its address, pin or Continue, it picks up where it was, and a key rotation keeps it. Unsigned apps start fresh after a restart. Each change is saved at once, in `~/.local/share/rapidkoi/`, safely: a crash leaves the old file or the new one. A file that can't be read is set aside with a notice; one from a newer Rapidkoi is left alone.
- Apps at an address load in the background: the tab opens at once on a loading page (a spinner, then the time so far), and the window stays usable meanwhile. Each request may take 30 seconds. Nothing shows ✓ until every check has passed.
- Remote apps: `rapidkoi://counter.dafifi.net` (or `https://…`) opens a signed app from a real host over HTTPS, its certificate always checked, marked ✓ verified. The first key is trusted quietly and remembered; About says since when. Remote apps must be signed; plain http, IP addresses and redirects are refused.
- The info panel opens over the app, which dims behind it; a click outside or Escape closes it. Right-click a tab for Reload, Pin, About, Close and Forget; right-click a home tile for Open, Unpin and Forget. Home's pinned apps re-flow in a grid as the window resizes, and panel rows are clickable anywhere.
- Identity: each signed app gets its own key for you, made on this computer and signed in automatically, so apps can't tell they share a person; the Account page signs out, starts over as someone new, and holds your name, which apps can show (`{FIRST_NAME}`, `{LAST_NAME}`) without ever seeing it. The app contract is now version 2: one `handle(state, event)` for everything that happens (a fresh start, every open, the app's messages, Rapidkoi's answers), so an app reopened from its saved state still asks who you are. The Rust library and the counter use it; v1 apps no longer open.
