FROM scratch

COPY target/wasm32-wasi/release/server.wasm /

CMD ["server.wasm"]