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
    paths(list_topics, create_topic, list_posts, create_post),
    components(schemas(Topic, CreateTopic, Post, CreatePost))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/topics",
    responses(
        (status = 200, description = "List all forum topics", body = [Topic])
    )
)]
#[get("/topics")]
async fn list_topics(pool: &State<PgPool>) -> Json<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT * FROM topics ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(topics)
}

#[utoipa::path(
    post,
    path = "/topics",
    request_body = CreateTopic,
    responses(
        (status = 201, description = "Topic created", body = Topic)
    )
)]
#[post("/topics", data = "<payload>")]
async fn create_topic(pool: &State<PgPool>, payload: Json<CreateTopic>) -> Json<Topic> {
    let topic = sqlx::query_as::<_, Topic>(
        "INSERT INTO topics (title, author) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.author)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create topic");
    Json(topic)
}

#[utoipa::path(
    get,
    path = "/topics/{id}/posts",
    responses(
        (status = 200, description = "List posts for a topic", body = [Post])
    )
)]
#[get("/topics/<id>/posts")]
async fn list_posts(pool: &State<PgPool>, id: i32) -> Json<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE topic_id = $1 ORDER BY created_at ASC")
        .bind(id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(posts)
}

#[utoipa::path(
    post,
    path = "/topics/{id}/posts",
    request_body = CreatePost,
    responses(
        (status = 201, description = "Post created", body = Post)
    )
)]
#[post("/topics/<id>/posts", data = "<payload>")]
async fn create_post(pool: &State<PgPool>, id: i32, payload: Json<CreatePost>) -> Json<Post> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (topic_id, author, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(&payload.author)
    .bind(&payload.content)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create post");
    Json(post)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5497/forum_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_topics, create_topic, list_posts, create_post])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
