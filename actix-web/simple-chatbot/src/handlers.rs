use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use regex::Regex;
use crate::models::*;

// --- Pattern Handlers ---

#[utoipa::path(
    get,
    path = "/patterns",
    responses(
        (status = 200, description = "List all response patterns", body = [Pattern])
    )
)]
#[get("/patterns")]
pub async fn list_patterns(pool: web::Data<PgPool>) -> impl Responder {
    let patterns = sqlx::query_as::<_, Pattern>("SELECT * FROM patterns")
        .fetch_all(pool.get_ref())
        .await;

    match patterns {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching patterns"),
    }
}

#[utoipa::path(
    post,
    path = "/patterns",
    request_body = CreatePattern,
    responses(
        (status = 201, description = "Pattern created", body = Pattern)
    )
)]
#[post("/patterns")]
pub async fn create_pattern(pool: web::Data<PgPool>, payload: web::Json<CreatePattern>) -> impl Responder {
    let p = sqlx::query_as::<_, Pattern>(
        "INSERT INTO patterns (pattern, response) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.pattern)
    .bind(&payload.response)
    .fetch_one(pool.get_ref())
    .await;

    match p {
        Ok(p) => HttpResponse::Created().json(p),
        Err(_) => HttpResponse::InternalServerError().body("Error creating pattern"),
    }
}

// --- Chat Handler ---

#[utoipa::path(
    post,
    path = "/chat",
    request_body = MessageRequest,
    responses(
        (status = 200, description = "Bot response", body = ChatLog)
    )
)]
#[post("/chat")]
pub async fn chat(pool: web::Data<PgPool>, payload: web::Json<MessageRequest>) -> impl Responder {
    let patterns = sqlx::query_as::<_, Pattern>("SELECT * FROM patterns")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    let mut bot_response = "I'm not sure how to respond to that.".to_string();

    for p in patterns {
        if let Ok(re) = Regex::new(&p.pattern) {
            if re.is_match(&payload.message) {
                bot_response = p.response;
                break;
            }
        }
    }

    // Log the chat
    let log = sqlx::query_as::<_, ChatLog>(
        "INSERT INTO chat_logs (user_message, bot_response) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.message)
    .bind(&bot_response)
    .fetch_one(pool.get_ref())
    .await;

    match log {
        Ok(l) => HttpResponse::Ok().json(l),
        Err(_) => HttpResponse::InternalServerError().body("Error logging chat"),
    }
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "List chat history", body = [ChatLog])
    )
)]
#[get("/history")]
pub async fn list_history(pool: web::Data<PgPool>) -> impl Responder {
    let history = sqlx::query_as::<_, ChatLog>("SELECT * FROM chat_logs ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await;

    match history {
        Ok(h) => HttpResponse::Ok().json(h),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching history"),
    }
}
