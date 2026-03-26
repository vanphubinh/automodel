mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_count_and_avg_age() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "analytics_count").await;

    let result = generated::analytics::get_user_count_and_avg_age(pool)
        .await
        .unwrap();
    assert!(result.count.unwrap_or(0) > 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_activity_summary() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "analytics_summary").await;

    let results = generated::analytics::get_user_activity_summary(pool)
        .await
        .unwrap();
    assert!(!results.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_hierarchical_user_data() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "analytics_hier").await;

    let results = generated::analytics::get_hierarchical_user_data(pool)
        .await
        .unwrap();
    // May return empty if no referral chains exist — just verify no error
    let _ = results;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_activity_with_posts() {
    let pool = common::get_pool().await;
    let since = chrono::Utc::now() - chrono::Duration::days(365 * 10);
    let start_date = chrono::Utc::now() - chrono::Duration::days(365 * 10);
    let end_date = chrono::Utc::now() + chrono::Duration::days(1);

    let results =
        generated::analytics::get_user_activity_with_posts(pool, since, start_date, end_date)
            .await
            .unwrap();
    // May be empty if no posts exist — just verify no error
    let _ = results;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_engagement_metrics() {
    let pool = common::get_pool().await;

    let results = generated::analytics::get_user_engagement_metrics(pool, 0, 100)
        .await
        .unwrap();
    // May be empty — just verify no error
    let _ = results;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_time_series_user_registrations() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "analytics_ts").await;

    let start_date = chrono::Utc::now() - chrono::Duration::days(365);
    let end_date = chrono::Utc::now() + chrono::Duration::days(1);

    let results =
        generated::analytics::get_time_series_user_registrations(pool, start_date, end_date, 0)
            .await
            .unwrap();
    let _ = results;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_users_with_timezone_info() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "analytics_tz").await;

    let start_date = chrono::Utc::now() - chrono::Duration::days(365);
    let end_date = chrono::Utc::now() + chrono::Duration::days(1);

    let results = generated::analytics::get_users_with_timezone_info(
        pool,
        "UTC".to_string(),
        start_date,
        end_date,
        rust_decimal::Decimal::ZERO,
        rust_decimal::Decimal::new(99999, 0),
    )
    .await
    .unwrap();
    let _ = results;
}
