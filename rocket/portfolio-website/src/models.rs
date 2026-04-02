use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub link: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProject {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub level: String, // 'Beginner', 'Intermediate', 'Expert'
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSkill {
    pub name: String,
    pub level: String,
}
