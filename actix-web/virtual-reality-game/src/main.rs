mod models;
mod handlers;

use actix_web::{web, App, HttpServer, middleware};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use crate::handlers::*;
use crate::models::*;
use common_utils::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(list_players, create_player),
    components(schemas(Player, CreatePlayer))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5484/vr_game_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(middleware::Logger::default())
            .service(list_players)
            .service(create_player)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8093))?
    .run()
    .await
}
