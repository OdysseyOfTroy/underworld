use sqlx::SqlitePool;
use crate::db::items::Item;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Merchant {
    pub id: i64,
    pub name: String,
    pub markup_low: f64,
    pub markup_high: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MerchantStock {
    pub id: i64,
    pub merchant_id: i64,
    pub item_id: i64,
    pub quantity: Option<i64>,
    pub override_price: Option<f64>,
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    markup_low: f64,
    markup_high: f64,
    notes: Option<&str>,
) -> Result<Merchant, sqlx::Error> {
    let merchant = sqlx::query_as!(
        Merchant,
        "INSERT INTO merchants (name, markup_low, markup_high, notes)
         VALUES (?, ?, ?, ?)
         RETURNING *",
        name,
        markup_low,
        markup_high,
        notes,
    )
    .fetch_one(pool)
    .await?;

    Ok(merchant)
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Merchant>, sqlx::Error> {
    let merchants = sqlx::query_as!(Merchant, "SELECT * FROM merchants ORDER BY name ASC")
        .fetch_all(pool)
        .await?;

    Ok(merchants)
}

pub async fn edit(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    markup_low: f64,
    markup_high: f64,
    notes: Option<&str>,
) -> Result<Merchant, sqlx::Error> {
    let merchant = sqlx::query_as!(
        Merchant,
        "UPDATE merchants
         SET name = ?, markup_low = ?, markup_high = ?, notes = ?
         WHERE id = ?
         RETURNING *",
        name,
        markup_low,
        markup_high,
        notes,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(merchant)
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM merchants WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}

// Stock management
pub async fn add_stock(
    pool: &SqlitePool,
    merchant_id: i64,
    item_id: i64,
    quantity: Option<i64>,
    override_price: Option<f64>,
) -> Result<MerchantStock, sqlx::Error> {
    let stock = sqlx::query_as::<_, MerchantStock>(
        "INSERT INTO merchant_stock (merchant_id, item_id, quantity, override_price)
         VALUES (?, ?, ?, ?)
         RETURNING *",
    )
    .bind(merchant_id)
    .bind(item_id)
    .bind(quantity)
    .bind(override_price)
    .fetch_one(pool)
    .await?;

    Ok(stock)
}

pub async fn remove_stock(pool: &SqlitePool, stock_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM merchant_stock WHERE id = ?", stock_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_stock(pool: &SqlitePool, merchant_id: i64) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as!(
        Item,
        "SELECT i.* FROM items i
         INNER JOIN merchant_stock ms ON ms.item_id = i.id
         WHERE ms.merchant_id = ?
         ORDER BY i.name ASC",
        merchant_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}
