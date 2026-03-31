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
    paths(list_accounts, create_account, add_transaction, list_transactions),
    components(schemas(Account, CreateAccount, TransactionRecord, CreateTransaction))
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
    .bind(tx_req.amount)
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_accounts, create_account, add_transaction, list_transactions])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
