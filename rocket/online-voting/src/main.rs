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
    paths(list_polls, create_poll, add_candidate, cast_vote, get_results),
    components(schemas(Poll, CreatePoll, Candidate, CreateCandidate, PollResult, CastVote))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/polls",
    responses(
        (status = 200, description = "List all polls", body = [Poll])
    )
)]
#[get("/polls")]
async fn list_polls(pool: &State<PgPool>) -> Json<Vec<Poll>> {
    let polls = sqlx::query_as::<_, Poll>("SELECT * FROM polls ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(polls)
}

#[utoipa::path(
    post,
    path = "/polls",
    request_body = CreatePoll,
    responses(
        (status = 201, description = "Poll created", body = Poll)
    )
)]
#[post("/polls", data = "<payload>")]
async fn create_poll(pool: &State<PgPool>, payload: Json<CreatePoll>) -> Json<Poll> {
    let poll = sqlx::query_as::<_, Poll>(
        "INSERT INTO polls (title, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create poll");
    Json(poll)
}

#[utoipa::path(
    post,
    path = "/polls/{id}/candidates",
    request_body = CreateCandidate,
    responses(
        (status = 201, description = "Candidate added", body = Candidate)
    )
)]
#[post("/polls/<id>/candidates", data = "<payload>")]
async fn add_candidate(pool: &State<PgPool>, id: i32, payload: Json<CreateCandidate>) -> Json<Candidate> {
    let candidate = sqlx::query_as::<_, Candidate>(
        "INSERT INTO candidates (poll_id, name) VALUES ($1, $2) RETURNING *"
    )
    .bind(id)
    .bind(&payload.name)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to add candidate");
    Json(candidate)
}

#[utoipa::path(
    post,
    path = "/polls/{id}/vote",
    request_body = CastVote,
    responses(
        (status = 200, description = "Vote cast")
    )
)]
#[post("/polls/<id>/vote", data = "<payload>")]
async fn cast_vote(pool: &State<PgPool>, id: i32, payload: Json<CastVote>) -> rocket::http::Status {
    let _res = sqlx::query(
        "INSERT INTO votes (poll_id, candidate_id) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(payload.candidate_id)
    .execute(pool.inner())
    .await;
    
    rocket::http::Status::Ok
}

#[utoipa::path(
    get,
    path = "/polls/{id}/results",
    responses(
        (status = 200, description = "Get poll results", body = [PollResult])
    )
)]
#[get("/polls/<id>/results")]
async fn get_results(pool: &State<PgPool>, id: i32) -> Json<Vec<PollResult>> {
    let results = sqlx::query_as::<_, PollResult>(
        "SELECT c.name as candidate_name, COUNT(v.id) as vote_count 
         FROM candidates c 
         LEFT JOIN votes v ON c.id = v.candidate_id 
         WHERE c.poll_id = $1 
         GROUP BY c.id, c.name"
    )
    .bind(id)
    .fetch_all(pool.inner())
    .await
    .unwrap_or_default();
    
    Json(results)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5494/voting_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_polls, create_poll, add_candidate, cast_vote, get_results])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
