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
    path = "/suggest",
    request_body = CreateInteraction,
    responses(
        (status = 200, description = "Intent suggested", body = SuggestionResponse)
    )
)]
#[post("/suggest")]
pub async fn suggest_intent(pool: web::Data<PgPool>, payload: web::Json<CreateInteraction>) -> Result<impl Responder, ApiError> {
    let query = payload.user_query.to_lowercase();
    
    // Deep Logic Simulation: Keyword-based Intent Classification
    let (intent, recommendation, confidence) = if query.contains("meeting") || query.contains("schedule") || query.contains("calendar") {
        (Intent::Schedule, "I've found a slot at 2 PM today. Should I book it?".to_string(), 0.95)
    } else if query.contains("remind") || query.contains("don't forget") {
        (Intent::Reminder, "I'll remind you about this in 1 hour.".to_string(), 0.88)
    } else if query.contains("search") || query.contains("find") || query.contains("who is") {
        (Intent::Search, "Searching the web for relevant information...".to_string(), 0.82)
    } else if query.contains("email") || query.contains("send to") {
        (Intent::Email, "Drafting an email to the recipient. Ready to send?".to_string(), 0.90)
    } else {
        (Intent::Other, "I'm not sure I understand. Could you rephrase?".to_string(), 0.40)
    };

    // Save interaction
    sqlx::query(
        "INSERT INTO interactions (user_query, ai_response, confidence_score) VALUES ($1, $2, $3)"
    )
    .bind(&payload.user_query)
    .bind(&recommendation)
    .bind(confidence)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(SuggestionResponse {
        intent,
        recommendation,
        confidence,
    }))
}
