#!/bin/bash

# build
./build-playground.sh

# deploy server
echo "deploy server ..."
cargo shuttle deploy --name wdl-playground --allow-dirty

# open `wdl-playground.shuttleapp.rs` in your browser
