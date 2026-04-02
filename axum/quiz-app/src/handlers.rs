use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/questions",
    responses(
        (status = 200, description = "List all questions", body = [Question])
    )
)]
pub async fn list_questions(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let q = sqlx::query_as::<_, Question>("SELECT * FROM questions ORDER BY created_at")
        .fetch_all(&pool)
        .await?;
    Ok(Json(q))
}

#[utoipa::path(
    post,
    path = "/questions",
    request_body = CreateQuestion,
    responses(
        (status = 201, description = "Question created", body = Question)
    )
)]
pub async fn create_question(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuestion>,
) -> Result<impl IntoResponse, ApiError> {
    let q = sqlx::query_as::<_, Question>(
        "INSERT INTO questions (question, answer, difficulty) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.question)
    .bind(&payload.answer)
    .bind(payload.difficulty.unwrap_or(1))
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(q)))
}

#[utoipa::path(
    post,
    path = "/questions/{id}/attempt",
    request_body = SubmitAttempt,
    responses(
        (status = 201, description = "Attempt recorded", body = Attempt),
        (status = 404, description = "Question not found")
    )
)]
pub async fn submit_attempt(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<SubmitAttempt>,
) -> Result<impl IntoResponse, ApiError> {
    let q = sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match q {
        Some(q) => {
            let is_correct = q.answer.trim().to_lowercase() == payload.user_answer.trim().to_lowercase();
            let attempt = sqlx::query_as::<_, Attempt>(
                "INSERT INTO attempts (question_id, user_answer, is_correct) VALUES ($1, $2, $3) RETURNING *"
            )
            .bind(id)
            .bind(&payload.user_answer)
            .bind(is_correct)
            .fetch_one(&pool)
            .await?;
            Ok((StatusCode::CREATED, Json(attempt)))
        },
        None => Err(ApiError::NotFound),
    }
}
