#!/bin/sh
# Builds Rapidkoi's Settings app (guest/rust/settings) and packs it,
# unsigned, into assets/apps/settings.rkb (PRP-038, PRP-075), declaring the
# built-in permissions it needs: only a bundle built into ./main may hold
# them. Run from the repository's root, after `lughac build -O 2 src/main.la -o
# main`. The .rkb is committed, so building ./main never needs cargo.
# Rebuild ./main after: it embeds the bundle. Its icon (every app has one,
# PRP-060) is guest/rust/settings/icon.svg. Rapidkoi's version, from
# lugha.toml, is built into About and given to the bundle.
set -eu
version=$(sed -n 's/^version = "\(.*\)"/\1/p' lugha.toml | head -n 1)
a=guest/rust/settings/assets
RAPIDKOI_VERSION="$version" cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p settings
./main pack guest/rust/target/wasm32-unknown-unknown/release/settings.wasm \
    --name "Settings" --version "$version" --icon guest/rust/settings/icon.svg --color primary=#71717a \
    --asset brush.svg=$a/brush.svg --asset startup.svg=$a/startup.svg --asset user.svg=$a/user.svg \
    --asset shield.svg=$a/shield.svg --asset info.svg=$a/info.svg \
    --asset search.svg=$a/search.svg \
    --permission change_settings --permission read_you --permission read_publishers \
    --control address=settings \
    --control settings=app_settings.permissions --control settings=app_settings.recent \
    -o assets/apps/settings.rkb --force
