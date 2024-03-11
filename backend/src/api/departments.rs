use actix_web::{HttpResponse, Responder, get, web::Data};
use serde::{Serialize};
use sqlx::query_as;
use std::format;
use utoipa::ToSchema;
use log::{log, Level};
use crate::api::{AppState, log_query_as};

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    dept_code: String,
    dept_title: String,
    school_code: String,
    school_title: String,
}

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

