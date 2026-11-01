#!/bin/sh
# Builds Rapidkoi's new-tab app (guest/rust/newtab) and packs it, unsigned,
# into assets/apps/newtab.rkb (PRP-027), declaring the built-in permissions
# it needs (PRP-030) and the frame it shows (PRP-031): only a bundle built
# into ./main may hold them. Run from the repository's root, after
# `lughac build -O 2 src/main.la -o main`. The .rkb is committed, so building
# ./main never needs cargo. Rebuild ./main after: it embeds the bundle.
# Its pictures are guest/rust/newtab/assets/, each packed under its name.
set -eu
cargo build --manifest-path guest/rust/Cargo.toml --release --target wasm32-unknown-unknown -p newtab
a=guest/rust/newtab/assets
./main pack guest/rust/target/wasm32-unknown-unknown/release/newtab.wasm \
    --name "New tab" --version 0.1.0 \
    --asset wordmark.svg=$a/wordmark.svg --asset restore-apps.svg=$a/restore-apps.svg \
    --asset face-amina.svg=$a/face-amina.svg --asset face-wanjiku.svg=$a/face-wanjiku.svg \
    --asset update-notes.svg=$a/update-notes.svg --asset notice-notes.svg=$a/notice-notes.svg \
    --asset beach.svg=$a/beach.svg --asset wash-notes.svg=$a/wash-notes.svg \
    --asset wash-counter.svg=$a/wash-counter.svg --asset wash-settings.svg=$a/wash-settings.svg \
    --asset notes.svg=$a/notes.svg --asset counter.svg=$a/counter.svg \
    --asset trip.svg=$a/trip.svg --asset settings.svg=$a/settings.svg \
    --asset wash-fill.svg=$a/wash-fill.svg \
    --permission read_pins --permission read_left_off --permission open_apps --permission restore_session \
    --control address=settings --control address=share \
    --control settings=app_settings.permissions \
    -o assets/apps/newtab.rkb --force
