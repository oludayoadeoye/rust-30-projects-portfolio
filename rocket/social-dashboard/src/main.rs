#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use crate::models::*;
use common_utils::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(list_platforms, create_platform, list_metrics, add_metric),
    components(schemas(Platform, CreatePlatform, Metric, UpdateMetric))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/platforms",
    responses(
        (status = 200, description = "List all platforms", body = [Platform])
    )
)]
#[get("/platforms")]
async fn list_platforms(pool: &State<PgPool>) -> Json<Vec<Platform>> {
    let platforms = sqlx::query_as::<_, Platform>("SELECT * FROM platforms")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(platforms)
}

#[utoipa::path(
    post,
    path = "/platforms",
    request_body = CreatePlatform,
    responses(
        (status = 201, description = "Platform created", body = Platform)
    )
)]
#[post("/platforms", data = "<payload>")]
async fn create_platform(pool: &State<PgPool>, payload: Json<CreatePlatform>) -> Json<Platform> {
    let platform = sqlx::query_as::<_, Platform>(
        "INSERT INTO platforms (name, handle) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.handle)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create platform");
    Json(platform)
}

#[utoipa::path(
    get,
    path = "/platforms/{id}/metrics",
    responses(
        (status = 200, description = "List metrics for a platform", body = [Metric])
    )
)]
#[get("/platforms/<id>/metrics")]
async fn list_metrics(pool: &State<PgPool>, id: i32) -> Json<Vec<Metric>> {
    let metrics = sqlx::query_as::<_, Metric>("SELECT * FROM metrics WHERE platform_id = $1 ORDER BY recorded_at DESC")
        .bind(id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(metrics)
}

#[utoipa::path(
    post,
    path = "/platforms/{id}/metrics",
    request_body = UpdateMetric,
    responses(
        (status = 201, description = "Metric added", body = Metric)
    )
)]
#[post("/platforms/<id>/metrics", data = "<payload>")]
async fn add_metric(pool: &State<PgPool>, id: i32, payload: Json<UpdateMetric>) -> Json<Metric> {
    let metric = sqlx::query_as::<_, Metric>(
        "INSERT INTO metrics (platform_id, followers, likes, posts_count) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(id)
    .bind(payload.followers)
    .bind(payload.likes)
    .bind(payload.posts_count)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to add metric");
    Json(metric)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5478/social_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_platforms, create_platform, list_metrics, add_metric])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
