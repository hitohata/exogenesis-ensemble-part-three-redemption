use axum::Router;
use axum::routing::get;

pub fn bucket_routes() -> Router {
    Router::new()
        .route("/id", get(|| async {}))
}