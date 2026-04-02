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
    paths(analyze_text, list_analyses),
    components(schemas(NlpAnalysis, AnalyzeRequest))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/analyses",
    responses(
        (status = 200, description = "List all NLP analyses", body = [NlpAnalysis])
    )
)]
#[get("/analyses")]
async fn list_analyses(pool: &State<PgPool>) -> Json<Vec<NlpAnalysis>> {
    let analyses = sqlx::query_as::<_, NlpAnalysis>("SELECT * FROM nlp_analyses ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(analyses)
}

#[utoipa::path(
    post,
    path = "/analyze",
    request_body = AnalyzeRequest,
    responses(
        (status = 201, description = "Text analyzed", body = NlpAnalysis)
    )
)]
#[post("/analyze", data = "<payload>")]
async fn analyze_text(pool: &State<PgPool>, payload: Json<AnalyzeRequest>) -> Json<NlpAnalysis> {
    // Simulate NLP analysis
    let sentiment = if payload.text.contains("good") || payload.text.contains("happy") {
        "Positive"
    } else if payload.text.contains("bad") || payload.text.contains("sad") {
        "Negative"
    } else {
        "Neutral"
    };

    let analysis = sqlx::query_as::<_, NlpAnalysis>(
        "INSERT INTO nlp_analyses (input_text, sentiment, language) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.text)
    .bind(sentiment)
    .bind("en")
    .fetch_one(pool.inner())
    .await
    .expect("Failed to save analysis");
    Json(analysis)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5496/nlp_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_analyses, analyze_text])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
