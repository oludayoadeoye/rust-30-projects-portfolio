mod models;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use dotenvy::dotenv;
use crate::handlers::*;
use crate::models::*;
use common_utils::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(preview_markdown, save_snippet, list_snippets),
    components(schemas(Snippet, CreateSnippet, PreviewResponse))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5471/markdown_db".to_string());

    let pool = init_db(&database_url).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/preview", post(preview_markdown))
        .route("/snippets", get(list_snippets).post(save_snippet))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3019));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
