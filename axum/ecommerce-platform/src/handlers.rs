use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Product, CreateProduct, Order, PlaceOrder};

#[utoipa::path(
    get,
    path = "/products",
    responses(
        (status = 200, description = "List all products", body = [Product])
    )
)]
pub async fn list_products(State(pool): State<PgPool>) -> impl IntoResponse {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY name")
        .fetch_all(&pool)
        .await;

    match products {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching products").into_response(),
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
pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (id, name, price, stock) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.name)
    .bind(payload.price)
    .bind(payload.stock)
    .fetch_one(&pool)
    .await;

    match product {
        Ok(product) => (StatusCode::CREATED, Json(product)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating product").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/orders",
    request_body = PlaceOrder,
    responses(
        (status = 201, description = "Order placed", body = Order),
        (status = 400, description = "Insufficient stock or invalid product")
    )
)]
pub async fn place_order(
    State(pool): State<PgPool>,
    Json(payload): Json<PlaceOrder>,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Transaction failed").into_response(),
    };

    let order_id = Uuid::new_v4();
    let mut total_price = Decimal::ZERO;

    for item in payload.items {
        // 1. Check stock and get price
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1 FOR UPDATE")
            .bind(item.product_id)
            .fetch_optional(&mut *tx)
            .await;

        match product {
            Ok(Some(p)) if p.stock >= item.quantity => {
                total_price += p.price * Decimal::from(item.quantity);
                
                // 2. Decrement stock
                if let Err(_) = sqlx::query("UPDATE products SET stock = stock - $1 WHERE id = $2")
                    .bind(item.quantity)
                    .bind(item.product_id)
                    .execute(&mut *tx)
                    .await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Update failed").into_response();
                    }

                // 3. Create order item
                if let Err(_) = sqlx::query(
                    "INSERT INTO order_items (order_id, product_id, quantity, price_at_order) VALUES ($1, $2, $3, $4)"
                )
                .bind(order_id)
                .bind(item.product_id)
                .bind(item.quantity)
                .bind(p.price)
                .execute(&mut *tx)
                .await {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Item insert failed").into_response();
                }
            },
            Ok(Some(_)) => return (StatusCode::BAD_REQUEST, "Insufficient stock").into_response(),
            _ => return (StatusCode::BAD_REQUEST, "Product not found").into_response(),
        }
    }

    // 4. Create order
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (id, total_price) VALUES ($1, $2) RETURNING *"
    )
    .bind(order_id)
    .bind(total_price)
    .fetch_one(&mut *tx)
    .await;

    if let Ok(o) = order {
        if tx.commit().await.is_ok() {
            return (StatusCode::CREATED, Json(o)).into_response();
        }
    }

    (StatusCode::INTERNAL_SERVER_ERROR, "Order finalization failed").into_response()
}
