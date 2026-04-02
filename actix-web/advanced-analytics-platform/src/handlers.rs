use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/datasets",
    responses(
        (status = 200, description = "List datasets", body = [Dataset])
    )
)]
#[get("/datasets")]
pub async fn list_datasets(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let datasets = sqlx::query_as::<_, Dataset>("SELECT * FROM datasets")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(datasets))
}

#[utoipa::path(
    post,
    path = "/datasets",
    request_body = CreateDataset,
    responses(
        (status = 201, description = "Dataset created", body = Dataset)
    )
)]
#[post("/datasets")]
pub async fn create_dataset(pool: web::Data<PgPool>, payload: web::Json<CreateDataset>) -> Result<impl Responder, ApiError> {
    let dataset = sqlx::query_as::<_, Dataset>(
        "INSERT INTO datasets (name, row_count, status) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.name)
    .bind(payload.row_count)
    .bind("Processing")
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(dataset))
}
