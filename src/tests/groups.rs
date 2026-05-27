mod common;

use underworld::db::{groups, merchants};

#[tokio::test]
async fn test_create_group() {
    let pool = common::setup_test_db().await;

    let group = groups::create(&pool, "Ironforge", "location", None)
        .await
        .expect("Failed to create group");

    assert_eq!(group.name, "Ironforge");
    assert_eq!(group.group_type, "location");
    assert!(group.parent_id.is_none());
}

#[tokio::test]
async fn test_nested_group() {
    let pool = common::setup_test_db().await;

    let parent = groups::create(&pool, "Ironforge", "location", None).await.unwrap();
    let child = groups::create(&pool, "Blacksmith District", "location", Some(parent.id))
        .await
        .expect("Failed to create nested group");

    assert_eq!(child.parent_id, Some(parent.id));
}

#[tokio::test]
async fn test_assign_and_get_merchants_in_group() {
    let pool = common::setup_test_db().await;

    let group = groups::create(&pool, "Ironforge", "location", None).await.unwrap();
    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();

    groups::assign_merchant(&pool, merchant.id, group.id)
        .await
        .expect("Failed to assign merchant to group");

    let members = groups::get_merchants_in_group(&pool, group.id)
        .await
        .expect("Failed to get merchants in group");

    assert_eq!(members.len(), 1);
    assert_eq!(members[0].name, "Grumli");
}

#[tokio::test]
async fn test_unassign_merchant_from_group() {
    let pool = common::setup_test_db().await;

    let group = groups::create(&pool, "Ironforge", "location", None).await.unwrap();
    let merchant = merchants::create(&pool, "Grumli", 0.1, 0.5, None).await.unwrap();

    groups::assign_merchant(&pool, merchant.id, group.id).await.unwrap();
    groups::unassign_merchant(&pool, merchant.id, group.id)
        .await
        .expect("Failed to unassign merchant");

    let members = groups::get_merchants_in_group(&pool, group.id).await.unwrap();
    assert!(members.is_empty());
}
