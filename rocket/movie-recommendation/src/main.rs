#[macro_use] extern crate rocket;

mod models;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{sqlx, Connection, Database};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::*;

#[derive(Database)]
#[database("movie_db")]
pub struct Db(sqlx::PgPool);

#[derive(OpenApi)]
#[openapi(
    paths(list_movies, create_movie, get_movie, add_rating, list_ratings),
    components(schemas(Movie, CreateMovie, Rating, CreateRating))
)]
struct ApiDoc;

#[utoipa::path(get, path = "/movies")]
#[get("/movies")]
pub async fn list_movies(mut db: Connection<Db>) -> Option<Json<Vec<Movie>>> {
    sqlx::query_as::<_, Movie>("SELECT * FROM movies ORDER BY title ASC")
        .fetch_all(&mut **db)
        .await
        .ok()
        .map(Json)
}

#[utoipa::path(post, path = "/movies", request_body = CreateMovie)]
#[post("/movies", data = "<movie>")]
pub async fn create_movie(mut db: Connection<Db>, movie: Json<CreateMovie>) -> Option<Json<Movie>> {
    sqlx::query_as::<_, Movie>(
        "INSERT INTO movies (title, genre, release_year, description) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&movie.title)
    .bind(&movie.genre)
    .bind(movie.release_year)
    .bind(&movie.description)
    .fetch_one(&mut **db)
    .await
    .ok()
    .map(Json)
}

#[utoipa::path(get, path = "/movies/{id}")]
#[get("/movies/<id>")]
pub async fn get_movie(mut db: Connection<Db>, id: i32) -> Result<Json<Movie>, Status> {
    sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?
        .map(Json)
        .ok_or(Status::NotFound)
}

#[utoipa::path(post, path = "/movies/{id}/ratings", request_body = CreateRating)]
#[post("/movies/<id>/ratings", data = "<rating>")]
pub async fn add_rating(mut db: Connection<Db>, id: i32, rating: Json<CreateRating>) -> Result<Json<Rating>, Status> {
    sqlx::query_as::<_, Rating>(
        "INSERT INTO ratings (movie_id, user_id, score, comment) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(id)
    .bind(rating.user_id)
    .bind(rating.score)
    .bind(&rating.comment)
    .fetch_one(&mut **db)
    .await
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[utoipa::path(get, path = "/movies/{id}/ratings")]
#[get("/movies/<id>/ratings")]
pub async fn list_ratings(mut db: Connection<Db>, id: i32) -> Option<Json<Vec<Rating>>> {
    sqlx::query_as::<_, Rating>("SELECT * FROM ratings WHERE movie_id = $1 ORDER BY created_at DESC")
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
        .mount("/", routes![list_movies, create_movie, get_movie, add_rating, list_ratings])
        .mount("/", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
