use crate::api::get_course_options::SqlxError;
//use crate::UserData;
use crate::db::get_pool;
use crate::model::{ClassStatus, EnrollmentStatus, SchedulePrint, WeekdayScheduled};
use actix_web::{post, web, HttpResponse, Responder, ResponseError, get};
use itertools::Itertools;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{query, FromRow, Postgres, QueryBuilder};
use std::collections::HashMap;
use std::fmt;
use utoipa::ToSchema;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(ToSchema)]
pub struct Term {
    term_id: i32,
    year: i32,
    term_name: String,
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all terms available", body = [Term]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/terms")]
pub async fn get_term() -> impl Responder {
    let pool = get_pool().await.unwrap();
    //let mut connection = pool.acquire().await.unwrap();
    let mut year_groups = HashMap::new();
    let data = query!("SELECT DISTINCT academic_term FROM classes ORDER BY academic_term DESC")
        .fetch_all(pool)
        .await?;
    for (year, terms) in &data
        .iter()
        .map(|row| {
            let year: i32 = row.academic_term % 1000 / 10;
            let name = match row.academic_term % 10 {
                1 => format!("Fall 20{}", year),
                5 => format!("Spring 20{}", year + 1),
                8 => format!("Summer 20{}", year + 1),
                _ => format!("Unknown 20{}", year),
            };
            Term {
                term_id: row.academic_term,
                year,
                term_name: name,
            }
        })
        .group_by(|t| t.year)
    {
        year_groups.insert(
            format!("20{}-20{}", year, year + 1),
            terms.collect::<Vec<Term>>(),
        );
    }
    Ok::<_, SqlxError>(HttpResponse::Ok().json(year_groups))
}
