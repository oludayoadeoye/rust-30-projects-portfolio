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
        list_users,
        create_user,
        list_tasks,
        create_task,
        update_task,
        delete_task
    ),
    components(
        schemas(User, CreateUser, Task, CreateTask, UpdateTask)
    ),
    tags(
        (name = "tasks", description = "Team task management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5439/task_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/users", get(list_users).post(create_user))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(list_tasks).put(update_task).delete(delete_task))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3007));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
