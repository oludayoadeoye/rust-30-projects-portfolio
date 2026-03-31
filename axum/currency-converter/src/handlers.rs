use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{ExchangeRate, CreateExchangeRate, Conversion, PerformConversion};

#[utoipa::path(
    get,
    path = "/rates",
    responses(
        (status = 200, description = "List all exchange rates", body = [ExchangeRate])
    )
)]
pub async fn list_rates(State(pool): State<PgPool>) -> impl IntoResponse {
    let rates = sqlx::query_as::<_, ExchangeRate>("SELECT * FROM exchange_rates ORDER BY currency_code")
        .fetch_all(&pool)
        .await;

    match rates {
        Ok(rates) => (StatusCode::OK, Json(rates)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching rates").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/rates",
    request_body = CreateExchangeRate,
    responses(
        (status = 201, description = "Rate created or updated", body = ExchangeRate)
    )
)]
pub async fn upsert_rate(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateExchangeRate>,
) -> impl IntoResponse {
    let rate = sqlx::query_as::<_, ExchangeRate>(
        "INSERT INTO exchange_rates (currency_code, rate_to_usd) VALUES ($1, $2) \
         ON CONFLICT (currency_code) DO UPDATE SET rate_to_usd = EXCLUDED.rate_to_usd, updated_at = NOW() RETURNING *"
    )
    .bind(payload.currency_code.to_uppercase())
    .bind(payload.rate_to_usd)
    .fetch_one(&pool)
    .await;

    match rate {
        Ok(rate) => (StatusCode::CREATED, Json(rate)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error saving rate").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/convert",
    request_body = PerformConversion,
    responses(
        (status = 200, description = "Conversion successful", body = Conversion),
        (status = 400, description = "Invalid currency code")
    )
)]
pub async fn convert_currency(
    State(pool): State<PgPool>,
    Json(payload): Json<PerformConversion>,
) -> impl IntoResponse {
    // 1. Get rates
    let from_rate = sqlx::query_scalar::<_, f64>("SELECT rate_to_usd FROM exchange_rates WHERE currency_code = $1")
        .bind(payload.from_currency.to_uppercase())
        .fetch_optional(&pool)
        .await;

    let to_rate = sqlx::query_scalar::<_, f64>("SELECT rate_to_usd FROM exchange_rates WHERE currency_code = $1")
        .bind(payload.to_currency.to_uppercase())
        .fetch_optional(&pool)
        .await;

    match (from_rate, to_rate) {
        (Ok(Some(from)), Ok(Some(to))) => {
            let result = (payload.amount / from) * to;
            
            // 2. Record conversion
            let conv = sqlx::query_as::<_, Conversion>(
                "INSERT INTO conversions (from_currency, to_currency, amount, result) VALUES ($1, $2, $3, $4) RETURNING *"
            )
            .bind(payload.from_currency.to_uppercase())
            .bind(payload.to_currency.to_uppercase())
            .bind(payload.amount)
            .bind(result)
            .fetch_one(&pool)
            .await;

            match conv {
                Ok(c) => (StatusCode::OK, Json(c)).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error recording conversion").into_response(),
            }
        },
        _ => (StatusCode::BAD_REQUEST, "One or both currency codes not found").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/conversions",
    responses(
        (status = 200, description = "List conversion history", body = [Conversion])
    )
)]
pub async fn list_conversions(State(pool): State<PgPool>) -> impl IntoResponse {
    let history = sqlx::query_as::<_, Conversion>("SELECT * FROM conversions ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await;

    match history {
        Ok(history) => (StatusCode::OK, Json(history)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching history").into_response(),
    }
}
