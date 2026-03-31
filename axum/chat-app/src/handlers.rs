use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Room, CreateRoom, Message, CreateMessage};

#[utoipa::path(
    get,
    path = "/rooms",
    responses(
        (status = 200, description = "List all chat rooms", body = [Room])
    )
)]
pub async fn list_rooms(State(pool): State<PgPool>) -> impl IntoResponse {
    let rooms = sqlx::query_as::<_, Room>("SELECT * FROM rooms ORDER BY name")
        .fetch_all(&pool)
        .await;

    match rooms {
        Ok(rooms) => (StatusCode::OK, Json(rooms)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching rooms").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/rooms",
    request_body = CreateRoom,
    responses(
        (status = 201, description = "Room created", body = Room)
    )
)]
pub async fn create_room(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateRoom>,
) -> impl IntoResponse {
    let room = sqlx::query_as::<_, Room>(
        "INSERT INTO rooms (name) VALUES ($1) RETURNING *"
    )
    .bind(payload.name)
    .fetch_one(&pool)
    .await;

    match room {
        Ok(room) => (StatusCode::CREATED, Json(room)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating room").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/rooms/{id}/messages",
    responses(
        (status = 200, description = "List messages in a room", body = [Message]),
        (status = 404, description = "Room not found")
    )
)]
pub async fn list_messages(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let messages = sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE room_id = $1 ORDER BY sent_at ASC")
        .bind(id)
        .fetch_all(&pool)
        .await;

    match messages {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching messages").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/rooms/{id}/messages",
    request_body = CreateMessage,
    responses(
        (status = 201, description = "Message sent", body = Message),
        (status = 404, description = "Room not found")
    )
)]
pub async fn create_message(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateMessage>,
) -> impl IntoResponse {
    let message = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (room_id, sender, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(payload.sender)
    .bind(payload.content)
    .fetch_one(&pool)
    .await;

    match message {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error sending message").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/rooms/{id}",
    responses(
        (status = 204, description = "Room deleted")
    )
)]
pub async fn delete_room(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM rooms WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting room").into_response(),
    }
}
