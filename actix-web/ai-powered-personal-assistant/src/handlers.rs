use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/interactions",
    responses(
        (status = 200, description = "List interactions", body = [Interaction])
    )
)]
#[get("/interactions")]
pub async fn list_interactions(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let interactions = sqlx::query_as::<_, Interaction>("SELECT * FROM interactions ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(interactions))
}

#[utoipa::path(
    post,
    path = "/interactions",
    request_body = CreateInteraction,
    responses(
        (status = 201, description = "Interaction recorded", body = Interaction)
    )
)]
#[post("/interactions")]
pub async fn create_interaction(pool: web::Data<PgPool>, payload: web::Json<CreateInteraction>) -> Result<impl Responder, ApiError> {
    let response = format!("Simulated AI response for: {}", payload.user_query);
    let interaction = sqlx::query_as::<_, Interaction>(
        "INSERT INTO interactions (user_query, ai_response, confidence_score) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.user_query)
    .bind(response)
    .bind(0.95)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(interaction))
}
