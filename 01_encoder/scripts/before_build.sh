#!/bin/bash

echo "==============="
echo "BEFORE BUILD"
echo "==============="

# * Create pkg folder in dist directory
mkdir -p ./dist/pkg

# * Copy index.html to dist directory
cp -v ./index.html ./dist/

# * Copy scripts to dist directory
cp -v ./main.js ./dist/
cp -v ./functions.js ./dist/

# * Copy styles to dist directory
cp -v ./main.css ./dist/

# * Copy favicon.ico to dist directory
cp -v ./favicon.ico ./dist/