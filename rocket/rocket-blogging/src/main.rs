#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::*;
use common_utils::init_db;

#[derive(OpenApi)]
#[openapi(
    paths(list_users, create_user, list_posts, create_post),
    components(schemas(User, CreateUser, BlogPost, CreateBlogPost))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    )
)]
#[get("/users")]
async fn list_users(pool: &State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, created_at FROM users")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(users)
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
#[post("/users", data = "<payload>")]
async fn create_user(pool: &State<PgPool>, payload: Json<CreateUser>) -> Json<User> {
    let password_hash = hash(&payload.password, DEFAULT_COST).expect("Failed to hash password");
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, created_at"
    )
    .bind(&payload.username)
    .bind(password_hash)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create user");
    Json(user)
}

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "List all blog posts", body = [BlogPost])
    )
)]
#[get("/posts")]
async fn list_posts(pool: &State<PgPool>) -> Json<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM posts ORDER BY published_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(posts)
}

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreateBlogPost,
    responses(
        (status = 201, description = "Post created", body = BlogPost)
    )
)]
#[post("/posts", data = "<payload>")]
async fn create_post(pool: &State<PgPool>, payload: Json<CreateBlogPost>) -> Json<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>(
        "INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(payload.user_id)
    .bind(&payload.title)
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
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5477/blogging_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_users, create_user, list_posts, create_post])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
