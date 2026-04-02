use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use chrono::Utc;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/time",
    params(TimeZoneQuery),
    responses(
        (status = 200, description = "Get current time", body = TimeResponse)
    )
)]
pub async fn get_current_time(
    Query(params): Query<TimeZoneQuery>,
) -> impl IntoResponse {
    Json(TimeResponse {
        current_time: Utc::now(),
        timezone: params.tz.unwrap_or_else(|| "UTC".to_string()),
    })
}

#[utoipa::path(
    get,
    path = "/alarms",
    responses(
        (status = 200, description = "List all alarms", body = [Alarm])
    )
)]
pub async fn list_alarms(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let alarms = sqlx::query_as::<_, Alarm>("SELECT * FROM alarms ORDER BY alarm_time")
        .fetch_all(&pool)
        .await?;
    Ok(Json(alarms))
}

#[utoipa::path(
    post,
    path = "/alarms",
    request_body = CreateAlarm,
    responses(
        (status = 201, description = "Alarm created", body = Alarm)
    )
)]
pub async fn create_alarm(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAlarm>,
) -> Result<impl IntoResponse, ApiError> {
    let alarm = sqlx::query_as::<_, Alarm>(
        "INSERT INTO alarms (name, alarm_time) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(payload.alarm_time)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(alarm)))
}
