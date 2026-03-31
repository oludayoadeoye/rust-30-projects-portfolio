#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("contact_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_messages, create_message, update_status, delete_message),
    components(schemas(ContactMessage, CreateMessage, UpdateStatus))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/messages")]
#[get("/messages")]
pub async fn list_messages(mut db: Connection<Db>) -> Option<Json<Vec<ContactMessage>>> {
    sqlx::query_as::<_, ContactMessage>("SELECT * FROM messages ORDER BY created_at DESC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/messages", request_body = CreateMessage)]
#[post("/messages", data = "<msg>")]
pub async fn create_message(mut db: Connection<Db>, msg: Json<CreateMessage>) -> Option<Json<ContactMessage>> {
    sqlx::query_as::<_, ContactMessage>(
        "INSERT INTO messages (sender_name, sender_email, subject, body) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&msg.sender_name)
    .bind(&msg.sender_email)
    .bind(&msg.subject)
    .bind(&msg.body)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(put, path = "/messages/{id}/status", request_body = UpdateStatus)]
#[put("/messages/<id>/status", data = "<status>")]
pub async fn update_status(mut db: Connection<Db>, id: i32, status: Json<UpdateStatus>) -> Option<Json<ContactMessage>> {
    sqlx::query_as::<_, ContactMessage>(
        "UPDATE messages SET status = $1 WHERE id = $2 RETURNING *"
    )
    .bind(&status.status)
    .bind(id)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(delete, path = "/messages/{id}")]
#[delete("/messages/<id>")]
pub async fn delete_message(mut db: Connection<Db>, id: i32) -> rocket::http::Status {
    match sqlx::query("DELETE FROM messages WHERE id = $1")
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
        .mount("/", routes![list_messages, create_message, update_status, delete_message])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
