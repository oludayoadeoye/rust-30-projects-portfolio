#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("quote_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_quotes, create_quote, get_random_quote, delete_quote),
    components(schemas(Quote, CreateQuote, UpdateQuote))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/quotes")]
#[get("/quotes")]
pub async fn list_quotes(mut db: Connection<Db>) -> Option<Json<Vec<Quote>>> {
    sqlx::query_as::<_, Quote>("SELECT * FROM quotes ORDER BY id")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/quotes", request_body = CreateQuote)]
#[post("/quotes", data = "<quote>")]
pub async fn create_quote(mut db: Connection<Db>, quote: Json<CreateQuote>) -> Option<Json<Quote>> {
    sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (text, author, category) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&quote.text)
    .bind(&quote.author)
    .bind(&quote.category)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(get, path = "/quotes/random")]
#[get("/quotes/random")]
pub async fn get_random_quote(mut db: Connection<Db>) -> Option<Json<Quote>> {
    sqlx::query_as::<_, Quote>("SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1")
        .fetch_one(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(delete, path = "/quotes/{id}")]
#[delete("/quotes/<id>")]
pub async fn delete_quote(mut db: Connection<Db>, id: i32) -> rocket::http::Status {
    match sqlx::query("DELETE FROM quotes WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await {
            Ok(res) if res.rows_affected() > 0 => rocket::http::Status::NoContent,
            _ => rocket::http::Status::NotFound,
        }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_quotes, create_quote, get_random_quote, delete_quote])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
