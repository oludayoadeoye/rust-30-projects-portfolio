use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Workout {
    pub id: i32,
    pub name: String,
    pub workout_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWorkout {
    pub name: String,
    pub workout_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Exercise {
    pub id: i32,
    pub workout_id: i32,
    pub name: String,
    pub sets: i32,
    pub reps: i32,
    pub weight_kg: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExercise {
    pub name: String,
    pub sets: i32,
    pub reps: i32,
    pub weight_kg: Option<f64>,
}
