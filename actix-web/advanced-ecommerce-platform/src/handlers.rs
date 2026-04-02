use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/products",
    responses(
        (status = 200, description = "List all products", body = [Product])
    )
)]
#[get("/products")]
pub async fn list_products(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY name")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = CreateProduct,
    responses(
        (status = 201, description = "Product created", body = Product)
    )
)]
#[post("/products")]
pub async fn create_product(pool: web::Data<PgPool>, payload: web::Json<CreateProduct>) -> Result<impl Responder, ApiError> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (id, name, description, price, stock_quantity) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(payload.stock_quantity)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(product))
}

#[utoipa::path(
    post,
    path = "/orders",
    request_body = CreateOrder,
    responses(
        (status = 201, description = "Order created", body = Order)
    )
)]
#[post("/orders")]
pub async fn create_order(pool: web::Data<PgPool>, payload: web::Json<CreateOrder>) -> Result<impl Responder, ApiError> {
    let mut total = 0.0;
    // Simple logic to calculate total (in real app, fetch prices from DB)
    for item in &payload.items {
        total += 100.0 * item.quantity as f64; // Placeholder price
    }

    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.customer_id)
    .bind(total)
    .bind("Pending")
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(order))
}
