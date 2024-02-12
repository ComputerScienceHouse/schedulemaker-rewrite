use sqlx::{Pool, MySql};
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<MySql>> = OnceCell::const_new();

pub async fn get_pool() -> Result<&'static Pool<MySql>, sqlx::Error> {
    POOL.get_or_try_init(|| async { Pool::connect(&std::env::var("DATABASE_URL").unwrap()).await })
        .await
}
