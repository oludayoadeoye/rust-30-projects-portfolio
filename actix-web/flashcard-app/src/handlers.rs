use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Deck Handlers ---

#[utoipa::path(
    get,
    path = "/decks",
    responses(
        (status = 200, description = "List all decks", body = [Deck])
    )
)]
#[get("/decks")]
pub async fn list_decks(pool: web::Data<PgPool>) -> impl Responder {
    let decks = sqlx::query_as::<_, Deck>("SELECT * FROM decks ORDER BY name")
        .fetch_all(pool.get_ref())
        .await;

    match decks {
        Ok(decks) => HttpResponse::Ok().json(decks),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching decks"),
    }
}

#[utoipa::path(
    post,
    path = "/decks",
    request_body = CreateDeck,
    responses(
        (status = 201, description = "Deck created", body = Deck)
    )
)]
#[post("/decks")]
pub async fn create_deck(pool: web::Data<PgPool>, payload: web::Json<CreateDeck>) -> impl Responder {
    let deck = sqlx::query_as::<_, Deck>(
        "INSERT INTO decks (name, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(pool.get_ref())
    .await;

    match deck {
        Ok(deck) => HttpResponse::Created().json(deck),
        Err(_) => HttpResponse::InternalServerError().body("Error creating deck"),
    }
}

// --- Card Handlers ---

#[utoipa::path(
    get,
    path = "/decks/{id}/cards",
    responses(
        (status = 200, description = "List cards in a deck", body = [Card])
    )
)]
#[get("/decks/{id}/cards")]
pub async fn list_cards(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let cards = sqlx::query_as::<_, Card>("SELECT * FROM cards WHERE deck_id = $1 ORDER BY created_at ASC")
        .bind(id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match cards {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching cards"),
    }
}

#[utoipa::path(
    post,
    path = "/decks/{id}/cards",
    request_body = CreateCard,
    responses(
        (status = 201, description = "Card created", body = Card)
    )
)]
#[post("/decks/{id}/cards")]
pub async fn create_card(pool: web::Data<PgPool>, id: web::Path<i32>, payload: web::Json<CreateCard>) -> impl Responder {
    let card = sqlx::query_as::<_, Card>(
        "INSERT INTO cards (deck_id, front, back) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id.into_inner())
    .bind(&payload.front)
    .bind(&payload.back)
    .fetch_one(pool.get_ref())
    .await;

    match card {
        Ok(card) => HttpResponse::Created().json(card),
        Err(_) => HttpResponse::InternalServerError().body("Error creating card"),
    }
}

#[utoipa::path(
    put,
    path = "/cards/{id}/review",
    request_body = ReviewResult,
    responses(
        (status = 200, description = "Card reviewed", body = Card)
    )
)]
#[put("/cards/{id}/review")]
pub async fn review_card(pool: web::Data<PgPool>, id: web::Path<i32>, payload: web::Json<ReviewResult>) -> impl Responder {
    let adjustment = if payload.correct { 1 } else { -1 };
    
    let card = sqlx::query_as::<_, Card>(
        "UPDATE cards SET mastery_level = GREATEST(0, mastery_level + $1), last_reviewed_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(adjustment)
    .bind(id.into_inner())
    .fetch_optional(pool.get_ref())
    .await;

    match card {
        Ok(Some(card)) => HttpResponse::Ok().json(card),
        Ok(None) => HttpResponse::NotFound().body("Card not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error reviewing card"),
    }
}
