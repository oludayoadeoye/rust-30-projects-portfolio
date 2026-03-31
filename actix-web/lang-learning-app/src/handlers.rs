use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Vocabulary Handlers ---

#[utoipa::path(
    get,
    path = "/vocab",
    responses(
        (status = 200, description = "List all vocabulary", body = [Vocabulary])
    )
)]
#[get("/vocab")]
pub async fn list_vocab(pool: web::Data<PgPool>) -> impl Responder {
    let vocab = sqlx::query_as::<_, Vocabulary>("SELECT * FROM vocabularies ORDER BY word")
        .fetch_all(pool.get_ref())
        .await;

    match vocab {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching vocabulary"),
    }
}

#[utoipa::path(
    post,
    path = "/vocab",
    request_body = CreateVocabulary,
    responses(
        (status = 201, description = "Vocabulary created", body = Vocabulary)
    )
)]
#[post("/vocab")]
pub async fn create_vocab(pool: web::Data<PgPool>, payload: web::Json<CreateVocabulary>) -> impl Responder {
    let v = sqlx::query_as::<_, Vocabulary>(
        "INSERT INTO vocabularies (word, translation, language) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.word)
    .bind(&payload.translation)
    .bind(&payload.language)
    .fetch_one(pool.get_ref())
    .await;

    match v {
        Ok(v) => HttpResponse::Created().json(v),
        Err(_) => HttpResponse::InternalServerError().body("Error creating vocabulary"),
    }
}

// --- Lesson Handlers ---

#[utoipa::path(
    get,
    path = "/lessons",
    responses(
        (status = 200, description = "List all lessons", body = [Lesson])
    )
)]
#[get("/lessons")]
pub async fn list_lessons(pool: web::Data<PgPool>) -> impl Responder {
    let lessons = sqlx::query_as::<_, Lesson>("SELECT * FROM lessons ORDER BY difficulty ASC")
        .fetch_all(pool.get_ref())
        .await;

    match lessons {
        Ok(l) => HttpResponse::Ok().json(l),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching lessons"),
    }
}

#[utoipa::path(
    post,
    path = "/lessons",
    request_body = CreateLesson,
    responses(
        (status = 201, description = "Lesson created", body = Lesson)
    )
)]
#[post("/lessons")]
pub async fn create_lesson(pool: web::Data<PgPool>, payload: web::Json<CreateLesson>) -> impl Responder {
    let l = sqlx::query_as::<_, Lesson>(
        "INSERT INTO lessons (title, content, difficulty) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.difficulty.unwrap_or(1))
    .fetch_one(pool.get_ref())
    .await;

    match l {
        Ok(l) => HttpResponse::Created().json(l),
        Err(_) => HttpResponse::InternalServerError().body("Error creating lesson"),
    }
}
