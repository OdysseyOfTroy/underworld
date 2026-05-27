use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub mod items;
pub mod merchants;
pub mod groups;

pub async fn init_pool(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&format!("sqlite://{}?mode=rwc", db_path))
                .await?;

    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            category    TEXT NOT NULL,
            base_value  REAL NOT NULL,
            description TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS groups (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            group_type  TEXT NOT NULL,
            parent_id   INTEGER REFERENCES groups(id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS merchants (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            markup_low  REAL NOT NULL,
            markup_high REAL NOT NULL,
            notes       TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS merchant_groups (
            merchant_id INTEGER REFERENCES merchants(id),
            group_id    INTEGER REFERENCES groups(id),
            PRIMARY KEY (merchant_id, group_id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS merchant_stock (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            merchant_id     INTEGER NOT NULL REFERENCES merchants(id),
            item_id         INTEGER NOT NULL REFERENCES items(id),
            quantity        INTEGER,
            override_price  REAL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
