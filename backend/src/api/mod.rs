use actix_web::{
    body::BoxBody,
    web::{self, scope, Data},
    App, HttpResponse, HttpServer,
};
mod departments;
mod get_course_options;
mod terms;
use actix_cors::Cors;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(OpenApi)]
#[openapi(
        paths(
            get_course_options::get_course_options,
            terms::get_terms,
            departments::get_departments,
        ),
        components(
            schemas(
                model::CourseOption,
                model::Time,
                model::WeekDay,
                model::Building,
                model::Search,
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
    let app_data = get_app_data().await;
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(
                scope("/api")
                    .service(get_course_options::get_course_options)
                    .service(terms::get_terms)
                    .service(departments::get_departments)
                    .route("/openapi.json", web::get().to(open_api_spec))
                    .service(
                        SwaggerUi::new("/docs/{_:.*}").url("/api/openapi.json", ApiDoc::openapi()),
                    ),
            )
            .app_data(app_data.clone())
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

pub async fn get_app_data() -> Data<AppState> {
    let db_pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Could not connect to database");
    Data::new(AppState { db: db_pool })
}
