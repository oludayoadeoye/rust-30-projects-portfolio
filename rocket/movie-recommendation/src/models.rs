use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub genre: String,
    pub release_year: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMovie {
    pub title: String,
    pub genre: String,
    pub release_year: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Rating {
    pub id: i32,
    pub movie_id: i32,
    pub user_id: Uuid,
    pub score: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRating {
    pub user_id: Uuid,
    pub score: i32,
    pub comment: Option<String>,
}
