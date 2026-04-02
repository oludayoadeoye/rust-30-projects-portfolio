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
    path = "/todos",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
pub async fn list_todos(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created", body = Todo)
    )
)]
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, ApiError> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING *"
    )
    .bind(&payload.title)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, ApiError> {
    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET \
         title = COALESCE($1, title), \
         completed = COALESCE($2, completed) \
         WHERE id = $3 RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.completed)
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match todo {
        Some(t) => Ok(Json(t)),
        None => Err(ApiError::NotFound),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
