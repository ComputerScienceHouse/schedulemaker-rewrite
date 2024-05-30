use crate::{api::AppState, model::Term};
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::{log, Level};
use sqlx::query_as;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all terms available", body = [Vec<i32>]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/terms")]
pub async fn get_terms(state: Data<AppState>) -> impl Responder {
    log!(Level::Info, "GET /terms");

    match query_as!(
        Term,
        "SELECT DISTINCT term FROM academicterms ORDER BY term DESC",
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(terms) => HttpResponse::Ok().json(terms),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
