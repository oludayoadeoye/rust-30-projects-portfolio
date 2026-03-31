use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;

// --- Job Handlers ---

#[utoipa::path(
    get,
    path = "/jobs",
    responses(
        (status = 200, description = "List all jobs", body = [Job])
    )
)]
#[get("/jobs")]
pub async fn list_jobs(pool: web::Data<PgPool>) -> impl Responder {
    let jobs = sqlx::query_as::<_, Job>("SELECT * FROM jobs ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await;

    match jobs {
        Ok(j) => HttpResponse::Ok().json(j),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching jobs"),
    }
}

#[utoipa::path(
    post,
    path = "/jobs",
    request_body = CreateJob,
    responses(
        (status = 201, description = "Job created", body = Job)
    )
)]
#[post("/jobs")]
pub async fn create_job(pool: web::Data<PgPool>, payload: web::Json<CreateJob>) -> impl Responder {
    let job = sqlx::query_as::<_, Job>(
        "INSERT INTO jobs (title, company, location, description, salary_range) \
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.company)
    .bind(&payload.location)
    .bind(&payload.description)
    .bind(&payload.salary_range)
    .fetch_one(pool.get_ref())
    .await;

    match job {
        Ok(j) => HttpResponse::Created().json(j),
        Err(_) => HttpResponse::InternalServerError().body("Error creating job"),
    }
}

// --- Applicant Handlers ---

#[utoipa::path(
    get,
    path = "/jobs/{id}/applicants",
    responses(
        (status = 200, description = "List applicants for a job", body = [Applicant])
    )
)]
#[get("/jobs/{id}/applicants")]
pub async fn list_applicants(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let applicants = sqlx::query_as::<_, Applicant>("SELECT * FROM applicants WHERE job_id = $1 ORDER BY applied_at ASC")
        .bind(id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match applicants {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching applicants"),
    }
}

#[utoipa::path(
    post,
    path = "/jobs/{id}/apply",
    request_body = CreateApplicant,
    responses(
        (status = 201, description = "Application submitted", body = Applicant)
    )
)]
#[post("/jobs/{id}/apply")]
pub async fn apply_job(pool: web::Data<PgPool>, id: web::Path<i32>, payload: web::Json<CreateApplicant>) -> impl Responder {
    let applicant = sqlx::query_as::<_, Applicant>(
        "INSERT INTO applicants (job_id, name, email, resume_url) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(id.into_inner())
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.resume_url)
    .fetch_one(pool.get_ref())
    .await;

    match applicant {
        Ok(a) => HttpResponse::Created().json(a),
        Err(_) => HttpResponse::InternalServerError().body("Error applying for job"),
    }
}
