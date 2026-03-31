use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use serde::Deserialize;
use crate::models::{Note, CreateNote, UpdateNote};

#[derive(Deserialize)]
pub struct TagQuery {
    pub tag: Option<String>,
}

#[utoipa::path(
    get,
    path = "/notes",
    params(
        ("tag" = Option<String>, Query, description = "Filter by tag")
    ),
    responses(
        (status = 200, description = "List notes", body = [Note])
    )
)]
pub async fn list_notes(
    State(pool): State<PgPool>,
    Query(query): Query<TagQuery>,
) -> impl IntoResponse {
    let notes = if let Some(tag) = query.tag {
        sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE $1 = ANY(tags) ORDER BY updated_at DESC")
            .bind(tag)
            .fetch_all(&pool)
            .await
    } else {
        sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY updated_at DESC")
            .fetch_all(&pool)
            .await
    };

    match notes {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching notes").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/notes",
    request_body = CreateNote,
    responses(
        (status = 201, description = "Note created", body = Note)
    )
)]
pub async fn create_note(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNote>,
) -> impl IntoResponse {
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (title, content, tags) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.tags.unwrap_or_default())
    .fetch_one(&pool)
    .await;

    match note {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating note").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/notes/{id}",
    responses(
        (status = 200, description = "Get note by id", body = Note),
        (status = 404, description = "Note not found")
    )
)]
pub async fn get_note(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match note {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Note not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching note").into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/notes/{id}",
    request_body = UpdateNote,
    responses(
        (status = 200, description = "Note updated", body = Note),
        (status = 404, description = "Note not found")
    )
)]
pub async fn update_note(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    let note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET title = COALESCE($1, title), content = COALESCE($2, content), tags = COALESCE($3, tags), updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.tags)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match note {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Note not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating note").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/notes/{id}",
    responses(
        (status = 204, description = "Note deleted"),
        (status = 404, description = "Note not found")
    )
)]
pub async fn delete_note(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Note not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting note").into_response(),
    }
}
