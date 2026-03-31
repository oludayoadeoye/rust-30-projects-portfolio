use actix_web::{get, post, web, HttpResponse, Responder, http::header};
use sqlx::PgPool;
use nanoid::nanoid;
use crate::models::{UrlMapping, ShortenRequest};

#[utoipa::path(
    post,
    path = "/shorten",
    request_body = ShortenRequest,
    responses(
        (status = 201, description = "URL shortened", body = UrlMapping)
    )
)]
#[post("/shorten")]
pub async fn shorten_url(pool: web::Data<PgPool>, payload: web::Json<ShortenRequest>) -> impl Responder {
    let code = nanoid!(7);
    let mapping = sqlx::query_as::<_, UrlMapping>(
        "INSERT INTO urls (code, original_url) VALUES ($1, $2) RETURNING *"
    )
    .bind(code)
    .bind(&payload.url)
    .fetch_one(pool.get_ref())
    .await;

    match mapping {
        Ok(m) => HttpResponse::Created().json(m),
        Err(_) => HttpResponse::InternalServerError().body("Error shortening URL"),
    }
}

#[utoipa::path(
    get,
    path = "/{code}",
    responses(
        (status = 302, description = "Redirect to original URL"),
        (status = 404, description = "Short code not found")
    )
)]
#[get("/{code}")]
pub async fn redirect_url(pool: web::Data<PgPool>, code: web::Path<String>) -> impl Responder {
    let mapping = sqlx::query_as::<_, UrlMapping>("SELECT * FROM urls WHERE code = $1")
        .bind(code.as_str())
        .fetch_optional(pool.get_ref())
        .await;

    match mapping {
        Ok(Some(m)) => {
            // Increment hit count asynchronously
            let _ = sqlx::query("UPDATE urls SET hits = hits + 1 WHERE id = $1")
                .bind(m.id)
                .execute(pool.get_ref())
                .await;

            HttpResponse::Found()
                .append_header((header::LOCATION, m.original_url))
                .finish()
        },
        Ok(None) => HttpResponse::NotFound().body("Short code not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching URL"),
    }
}

#[utoipa::path(
    get,
    path = "/stats/{code}",
    responses(
        (status = 200, description = "Get URL stats", body = UrlMapping),
        (status = 404, description = "Short code not found")
    )
)]
#[get("/stats/{code}")]
pub async fn get_stats(pool: web::Data<PgPool>, code: web::Path<String>) -> impl Responder {
    let mapping = sqlx::query_as::<_, UrlMapping>("SELECT * FROM urls WHERE code = $1")
        .bind(code.as_str())
        .fetch_optional(pool.get_ref())
        .await;

    match mapping {
        Ok(Some(m)) => HttpResponse::Ok().json(m),
        Ok(None) => HttpResponse::NotFound().body("Short code not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching stats"),
    }
}
