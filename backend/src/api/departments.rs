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
    department_code: String,
    department_name: String,
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
pub async fn get_departments() -> impl Responder {
    let data = query!("SELECT DISTINCT d.code AS dcode, d.title AS dtitle, s.code AS scode, s.title AS stitle FROM departments d, schools s WHERE d.school = s.id")
        .fetch_all(get_pool().await?)
        .await?;

    let departments = data
        .iter()
        .map(|row| {
            Department {
                department_code: row.dcode.clone(),
                department_name: row.dtitle.clone(),
                school_code: row.scode.clone(),
                school_name: row.stitle.clone(),
            }
        })
        .collect::<Vec<Department>>();

    Ok::<_, SqlxError>(HttpResponse::Ok().json(departments))
}

