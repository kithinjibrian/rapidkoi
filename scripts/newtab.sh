#!/bin/sh
# Builds Rapidkoi's new-tab app (guest/rust/newtab) and packs it, unsigned,
# into assets/apps/newtab.rkb (PRP-027), declaring the built-in permissions
# it needs (PRP-030) and the frame it shows (PRP-031): only a bundle built
# into ./main may hold them. Run from the repository's root, after
# `lughac build -O 2 src/main.la -o main`. The .rkb is committed, so building
# ./main never needs cargo. Rebuild ./main after: it embeds the bundle.
set -eu
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p newtab
./main pack guest/rust/target/wasm32-unknown-unknown/release/newtab.wasm \
    --name "New tab" --version 0.1.0 \
    --permission read_pins --permission read_left_off --permission open_apps --permission restore_session \
    --control address=settings --control address=share \
    --control settings=app_settings.permissions \
    -o assets/apps/newtab.rkb --force
