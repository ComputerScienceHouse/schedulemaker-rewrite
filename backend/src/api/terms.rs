use actix_web::{HttpResponse, Responder, get, web::Data};
use sqlx::query_as;
use log::{log, Level};
use crate::api::AppState;
use crate::model::Term;
use crate::db::log_query_as;

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

    match log_query_as(
        query_as!(
            Term,
            "SELECT DISTINCT term FROM academicterms ORDER BY term DESC",
        )
        .fetch_all(&state.db)
        .await,
        None,
    )
    .await {
        Ok((_, terms)) => HttpResponse::Ok().json(terms),
        Err(e) => return e,
    }
}

