#!/bin/bash

#rustup target add wasm32-wasi

cargo build --bin server --target wasm32-wasi --release