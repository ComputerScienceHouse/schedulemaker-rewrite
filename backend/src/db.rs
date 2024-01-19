use sqlx::{Pool, Postgres};
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_pool() -> Result<&'static Pool<Postgres>, sqlx::Error> {
    POOL.get_or_try_init(|| async { Pool::connect(&std::env::var("DATABASE_URL").unwrap()).await })
        .await
}
