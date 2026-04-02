use sqlx::postgres::{PgPool, PgPoolOptions};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub async fn init_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to Postgres")
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
