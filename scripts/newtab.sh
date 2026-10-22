#!/bin/sh
# Builds Rapidkoi's new-tab app (guest/rust/newtab) and packs it, unsigned,
# into assets/apps/newtab.rkb (PRP-027). Run from the repository's root,
# after `lughac build src/main.la -o main`. The .rkb is committed, so
# building ./main never needs cargo.
set -eu
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p newtab
./main pack guest/rust/target/wasm32-unknown-unknown/release/newtab.wasm \
    --name "New tab" --version 0.1.0 -o assets/apps/newtab.rkb --force
