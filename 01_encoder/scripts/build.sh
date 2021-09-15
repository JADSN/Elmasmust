#!/bin/bash

echo "==============="
echo "BUILD"
echo "==============="

wasm-pack build --target no-modules
# @wasm-pack build -- --target no-modules --features wee_alloc