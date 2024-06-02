use crate::{api::AppState, model::CourseInfo};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use log::{log, Level};
use sqlx::query_as;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all courses offered by a department", body = [Vec<CourseInfo>]),
        (status = 500, description = "Error created by query"),
    )
)]
#[get("/courses/{term}/{department_id}")]
pub async fn get_courses(state: Data<AppState>, path: Path<(i16, i32)>) -> impl Responder {
    let (term, department_id): (i16, i32) = path.into_inner();
    log!(Level::Info, "GET /courses/{}/{}", term, department_id);

    match query_as!(CourseInfo, "SELECT c.id, CONCAT(d.code, '-', c.course) as code, c.title, c.description FROM courses c, departments d, sections s WHERE c.department = d.id AND s.course = c.id AND s.status != 'X' AND d.id = $1 AND c.term = $2", department_id, term).fetch_all(&state.db).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
