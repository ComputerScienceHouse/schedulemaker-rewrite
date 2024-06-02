use crate::{
    api::AppState,
    model::{Term, Year},
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::{log, Level};
use sqlx::query_as;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all terms available, grouped by year", body = [Vec<Year>]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/terms")]
pub async fn get_terms(state: Data<AppState>) -> impl Responder {
    log!(Level::Info, "GET /terms");

    struct Info {
        term: i32,
    }

    match query_as!(
        Info,
        "SELECT DISTINCT term FROM academicterms ORDER BY term DESC",
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(mut terms) => {
            let mut years: Vec<Year> = vec![];

            while !terms.is_empty() {
                let base: i32 = terms[0].term % 1000 / 10;
                let year_terms: Vec<Term> = terms
                    .iter()
                    .take_while(|&t| t.term % 1000 / 10 == base)
                    .map(|t| Term {
                        term_id: t.term,
                        term_name: match t.term % 10 {
                            1 => format!("Fall 20{}", base),
                            5 => format!("Spring 20{}", base + 1),
                            8 => format!("Summer 20{}", base + 1),
                            _ => String::new(),
                        },
                    })
                    .collect();

                terms = terms
                    .into_iter()
                    .filter(|t| t.term % 1000 / 10 != base)
                    .collect::<Vec<Info>>();

                years.push(Year {
                    year: format!("20{} - 20{}", base, base + 1),
                    terms: year_terms,
                });
            }

            HttpResponse::Ok().json(years)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
