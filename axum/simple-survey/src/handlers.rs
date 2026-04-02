use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

// --- Survey Handlers ---

#[utoipa::path(
    get,
    path = "/surveys",
    responses(
        (status = 200, description = "List all surveys", body = [Survey])
    )
)]
pub async fn list_surveys(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let surveys = sqlx::query_as::<_, Survey>("SELECT * FROM surveys ORDER BY created_at")
        .fetch_all(&pool)
        .await?;
    Ok(Json(surveys))
}

#[utoipa::path(
    post,
    path = "/surveys",
    request_body = CreateSurvey,
    responses(
        (status = 201, description = "Survey created", body = Survey)
    )
)]
pub async fn create_survey(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSurvey>,
) -> Result<impl IntoResponse, ApiError> {
    let survey = sqlx::query_as::<_, Survey>(
        "INSERT INTO surveys (title, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(survey)))
}

// --- Response Handlers ---

#[utoipa::path(
    get,
    path = "/surveys/{id}/responses",
    responses(
        (status = 200, description = "List responses for a survey", body = [Response])
    )
)]
pub async fn list_responses(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let res = sqlx::query_as::<_, Response>("SELECT * FROM responses WHERE survey_id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/surveys/{id}/responses",
    request_body = CreateResponse,
    responses(
        (status = 201, description = "Response submitted", body = Response)
    )
)]
pub async fn submit_response(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateResponse>,
) -> Result<impl IntoResponse, ApiError> {
    let res = sqlx::query_as::<_, Response>(
        "INSERT INTO responses (survey_id, respondent_name, answers) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(&payload.respondent_name)
    .bind(&payload.answers)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(res)))
}
