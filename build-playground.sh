#!/bin/bash

# build docu
echo "build docu ..."
cd doc
mdbook build
cd ..

# build wasm functions
echo "build wasm ..."
cd crates/wasm
wasm-pack build -t web -d ../../wdl-playground-ui/wasm --release
cd ../..

# install npm packages
echo "install npm packages ..."
cd wdl-playground-ui
npm install
cd ..
