use actix_web::{
    body::BoxBody,
    web::{self, scope},
    App, HttpResponse, HttpServer,
};
mod get_course_options;
mod terms;
mod departments;
use actix_cors::Cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model;
// pub struct UserData<'a> {
//     pool: &'a Pool<Postgres>,
// }

#[derive(OpenApi)]
#[openapi(
        paths(
            get_course_options::get_course_options,
            terms::get_terms,
            departments::get_departments,
        ),
        components(
            schemas(
                get_course_options::CourseOption,
                get_course_options::Time,
                get_course_options::WeekDay,
                get_course_options::Building,
                get_course_options::Search,
                model::WeekdayScheduled,
                model::ClassStatus,
                model::EnrollmentStatus,
                model::SchedulePrint,
            )
        ),
        tags(
            (name = "ScheduleMaker API", description = "")
        )
    )
]
struct ApiDoc;

pub async fn serve() {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .max_age(3600);

        App::new().wrap(cors).service(
            scope("/api")
                .service(get_course_options::get_course_options)
                .service(terms::get_terms)
                .service(departments::get_departments)
                .route("/openapi.json", web::get().to(open_api_spec))
                .service(
                    SwaggerUi::new("/docs/{_:.*}").url("/api/openapi.json", ApiDoc::openapi()),
                ),
        )
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run()
    .await
    .unwrap()
}

pub async fn open_api_spec() -> HttpResponse<BoxBody> {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
