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
    paths(list_scenes, create_scene, add_anchor, get_anchors),
    components(schemas(ARScene, CreateScene, Anchor, CreateAnchor))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/scenes",
    responses(
        (status = 200, description = "List all AR scenes", body = [ARScene])
    )
)]
#[get("/scenes")]
async fn list_scenes(pool: &State<PgPool>) -> Json<Vec<ARScene>> {
    let scenes = sqlx::query_as::<_, ARScene>("SELECT * FROM ar_scenes ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(scenes)
}

#[utoipa::path(
    post,
    path = "/scenes",
    request_body = CreateScene,
    responses(
        (status = 201, description = "AR Scene created", body = ARScene)
    )
)]
#[post("/scenes", data = "<payload>")]
async fn create_scene(pool: &State<PgPool>, payload: Json<CreateScene>) -> Json<ARScene> {
    let scene = sqlx::query_as::<_, ARScene>(
        "INSERT INTO ar_scenes (name, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create scene");
    Json(scene)
}

#[utoipa::path(
    get,
    path = "/scenes/{id}/anchors",
    responses(
        (status = 200, description = "List anchors for a scene", body = [Anchor])
    )
)]
#[get("/scenes/<id>/anchors")]
async fn get_anchors(pool: &State<PgPool>, id: i32) -> Json<Vec<Anchor>> {
    let anchors = sqlx::query_as::<_, Anchor>("SELECT * FROM ar_anchors WHERE scene_id = $1")
        .bind(id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(anchors)
}

#[utoipa::path(
    post,
    path = "/scenes/{id}/anchors",
    request_body = CreateAnchor,
    responses(
        (status = 201, description = "Anchor added", body = Anchor)
    )
)]
#[post("/scenes/<id>/anchors", data = "<payload>")]
async fn add_anchor(pool: &State<PgPool>, id: i32, payload: Json<CreateAnchor>) -> Json<Anchor> {
    let anchor = sqlx::query_as::<_, Anchor>(
        "INSERT INTO ar_anchors (scene_id, spatial_data, label) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(&payload.spatial_data)
    .bind(&payload.label)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to add anchor");
    Json(anchor)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5498/ar_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_scenes, create_scene, get_anchors, add_anchor])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
