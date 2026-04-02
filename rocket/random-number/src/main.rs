#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use rand::Rng;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use crate::models::*;
use common_utils::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(generate_number, list_history),
    components(schemas(GeneratedNumber, RandomRequest))
)]
struct ApiDoc;

#[utoipa::path(
    post,
    path = "/generate",
    request_body = RandomRequest,
    responses(
        (status = 201, description = "Number generated", body = GeneratedNumber)
    )
)]
#[post("/generate", data = "<payload>")]
async fn generate_number(pool: &State<PgPool>, payload: Json<RandomRequest>) -> Json<GeneratedNumber> {
    let val = {
        let mut rng = rand::thread_rng();
        rng.gen_range(payload.min..=payload.max)
    };

    let number = sqlx::query_as::<_, GeneratedNumber>(
        "INSERT INTO generated_numbers (value, min_range, max_range) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(val)
    .bind(payload.min)
    .bind(payload.max)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to insert number");

    Json(number)
}

#[utoipa::path(
    get,
    path = "/history",
    responses(
        (status = 200, description = "List history", body = [GeneratedNumber])
    )
)]
#[get("/history")]
async fn list_history(pool: &State<PgPool>) -> Json<Vec<GeneratedNumber>> {
    let history = sqlx::query_as::<_, GeneratedNumber>("SELECT * FROM generated_numbers ORDER BY generated_at DESC")
        .fetch_all(pool.inner())
        .await
        .expect("Failed to fetch history");

    Json(history)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5475/random_db".to_string());

    let pool = init_db(&database_url).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![generate_number, list_history])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
