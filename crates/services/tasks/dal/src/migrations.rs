use crate::connections::sqlx_postgres::POSTGRES_POOL;

#[expect(clippy::expect_used, clippy::missing_panics_doc)]
pub async fn run_migrations() {
    eprintln!("Migrating database...");

    let mut migrations = sqlx::migrate!("./migrations");
    migrations.ignore_missing = true;
    migrations
        .run(&*POSTGRES_POOL)
        .await
        .expect("Failed to run migrations");
    eprintln!("Database migration successful.");
}
