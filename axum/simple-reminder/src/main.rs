mod models;
mod handlers;

use axum::{
    routing::{get, put},
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
    paths(list_reminders, create_reminder, complete_reminder),
    components(schemas(Reminder, CreateReminder))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5469/reminder_db".to_string());

    let pool = init_db(&database_url).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/reminders", get(list_reminders).post(create_reminder))
        .route("/reminders/:id/complete", put(complete_reminder))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3017));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
