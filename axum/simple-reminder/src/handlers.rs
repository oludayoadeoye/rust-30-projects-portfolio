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
    path = "/reminders",
    responses(
        (status = 200, description = "List all reminders", body = [Reminder])
    )
)]
pub async fn list_reminders(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let r = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders ORDER BY remind_at ASC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(r))
}

#[utoipa::path(
    post,
    path = "/reminders",
    request_body = CreateReminder,
    responses(
        (status = 201, description = "Reminder created", body = Reminder)
    )
)]
pub async fn create_reminder(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateReminder>,
) -> Result<impl IntoResponse, ApiError> {
    let r = sqlx::query_as::<_, Reminder>(
        "INSERT INTO reminders (title, description, remind_at) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.remind_at)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(r)))
}

#[utoipa::path(
    put,
    path = "/reminders/{id}/complete",
    responses(
        (status = 200, description = "Reminder marked as completed", body = Reminder),
        (status = 404, description = "Reminder not found")
    )
)]
pub async fn complete_reminder(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let r = sqlx::query_as::<_, Reminder>(
        "UPDATE reminders SET is_completed = TRUE WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match r {
        Some(r) => Ok(Json(r)),
        None => Err(ApiError::NotFound),
    }
}
