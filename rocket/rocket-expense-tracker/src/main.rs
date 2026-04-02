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
    paths(list_categories, create_category, list_expenses, create_expense),
    components(schemas(Category, CreateCategory, Expense, CreateExpense))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/categories",
    responses(
        (status = 200, description = "List categories", body = [Category])
    )
)]
#[get("/categories")]
async fn list_categories(pool: &State<PgPool>) -> Json<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(categories)
}

#[utoipa::path(
    post,
    path = "/categories",
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Category created", body = Category)
    )
)]
#[post("/categories", data = "<payload>")]
async fn create_category(pool: &State<PgPool>, payload: Json<CreateCategory>) -> Json<Category> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name) VALUES ($1) RETURNING *"
    )
    .bind(&payload.name)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create category");
    Json(category)
}

#[utoipa::path(
    get,
    path = "/expenses",
    responses(
        (status = 200, description = "List expenses", body = [Expense])
    )
)]
#[get("/expenses")]
async fn list_expenses(pool: &State<PgPool>) -> Json<Vec<Expense>> {
    let expenses = sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY expense_date DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(expenses)
}

#[utoipa::path(
    post,
    path = "/expenses",
    request_body = CreateExpense,
    responses(
        (status = 201, description = "Expense created", body = Expense)
    )
)]
#[post("/expenses", data = "<payload>")]
async fn create_expense(pool: &State<PgPool>, payload: Json<CreateExpense>) -> Json<Expense> {
    let expense = sqlx::query_as::<_, Expense>(
        "INSERT INTO expenses (category_id, amount, description, expense_date) \
         VALUES ($1, $2, $3, COALESCE($4, CURRENT_DATE)) RETURNING *"
    )
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(&payload.description)
    .bind(payload.expense_date)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create expense");
    Json(expense)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5476/expense_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_categories, create_category, list_expenses, create_expense])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
