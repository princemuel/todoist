use std::env;
use std::sync::LazyLock;

use sqlx::postgres::{PgPool, PgPoolOptions};

#[allow(clippy::expect_used)]
pub static POSTGRES_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    dotenvy::dotenv().ok();
    let conn_url = env::var("DATABASE_URL")
        .unwrap_or("postgresql://user:password@localhost:5432/dbname".to_string());
    let max_connections = env::var("POOL_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(5);

    let pool = PgPoolOptions::new().max_connections(max_connections);
    pool.connect_lazy(&conn_url)
        .expect("Failed to create a db connection pool")
});
