use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/processes",
    responses(
        (status = 200, description = "List all processes", body = [Process])
    )
)]
#[get("/processes")]
pub async fn list_processes(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let processes = sqlx::query_as::<_, Process>("SELECT * FROM processes ORDER BY name")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(processes))
}

#[utoipa::path(
    post,
    path = "/processes",
    request_body = CreateProcess,
    responses(
        (status = 201, description = "Process created", body = Process)
    )
)]
#[post("/processes")]
pub async fn create_process(pool: web::Data<PgPool>, payload: web::Json<CreateProcess>) -> Result<impl Responder, ApiError> {
    let process = sqlx::query_as::<_, Process>(
        "INSERT INTO processes (name, status, cpu_usage, memory_usage) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.status)
    .bind(0.0)
    .bind(0.0)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(process))
}
