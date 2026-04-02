use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ARScene {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateScene {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Anchor {
    pub id: i32,
    pub scene_id: i32,
    pub spatial_data: serde_json::Value,
    pub label: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAnchor {
    pub spatial_data: serde_json::Value,
    pub label: String,
}
