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
