use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{User, CreateUser, Task, CreateTask, UpdateTask};

// --- User Handlers ---

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    )
)]
pub async fn list_users(State(pool): State<PgPool>) -> impl IntoResponse {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY username")
        .fetch_all(&pool)
        .await;

    match users {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching users").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.username)
    .bind(payload.email)
    .fetch_one(&pool)
    .await;

    match user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response(),
    }
}

// --- Task Handlers ---

#[utoipa::path(
    get,
    path = "/tasks",
    responses(
        (status = 200, description = "List all tasks", body = [Task])
    )
)]
pub async fn list_tasks(State(pool): State<PgPool>) -> impl IntoResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await;

    match tasks {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching tasks").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/tasks",
    request_body = CreateTask,
    responses(
        (status = 201, description = "Task created", body = Task)
    )
)]
pub async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, title, description, priority, assigned_to, due_date) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.priority.unwrap_or_else(|| "MEDIUM".to_string()))
    .bind(payload.assigned_to)
    .bind(payload.due_date)
    .fetch_one(&pool)
    .await;

    match task {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating task: {}", e)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/tasks/{id}",
    request_body = UpdateTask,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 404, description = "Task not found")
    )
)]
pub async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET title = COALESCE($1, title), description = COALESCE($2, description), \
         status = COALESCE($3, status), priority = COALESCE($4, priority), \
         assigned_to = COALESCE($5, assigned_to), due_date = COALESCE($6, due_date), updated_at = NOW() \
         WHERE id = $7 RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.status)
    .bind(payload.priority)
    .bind(payload.assigned_to)
    .bind(payload.due_date)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match task {
        Ok(Some(task)) => (StatusCode::OK, Json(task)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating task").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    responses(
        (status = 204, description = "Task deleted")
    )
)]
pub async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting task").into_response(),
    }
}
