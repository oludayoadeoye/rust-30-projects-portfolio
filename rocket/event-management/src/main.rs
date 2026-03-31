#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("event_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_events, create_event, register_attendee, list_attendees),
    components(schemas(Event, CreateEvent, Attendee, RegisterAttendee))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/events")]
#[get("/events")]
pub async fn list_events(mut db: Connection<Db>) -> Option<Json<Vec<Event>>> {
    sqlx::query_as::<_, Event>("SELECT * FROM events ORDER BY start_time ASC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/events", request_body = CreateEvent)]
#[post("/events", data = "<event>")]
pub async fn create_event(mut db: Connection<Db>, event: Json<CreateEvent>) -> Option<Json<Event>> {
    sqlx::query_as::<_, Event>(
        "INSERT INTO events (name, description, location, start_time, capacity) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&event.name)
    .bind(&event.description)
    .bind(&event.location)
    .bind(event.start_time)
    .bind(event.capacity)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(post, path = "/events/{id}/register", request_body = RegisterAttendee)]
#[post("/events/<id>/register", data = "<attendee>")]
pub async fn register_attendee(mut db: Connection<Db>, id: i32, attendee: Json<RegisterAttendee>) -> Result<Json<Attendee>, Status> {
    // 1. Check capacity
    let stats = sqlx::query_as::<_, (i32, i64)>("SELECT capacity, (SELECT COUNT(*) FROM attendees WHERE event_id = $1) FROM events WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    match stats {
        Some(s) if s.1 < s.0 as i64 => {
            // 2. Register
            let res = sqlx::query_as::<_, Attendee>(
                "INSERT INTO attendees (event_id, name, email) VALUES ($1, $2, $3) RETURNING *"
            )
            .bind(id)
            .bind(&attendee.name)
            .bind(&attendee.email)
            .fetch_one(&mut **db)
            .await;

            match res {
                Ok(a) => Ok(Json(a)),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Some(_) => Err(Status::BadRequest), // Full
        None => Err(Status::NotFound),
    }
}

#[utoipa::path(get, path = "/events/{id}/attendees")]
#[get("/events/<id>/attendees")]
pub async fn list_attendees(mut db: Connection<Db>, id: i32) -> Option<Json<Vec<Attendee>>> {
    sqlx::query_as::<_, Attendee>("SELECT * FROM attendees WHERE event_id = $1 ORDER BY registered_at ASC")
        .bind(id)
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_events, create_event, register_attendee, list_attendees])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
