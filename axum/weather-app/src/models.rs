use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WeatherReport {
    pub id: i32,
    pub city: String,
    pub temperature: f64,
    pub condition: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWeatherReport {
    pub city: String,
    pub temperature: f64,
    pub condition: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWeatherReport {
    pub city: Option<String>,
    pub temperature: Option<f64>,
    pub condition: Option<String>,
}
