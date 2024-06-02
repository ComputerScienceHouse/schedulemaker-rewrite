use crate::{api::AppState, model::SchoolRecord};
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::{log, Level};
use sqlx::query_as;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all schools", body = [Vec<SchoolRecord>]),
        (status = 500, description = "Error created by query"),
    )
)]
#[get("/schools")]
pub async fn get_schools(state: Data<AppState>) -> impl Responder {
    log!(Level::Info, "GET /schools");

    match query_as!(SchoolRecord, "SELECT id, code, title FROM schools")
        .fetch_all(&state.db)
        .await
    {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
