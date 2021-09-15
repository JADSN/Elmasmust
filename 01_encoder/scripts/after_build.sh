#!/bin/bash

# PACKAGE_NAME=$(awk 'NR == 2 {print $0}' ./Cargo.toml | cut -d " " -f 3 | tr '"' " ")
PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")

echo "==============="
echo "AFTER BUILD"
echo "==============="

cp -v ./pkg/${PACKAGE_NAME}_bg.wasm ./dist/pkg
cp -v ./pkg/${PACKAGE_NAME}.js ./dist/pkg

# Optimize for size.
# wasm-opt -Os ./pkg/${PACKAGE_NAME}_bg.wasm -o ./dist/pkg/${PACKAGE_NAME}_bg.wasm

# Optimize aggressively for size.
wasm-opt -Oz ./pkg/${PACKAGE_NAME}_bg.wasm -o ./dist/pkg/${PACKAGE_NAME}_bg.wasm

# Optimize for speed.
# wasm-opt -O ./pkg/${PACKAGE_NAME}_bg.wasm -o ./dist/pkg/${PACKAGE_NAME}_bg.wasm

# Optimize aggressively for speed.
# wasm-opt -O3 ./pkg/${PACKAGE_NAME}_bg.wasm -o ./dist/pkg/${PACKAGE_NAME}_bg.wasm"

# @wasm-strip ./dist/pkg/${PACKAGE_NAME}_bg.wasm
# @wasm-opt -Oz ./pkg/${PACKAGE_NAME}_bg.wasm -o ./dist/pkg/${PACKAGE_NAME}_bg.wasm

wasm-strip ./dist/pkg/${PACKAGE_NAME}_bg.wasm
