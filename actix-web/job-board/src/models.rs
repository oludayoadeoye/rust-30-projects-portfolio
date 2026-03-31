use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub description: String,
    pub salary_range: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJob {
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub description: String,
    pub salary_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Applicant {
    pub id: i32,
    pub job_id: i32,
    pub name: String,
    pub email: String,
    pub resume_url: Option<String>,
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateApplicant {
    pub name: String,
    pub email: String,
    pub resume_url: Option<String>,
}
