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
