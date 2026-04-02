use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use pulldown_cmark::{html, Options, Parser};
use crate::models::*;
use common_utils::ApiError;

fn render_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[utoipa::path(
    post,
    path = "/preview",
    request_body = CreateSnippet,
    responses(
        (status = 200, description = "Render markdown preview", body = PreviewResponse)
    )
)]
pub async fn preview_markdown(
    Json(payload): Json<CreateSnippet>,
) -> impl IntoResponse {
    let html = render_markdown(&payload.content);
    Json(PreviewResponse { html })
}

#[utoipa::path(
    post,
    path = "/snippets",
    request_body = CreateSnippet,
    responses(
        (status = 201, description = "Snippet saved", body = Snippet)
    )
)]
pub async fn save_snippet(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSnippet>,
) -> Result<impl IntoResponse, ApiError> {
    let html_content = render_markdown(&payload.content);
    let snippet = sqlx::query_as::<_, Snippet>(
        "INSERT INTO snippets (title, content, html_content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(html_content)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(snippet)))
}

#[utoipa::path(
    get,
    path = "/snippets",
    responses(
        (status = 200, description = "List all snippets", body = [Snippet])
    )
)]
pub async fn list_snippets(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let s = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(s))
}
