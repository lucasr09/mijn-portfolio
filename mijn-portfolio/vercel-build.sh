#!/bin/sh
set -e

# Vercel's build-container zet $HOME op /vercel, terwijl de build als root
# draait (echte home is /root). rustup's installer ziet die mismatch als een
# mogelijk sudo-probleem en weigert dan te installeren - dus expliciet gelijk
# zetten voordat rustup draait. CARGO_HOME/RUSTUP_HOME ook hardcoden, want
# Vercel's image heeft die zelf al op iets anders (/rust) staan, waardoor
# rustup daar installeert i.p.v. op het pad dat we hierna verwachten.
export HOME=/root
export CARGO_HOME=/root/.cargo
export RUSTUP_HOME=/root/.rustup

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal
. "$CARGO_HOME/env"
rustup target add wasm32-unknown-unknown

# cargo-binstall downloadt een kant-en-klare trunk-binary, maar die bleek
# gelinkt tegen een nieuwere glibc dan Vercel's build-image heeft
# ("GLIBC_2.35 not found"). Compileer Trunk daarom gewoon vanaf source -
# kost wat meer tijd, maar kan nooit meer een glibc-mismatch geven.
cargo install trunk --locked

trunk build --release
