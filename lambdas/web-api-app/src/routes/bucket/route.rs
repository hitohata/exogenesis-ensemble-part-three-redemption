//! This is the route of the bucket resource.

use crate::routes::bucket::bucket_function::{get_days, get_months, get_objects, get_years};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use axum::extract::Path;
use serde_json::json;

/// This is the root route of the bucket resource.
pub fn bucket_routes() -> Router {
    let bucket_route = Router::new()
        .route(
            "/videos",
            get(Json(videos_handler)),
        )
        .route(
            "/videos/years/:year/months",
            get(Json(months_videos_handler)),
        )
        .route("/videos/years/:year/months/:month/days", get(Json(days_videos_handler)))
        .route(
            "/videos/years/:year/months/:month/days/:day/objects",
            get(Json(get_objects_handler)),
        );

    bucket_route
}

/// The wrapper of the get year
async fn videos_handler(Path(path): Path<String>) -> (StatusCode, serde_json::Value) {
    match get_years().await {
        Ok(years) => {
            (
                StatusCode::OK,
                json!(years)
            )
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                json!({"error": e})
            )
        }
    }
}

/// The wrapper of the get_months
async fn months_videos_handler(Path(year): Path<usize>) -> (StatusCode, serde_json::Value) {
    match get_months(year).await {
        Ok(months) => {
            (
                StatusCode::OK,
                json!(months)
            )
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                json!({"error": e})
            )
        }
    }
}

/// The wrapper of the get_days
async fn days_videos_handler(Path((year, month)): Path<(usize, usize)>) -> (StatusCode, serde_json::Value) {
    match get_days(year, month).await {
        Ok(days) => {
            (
                StatusCode::OK,
                json!(days)
            )
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                json!({"error": e})
            )
        }
    }
}

/// The wrapper of the get_objects
async fn get_objects_handler(Path((year, month, day)): Path<(usize, usize, usize)>) -> (StatusCode, serde_json::Value) {
    match get_objects(year, month, day).await {
        Ok(video_objects) => {
            (
                StatusCode::OK,
                json!(video_objects)
            )
        }
        Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                json!({"error": e})
            )
        }
    }
}
