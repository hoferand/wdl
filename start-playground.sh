#!/bin/bash

# build wasm functions
cd crates/wasm
wasm-pack build -t web -d ../../lang-playground/wasm --release
cd ../..

# install npm packages
cd lang-playground
npm install
cd ..

# start server
cargo run --bin server

# open `localhost:3000/index.html` in your browser
