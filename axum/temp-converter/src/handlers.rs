use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

fn convert(from: &str, to: &str, val: f64) -> Option<f64> {
    let celsius = match from {
        "C" => val,
        "F" => (val - 32.0) * 5.0 / 9.0,
        "K" => val - 273.15,
        _ => return None,
    };

    match to {
        "C" => Some(celsius),
        "F" => Some(celsius * 9.0 / 5.0 + 32.0),
        "K" => Some(celsius + 273.15),
        _ => None,
    }
}

#[utoipa::path(
    post,
    path = "/convert",
    request_body = TempRequest,
    responses(
        (status = 200, description = "Conversion successful", body = TempConversion)
    )
)]
pub async fn convert_temp(
    State(pool): State<PgPool>,
    Json(payload): Json<TempRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let output_value = convert(&payload.from, &payload.to, payload.value)
        .ok_or(ApiError::Internal)?;

    let conversion = sqlx::query_as::<_, TempConversion>(
        "INSERT INTO temp_conversions (input_unit, input_value, output_unit, output_value) \
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
        (status = 200, description = "List all temperature conversions", body = [TempConversion])
    )
)]
pub async fn list_history(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let h = sqlx::query_as::<_, TempConversion>("SELECT * FROM temp_conversions ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(h))
}
