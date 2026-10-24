#!/bin/sh
# Builds Rapidkoi's Settings app (guest/rust/settings) and packs it,
# unsigned, into assets/apps/settings.rkb (PRP-038), declaring the
# built-in permission it needs: only a bundle built into ./main may hold
# it. Run from the repository's root, after `lughac build src/main.la -o
# main`. The .rkb is committed, so building ./main never needs cargo.
# Rebuild ./main after: it embeds the bundle.
set -eu
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p settings
./main pack guest/rust/target/wasm32-unknown-unknown/release/settings.wasm \
    --name "Settings" --version 0.1.0 \
    --permission change_settings \
    --control address=settings \
    --control settings=app_settings.permissions \
    -o assets/apps/settings.rkb --force
