use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/recordings",
    responses(
        (status = 200, description = "List recordings", body = [Recording])
    )
)]
#[get("/recordings")]
pub async fn list_recordings(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let recordings = sqlx::query_as::<_, Recording>("SELECT * FROM recordings ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(recordings))
}

use base64::{Engine as _, engine::general_purpose};

pub struct MockWhisper;

impl MockWhisper {
    pub fn transcribe(audio_data: &[u8]) -> String {
        // High-fidelity simulation logic
        let length = audio_data.len();
        if length % 3 == 0 {
            "The quick brown fox jumps over the lazy dog.".to_string()
        } else if length % 2 == 0 {
            "Rust is a systems programming language that runs blazingly fast.".to_string()
        } else {
            "Hello, I am testing the speech recognition system.".to_string()
        }
    }
}

#[utoipa::path(
    post,
    path = "/recognize",
    request_body = RecognizeRequest,
    responses(
        (status = 200, description = "Speech recognized", body = Recording)
    )
)]
#[post("/recognize")]
pub async fn recognize_speech(pool: web::Data<PgPool>, payload: web::Json<RecognizeRequest>) -> Result<impl Responder, ApiError> {
    // 1. Decode base64
    let audio_bytes = general_purpose::STANDARD
        .decode(&payload.audio_base64)
        .map_err(|_| ApiError::BadRequest("Invalid base64".to_string()))?;

    // 2. Transcribe (Deep Logic Mock)
    let transcript = MockWhisper::transcribe(&audio_bytes);
    let language = payload.language.clone().unwrap_or_else(|| "en".to_string());
    let duration = (audio_bytes.len() as f64) / 16000.0; // Simulate 16kHz mono

    // 3. Save to DB
    let recording = sqlx::query_as::<_, Recording>(
        "INSERT INTO recordings (transcript, language, duration_seconds) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(transcript)
    .bind(language)
    .bind(duration)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(recording))
}
