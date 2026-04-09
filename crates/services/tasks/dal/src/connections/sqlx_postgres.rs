use std::sync::LazyLock;

use settings::prelude::CONFIG;
use sqlx::postgres::PgPool;

#[expect(clippy::expect_used)]
pub static POSTGRES_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    let pool = CONFIG.database.pool_opts();
    pool.connect_lazy(CONFIG.database.url().as_str())
        .expect("Failed to create a db connection pool")
});
