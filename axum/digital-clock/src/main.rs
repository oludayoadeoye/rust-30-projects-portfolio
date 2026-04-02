mod models;
mod handlers;

use axum::{
    routing::get,
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
    paths(get_current_time, list_alarms, create_alarm),
    components(schemas(Alarm, CreateAlarm, TimeResponse))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5467/clock_db".to_string());

    let pool = init_db(&database_url).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/time", get(get_current_time))
        .route("/alarms", get(list_alarms).post(create_alarm))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3015));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
