use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Recording {
    pub id: i32,
    pub transcript: String,
    pub language: String,
    pub duration_seconds: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRecording {
    pub transcript: String,
    pub language: String,
    pub duration_seconds: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecognizeRequest {
    pub audio_base64: String,
    pub language: Option<String>,
}
