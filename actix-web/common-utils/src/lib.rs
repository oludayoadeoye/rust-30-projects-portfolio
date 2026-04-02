use sqlx::postgres::{PgPool, PgPoolOptions};
use actix_web::{
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
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
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    Internal,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::json())
            .json(json!({ "error": self.to_string() }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
