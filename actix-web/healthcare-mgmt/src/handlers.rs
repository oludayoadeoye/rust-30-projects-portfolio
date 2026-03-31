use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::*;

// --- Patient Handlers ---

#[utoipa::path(
    get,
    path = "/patients",
    responses(
        (status = 200, description = "List all patients", body = [Patient])
    )
)]
#[get("/patients")]
pub async fn list_patients(pool: web::Data<PgPool>) -> impl Responder {
    let patients = sqlx::query_as::<_, Patient>("SELECT * FROM patients ORDER BY name")
        .fetch_all(pool.get_ref())
        .await;

    match patients {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching patients"),
    }
}

#[utoipa::path(
    post,
    path = "/patients",
    request_body = CreatePatient,
    responses(
        (status = 201, description = "Patient created", body = Patient)
    )
)]
#[post("/patients")]
pub async fn create_patient(pool: web::Data<PgPool>, payload: web::Json<CreatePatient>) -> impl Responder {
    let patient = sqlx::query_as::<_, Patient>(
        "INSERT INTO patients (id, name, date_of_birth, gender, contact_number) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(payload.date_of_birth)
    .bind(&payload.gender)
    .bind(&payload.contact_number)
    .fetch_one(pool.get_ref())
    .await;

    match patient {
        Ok(p) => HttpResponse::Created().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error creating patient"),
    }
}

// --- Appointment Handlers ---

#[utoipa::path(
    get,
    path = "/patients/{id}/appointments",
    responses(
        (status = 200, description = "List appointments for a patient", body = [Appointment])
    )
)]
#[get("/patients/{id}/appointments")]
pub async fn list_appointments(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let appointments = sqlx::query_as::<_, Appointment>("SELECT * FROM appointments WHERE patient_id = $1 ORDER BY appointment_date ASC")
        .bind(id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match appointments {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching appointments"),
    }
}

#[utoipa::path(
    post,
    path = "/patients/{id}/appointments",
    request_body = CreateAppointment,
    responses(
        (status = 201, description = "Appointment scheduled", body = Appointment)
    )
)]
#[post("/patients/{id}/appointments")]
pub async fn schedule_appointment(pool: web::Data<PgPool>, id: web::Path<Uuid>, payload: web::Json<CreateAppointment>) -> impl Responder {
    let appointment = sqlx::query_as::<_, Appointment>(
        "INSERT INTO appointments (patient_id, appointment_date, reason) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id.into_inner())
    .bind(payload.appointment_date)
    .bind(&payload.reason)
    .fetch_one(pool.get_ref())
    .await;

    match appointment {
        Ok(a) => HttpResponse::Created().json(a),
        Err(_) => HttpResponse::InternalServerError().body("Error scheduling appointment"),
    }
}
