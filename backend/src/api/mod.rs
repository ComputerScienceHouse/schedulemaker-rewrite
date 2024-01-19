use actix_web::{App, HttpServer};
mod get_course_options;
use actix_cors::Cors;

// pub struct UserData<'a> {
//     pool: &'a Pool<Postgres>,
// }

pub async fn serve() {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_course_options::get_course_options)
    })
    .bind(("127.0.0.1", 3000))
    .unwrap()
    .run()
    .await
    .unwrap()
}
