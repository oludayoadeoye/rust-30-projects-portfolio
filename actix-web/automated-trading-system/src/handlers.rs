use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/trades",
    responses(
        (status = 200, description = "List trades", body = [Trade])
    )
)]
#[get("/trades")]
pub async fn list_trades(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let trades = sqlx::query_as::<_, Trade>("SELECT * FROM trades ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(trades))
}

#[utoipa::path(
    post,
    path = "/trades",
    request_body = CreateTrade,
    responses(
        (status = 201, description = "Trade created", body = Trade)
    )
)]
#[post("/trades")]
pub async fn create_trade(pool: web::Data<PgPool>, payload: web::Json<CreateTrade>) -> Result<impl Responder, ApiError> {
    let trade = sqlx::query_as::<_, Trade>(
        "INSERT INTO trades (symbol, quantity, price, side) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.symbol)
    .bind(payload.quantity)
    .bind(150.0) // Simulated price
    .bind(&payload.side)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(trade))
}
