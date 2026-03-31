#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{Acquire, Postgres, Transaction};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("home_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_devices, create_device, update_device_state, list_logs),
    components(schemas(Device, CreateDevice, UpdateState, DeviceLog))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/devices")]
#[get("/devices")]
pub async fn list_devices(mut db: Connection<Db>) -> Option<Json<Vec<Device>>> {
    sqlx::query_as::<_, Device>("SELECT * FROM devices ORDER BY name")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/devices", request_body = CreateDevice)]
#[post("/devices", data = "<device>")]
pub async fn create_device(mut db: Connection<Db>, device: Json<CreateDevice>) -> Option<Json<Device>> {
    sqlx::query_as::<_, Device>(
        "INSERT INTO devices (id, name, device_type, state) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&device.name)
    .bind(&device.device_type)
    .bind(&device.state)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(put, path = "/devices/{id}/state", request_body = UpdateState)]
#[put("/devices/<id>/state", data = "<payload>")]
pub async fn update_device_state(mut db: Connection<Db>, id: uuid::Uuid, payload: Json<UpdateState>) -> Result<Json<Device>, Status> {
    let mut tx: Transaction<'_, Postgres> = db.begin().await.map_err(|_| Status::InternalServerError)?;

    // 1. Get old state
    let old_state = sqlx::query_scalar::<_, String>("SELECT state FROM devices WHERE id = $1 FOR UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(old) = old_state {
        // 2. Update state
        let device = sqlx::query_as::<_, Device>(
            "UPDATE devices SET state = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(&payload.state)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Status::InternalServerError)?;

        // 3. Log change
        sqlx::query("INSERT INTO device_logs (device_id, old_state, new_state) VALUES ($1, $2, $3)")
            .bind(id)
            .bind(old)
            .bind(&payload.state)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::InternalServerError)?;

        tx.commit().await.map_err(|_| Status::InternalServerError)?;
        Ok(Json(device))
    } else {
        Err(Status::NotFound)
    }
}

#[utoipa::path(get, path = "/devices/{id}/logs")]
#[get("/devices/<id>/logs")]
pub async fn list_logs(mut db: Connection<Db>, id: uuid::Uuid) -> Option<Json<Vec<DeviceLog>>> {
    sqlx::query_as::<_, DeviceLog>("SELECT * FROM device_logs WHERE device_id = $1 ORDER BY recorded_at DESC")
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
        .mount("/", routes![list_devices, create_device, update_device_state, list_logs])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
