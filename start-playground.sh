#!/bin/bash

# build wasm functions
echo "build wasm..."
cd crates/wasm
wasm-pack build -t web -d ../../lang-playground/wasm --release
cd ../..

# install npm packages
echo "install npm packages..."
cd lang-playground
npm install
cd ..

# start server
echo "start server..."
cargo run --bin server --release

# open `localhost:3000/index.html` in your browser
