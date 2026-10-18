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
