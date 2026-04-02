use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::*;
use common_utils::ApiError;

// --- Category Handlers ---

#[utoipa::path(
    get,
    path = "/categories",
    responses(
        (status = 200, description = "List all categories", body = [Category])
    )
)]
pub async fn list_categories(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
        .fetch_all(&pool)
        .await?;
    Ok(Json(categories))
}

#[utoipa::path(
    post,
    path = "/categories",
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Category created", body = Category)
    )
)]
pub async fn create_category(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCategory>,
) -> Result<impl IntoResponse, ApiError> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name) VALUES ($1) RETURNING *"
    )
    .bind(&payload.name)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(category)))
}

// --- Expense Handlers ---

#[utoipa::path(
    get,
    path = "/expenses",
    responses(
        (status = 200, description = "List all expenses", body = [Expense])
    )
)]
pub async fn list_expenses(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, ApiError> {
    let expenses = sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY expense_date DESC")
        .fetch_all(&pool)
        .await?;
    Ok(Json(expenses))
}

#[utoipa::path(
    post,
    path = "/expenses",
    request_body = CreateExpense,
    responses(
        (status = 201, description = "Expense created", body = Expense)
    )
)]
pub async fn create_expense(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateExpense>,
) -> Result<impl IntoResponse, ApiError> {
    let expense = sqlx::query_as::<_, Expense>(
        "INSERT INTO expenses (category_id, amount, description, expense_date) \
         VALUES ($1, $2, $3, COALESCE($4, CURRENT_DATE)) RETURNING *"
    )
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(&payload.description)
    .bind(payload.expense_date)
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(expense)))
}
