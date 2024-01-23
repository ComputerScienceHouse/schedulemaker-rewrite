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
    department: String,
    college: String,
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
    let data = query!("SELECT DISTINCT academic_org, academic_group FROM classes ORDER BY academic_group")
        .fetch_all(get_pool().await?)
        .await?;

    let departments: Vec<Department> = data
        .iter()
        .map(|row| {
            Department {
                department: row.academic_org.clone().unwrap(),
                college: row.academic_group.clone().unwrap(),
            }
        })
        .collect::<Vec<Department>>();

    Ok::<_, SqlxError>(HttpResponse::Ok().json(departments))
}

