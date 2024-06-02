use crate::{
    api::AppState,
    model::{SectionInfo, TimeInfo},
};
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
        (status = 200, description = "Find all sections for a given course", body = [Vec<SectionInfo>]),
        (status = 500, description = "Error created by query"),
    )
)]
#[get("/sections/{course_id}")]
pub async fn get_sections(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let course_id = path.into_inner();
    log!(Level::Info, "GET /sections/{}", course_id);

    struct TempSectionInfo {
        id: i32,
        code: Option<String>,
        title: String,
        instructor: String,
    }

    match query_as!(
        TempSectionInfo,
        "SELECT s.id, CONCAT(d.code, '-', c.course, '-', s.section) as code, s.title, s.instructor FROM sections s, departments d, courses c WHERE s.course = c.id AND c.department = d.id AND s.status != 'X' AND s.course = $1",
        course_id
    )
    .fetch_all(&state.db)
    .await {
        Ok(temp_sections) => {
            let mut sections: Vec<SectionInfo> = vec![];

            for temp_section in temp_sections {
                let times = match query_as!(TimeInfo, "SELECT t.day, t.start, t.end, b.code as building_code, b.number as building_number, t.room FROM times t, buildings b WHERE t.building = b.number AND t.section = $1", temp_section.id).fetch_all(&state.db).await {
                    Ok(t) => t,
                    Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
                };

                sections.push(SectionInfo {
                    id: temp_section.id,
                    code: temp_section.code.unwrap(),
                    title: temp_section.title,
                    instructor: temp_section.instructor,
                    times
                });
            }

            HttpResponse::Ok().json(sections)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}
