use sqlx::SqlitePool;
use underworld::db;  // replace `underworld` with your actual crate name

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("Failed to create in-memory test database");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations on test database");

    pool
}
