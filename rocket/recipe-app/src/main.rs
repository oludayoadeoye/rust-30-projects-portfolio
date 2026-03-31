#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("recipe_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_recipes, create_recipe, get_recipe, delete_recipe),
    components(schemas(Recipe, CreateRecipe, UpdateRecipe))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/recipes")]
#[get("/recipes")]
pub async fn list_recipes(mut db: Connection<Db>) -> Option<Json<Vec<Recipe>>> {
    sqlx::query_as::<_, Recipe>("SELECT * FROM recipes ORDER BY id")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/recipes", request_body = CreateRecipe)]
#[post("/recipes", data = "<recipe>")]
pub async fn create_recipe(mut db: Connection<Db>, recipe: Json<CreateRecipe>) -> Option<Json<Recipe>> {
    sqlx::query_as::<_, Recipe>(
        "INSERT INTO recipes (title, instructions, ingredients) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&recipe.title)
    .bind(&recipe.instructions)
    .bind(&recipe.ingredients)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(get, path = "/recipes/{id}")]
#[get("/recipes/<id>")]
pub async fn get_recipe(mut db: Connection<Db>, id: i32) -> Option<Json<Recipe>> {
    sqlx::query_as::<_, Recipe>("SELECT * FROM recipes WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[utoipa::path(delete, path = "/recipes/{id}")]
#[delete("/recipes/<id>")]
pub async fn delete_recipe(mut db: Connection<Db>, id: i32) -> rocket::http::Status {
    match sqlx::query("DELETE FROM recipes WHERE id = $1")
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
        .mount("/", routes![list_recipes, create_recipe, get_recipe, delete_recipe])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
