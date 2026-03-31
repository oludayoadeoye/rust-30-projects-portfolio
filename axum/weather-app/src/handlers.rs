use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::{WeatherReport, CreateWeatherReport, UpdateWeatherReport};

#[derive(Deserialize)]
pub struct CityQuery {
    pub city: Option<String>,
}

#[utoipa::path(
    get,
    path = "/weather",
    params(
        ("city" = Option<String>, Query, description = "Filter by city name")
    ),
    responses(
        (status = 200, description = "List weather reports", body = [WeatherReport])
    )
)]
pub async fn list_reports(
    State(pool): State<PgPool>,
    Query(query): Query<CityQuery>,
) -> impl IntoResponse {
    let reports = if let Some(city) = query.city {
        sqlx::query_as::<_, WeatherReport>("SELECT * FROM weather_reports WHERE city ILIKE $1 ORDER BY recorded_at DESC")
            .bind(format!("%{}%", city))
            .fetch_all(&pool)
            .await
    } else {
        sqlx::query_as::<_, WeatherReport>("SELECT * FROM weather_reports ORDER BY recorded_at DESC")
            .fetch_all(&pool)
            .await
    };

    match reports {
        Ok(reports) => (StatusCode::OK, Json(reports)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching reports").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/weather",
    request_body = CreateWeatherReport,
    responses(
        (status = 201, description = "Report created", body = WeatherReport)
    )
)]
pub async fn create_report(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateWeatherReport>,
) -> impl IntoResponse {
    let report = sqlx::query_as::<_, WeatherReport>(
        "INSERT INTO weather_reports (city, temperature, condition) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(payload.city)
    .bind(payload.temperature)
    .bind(payload.condition)
    .fetch_one(&pool)
    .await;

    match report {
        Ok(report) => (StatusCode::CREATED, Json(report)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating report").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/weather/{id}",
    responses(
        (status = 200, description = "Get report by id", body = WeatherReport),
        (status = 404, description = "Report not found")
    )
)]
pub async fn get_report(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let report = sqlx::query_as::<_, WeatherReport>("SELECT * FROM weather_reports WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match report {
        Ok(Some(report)) => (StatusCode::OK, Json(report)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Report not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching report").into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/weather/{id}",
    request_body = UpdateWeatherReport,
    responses(
        (status = 200, description = "Report updated", body = WeatherReport),
        (status = 404, description = "Report not found")
    )
)]
pub async fn update_report(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateWeatherReport>,
) -> impl IntoResponse {
    let report = sqlx::query_as::<_, WeatherReport>(
        "UPDATE weather_reports SET city = COALESCE($1, city), temperature = COALESCE($2, temperature), condition = COALESCE($3, condition) WHERE id = $4 RETURNING *"
    )
    .bind(payload.city)
    .bind(payload.temperature)
    .bind(payload.condition)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match report {
        Ok(Some(report)) => (StatusCode::OK, Json(report)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Report not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating report").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/weather/{id}",
    responses(
        (status = 204, description = "Report deleted"),
        (status = 404, description = "Report not found")
    )
)]
pub async fn delete_report(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM weather_reports WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Report not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting report").into_response(),
    }
}
