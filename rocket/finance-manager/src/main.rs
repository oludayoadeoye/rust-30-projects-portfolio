#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{Postgres, Transaction, Acquire};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;
use crate::models::*;

#[derive(Database)]
#[database("finance_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_accounts, create_account, add_transaction, list_transactions, get_forecast),
    components(schemas(Account, CreateAccount, TransactionRecord, CreateTransaction, ForecastResponse))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/accounts")]
#[get("/accounts")]
pub async fn list_accounts(mut db: Connection<Db>) -> Option<Json<Vec<Account>>> {
    sqlx::query_as::<_, Account>("SELECT * FROM accounts ORDER BY name")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/accounts", request_body = CreateAccount)]
#[post("/accounts", data = "<account>")]
pub async fn create_account(mut db: Connection<Db>, account: Json<CreateAccount>) -> Option<Json<Account>> {
    sqlx::query_as::<_, Account>(
        "INSERT INTO accounts (id, name, account_type) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&account.name)
    .bind(&account.account_type)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(post, path = "/accounts/{id}/transactions", request_body = CreateTransaction)]
#[post("/accounts/<id>/transactions", data = "<tx_req>")]
pub async fn add_transaction(mut db: Connection<Db>, id: Uuid, tx_req: Json<CreateTransaction>) -> Result<Json<TransactionRecord>, Status> {
    let mut tx: Transaction<'_, Postgres> = db.begin().await.map_err(|_| Status::InternalServerError)?;

    // 1. Record transaction
    let record = sqlx::query_as::<_, TransactionRecord>(
        "INSERT INTO transactions (account_id, amount, description, category) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(id)
    .bind(tx_req.amount.to_string().parse::<f64>().unwrap_or(0.0))
    .bind(&tx_req.description)
    .bind(&tx_req.category)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    // 2. Update balance
    sqlx::query("UPDATE accounts SET balance = balance + $1 WHERE id = $2")
        .bind(tx_req.amount)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|_| Status::InternalServerError)?;

    tx.commit().await.map_err(|_| Status::InternalServerError)?;
    Ok(Json(record))
}

#[utoipa::path(get, path = "/accounts/{id}/transactions")]
#[get("/accounts/<id>/transactions")]
pub async fn list_transactions(mut db: Connection<Db>, id: Uuid) -> Option<Json<Vec<TransactionRecord>>> {
    sqlx::query_as::<_, TransactionRecord>("SELECT * FROM transactions WHERE account_id = $1 ORDER BY transaction_date DESC")
        .bind(id)
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(get, path = "/accounts/{id}/forecast")]
#[get("/accounts/<id>/forecast")]
pub async fn get_forecast(mut db: Connection<Db>, id: Uuid) -> Result<Json<ForecastResponse>, Status> {
    // 1. Get current balance
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let current_balance = account.balance.to_string().parse::<f64>().unwrap_or(0.0);

    // 2. Get last 6 months of transactions to calculate monthly average
    let transactions = sqlx::query_as::<_, TransactionRecord>(
        "SELECT * FROM transactions WHERE account_id = $1 AND transaction_date > NOW() - INTERVAL '6 months' ORDER BY transaction_date DESC"
    )
    .bind(id)
    .fetch_all(&mut **db)
    .await
    .map_err(|_| Status::InternalServerError)?;

    let total_amount: f64 = transactions.iter().map(|tx| tx.amount).sum();
    let monthly_average = if transactions.is_empty() { 0.0 } else { total_amount / 6.0 };

    // 3. Project 6 months into the future
    let mut projections = Vec::new();
    let mut projected_balance = current_balance;
    for _ in 0..6 {
        projected_balance += monthly_average;
        projections.push(projected_balance);
    }

    Ok(Json(ForecastResponse {
        current_balance,
        monthly_average,
        projections,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_accounts, create_account, add_transaction, list_transactions, get_forecast])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
