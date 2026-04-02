use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use rust_decimal::prelude::*;
use crate::models::*;
use common_utils::ApiError;

#[utoipa::path(
    post,
    path = "/calculate",
    request_body = CalculateRequest,
    responses(
        (status = 200, description = "Calculation successful", body = InterestCalculation)
    )
)]
pub async fn calculate_interest(
    State(pool): State<PgPool>,
    Json(payload): Json<CalculateRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let p = payload.principal.to_f64().unwrap_or(0.0);
    let r = payload.rate / 100.0;
    let t = payload.time_years;
    let n = payload.compound_frequency.unwrap_or(1) as f64;

    // A = P(1 + r/n)^(nt)
    let amount = p * (1.0 + r / n).powf(n * t);
    let interest = amount - p;

    let total_amount = Decimal::from_f64(amount).unwrap_or_default();
    let interest_earned = Decimal::from_f64(interest).unwrap_or_default();

    let calculation = sqlx::query_as::<_, InterestCalculation>(
        "INSERT INTO interest_calculations (principal, rate, time_years, compound_frequency, total_amount, interest_earned) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(payload.principal)
    .bind(payload.rate)
    .bind(payload.time_years)
    .bind(payload.compound_frequency.unwrap_or(1))
    .bind(total_amount)
    .bind(interest_earned)
    .fetch_one(&pool)
    .await?;

    Ok(Json(calculation))
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "List all interest calculations", body = [InterestCalculation])
    )
)]
pub async fn list_history(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let h = sqlx::query_as::<_, InterestCalculation>("SELECT * FROM interest_calculations ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(h))
}
