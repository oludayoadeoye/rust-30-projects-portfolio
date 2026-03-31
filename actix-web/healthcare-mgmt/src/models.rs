use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Patient {
    pub id: Uuid,
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub contact_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePatient {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub contact_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Appointment {
    pub id: i32,
    pub patient_id: Uuid,
    pub appointment_date: DateTime<Utc>,
    pub reason: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAppointment {
    pub appointment_date: DateTime<Utc>,
    pub reason: String,
}
