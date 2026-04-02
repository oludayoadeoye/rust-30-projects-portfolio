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
    paths(list_pages, create_page, get_page, update_page, delete_page),
    components(schemas(Page, CreatePage, UpdatePage))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/pages",
    responses(
        (status = 200, description = "List all pages", body = [Page])
    )
)]
#[get("/pages")]
async fn list_pages(pool: &State<PgPool>) -> Json<Vec<Page>> {
    let pages = sqlx::query_as::<_, Page>("SELECT * FROM pages ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(pages)
}

#[utoipa::path(
    post,
    path = "/pages",
    request_body = CreatePage,
    responses(
        (status = 201, description = "Page created", body = Page)
    )
)]
#[post("/pages", data = "<payload>")]
async fn create_page(pool: &State<PgPool>, payload: Json<CreatePage>) -> Json<Page> {
    let page = sqlx::query_as::<_, Page>(
        "INSERT INTO pages (title, slug, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.slug)
    .bind(&payload.content)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create page");
    Json(page)
}

#[utoipa::path(
    get,
    path = "/pages/{id}",
    responses(
        (status = 200, description = "Get page details", body = Page),
        (status = 404, description = "Page not found")
    )
)]
#[get("/pages/<id>")]
async fn get_page(pool: &State<PgPool>, id: i32) -> Option<Json<Page>> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.inner())
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[utoipa::path(
    put,
    path = "/pages/{id}",
    request_body = UpdatePage,
    responses(
        (status = 200, description = "Page updated", body = Page),
        (status = 404, description = "Page not found")
    )
)]
#[put("/pages/<id>", data = "<payload>")]
async fn update_page(pool: &State<PgPool>, id: i32, payload: Json<UpdatePage>) -> Option<Json<Page>> {
    let page = sqlx::query_as::<_, Page>(
        "UPDATE pages SET title = COALESCE($1, title), content = COALESCE($2, content), updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(id)
    .fetch_optional(pool.inner())
    .await
    .ok()
    .flatten()
    .map(Json);
    page
}

#[utoipa::path(
    delete,
    path = "/pages/{id}",
    responses(
        (status = 204, description = "Page deleted"),
        (status = 404, description = "Page not found")
    )
)]
#[delete("/pages/<id>")]
async fn delete_page(pool: &State<PgPool>, id: i32) -> rocket::http::Status {
    match sqlx::query("DELETE FROM pages WHERE id = $1")
        .bind(id)
        .execute(pool.inner())
        .await {
            Ok(res) if res.rows_affected() > 0 => rocket::http::Status::NoContent,
            _ => rocket::http::Status::NotFound,
        }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5495/cms_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_pages, create_page, get_page, update_page, delete_page])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
