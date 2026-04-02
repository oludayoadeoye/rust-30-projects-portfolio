use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

// --- Author Handlers ---

#[utoipa::path(
    get,
    path = "/authors",
    responses(
        (status = 200, description = "List all authors", body = [Author])
    )
)]
pub async fn list_authors(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let authors = sqlx::query_as::<_, Author>("SELECT * FROM authors ORDER BY name")
        .fetch_all(&pool)
        .await?;
    Ok(Json(authors))
}

#[utoipa::path(
    post,
    path = "/authors",
    request_body = CreateAuthor,
    responses(
        (status = 201, description = "Author created", body = Author)
    )
)]
pub async fn create_author(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateAuthor>,
) -> Result<impl IntoResponse, ApiError> {
    let author = sqlx::query_as::<_, Author>(
        "INSERT INTO authors (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(author)))
}

// --- Post Handlers ---

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "List all blog posts", body = [BlogPost])
    )
)]
pub async fn list_blog_posts(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM posts ORDER BY published_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(posts))
}

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreateBlogPost,
    responses(
        (status = 201, description = "Post created", body = BlogPost)
    )
)]
pub async fn create_blog_post(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBlogPost>,
) -> Result<impl IntoResponse, ApiError> {
    let post = sqlx::query_as::<_, BlogPost>(
        "INSERT INTO posts (author_id, title, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(payload.author_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(post)))
}
