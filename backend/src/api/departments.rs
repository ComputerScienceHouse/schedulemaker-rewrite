use crate::api::AppState;
use crate::model::Department;
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::{log, Level};
use sqlx::query_as;
use std::format;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all departments", body = [Department]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/departments")]
pub async fn get_departments(state: Data<AppState>) -> impl Responder {
    log!(Level::Info, "GET /departments");

    match query_as!(
            Department,
            "SELECT DISTINCT d.code AS dept_code, d.title AS dept_title, s.code AS school_code, s.title AS school_title FROM departments d, schools s WHERE d.school = s.id",
        )
        .fetch_all(&state.db)
    .await {
        Ok(depts) => HttpResponse::Ok().json(depts),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
