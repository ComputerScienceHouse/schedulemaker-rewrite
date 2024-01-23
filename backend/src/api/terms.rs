use crate::api::get_course_options::SqlxError;
use crate::db::get_pool;
use actix_web::{HttpResponse, Responder, get};
use sqlx::query;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all terms available", body = [Term]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/terms")]
pub async fn get_terms() -> impl Responder {
    let data = query!("SELECT DISTINCT academic_term FROM classes ORDER BY academic_term DESC")
        .fetch_all(get_pool().await?)
        .await?;

    let terms: Vec<i32> = data
        .iter()
        .map(|row| {
            row.academic_term.unwrap()
        })
        .collect::<Vec<i32>>();

    Ok::<_, SqlxError>(HttpResponse::Ok().json(terms))
}

