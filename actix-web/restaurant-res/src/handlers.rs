use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Table Handlers ---

#[utoipa::path(
    get,
    path = "/tables",
    responses(
        (status = 200, description = "List all tables", body = [RestaurantTable])
    )
)]
#[get("/tables")]
pub async fn list_tables(pool: web::Data<PgPool>) -> impl Responder {
    let tables = sqlx::query_as::<_, RestaurantTable>("SELECT * FROM restaurant_tables ORDER BY table_number")
        .fetch_all(pool.get_ref())
        .await;

    match tables {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching tables"),
    }
}

#[utoipa::path(
    post,
    path = "/tables",
    request_body = CreateTableRow,
    responses(
        (status = 201, description = "Table created", body = RestaurantTable)
    )
)]
#[post("/tables")]
pub async fn create_table(pool: web::Data<PgPool>, payload: web::Json<CreateTableRow>) -> impl Responder {
    let table = sqlx::query_as::<_, RestaurantTable>(
        "INSERT INTO restaurant_tables (table_number, capacity) VALUES ($1, $2) RETURNING *"
    )
    .bind(payload.table_number)
    .bind(payload.capacity)
    .fetch_one(pool.get_ref())
    .await;

    match table {
        Ok(t) => HttpResponse::Created().json(t),
        Err(_) => HttpResponse::InternalServerError().body("Error creating table"),
    }
}

// --- Reservation Handlers ---

#[utoipa::path(
    get,
    path = "/reservations",
    responses(
        (status = 200, description = "List all reservations", body = [Reservation])
    )
)]
#[get("/reservations")]
pub async fn list_reservations(pool: web::Data<PgPool>) -> impl Responder {
    let res = sqlx::query_as::<_, Reservation>("SELECT * FROM reservations ORDER BY reservation_time ASC")
        .fetch_all(pool.get_ref())
        .await;

    match res {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching reservations"),
    }
}

#[utoipa::path(
    post,
    path = "/reservations",
    request_body = CreateReservation,
    responses(
        (status = 201, description = "Reservation created", body = Reservation),
        (status = 400, description = "Table not available or insufficient capacity")
    )
)]
#[post("/reservations")]
pub async fn create_reservation(pool: web::Data<PgPool>, payload: web::Json<CreateReservation>) -> impl Responder {
    // 1. Check table capacity and existing overlapping reservations
    let table = sqlx::query_as::<_, RestaurantTable>("SELECT * FROM restaurant_tables WHERE id = $1")
        .bind(payload.table_id)
        .fetch_optional(pool.get_ref())
        .await;

    match table {
        Ok(Some(t)) if t.capacity >= payload.num_guests => {
            // Check for overlap (simplistic: same time)
            let overlap = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM reservations WHERE table_id = $1 AND reservation_time = $2"
            )
            .bind(payload.table_id)
            .bind(payload.reservation_time)
            .fetch_one(pool.get_ref())
            .await;

            if let Ok(0) = overlap {
                let res = sqlx::query_as::<_, Reservation>(
                    "INSERT INTO reservations (table_id, customer_name, reservation_time, num_guests) \
                     VALUES ($1, $2, $3, $4) RETURNING *"
                )
                .bind(payload.table_id)
                .bind(&payload.customer_name)
                .bind(payload.reservation_time)
                .bind(payload.num_guests)
                .fetch_one(pool.get_ref())
                .await;

                match res {
                    Ok(r) => HttpResponse::Created().json(r),
                    Err(_) => HttpResponse::InternalServerError().body("Error creating reservation"),
                }
            } else {
                HttpResponse::BadRequest().body("Table already reserved for this time")
            }
        },
        Ok(Some(_)) => HttpResponse::BadRequest().body("Table capacity exceeded"),
        Ok(None) => HttpResponse::NotFound().body("Table not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}
