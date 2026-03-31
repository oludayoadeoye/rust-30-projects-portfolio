use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Product Handlers ---

#[utoipa::path(
    get,
    path = "/products",
    responses(
        (status = 200, description = "List all products", body = [Product])
    )
)]
#[get("/products")]
pub async fn list_products(pool: web::Data<PgPool>) -> impl Responder {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY sku")
        .fetch_all(pool.get_ref())
        .await;

    match products {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching products"),
    }
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
pub async fn create_product(pool: web::Data<PgPool>, payload: web::Json<CreateProduct>) -> impl Responder {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (sku, name, quantity, price) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.sku)
    .bind(&payload.name)
    .bind(payload.quantity)
    .bind(payload.price)
    .fetch_one(pool.get_ref())
    .await;

    match product {
        Ok(p) => HttpResponse::Created().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error creating product"),
    }
}

#[utoipa::path(
    put,
    path = "/products/{id}/stock",
    request_body = UpdateStock,
    responses(
        (status = 200, description = "Stock updated", body = Product),
        (status = 404, description = "Product not found")
    )
)]
#[put("/products/{id}/stock")]
pub async fn update_stock(pool: web::Data<PgPool>, id: web::Path<i32>, payload: web::Json<UpdateStock>) -> impl Responder {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    // 1. Update product quantity
    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET quantity = quantity + $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(payload.amount)
    .bind(id.into_inner())
    .fetch_optional(&mut *tx)
    .await;

    match product {
        Ok(Some(p)) => {
            // 2. Log the change
            let log_result = sqlx::query(
                "INSERT INTO inventory_logs (product_id, change_amount, reason) VALUES ($1, $2, $3)"
            )
            .bind(p.id)
            .bind(payload.amount)
            .bind(&payload.reason)
            .execute(&mut *tx)
            .await;

            if log_result.is_err() {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().body("Error logging inventory change");
            }

            if tx.commit().await.is_err() {
                return HttpResponse::InternalServerError().body("Failed to commit transaction");
            }

            HttpResponse::Ok().json(p)
        },
        Ok(None) => {
            let _ = tx.rollback().await;
            HttpResponse::NotFound().body("Product not found")
        },
        Err(_) => {
            let _ = tx.rollback().await;
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

#[utoipa::path(
    get,
    path = "/products/{id}/logs",
    responses(
        (status = 200, description = "List inventory logs for a product", body = [InventoryLog])
    )
)]
#[get("/products/{id}/logs")]
pub async fn list_logs(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let logs = sqlx::query_as::<_, InventoryLog>(
        "SELECT * FROM inventory_logs WHERE product_id = $1 ORDER BY recorded_at DESC"
    )
    .bind(id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match logs {
        Ok(l) => HttpResponse::Ok().json(l),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching logs"),
    }
}
