#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("bmi_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_records, create_record, get_stats),
    components(schemas(BmiRecord, CreateBmiRecord, BmiStats))
)]
struct ApiDoc;

fn calculate_bmi_category(bmi: f64) -> String {
    if bmi < 18.5 { "Underweight".to_string() }
    else if bmi < 25.0 { "Normal weight".to_string() }
    else if bmi < 30.0 { "Overweight".to_string() }
    else { "Obesity".to_string() }
}

#[utoipa::path(get, path = "/bmi")]
#[get("/bmi")]
pub async fn list_records(mut db: Connection<Db>) -> Option<Json<Vec<BmiRecord>>> {
    sqlx::query_as::<_, BmiRecord>("SELECT * FROM bmi_records ORDER BY recorded_at DESC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/bmi", request_body = CreateBmiRecord)]
#[post("/bmi", data = "<record>")]
pub async fn create_record(mut db: Connection<Db>, record: Json<CreateBmiRecord>) -> Option<Json<BmiRecord>> {
    let bmi = record.weight_kg / ((record.height_cm / 100.0).powi(2));
    let category = calculate_bmi_category(bmi);

    sqlx::query_as::<_, BmiRecord>(
        "INSERT INTO bmi_records (user_name, height_cm, weight_kg, bmi, category) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&record.user_name)
    .bind(record.height_cm)
    .bind(record.weight_kg)
    .bind(bmi)
    .bind(category)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(get, path = "/bmi/stats")]
#[get("/bmi/stats")]
pub async fn get_stats(mut db: Connection<Db>) -> Option<Json<BmiStats>> {
    let result = sqlx::query_as::<_, (Option<f64>, i64)>("SELECT AVG(bmi), COUNT(*) FROM bmi_records")
        .fetch_one(&mut **db)
        .await
        .ok()?;

    Some(Json(BmiStats {
        average_bmi: result.0.unwrap_or(0.0),
        total_records: result.1,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_records, create_record, get_stats])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
