use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/translations",
    responses(
        (status = 200, description = "List translations", body = [Translation])
    )
)]
#[get("/translations")]
pub async fn list_translations(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let translations = sqlx::query_as::<_, Translation>("SELECT * FROM translations ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(translations))
}

#[utoipa::path(
    post,
    path = "/translations",
    request_body = CreateTranslation,
    responses(
        (status = 201, description = "Translation created", body = Translation)
    )
)]
#[post("/translations")]
pub async fn create_translation(pool: web::Data<PgPool>, payload: web::Json<CreateTranslation>) -> Result<impl Responder, ApiError> {
    let translated = format!("Translated({}) : {}", payload.target_lang, payload.original_text);
    let translation = sqlx::query_as::<_, Translation>(
        "INSERT INTO translations (original_text, translated_text, source_lang, target_lang) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.original_text)
    .bind(translated)
    .bind(&payload.source_lang)
    .bind(&payload.target_lang)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Created().json(translation))
}
