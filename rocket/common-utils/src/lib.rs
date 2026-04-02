use sqlx::postgres::{PgPool, PgPoolOptions};
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

// In Rocket, we typically handle errors via Catcher or Responder.
// This is a simplified version for common use.
