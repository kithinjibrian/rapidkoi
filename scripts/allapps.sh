#!/bin/sh
# Builds Rapidkoi's All apps (guest/rust/allapps, PRP-067) and packs it,
# unsigned, into assets/apps/allapps.rkb, declaring the built-in permissions
# it needs: read_apps, open_apps and manage_pins. Only a bundle built into
# ./main may hold them. Run from the repository's root, after
# `lughac build -O 2 src/main.la -o main`; rebuild ./main after: it embeds
# the bundle. Its icon (every app has one, PRP-060) is guest/rust/allapps/icon.svg.
set -eu
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p allapps
./main pack guest/rust/target/wasm32-unknown-unknown/release/allapps.wasm \
    --name "All apps" --version 0.1.0 --icon guest/rust/allapps/icon.svg --color primary=#ff7700 \
    --asset search.svg=guest/rust/allapps/assets/search.svg \
    --permission read_apps --permission open_apps --permission manage_pins \
    --control address=settings --control settings=app_settings.permissions \
    -o assets/apps/allapps.rkb --force
