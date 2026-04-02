use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/drones",
    responses(
        (status = 200, description = "List drones", body = [Drone])
    )
)]
#[get("/drones")]
pub async fn list_drones(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let drones = sqlx::query_as::<_, Drone>("SELECT * FROM drones")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(drones))
}

#[utoipa::path(
    post,
    path = "/drones",
    request_body = RegisterDrone,
    responses(
        (status = 201, description = "Drone registered", body = Drone)
    )
)]
#[post("/drones")]
pub async fn register_drone(pool: web::Data<PgPool>, payload: web::Json<RegisterDrone>) -> Result<impl Responder, ApiError> {
    let drone = sqlx::query_as::<_, Drone>(
        "INSERT INTO drones (drone_id, status, battery_level) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.drone_id)
    .bind("Inactive")
    .bind(100.0)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(drone))
}
