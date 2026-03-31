use actix_web::{get, post, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Palette, CreatePalette};

#[utoipa::path(
    get,
    path = "/palettes",
    responses(
        (status = 200, description = "List all palettes", body = [Palette])
    )
)]
#[get("/palettes")]
pub async fn list_palettes(pool: web::Data<PgPool>) -> impl Responder {
    let palettes = sqlx::query_as::<_, Palette>("SELECT * FROM palettes ORDER BY name")
        .fetch_all(pool.get_ref())
        .await;

    match palettes {
        Ok(palettes) => HttpResponse::Ok().json(palettes),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching palettes"),
    }
}

#[utoipa::path(
    post,
    path = "/palettes",
    request_body = CreatePalette,
    responses(
        (status = 201, description = "Palette created", body = Palette)
    )
)]
#[post("/palettes")]
pub async fn create_palette(pool: web::Data<PgPool>, payload: web::Json<CreatePalette>) -> impl Responder {
    let palette = sqlx::query_as::<_, Palette>(
        "INSERT INTO palettes (name, colors) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.colors)
    .fetch_one(pool.get_ref())
    .await;

    match palette {
        Ok(palette) => HttpResponse::Created().json(palette),
        Err(_) => HttpResponse::InternalServerError().body("Error creating palette"),
    }
}

#[utoipa::path(
    get,
    path = "/palettes/{id}",
    responses(
        (status = 200, description = "Get palette by id", body = Palette),
        (status = 404, description = "Palette not found")
    )
)]
#[get("/palettes/{id}")]
pub async fn get_palette(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let palette = sqlx::query_as::<_, Palette>("SELECT * FROM palettes WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await;

    match palette {
        Ok(Some(palette)) => HttpResponse::Ok().json(palette),
        Ok(None) => HttpResponse::NotFound().body("Palette not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching palette"),
    }
}

#[utoipa::path(
    delete,
    path = "/palettes/{id}",
    responses(
        (status = 204, description = "Palette deleted")
    )
)]
#[delete("/palettes/{id}")]
pub async fn delete_palette(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM palettes WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting palette"),
    }
}
