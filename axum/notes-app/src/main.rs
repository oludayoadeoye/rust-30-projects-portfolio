mod models;
mod handlers;

use axum::{
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use dotenvy::dotenv;
use crate::handlers::*;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        list_notes,
        create_note,
        get_note,
        update_note,
        delete_note
    ),
    components(
        schemas(Note, CreateNote, UpdateNote)
    ),
    tags(
        (name = "notes", description = "Notes management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5436/notes_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/notes", get(list_notes).post(create_note))
        .route("/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3004));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
