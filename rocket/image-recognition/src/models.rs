use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ImageAnalysis {
    pub id: i32,
    pub image_url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadImage {
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ImageTag {
    pub id: i32,
    pub analysis_id: i32,
    pub label: String,
    pub confidence: f64,
}
