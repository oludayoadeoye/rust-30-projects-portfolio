use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Todo, CreateTodo, UpdateTodo};

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
pub async fn list_todos(State(pool): State<PgPool>) -> impl IntoResponse {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await;

    match todos {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching todos").into_response(),
    }
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
) -> impl IntoResponse {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.description)
    .fetch_one(&pool)
    .await;

    match todo {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating todo").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Get todo by id", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn get_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match todo {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching todo").into_response(),
    }
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
) -> impl IntoResponse {
    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = COALESCE($1, title), description = COALESCE($2, description), completed = COALESCE($3, completed) WHERE id = $4 RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.completed)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match todo {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating todo").into_response(),
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
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Todo not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting todo").into_response(),
    }
}
