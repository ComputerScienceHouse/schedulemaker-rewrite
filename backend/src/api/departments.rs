use actix_web::{HttpResponse, Responder, get, web::Data};
use sqlx::query_as;
use std::format;
use log::{log, Level};
use crate::api::AppState;
use crate::model::Department;
use crate::db::log_query_as;

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

    match log_query_as(
        query_as!(
            Department,
            "SELECT DISTINCT d.code AS dept_code, d.title AS dept_title, s.code AS school_code, s.title AS school_title FROM departments d, schools s WHERE d.school = s.id",
        )
        .fetch_all(&state.db)
        .await,
        None,
    )
    .await {
        Ok((_, depts)) => HttpResponse::Ok().json(depts),
        Err(e) => return e,
    }
}

