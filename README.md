# VOLO-HTTP-WASI

Migrate [volo-http](https://github.com/cloudwego/volo/tree/http) to WASI

## Coding

Cargo.toml
```toml
tokio_wasi = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
http = { version = "0.2" }
volo-http-wasi = { git = "ssh://git@github.com/lz1998/volo-http-wasi.git", branch = "main"}
```

src/main.rs
```rust
use std::net::SocketAddr;

use http::{Method, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use volo_http_wasi::{
    handler::HandlerService,
    request::Json,
    route::{Route, Router, Server},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Router::new()
        .route(
            "/ping",
            Route::builder()
                .post(HandlerService::new(json))
                .get(HandlerService::new(ping))
                .build(),
        )
        .serve(SocketAddr::from(([127, 0, 0, 1], 3000)))
        .await
        .unwrap();
}

async fn ping(u: Uri, m: Method) -> &'static str {
    println!("{m:?} {u:?}");
    "pong"
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

async fn json(Json(mut req): Json<Person>) -> Result<Json<Person>, (StatusCode, &'static str)> {
    if !req.phones.is_empty() {
        println!("{req:?}");
        req.name.push('a');
        req.phones.push(String::from("123"));
        Ok(Json(req))
    } else {
        Err((StatusCode::BAD_REQUEST, "phones is empty"))
    }
}
```


## Run Locally

1. [Install WasmEdge](https://wasmedge.org/docs/start/install/)
2. `rustup target add wasm32-wasi`
3. Compile: `cargo build --target wasm32-wasi --release`
4. Run: `wasmedge ./target/wasm32-wasi/release/server.wasm`


## Run in Docker

1. Update Docker to the latest version, and open the following options in `Settings` - `Features in development`
   - [x] `Use containerd for pulling and storing images`
   - [x] `Enable Wasm`
2. Build docker image
    ```bash
    cargo build --target wasm32-wasi --release
    
    docker build -t wasm-http-server .
    ```
3. Run docker image
    ```bash
    docker run --rm -d -p 3000:3000 \
        --name=wasm-http-server \
        --runtime=io.containerd.wasmedge.v1 \
        wasm-http-server
    ```
   
## Test
```bash
curl http://localhost:3000/ping

curl --location 'http://localhost:3000/ping' \
--header 'Content-Type: application/json' \
--data '{
    "name": "a",
    "age": 1,
    "phones": [
        "a"
    ]
}'
```
