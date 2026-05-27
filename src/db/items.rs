use sqlx::SqlitePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub base_value: f64,
    pub description: Option<String>,
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    category: &str,
    base_value: f64,
    description: Option<&str>,
) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(
        Item,
        "INSERT INTO items (name, category, base_value, description)
         VALUES (?, ?, ?, ?)
         RETURNING *",
        name,
        category,
        base_value,
        description,
    )
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as!(Item, "SELECT * FROM items ORDER BY name ASC")
        .fetch_all(pool)
        .await?;

    Ok(items)
}

pub async fn search_by_name(pool: &SqlitePool, query: &str) -> Result<Vec<Item>, sqlx::Error> {
    let pattern = format!("%{}%", query);

    let items = sqlx::query_as!(
        Item,
        "SELECT * FROM items WHERE name LIKE ? ORDER BY name ASC",
        pattern,
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM items WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn edit(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    category: &str,
    base_value: f64,
    description: Option<&str>,
) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as!(
        Item,
        "UPDATE items
         SET name = ?, category = ?, base_value = ?, description = ?
         WHERE id = ?
         RETURNING *",
        name,
        category,
        base_value,
        description,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(item)
}
