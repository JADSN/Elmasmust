#!/bin/bash

echo "==============="
echo "SETUP"
echo "==============="

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
sudo apt-get install binaryen wabt -y
cargo install wasm-snip
cargo install just