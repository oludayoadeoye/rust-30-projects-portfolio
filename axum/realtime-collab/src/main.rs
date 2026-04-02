mod models;
mod handlers;

use axum::{
    routing::get,
    Router,
    extract::FromRef,
};
use sqlx::postgres::{PgPoolOptions, PgPool};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::sync::broadcast;
use crate::handlers::*;
use crate::models::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tx: broadcast::Sender<String>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.pool.clone()
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        list_documents,
        create_document,
        get_document,
        update_document,
        delete_document
    ),
    components(
        schemas(Document, CreateDocument, UpdateDocument)
    ),
    tags(
        (name = "collab", description = "Real-time collaboration API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5440/collab_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let (tx, _rx) = broadcast::channel(100);
    let state = AppState { pool, tx };

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/documents", get(list_documents).post(create_document))
        .route("/documents/:id", get(get_document).put(update_document).delete(delete_document))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3008));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
