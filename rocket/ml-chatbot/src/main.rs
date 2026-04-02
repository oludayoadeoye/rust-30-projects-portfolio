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
    paths(create_session, get_session, send_message, list_sessions),
    components(schemas(ChatSession, CreateSession, Message, SendMessage))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/sessions",
    responses(
        (status = 200, description = "List all chat sessions", body = [ChatSession])
    )
)]
#[get("/sessions")]
async fn list_sessions(pool: &State<PgPool>) -> Json<Vec<ChatSession>> {
    let sessions = sqlx::query_as::<_, ChatSession>("SELECT * FROM chat_sessions")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(sessions)
}

#[utoipa::path(
    post,
    path = "/sessions",
    request_body = CreateSession,
    responses(
        (status = 201, description = "Session created", body = ChatSession)
    )
)]
#[post("/sessions", data = "<payload>")]
async fn create_session(pool: &State<PgPool>, payload: Json<CreateSession>) -> Json<ChatSession> {
    let session = sqlx::query_as::<_, ChatSession>(
        "INSERT INTO chat_sessions (user_id) VALUES ($1) RETURNING *"
    )
    .bind(&payload.user_id)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create session");
    Json(session)
}

#[utoipa::path(
    get,
    path = "/sessions/{id}",
    responses(
        (status = 200, description = "Get session details", body = ChatSession),
        (status = 404, description = "Session not found")
    )
)]
#[get("/sessions/<id>")]
async fn get_session(pool: &State<PgPool>, id: i32) -> Option<Json<ChatSession>> {
    sqlx::query_as::<_, ChatSession>("SELECT * FROM chat_sessions WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.inner())
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[utoipa::path(
    post,
    path = "/sessions/{id}/messages",
    request_body = SendMessage,
    responses(
        (status = 201, description = "Message sent", body = Message)
    )
)]
#[post("/sessions/<id>/messages", data = "<payload>")]
async fn send_message(pool: &State<PgPool>, id: i32, payload: Json<SendMessage>) -> Json<Message> {
    // 1. Save user message
    sqlx::query(
        "INSERT INTO messages (session_id, role, content) VALUES ($1, $2, $3)"
    )
    .bind(id)
    .bind("user")
    .bind(&payload.content)
    .execute(pool.inner())
    .await
    .expect("Failed to save user message");

    // 2. Perform Sentiment Analysis (Deep Logic)
    let content_lower = payload.content.to_lowercase();
    let sentiment = if content_lower.contains("happy") || content_lower.contains("great") || content_lower.contains("good") {
        "Positive"
    } else if content_lower.contains("sad") || content_lower.contains("bad") || content_lower.contains("angry") {
        "Negative"
    } else {
        "Neutral"
    };

    let ai_response = match sentiment {
        "Positive" => format!("I'm glad to hear that! How can I further assist with your {} experience?", payload.content),
        "Negative" => format!("I'm sorry you're feeling this way about {}. Is there anything I can do to help?", payload.content),
        _ => format!("Interesting. Tell me more about: {}", payload.content),
    };

    // 3. Save AI message
    let ai_msg = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (session_id, role, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind("assistant")
    .bind(ai_response)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to save AI message");

    Json(ai_msg)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5492/chatbot_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_sessions, create_session, get_session, send_message])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
