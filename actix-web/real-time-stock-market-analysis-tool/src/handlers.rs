use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/stocks",
    responses(
        (status = 200, description = "List stocks", body = [Stock])
    )
)]
#[get("/stocks")]
pub async fn list_stocks(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let stocks = sqlx::query_as::<_, Stock>("SELECT * FROM stocks ORDER BY symbol")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(stocks))
}

#[utoipa::path(
    post,
    path = "/stocks",
    request_body = UpdateStock,
    responses(
        (status = 200, description = "Stock updated", body = Stock)
    )
)]
#[post("/stocks")]
pub async fn update_stock(pool: web::Data<PgPool>, payload: web::Json<UpdateStock>) -> Result<impl Responder, ApiError> {
    let stock = sqlx::query_as::<_, Stock>(
        "INSERT INTO stocks (symbol, price, change_percent, last_updated) 
         VALUES ($1, $2, $3, CURRENT_TIMESTAMP) 
         ON CONFLICT (symbol) DO UPDATE 
         SET price = EXCLUDED.price, change_percent = EXCLUDED.change_percent, last_updated = CURRENT_TIMESTAMP 
         RETURNING *"
    )
    .bind(&payload.symbol)
    .bind(payload.price)
    .bind(payload.change_percent)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(stock))
}
