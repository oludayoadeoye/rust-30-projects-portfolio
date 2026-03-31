use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Workout Handlers ---

#[utoipa::path(
    get,
    path = "/workouts",
    responses(
        (status = 200, description = "List all workouts", body = [Workout])
    )
)]
#[get("/workouts")]
pub async fn list_workouts(pool: web::Data<PgPool>) -> impl Responder {
    let workouts = sqlx::query_as::<_, Workout>("SELECT * FROM workouts ORDER BY workout_date DESC")
        .fetch_all(pool.get_ref())
        .await;

    match workouts {
        Ok(w) => HttpResponse::Ok().json(w),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching workouts"),
    }
}

#[utoipa::path(
    post,
    path = "/workouts",
    request_body = CreateWorkout,
    responses(
        (status = 201, description = "Workout created", body = Workout)
    )
)]
#[post("/workouts")]
pub async fn create_workout(pool: web::Data<PgPool>, payload: web::Json<CreateWorkout>) -> impl Responder {
    let workout = sqlx::query_as::<_, Workout>(
        "INSERT INTO workouts (name, workout_date, notes) VALUES ($1, COALESCE($2, CURRENT_DATE), $3) RETURNING *"
    )
    .bind(&payload.name)
    .bind(payload.workout_date)
    .bind(&payload.notes)
    .fetch_one(pool.get_ref())
    .await;

    match workout {
        Ok(w) => HttpResponse::Created().json(w),
        Err(_) => HttpResponse::InternalServerError().body("Error creating workout"),
    }
}

// --- Exercise Handlers ---

#[utoipa::path(
    get,
    path = "/workouts/{id}/exercises",
    responses(
        (status = 200, description = "List exercises in a workout", body = [Exercise])
    )
)]
#[get("/workouts/{id}/exercises")]
pub async fn list_exercises(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let exercises = sqlx::query_as::<_, Exercise>("SELECT * FROM exercises WHERE workout_id = $1 ORDER BY created_at ASC")
        .bind(id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match exercises {
        Ok(e) => HttpResponse::Ok().json(e),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching exercises"),
    }
}

#[utoipa::path(
    post,
    path = "/workouts/{id}/exercises",
    request_body = CreateExercise,
    responses(
        (status = 201, description = "Exercise added", body = Exercise)
    )
)]
#[post("/workouts/{id}/exercises")]
pub async fn add_exercise(pool: web::Data<PgPool>, id: web::Path<i32>, payload: web::Json<CreateExercise>) -> impl Responder {
    let exercise = sqlx::query_as::<_, Exercise>(
        "INSERT INTO exercises (workout_id, name, sets, reps, weight_kg) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(id.into_inner())
    .bind(&payload.name)
    .bind(payload.sets)
    .bind(payload.reps)
    .bind(payload.weight_kg)
    .fetch_one(pool.get_ref())
    .await;

    match exercise {
        Ok(e) => HttpResponse::Created().json(e),
        Err(_) => HttpResponse::InternalServerError().body("Error adding exercise"),
    }
}
