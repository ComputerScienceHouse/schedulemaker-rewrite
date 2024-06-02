use crate::{api::AppState, model::Department};
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
        (status = 200, description = "List all departments under a school", body = [Vec<Department>]),
        (status = 500, description = "Error created by query"),
    ),
)]
#[get("/departments/{school_id}")]
pub async fn get_departments(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    log!(Level::Info, "GET /departments/{}", id);

    match query_as!(
        Department,
        "SELECT id, code, title FROM departments WHERE school = $1",
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(depts) => HttpResponse::Ok().json(depts),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
