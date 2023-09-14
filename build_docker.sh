#!/bin/bash

cargo build --bin server --target wasm32-wasi --release

docker build -t wasm-http-server .