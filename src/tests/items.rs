mod common;

use underworld::db::items;

#[tokio::test]
async fn test_create_item() {
    let pool = common::setup_test_db().await;

    let item = items::create(&pool, "Iron Sword", "Weapon", 50.0, Some("A basic sword"))
        .await
        .expect("Failed to create item");

    assert_eq!(item.name, "Iron Sword");
    assert_eq!(item.category, "Weapon");
    assert_eq!(item.base_value, 50.0);
    assert_eq!(item.description, Some("A basic sword".to_string()));
}

#[tokio::test]
async fn test_get_all_items() {
    let pool = common::setup_test_db().await;

    items::create(&pool, "Iron Sword", "Weapon", 50.0, None).await.unwrap();
    items::create(&pool, "Health Potion", "Potion", 25.0, None).await.unwrap();

    let all = items::get_all(&pool).await.expect("Failed to get all items");

    assert_eq!(all.len(), 2);
}

#[tokio::test]
async fn test_search_by_name() {
    let pool = common::setup_test_db().await;

    items::create(&pool, "Iron Sword", "Weapon", 50.0, None).await.unwrap();
    items::create(&pool, "Iron Shield", "Armour", 40.0, None).await.unwrap();
    items::create(&pool, "Health Potion", "Potion", 25.0, None).await.unwrap();

    let results = items::search_by_name(&pool, "Iron")
        .await
        .expect("Failed to search items");

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|i| i.name == "Iron Sword"));
    assert!(results.iter().any(|i| i.name == "Iron Shield"));
}

#[tokio::test]
async fn test_edit_item() {
    let pool = common::setup_test_db().await;

    let item = items::create(&pool, "Iron Sword", "Weapon", 50.0, None)
        .await
        .unwrap();

    let updated = items::edit(&pool, item.id, "Steel Sword", "Weapon", 75.0, None)
        .await
        .expect("Failed to edit item");

    assert_eq!(updated.name, "Steel Sword");
    assert_eq!(updated.base_value, 75.0);
}

#[tokio::test]
async fn test_delete_item() {
    let pool = common::setup_test_db().await;

    let item = items::create(&pool, "Iron Sword", "Weapon", 50.0, None)
        .await
        .unwrap();

    items::delete(&pool, item.id).await.expect("Failed to delete item");

    let all = items::get_all(&pool).await.unwrap();
    assert!(all.is_empty());
}
