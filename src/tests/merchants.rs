mod common;

use underworld::db::{items, merchants};

#[tokio::test]
async fn test_create_merchant() {
    let pool = common::setup_test_db().await;

    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, Some("Dwarven blacksmith"))
        .await
        .expect("Failed to create merchant");

    assert_eq!(merchant.name, "Grumli");
    assert_eq!(merchant.markup_low, 0.1);
    assert_eq!(merchant.markup_high, 0.5);
}

#[tokio::test]
async fn test_get_all_merchants() {
    let pool = common::setup_test_db().await;

    merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();
    merchants::create(&pool, "Mira", 0.2, 0.4, None).await.unwrap();

    let all = merchants::get_all(&pool).await.expect("Failed to get merchants");
    assert_eq!(all.len(), 2);
}

#[tokio::test]
async fn test_add_and_get_stock() {
    let pool = common::setup_test_db().await;

    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();
    let item = items::create(&pool, "Iron Sword", "Weapon", 50.0, None).await.unwrap();

    merchants::add_stock(&pool, merchant.id, item.id, Some(5), None)
        .await
        .expect("Failed to add stock");

    let stock = merchants::get_stock(&pool, merchant.id)
        .await
        .expect("Failed to get stock");

    assert_eq!(stock.len(), 1);
    assert_eq!(stock[0].name, "Iron Sword");
}

#[tokio::test]
async fn test_remove_stock() {
    let pool = common::setup_test_db().await;

    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();
    let item = items::create(&pool, "Iron Sword", "Weapon", 50.0, None).await.unwrap();

    let stock = merchants::add_stock(&pool, merchant.id, item.id, Some(5), None)
        .await
        .unwrap();

    merchants::remove_stock(&pool, stock.id)
        .await
        .expect("Failed to remove stock");

    let remaining = merchants::get_stock(&pool, merchant.id).await.unwrap();
    assert!(remaining.is_empty());
}

#[tokio::test]
async fn test_edit_merchant() {
    let pool = common::setup_test_db().await;

    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();

    let updated = merchants::edit(&pool, merchant.id, "Grumli the Elder", 0.2, 0.6, None)
        .await
        .expect("Failed to edit merchant");

    assert_eq!(updated.name, "Grumli the Elder");
    assert_eq!(updated.markup_low, 0.2);
}

#[tokio::test]
async fn test_delete_merchant() {
    let pool = common::setup_test_db().await;

    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();
    merchants::delete(&pool, merchant.id).await.expect("Failed to delete merchant");

    let all = merchants::get_all(&pool).await.unwrap();
    assert!(all.is_empty());
}
