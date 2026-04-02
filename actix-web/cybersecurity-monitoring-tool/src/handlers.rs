use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/alerts",
    responses(
        (status = 200, description = "List alerts", body = [Alert])
    )
)]
#[get("/alerts")]
pub async fn list_alerts(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let alerts = sqlx::query_as::<_, Alert>("SELECT * FROM alerts ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(alerts))
}

#[utoipa::path(
    post,
    path = "/alerts",
    request_body = CreateAlert,
    responses(
        (status = 201, description = "Alert created", body = Alert)
    )
)]
#[post("/alerts")]
pub async fn create_alert(pool: web::Data<PgPool>, payload: web::Json<CreateAlert>) -> Result<impl Responder, ApiError> {
    let alert = sqlx::query_as::<_, Alert>(
        "INSERT INTO alerts (severity, message, source_ip) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.severity)
    .bind(&payload.message)
    .bind(&payload.source_ip)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(alert))
}
