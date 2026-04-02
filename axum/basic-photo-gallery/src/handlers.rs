use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

// --- Album Handlers ---

#[utoipa::path(
    get,
    path = "/albums",
    responses(
        (status = 200, description = "List all albums", body = [Album])
    )
)]
pub async fn list_albums(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let albums = sqlx::query_as::<_, Album>("SELECT * FROM albums ORDER BY name")
        .fetch_all(&pool)
        .await?;
    Ok(Json(albums))
}

#[utoipa::path(
    post,
    path = "/albums",
    request_body = CreateAlbum,
    responses(
        (status = 201, description = "Album created", body = Album)
    )
)]
pub async fn create_album(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAlbum>,
) -> Result<impl IntoResponse, ApiError> {
    let album = sqlx::query_as::<_, Album>(
        "INSERT INTO albums (name, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(album)))
}

// --- Photo Handlers ---

#[utoipa::path(
    get,
    path = "/albums/{id}/photos",
    responses(
        (status = 200, description = "List photos in an album", body = [Photo])
    )
)]
pub async fn list_photos(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let photos = sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE album_id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(photos))
}

#[utoipa::path(
    post,
    path = "/albums/{id}/photos",
    request_body = CreatePhoto,
    responses(
        (status = 201, description = "Photo added to album", body = Photo)
    )
)]
pub async fn add_photo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreatePhoto>,
) -> Result<impl IntoResponse, ApiError> {
    let photo = sqlx::query_as::<_, Photo>(
        "INSERT INTO photos (album_id, title, url) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(&payload.title)
    .bind(&payload.url)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(photo)))
}
