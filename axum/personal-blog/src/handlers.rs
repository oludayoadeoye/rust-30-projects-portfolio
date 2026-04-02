use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Post, CreatePost};
use common_utils::ApiError;

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "List all posts", body = [Post])
    )
)]
pub async fn list_posts(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(posts))
}

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreatePost,
    responses(
        (status = 201, description = "Post created", body = Post)
    )
)]
pub async fn create_post(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePost>,
) -> Result<impl IntoResponse, ApiError> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, author) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.author)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(post)))
}

#[utoipa::path(
    get,
    path = "/posts/{id}",
    responses(
        (status = 200, description = "Get post by id", body = Post),
        (status = 404, description = "Post not found")
    )
)]
pub async fn get_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match post {
        Some(p) => Ok(Json(p)),
        None => Err(ApiError::NotFound),
    }
}
