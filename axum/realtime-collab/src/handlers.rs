use axum::{
    extract::{Path, State, ws::{WebSocket, WebSocketUpgrade, Message}},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use futures_util::{StreamExt, SinkExt};
use crate::models::{Document, CreateDocument, UpdateDocument};
use crate::AppState;

#[utoipa::path(
    get,
    path = "/documents",
    responses(
        (status = 200, description = "List all documents", body = [Document])
    )
)]
pub async fn list_documents(State(pool): State<PgPool>) -> impl IntoResponse {
    let docs = sqlx::query_as::<_, Document>("SELECT * FROM documents ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await;

    match docs {
        Ok(docs) => (StatusCode::OK, Json(docs)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching documents").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/documents",
    request_body = CreateDocument,
    responses(
        (status = 201, description = "Document created", body = Document)
    )
)]
pub async fn create_document(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateDocument>,
) -> impl IntoResponse {
    let doc = sqlx::query_as::<_, Document>(
        "INSERT INTO documents (id, title, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(payload.title)
    .bind(payload.content.unwrap_or_default())
    .fetch_one(&pool)
    .await;

    match doc {
        Ok(doc) => (StatusCode::CREATED, Json(doc)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating document").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/documents/{id}",
    responses(
        (status = 200, description = "Get document by id", body = Document),
        (status = 404, description = "Document not found")
    )
)]
pub async fn get_document(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let doc = sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match doc {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Document not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching document").into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/documents/{id}",
    request_body = UpdateDocument,
    responses(
        (status = 200, description = "Document updated", body = Document),
        (status = 404, description = "Document not found or version mismatch"),
        (status = 409, description = "Conflict: version mismatch")
    )
)]
pub async fn update_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateDocument>,
) -> impl IntoResponse {
    // Optimistic locking using version
    let doc = sqlx::query_as::<_, Document>(
        "UPDATE documents SET title = COALESCE($1, title), content = COALESCE($2, content), \
         version = version + 1, updated_at = NOW() \
         WHERE id = $3 AND version = $4 RETURNING *"
    )
    .bind(payload.title)
    .bind(&payload.content)
    .bind(id)
    .bind(payload.version)
    .fetch_optional(&state.pool)
    .await;

    match doc {
        Ok(Some(doc)) => {
            // Broadcast the update to all connected clients
            let msg = format!("update:{}", id);
            let _ = state.tx.send(msg);
            (StatusCode::OK, Json(doc)).into_response()
        },
        Ok(None) => (StatusCode::CONFLICT, "Version mismatch or document not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error updating document").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/documents/{id}",
    responses(
        (status = 204, description = "Document deleted")
    )
)]
pub async fn delete_document(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM documents WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting document").into_response(),
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Broadcast received messages (e.g. cursor positions, ephemeral edits)
            let _ = state.tx.send(text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
