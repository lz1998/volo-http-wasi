#!/bin/bash

docker run --rm -d -p 3000:3000 --name=wasm-http-server --runtime=io.containerd.wasmedge.v1 wasm-http-server