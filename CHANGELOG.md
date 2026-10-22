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
- Apps reaching the network: a signed app can make HTTPS requests to its own host and the servers it declares (`[network] servers` in `rapidkoi.toml`, `pack --server`), signed into its manifest. They're sent in the background, at most 4 at once, with no cookies kept and no redirects followed; anything else is refused before it leaves the computer, with a plain reason the app gets back. Plain HTTP reaches only this computer, and only for apps from it. The Access page lists each server with a switch, marks ones an update added as new, and an update that adds one says so once. Apps can sign in to their own server with the person's key (`docs/server-sign-in.md`; the Rust library's `SignIn`). Rapidkoi looks up every server name itself, both for an app's requests and for opening an app at an address. A name that points to this computer or the local network (private, link-local, shared ISP space, multicast and reserved ranges; IPv4 hidden in IPv6 included) is refused with a plain reason before anything is sent. Connections go only to the addresses it checked.

### Changed
- The redesign begins (on the `redesign` branch, DECISION-027): the window shows only the bare frame, an empty header bar with the window controls, an empty sidebar on the right and an empty content area (PRP-014). Tabs, the bar, home and the info panel are hidden until later redesign PRPs bring each back, in the frame or as a bundled app.
- The address box (PRP-015): a rounded field centred in the header bar, "Enter Address" on home, the app's address and trust marker on an app (click or Ctrl+K to type). While typing, a card drops under it with what you typed, then "Rapidkoi suggest" with the open tabs, pins and where-you-left-off that match; Enter or a suggestion opens it, Escape or a click outside closes it, and a failed open says why in the card. Suggestions are local only. The content area shows the current app again.
- Account and menu icons at the header bar's right (PRP-016), greyed out with "coming soon" tooltips until each gets its own PRP.
- kivuli 0.18 (PRP-017): every colour comes from Rapidkoi's theme, apps' content included, never the desktop's light or dark theme (DECISION-031). Light text, Rapidkoi's own buttons (normal, primary, danger and subtle, each with hover and pressed colours), fields whose edge turns blue while focused, and a header bar in the frame's colour. Apps' style words are unchanged.
- The old view is gone from the code (DECISION-030): the bar, tab rail, home, and the info panel with its Account and Access pages. Their logic stays; the info panel, Account and Access come back as native screens in later redesign PRPs.
- Rapidkoi's modules are grouped into folders (PRP-018): `window/`, `ui/`, `guest/`, `bundles/`, `trust/`, `net/`, `data/`, `people/` and `commands/`, with `main` and `shell` at the top; `theme` is `ui/tokens`. No change in behaviour.
- kivuli 0.48. The header bar redesigned on `kivuli.ui` (PRP-019): ← back, ⌂ home, → forward, the address box as one framed pill with a shield for a verified or trusted signed app and ☆ to pin, then account and menu; each a ghost icon button with Rapidkoi's own Iconsax icons. No reload. Rapidkoi's colours become a dark `kivuli.ui` palette.
- One tab per app (PRP-026): opening an app that's already open shows its tab instead of a second copy; there's at most one empty tab, and + goes to it.
- One address field on every tab (PRP-025): on an app it shows the address and you type straight in; share (greyed until links) at its end, then pin on apps. Ctrl+K is gone for now.
- Empty tabs (PRP-024): + adds an empty tab to the sidebar, and opening an address, a pin or "where you left off" from one fills it in place. The window starts with one, and closing the last tab leaves one. ⌂ is now the app's own home screen, disabled until links.
- The sidebar (PRP-023): pinned apps, a line, the open apps, then + (Ctrl+T). The current app is ringed in orange with a × (Ctrl+W); every app has a right-click menu (Close, Pin or Unpin, Forget).
- Site settings (PRP-022): ⚙ opens a card with the app (icon, name, version, description) and its sections: Account (Public key; Identity: your name, sign in or out, sign in automatically, start over as someone new), App settings (Servers, a switch each), App data (State, Clear and start over), and Forget this app. Destructive actions ask first in a dialog.
- The shield (PRP-021, kivuli 0.51): on every app page, white when verified, orange for caution, grey when unsigned. Clicking it opens a card with the app, "Connection secure ›" and "Signed app ›", each a page saying how Rapidkoi reached the app, who signed it, since when, and its version.
- A site settings button in the address box, right of the shield on every app page, greyed out until the settings exist (PRP-020).
- The address box's suggestions card is removed until its redesign; typing no longer shows suggestions, and Escape no longer clears the box.
- kivuli 0.49: the window controls (− □ ×) take Rapidkoi's palette as ghost buttons, muted until hovered, close red on hover.
- Rapidkoi's brand colour, `#FF7700`, is the accent: primary buttons, focus rings and a focused field's edge are orange, with dark text on orange.
- The header bar and sidebar stay flat `#242424`, with white icons and window controls (a brand gradient, orange `#FF7700` into pink `#FF2E63`, was tried and set aside).
