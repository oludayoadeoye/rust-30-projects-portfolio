use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/players",
    responses(
        (status = 200, description = "List players", body = [Player])
    )
)]
#[get("/players")]
pub async fn list_players(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let players = sqlx::query_as::<_, Player>("SELECT * FROM players ORDER BY score DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(players))
}

#[utoipa::path(
    post,
    path = "/players",
    request_body = CreatePlayer,
    responses(
        (status = 201, description = "Player created", body = Player)
    )
)]
#[post("/players")]
pub async fn create_player(pool: web::Data<PgPool>, payload: web::Json<CreatePlayer>) -> Result<impl Responder, ApiError> {
    let player = sqlx::query_as::<_, Player>(
        "INSERT INTO players (username, score, level) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.username)
    .bind(0)
    .bind(1)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(player))
}
