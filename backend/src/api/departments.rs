use crate::api::get_course_options::SqlxError;
use crate::db::get_pool;
use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize};
use sqlx::query;
use std::format;
use utoipa::ToSchema;

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    dept_code: String,
    dept_name: String,
    school_code: String,
    school_name: String,
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all departments", body = [Department]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/departments")]
pub async fn get_departments(data: Data<AppState>) -> impl Responder {
    log!(Level::Info, "GET /departments");

    match log_query_as(
        query_as!(
            Department,
            "SELECT DISTINCT d.code AS dept_code, d.title AS dept_title, s.code AS school_code, s.title AS school_title FROM departments d, schools s WHERE d.school = s.id",
        )
        .fetch_all(&state.db)
        .await,
        Some(transaction),
    )
    .await {
        Ok((_, depts)) => HttpResponse::Ok().json(depts),
        Err(e) => return e,
    }
}

