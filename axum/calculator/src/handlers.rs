use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Calculation, CreateCalculation};

#[utoipa::path(
    get,
    path = "/calculations",
    responses(
        (status = 200, description = "List calculation history", body = [Calculation])
    )
)]
pub async fn list_calculations(State(pool): State<PgPool>) -> impl IntoResponse {
    let history = sqlx::query_as::<_, Calculation>("SELECT * FROM calculations ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await;

    match history {
        Ok(history) => (StatusCode::OK, Json(history)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching history").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/calculations",
    request_body = CreateCalculation,
    responses(
        (status = 201, description = "Calculation saved", body = Calculation)
    )
)]
pub async fn create_calculation(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCalculation>,
) -> impl IntoResponse {
    let calc = sqlx::query_as::<_, Calculation>(
        "INSERT INTO calculations (expression, result) VALUES ($1, $2) RETURNING *"
    )
    .bind(payload.expression)
    .bind(payload.result)
    .fetch_one(&pool)
    .await;

    match calc {
        Ok(calc) => (StatusCode::CREATED, Json(calc)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error saving calculation").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/calculations/{id}",
    responses(
        (status = 200, description = "Get calculation by id", body = Calculation),
        (status = 404, description = "Calculation not found")
    )
)]
pub async fn get_calculation(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let calc = sqlx::query_as::<_, Calculation>("SELECT * FROM calculations WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match calc {
        Ok(Some(calc)) => (StatusCode::OK, Json(calc)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Calculation not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching calculation").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/calculations/{id}",
    responses(
        (status = 204, description = "Calculation deleted"),
        (status = 404, description = "Calculation not found")
    )
)]
pub async fn delete_calculation(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM calculations WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Calculation not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting calculation").into_response(),
    }
}
