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
