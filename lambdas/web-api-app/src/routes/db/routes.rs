use crate::routes::db::db_function::{get_days, get_months, get_objects, get_years};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

/// This is the root route of the bd resource.
pub fn db_routes() -> Router {
    let bucket_route = Router::new()
        .route("/videos", get(years_videos_handler))
        .route("/videos/years/:year/months", get(months_videos_handler))
        .route(
            "/videos/years/:year/months/:month/days",
            get(days_videos_handler),
        )
        .route(
            "/videos/years/:year/months/:month/days/:day/objects",
            get(get_objects_handler),
        );

    bucket_route
}

/// The wrapper of the get year
// async fn videos_handler() -> (StatusCode, Json<serde_json::Value>) {
async fn years_videos_handler() -> impl IntoResponse {
    match get_years().await {
        Ok(years) => (StatusCode::OK, Json(json!(years))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// The wrapper of the get_months
async fn months_videos_handler(Path(year): Path<usize>) -> impl IntoResponse {
    match get_months(year).await {
        Ok(months) => (StatusCode::OK, Json(json!(months))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// The wrapper of the get_days
async fn days_videos_handler(Path((year, month)): Path<(usize, usize)>) -> impl IntoResponse {
    match get_days(year, month).await {
        Ok(days) => (StatusCode::OK, Json(json!(days))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// The wrapper of the get_objects
async fn get_objects_handler(
    Path((year, month, day)): Path<(usize, usize, usize)>,
) -> impl IntoResponse {
    match get_objects(year, month, day).await {
        Ok(video_objects) => (StatusCode::OK, Json(json!(video_objects))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
