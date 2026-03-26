mod common;

use example_app::generated;
use rust_decimal::Decimal;
use std::str::FromStr;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_order() {
    let pool = common::get_pool().await;

    let result = generated::orders::insert_order(
        pool,
        1,
        "Widget".to_string(),
        Decimal::from_str("9.99").unwrap(),
    )
    .await
    .unwrap();

    assert!(result.id > 0);
    assert_eq!(result.tenant_id, 1);
    assert_eq!(result.product_name, "Widget");
    assert_eq!(result.amount, Decimal::from_str("9.99").unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_orders_by_tenant() {
    let pool = common::get_pool().await;

    // Insert orders for tenant 42
    generated::orders::insert_order(
        pool,
        42,
        "Alpha".to_string(),
        Decimal::from_str("10.00").unwrap(),
    )
    .await
    .unwrap();

    generated::orders::insert_order(
        pool,
        42,
        "Beta".to_string(),
        Decimal::from_str("20.00").unwrap(),
    )
    .await
    .unwrap();

    // Query by tenant — this should use partition pruning (single partition scan)
    let orders = generated::orders::get_orders_by_tenant(pool, 42)
        .await
        .unwrap();
    assert!(orders.len() >= 2);
    assert!(orders.iter().all(|o| o.tenant_id == 42));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_orders_by_product() {
    let pool = common::get_pool().await;

    let product = format!(
        "Gadget_{}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );

    generated::orders::insert_order(pool, 7, product.clone(), Decimal::from_str("5.50").unwrap())
        .await
        .unwrap();

    // Query by product name — missing partition key, all partitions scanned
    let orders = generated::orders::get_orders_by_product(pool, product.clone())
        .await
        .unwrap();
    assert!(!orders.is_empty());
    assert!(orders.iter().all(|o| o.product_name == product));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_orders_by_tenant_range() {
    let pool = common::get_pool().await;

    generated::orders::insert_order(
        pool,
        100,
        "RangeItem".to_string(),
        Decimal::from_str("15.00").unwrap(),
    )
    .await
    .unwrap();

    // Query by tenant range — range on hash key, no partition pruning
    let orders = generated::orders::get_orders_by_tenant_range(pool, 0)
        .await
        .unwrap();
    assert!(!orders.is_empty());
}
