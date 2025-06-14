#!/usr/bin/env bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup update stable
pip install dstack_sdk 

cargo build --release --target-dir ./build
./build/release/submitter $1 $2 $3