use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    post,
    path = "/convert",
    request_body = ConversionRequest,
    responses(
        (status = 200, description = "Conversion successful", body = Conversion)
    )
)]
pub async fn convert_units(
    State(pool): State<PgPool>,
    Json(payload): Json<ConversionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Simple conversion logic (hardcoded for brevity)
    let output_value = match (payload.from.as_str(), payload.to.as_str()) {
        ("km", "mi") => payload.value * 0.621371,
        ("mi", "km") => payload.value / 0.621371,
        ("kg", "lb") => payload.value * 2.20462,
        ("lb", "kg") => payload.value / 2.20462,
        _ => return Err(ApiError::Internal), // Unsupported conversion
    };

    let conversion = sqlx::query_as::<_, Conversion>(
        "INSERT INTO conversions (input_unit, input_value, output_unit, output_value) \
         VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.from)
    .bind(payload.value)
    .bind(&payload.to)
    .bind(output_value)
    .fetch_one(&pool)
    .await?;

    Ok(Json(conversion))
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "List conversion history", body = [Conversion])
    )
)]
pub async fn list_history(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let history = sqlx::query_as::<_, Conversion>("SELECT * FROM conversions ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(history))
}
