use actix_web::{
    body::BoxBody,
    web::{self, scope, Data},
    App, HttpResponse, HttpServer,
};
mod get_course_options;
mod terms;
mod departments;
use actix_cors::Cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sqlx::{Pool, Postgres, PoolOptions};
use log::{log, Level};
use std::env;

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
    println!("Successfully opened db connection");
    Data::new(AppState {
        db: db_pool,
    })
}

pub async fn open_transaction(db: &Pool<Postgres>) -> Result<Transaction<Postgres>, HttpResponse> {
    match db.try_begin().await {
        Ok(Some(t)) => Ok(t),
        Ok(None) => {
            log!(Level::Error, "Failed to open transaction");
            Err(HttpResponse::InternalServerError().body("Internal DB Error: Ok(None) transaction"))
        }
        Err(e) => {
            log!(Level::Error, "Failed to open transaction");
            Err(HttpResponse::InternalServerError().body(format!("Internal DB Error: {}", e)))
        }
    }
}

pub async fn log_query_as<T>(
    query: Result<Vec<T>, Error>,
    tx: Option<Transaction<'_, Postgres>>,
) -> Result<(Option<Transaction<'_, Postgres>>, Vec<T>), HttpResponse> {
    match query {
        Ok(v) => Ok((tx, v)),
        Err(e) -> {
            log!(Level::Warn, "DB Query failed: {}", e);
            if let Some(tx) = tx {
                match tx.rollback().await {
                    Ok(_) => {}
                    Err(tx_e) => {
                        log!(Level::Error, "Transaction failed to rollback: {}", tx_e);
                        return Err(HttpResponse::InternalServerError().body("Internal DB Error"));
                    }
                }
            }
            return Err(HttpResponse::InternalServerError().body("Internal DB Error"));
        }
    }
}

pub async fn log_query(
    query: Result<(), Error>,
    tx: Option<Transaction<'_, Postgres>>,
) -> Result<Option<Transaction<'_, Postgres>>, HttpResponse> {
    match query {
        Ok(_) => Ok(tx),
        Err(e) => {
            log!(Level::Warn, "DB Query failed: {}", e);
            if let Some(tx) = tx {
                match tx.rollback().await {
                    Ok(_) => {}
                    Err(tx_e) => {
                        log!(Level::Error, "Transaction failed to rollback: {}", tx_e);
                        return Err(HttpResponse::InternalServerError().body("Internal DB Error"));
                    }
                }
            }
        }
    }
}

