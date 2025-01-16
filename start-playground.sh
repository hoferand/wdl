#!/bin/bash

# build
./build-playground.sh

# deploy server
echo "start server ..."
shuttle run

# open `127.0.0.1:8000` in your browser
