use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/devices",
    responses(
        (status = 200, description = "List devices", body = [Device])
    )
)]
#[get("/devices")]
pub async fn list_devices(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let devices = sqlx::query_as::<_, Device>("SELECT * FROM devices")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(devices))
}

#[utoipa::path(
    post,
    path = "/devices",
    request_body = UpdateDevice,
    responses(
        (status = 200, description = "Device updated", body = Device)
    )
)]
#[post("/devices")]
pub async fn update_device(pool: web::Data<PgPool>, payload: web::Json<UpdateDevice>) -> Result<impl Responder, ApiError> {
    let device = sqlx::query_as::<_, Device>(
        "INSERT INTO devices (name, type_name, status, last_seen) 
         VALUES ($1, $2, $3, CURRENT_TIMESTAMP) 
         ON CONFLICT (name) DO UPDATE 
         SET status = EXCLUDED.status, last_seen = CURRENT_TIMESTAMP 
         RETURNING *"
    )
    .bind(&payload.name)
    .bind("Security Sensor")
    .bind(&payload.status)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(device))
}
