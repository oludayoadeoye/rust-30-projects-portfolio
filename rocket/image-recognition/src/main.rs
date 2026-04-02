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
    paths(upload_image, get_analysis, list_analyses),
    components(schemas(ImageAnalysis, UploadImage, ImageTag))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/analyses",
    responses(
        (status = 200, description = "List all image analyses", body = [ImageAnalysis])
    )
)]
#[get("/analyses")]
async fn list_analyses(pool: &State<PgPool>) -> Json<Vec<ImageAnalysis>> {
    let analyses = sqlx::query_as::<_, ImageAnalysis>("SELECT * FROM image_analyses")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(analyses)
}

#[utoipa::path(
    post,
    path = "/analyses",
    request_body = UploadImage,
    responses(
        (status = 201, description = "Image analysis started", body = ImageAnalysis)
    )
)]
#[post("/analyses", data = "<payload>")]
async fn upload_image(pool: &State<PgPool>, payload: Json<UploadImage>) -> Json<ImageAnalysis> {
    let analysis = sqlx::query_as::<_, ImageAnalysis>(
        "INSERT INTO image_analyses (image_url, status) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.image_url)
    .bind("Completed")
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create analysis");

    // Simulate recognition tags
    let _tag1 = sqlx::query(
        "INSERT INTO image_tags (analysis_id, label, confidence) VALUES ($1, $2, $3)"
    )
    .bind(analysis.id)
    .bind("Nature")
    .bind(0.98)
    .execute(pool.inner())
    .await;

    let _tag2 = sqlx::query(
        "INSERT INTO image_tags (analysis_id, label, confidence) VALUES ($1, $2, $3)"
    )
    .bind(analysis.id)
    .bind("Forest")
    .bind(0.85)
    .execute(pool.inner())
    .await;

    Json(analysis)
}

#[utoipa::path(
    get,
    path = "/analyses/{id}",
    responses(
        (status = 200, description = "Get analysis details with tags", body = ImageAnalysis),
        (status = 404, description = "Analysis not found")
    )
)]
#[get("/analyses/<id>")]
async fn get_analysis(pool: &State<PgPool>, id: i32) -> Option<Json<ImageAnalysis>> {
    sqlx::query_as::<_, ImageAnalysis>("SELECT * FROM image_analyses WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.inner())
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5493/image_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_analyses, upload_image, get_analysis])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
