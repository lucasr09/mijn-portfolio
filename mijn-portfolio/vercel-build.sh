#!/bin/sh
set -e

# Vercel's build-container zet $HOME op /vercel, terwijl de build als root
# draait (echte home is /root). rustup's installer ziet die mismatch als een
# mogelijk sudo-probleem en weigert dan te installeren - dus expliciet gelijk
# zetten voordat rustup draait.
export HOME=/root

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall trunk --no-confirm

trunk build --release
