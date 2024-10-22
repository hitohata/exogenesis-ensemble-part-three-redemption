use axum::{Json, Router};
use axum::routing::get;
use lambda_http::{run, tracing, Error};
use serde_json::{json, Value};

fn greet() -> Json<Value> {
    return Json(json!({"body": "hello world"}))
}

/// This project uses the [Axum](https://docs.rs/axum/latest/axum/).
/// The way of adoption of Axum refers to [this repo](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/http-axum/src/main.rs).
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(greet));

    run(app)
}
