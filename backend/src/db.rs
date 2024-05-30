use actix_web::HttpResponse;
use log::{log, Level};
use sqlx::{Error, Pool, Postgres, Transaction};
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_pool() -> Result<&'static Pool<Postgres>, Error> {
    POOL.get_or_try_init(|| async { Pool::connect(&std::env::var("DATABASE_URL").unwrap()).await })
        .await
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
