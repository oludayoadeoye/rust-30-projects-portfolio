#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("tip_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_bills, create_bill, get_stats),
    components(schemas(BillRecord, CreateBillRecord, BillStats))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/bills")]
#[get("/bills")]
pub async fn list_bills(mut db: Connection<Db>) -> Option<Json<Vec<BillRecord>>> {
    sqlx::query_as::<_, BillRecord>("SELECT * FROM bill_records ORDER BY created_at DESC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/bills", request_body = CreateBillRecord)]
#[post("/bills", data = "<bill>")]
pub async fn create_bill(mut db: Connection<Db>, bill: Json<CreateBillRecord>) -> Option<Json<BillRecord>> {
    let tip_amount = bill.total_bill * (bill.tip_percentage / 100.0);
    let total_per_person = (bill.total_bill + tip_amount) / (bill.num_people as f64);

    sqlx::query_as::<_, BillRecord>(
        "INSERT INTO bill_records (total_bill, tip_percentage, num_people, tip_amount, total_per_person) \
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(bill.total_bill)
    .bind(bill.tip_percentage)
    .bind(bill.num_people)
    .bind(tip_amount)
    .bind(total_per_person)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(get, path = "/bills/stats")]
#[get("/bills/stats")]
pub async fn get_stats(mut db: Connection<Db>) -> Option<Json<BillStats>> {
    let result = sqlx::query_as::<_, (Option<f64>, Option<f64>, i64)>("SELECT SUM(total_bill), SUM(tip_amount), COUNT(*) FROM bill_records")
        .fetch_one(&mut **db)
        .await
        .ok()?;

    Some(Json(BillStats {
        total_revenue: result.0.unwrap_or(0.0),
        total_tips: result.1.unwrap_or(0.0),
        record_count: result.2,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_bills, create_bill, get_stats])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
