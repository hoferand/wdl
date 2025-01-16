#!/bin/bash

# build
./build-playground.sh

# deploy server
echo "deploy server ..."
shuttle deploy --name wdl-playground

# open `wdl-playground-dt9y.shuttle.app` in your browser
