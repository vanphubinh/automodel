mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_product() {
    let pool = common::get_pool().await;

    let result = generated::products::insert_product(
        pool,
        "Test Product".to_string(),
        std::num::NonZeroI32::new(50).unwrap(),
        "product@example.com".to_string(),
        generated::types::public::ProductPriority::High,
    )
    .await
    .unwrap();
    assert_eq!(result.name, "Test Product");
    assert_eq!(
        result.priority,
        generated::types::public::ProductPriority::High
    );
    assert!(result.id > 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_products() {
    let pool = common::get_pool().await;

    // The migration inserts a default product, so this should succeed
    let result = generated::products::get_all_products(pool).await.unwrap();
    assert!(result.id > 0);
    assert_eq!(
        result.priority,
        generated::types::public::ProductPriority::Medium
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_products_by_priority() {
    let pool = common::get_pool().await;

    let urgent_products = generated::products::get_products_by_priority(
        pool,
        generated::types::public::ProductPriority::Urgent,
    )
    .await
    .unwrap();

    assert!(urgent_products.is_empty());

    let high_products = generated::products::get_products_by_priority(
        pool,
        generated::types::public::ProductPriority::High,
    )
    .await
    .unwrap();

    assert!(high_products.iter().any(|p| p.name == "Test Product"));
}
