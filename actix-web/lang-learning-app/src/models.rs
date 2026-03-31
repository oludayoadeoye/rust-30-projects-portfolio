use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Vocabulary {
    pub id: i32,
    pub word: String,
    pub translation: String,
    pub language: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVocabulary {
    pub word: String,
    pub translation: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub difficulty: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLesson {
    pub title: String,
    pub content: String,
    pub difficulty: Option<i32>,
}
