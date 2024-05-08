#!/bin/bash

# build docu
echo "build docu ..."
cd lang-doc
mdbook build
cd ..

# build wasm functions
echo "build wasm ..."
cd crates/wasm
wasm-pack build -t web -d ../../lang-playground/wasm --release
cd ../..

# install npm packages
echo "install npm packages ..."
cd lang-playground
npm install
cd ..
