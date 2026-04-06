use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;
use yfinance_rs::{YfClient, Ticker, Range, Interval};
use paft::prelude::*;
use polars::prelude::*;

#[utoipa::path(
    get,
    path = "/stocks",
    responses(
        (status = 200, description = "List monitored stocks", body = [Stock])
    )
)]
#[get("/stocks")]
pub async fn list_stocks(pool: web::Data<PgPool>) -> std::result::Result<impl Responder, ApiError> {
    let stocks = sqlx::query_as::<_, Stock>("SELECT * FROM stocks ORDER BY symbol")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(stocks))
}

#[utoipa::path(
    get,
    path = "/stocks/{symbol}/analyze",
    responses(
        (status = 200, description = "Analyze stock data", body = AnalysisResponse),
        (status = 404, description = "Symbol not found or data unavailable")
    )
)]
#[get("/stocks/{symbol}/analyze")]
pub async fn analyze_stock(symbol: web::Path<String>) -> std::result::Result<impl Responder, ApiError> {
    let sym = symbol.into_inner().to_uppercase();
    let client = YfClient::default();
    let ticker = Ticker::new(&client, &sym);

    // 1. Fetch 6 months of historical data (Async)
    let history = ticker.history(Some(Range::M6), Some(Interval::D1), false)
        .await
        .map_err(|e| {
            log::error!("Yahoo Finance error: {:?}", e);
            ApiError::NotFound
        })?;

    if history.is_empty() {
        return Err(ApiError::NotFound);
    }

    // 2. Convert to Polars DataFrame using paft's ToDataFrameVec trait
    // Note: In 0.51, we ensure we use the correct trait for conversion.
    let df = history.to_dataframe().map_err(|e| {
        log::error!("DataFrame conversion error: {:?}", e);
        ApiError::Internal
    })?;

    // 3. Perform Vectorized Analysis with Polars 0.51
    // In 0.51, we use rolling_mean with a simple window size for index-based rolling
    let analyzed = df.lazy()
        .with_columns([
            col("close.amount").rolling_mean(RollingOptionsFixedWindow {
                window_size: 20,
                min_periods: 1,
                ..Default::default()
            }).alias("sma_20"),
            col("close.amount").rolling_mean(RollingOptionsFixedWindow {
                window_size: 50,
                min_periods: 1,
                ..Default::default()
            }).alias("sma_50"),
        ])
        .collect()
        .map_err(|e| {
            log::error!("Polars analysis error: {:?}", e);
            ApiError::Internal
        })?;

    // 4. Extract results
    let current_price = history.last()
        .and_then(|c| c.close.amount().to_string().parse::<f64>().ok())
        .unwrap_or(0.0);
    
    let sma_20_series = analyzed.column("sma_20").map_err(|_| ApiError::Internal)?;
    let sma_50_series = analyzed.column("sma_50").map_err(|_| ApiError::Internal)?;
    
    let last_sma_20 = sma_20_series.f64().ok().and_then(|s| s.last());
    let last_sma_50 = sma_50_series.f64().ok().and_then(|s| s.last());

    let recommendation = match (last_sma_20, last_sma_50) {
        (Some(s20), Some(s50)) if s20 > s50 => "Strong Buy (Golden Cross)".to_string(),
        (Some(s20), Some(s50)) if s20 < s50 => "Sell (Death Cross)".to_string(),
        _ => "Neutral".to_string(),
    };

    Ok(HttpResponse::Ok().json(AnalysisResponse {
        symbol: sym,
        current_price,
        sma_20: last_sma_20,
        sma_50: last_sma_50,
        recommendation,
    }))
}
