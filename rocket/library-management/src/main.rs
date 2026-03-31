#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{Acquire, Postgres, Transaction};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use chrono::{Utc, Duration};
use crate::models::*;

#[derive(Database)]
#[database("library_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_books, create_book, create_member, loan_book, return_book),
    components(schemas(Book, CreateBook, Member, CreateMember, Loan, LoanRequest))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/books")]
#[get("/books")]
pub async fn list_books(mut db: Connection<Db>) -> Option<Json<Vec<Book>>> {
    sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY title ASC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/books", request_body = CreateBook)]
#[post("/books", data = "<book>")]
pub async fn create_book(mut db: Connection<Db>, book: Json<CreateBook>) -> Option<Json<Book>> {
    sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, author, isbn, total_copies, available_copies) \
         VALUES ($1, $2, $3, $4, $4) RETURNING *"
    )
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .bind(book.total_copies)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(post, path = "/members", request_body = CreateMember)]
#[post("/members", data = "<member>")]
pub async fn create_member(mut db: Connection<Db>, member: Json<CreateMember>) -> Option<Json<Member>> {
    sqlx::query_as::<_, Member>(
        "INSERT INTO members (id, name, email) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&member.name)
    .bind(&member.email)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(post, path = "/loans", request_body = LoanRequest)]
#[post("/loans", data = "<req>")]
pub async fn loan_book(mut db: Connection<Db>, req: Json<LoanRequest>) -> Result<Json<Loan>, Status> {
    let mut tx: Transaction<'_, Postgres> = db.begin().await.map_err(|_| Status::InternalServerError)?;

    // 1. Check availability
    let available = sqlx::query_scalar::<_, i32>("SELECT available_copies FROM books WHERE id = $1 FOR UPDATE")
        .bind(req.book_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| Status::InternalServerError)?;

    match available {
        Some(count) if count > 0 => {
            // 2. Decrement
            sqlx::query("UPDATE books SET available_copies = available_copies - 1 WHERE id = $1")
                .bind(req.book_id)
                .execute(&mut *tx)
                .await
                .map_err(|_| Status::InternalServerError)?;

            // 3. Create loan
            let due_at = Utc::now() + Duration::days(req.days);
            let loan = sqlx::query_as::<_, Loan>(
                "INSERT INTO loans (book_id, member_id, due_at) VALUES ($1, $2, $3) RETURNING *"
            )
            .bind(req.book_id)
            .bind(req.member_id)
            .bind(due_at)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| Status::InternalServerError)?;

            tx.commit().await.map_err(|_| Status::InternalServerError)?;
            Ok(Json(loan))
        },
        Some(_) => Err(Status::Conflict),
        None => Err(Status::NotFound),
    }
}

#[utoipa::path(put, path = "/loans/{id}/return")]
#[put("/loans/<id>/return")]
pub async fn return_book(mut db: Connection<Db>, id: i32) -> Result<Status, Status> {
    let mut tx: Transaction<'_, Postgres> = db.begin().await.map_err(|_| Status::InternalServerError)?;

    let loan = sqlx::query_as::<_, (i32,)>("SELECT book_id FROM loans WHERE id = $1 AND returned_at IS NULL FOR UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(l) = loan {
        sqlx::query("UPDATE loans SET returned_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::InternalServerError)?;

        sqlx::query("UPDATE books SET available_copies = available_copies + 1 WHERE id = $1")
            .bind(l.0)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::InternalServerError)?;

        tx.commit().await.map_err(|_| Status::InternalServerError)?;
        Ok(Status::NoContent)
    } else {
        Err(Status::NotFound)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![list_books, create_book, create_member, loan_book, return_book])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
