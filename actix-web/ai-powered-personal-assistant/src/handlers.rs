use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;
use google_ai_rs::{Client, AsSchema};
use serde::Deserialize;

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

#[derive(Debug, Deserialize, AsSchema)]
pub struct StructuredIntent {
    pub intent: String, // "Schedule", "Reminder", "Search", "Email", "Other"
    pub recommendation: String,
    pub confidence: f64,
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
    let api_key = std::env::var("GOOGLE_API_KEY").map_err(|_| ApiError::Internal)?;
    let client = Client::new(&api_key).await.map_err(|_| ApiError::Internal)?;
    
    // Use Gemini 1.5 Flash for fast structured analysis
    let model = client.typed_model::<StructuredIntent>("gemini-1.5-flash");
    
    let prompt = format!(
        "Analyze the following user query and return a structured JSON response with the intent, a helpful recommendation, and a confidence score (0.0 to 1.0).\
        Valid intents: Schedule, Reminder, Search, Email, Other.\
        User query: '{}'",
        payload.user_query
    );

    let structured = model.generate_content(prompt).await.map_err(|e| {
        log::error!("Gemini API error: {:?}", e);
        ApiError::Internal
    })?;

    // Map to response model
    let intent_enum = match structured.intent.as_str() {
        "Schedule" => Intent::Schedule,
        "Reminder" => Intent::Reminder,
        "Search" => Intent::Search,
        "Email" => Intent::Email,
        _ => Intent::Other,
    };

    // Save interaction
    sqlx::query(
        "INSERT INTO interactions (user_query, ai_response, confidence_score) VALUES ($1, $2, $3)"
    )
    .bind(&payload.user_query)
    .bind(&structured.recommendation)
    .bind(structured.confidence)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(SuggestionResponse {
        intent: intent_enum,
        recommendation: structured.recommendation,
        confidence: structured.confidence,
    }))
}
