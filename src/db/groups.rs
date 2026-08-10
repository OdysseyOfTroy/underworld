use crate::db::merchants::Merchant;
use sqlx::SqlitePool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub group_type: String,
    pub parent_id: Option<i64>,
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    group_type: &str,
    parent_id: Option<i64>,
) -> Result<Group, sqlx::Error> {
    let group = sqlx::query_as!(
        Group,
        "INSERT INTO groups (name, group_type, parent_id)
         VALUES (?, ?, ?)
         RETURNING *",
        name,
        group_type,
        parent_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(group)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Group>, sqlx::Error> {
    let groups = sqlx::query_as!(Group, "SELECT * FROM groups ORDER BY name ASC")
        .fetch_all(pool)
        .await?;

    Ok(groups)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM groups WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn edit(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    group_type: &str,
    parent_id: Option<i64>,
) -> Result<Group, sqlx::Error> {
    let group = sqlx::query_as!(
        Group,
        "UPDATE groups
         SET name = ?, group_type = ?, parent_id = ?
         WHERE id = ?
         RETURNING *",
        name,
        group_type,
        parent_id,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(group)
}

// Merchant <-> Group assignments
pub async fn assign_merchant(
    pool: &SqlitePool,
    merchant_id: i64,
    group_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT OR IGNORE INTO merchant_groups (merchant_id, group_id) VALUES (?, ?)",
        merchant_id,
        group_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn unassign_merchant(
    pool: &SqlitePool,
    merchant_id: i64,
    group_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM merchant_groups WHERE merchant_id = ? AND group_id = ?",
        merchant_id,
        group_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_merchants_in_group(
    pool: &SqlitePool,
    group_id: i64,
) -> Result<Vec<Merchant>, sqlx::Error> {
    let merchants = sqlx::query_as!(
        Merchant,
        "SELECT m.* FROM merchants m
         INNER JOIN merchant_groups mg ON mg.merchant_id = m.id
         WHERE mg.group_id = ?
         ORDER BY m.name ASC",
        group_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(merchants)
}
