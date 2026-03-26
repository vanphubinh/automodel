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
    )
    .await
    .unwrap();
    assert_eq!(result.name, "Test Product");
    assert!(result.id > 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_products() {
    let pool = common::get_pool().await;

    // The migration inserts a default product, so this should succeed
    let result = generated::products::get_all_products(pool).await.unwrap();
    assert!(result.id > 0);
}
