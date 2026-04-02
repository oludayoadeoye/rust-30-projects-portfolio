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
    paths(list_projects, create_project, list_skills, add_skill),
    components(schemas(Project, CreateProject, Skill, AddSkill))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/projects",
    responses(
        (status = 200, description = "List all portfolio projects", body = [Project])
    )
)]
#[get("/projects")]
async fn list_projects(pool: &State<PgPool>) -> Json<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM portfolio_projects ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(projects)
}

#[utoipa::path(
    post,
    path = "/projects",
    request_body = CreateProject,
    responses(
        (status = 201, description = "Project created", body = Project)
    )
)]
#[post("/projects", data = "<payload>")]
async fn create_project(pool: &State<PgPool>, payload: Json<CreateProject>) -> Json<Project> {
    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO portfolio_projects (title, description, tech_stack, link) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.tech_stack)
    .bind(&payload.link)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to create project");
    Json(project)
}

#[utoipa::path(
    get,
    path = "/skills",
    responses(
        (status = 200, description = "List all skills", body = [Skill])
    )
)]
#[get("/skills")]
async fn list_skills(pool: &State<PgPool>) -> Json<Vec<Skill>> {
    let skills = sqlx::query_as::<_, Skill>("SELECT * FROM portfolio_skills")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();
    Json(skills)
}

#[utoipa::path(
    post,
    path = "/skills",
    request_body = AddSkill,
    responses(
        (status = 201, description = "Skill added", body = Skill)
    )
)]
#[post("/skills", data = "<payload>")]
async fn add_skill(pool: &State<PgPool>, payload: Json<AddSkill>) -> Json<Skill> {
    let skill = sqlx::query_as::<_, Skill>(
        "INSERT INTO portfolio_skills (name, level) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.level)
    .fetch_one(pool.inner())
    .await
    .expect("Failed to add skill");
    Json(skill)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5499/portfolio_db".to_string());

    let pool = init_db(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    rocket::build()
        .manage(pool)
        .mount("/", routes![list_projects, create_project, list_skills, add_skill])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
