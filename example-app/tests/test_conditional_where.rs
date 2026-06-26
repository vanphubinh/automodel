mod common;

use example_app::generated;
use jiff_sqlx::ToSqlx;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_find_users_by_name_and_age() {
    let pool = common::get_pool().await;
    let email = common::unique_email("nameage");
    generated::users::insert_user(
        pool,
        "NameAge Test".to_string(),
        email,
        35,
        common::default_profile(),
    )
    .await
    .unwrap();

    let results = generated::users::find_users_by_name_and_age(
        pool,
        "%NameAge%".to_string(),
        Some(20),
        "NameAge Test".to_string(),
        Some(50),
    )
    .await
    .unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|u| u.name == "NameAge Test"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_recent_users() {
    let pool = common::get_pool().await;
    // Insert a user so there's at least one recent user
    common::insert_test_user(pool, "recent").await;

    let since = (jiff::Timestamp::now() - jiff::Span::new().hours(24)).to_sqlx();
    let users = generated::users::get_recent_users(pool, since)
        .await
        .unwrap();
    assert!(!users.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_active_users_by_age_range() {
    let pool = common::get_pool().await;
    // Insert a user with age=30 to ensure match
    common::insert_test_user(pool, "active_age").await;

    let users = generated::users::get_active_users_by_age_range(pool, 18, 99)
        .await
        .unwrap();
    assert!(!users.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_search_users_by_name_pattern() {
    let pool = common::get_pool().await;
    let email = common::unique_email("pattern_search");
    generated::users::insert_user(
        pool,
        "UniquePatternName".to_string(),
        email,
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let results =
        generated::users::search_users_by_name_pattern(pool, "%UniquePattern%".to_string())
            .await
            .unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|u| u.name == "UniquePatternName"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_search_users_advanced() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "advanced").await;

    // all params None → returns all users
    let results = generated::users::search_users_advanced(pool, None, None, None)
        .await
        .unwrap();
    assert!(!results.is_empty());

    // with name filter
    let results =
        generated::users::search_users_advanced(pool, Some("%advanced%".to_string()), None, None)
            .await
            .unwrap();
    assert!(!results.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_users_by_status() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "bystatus").await;
    // set status to Active
    generated::users::update_user_status(
        pool,
        example_app::generated::types::public::UserStatus::Active,
        user.id,
    )
    .await
    .unwrap();

    let results = generated::users::get_users_by_status(
        pool,
        example_app::generated::types::public::UserStatus::Active,
    )
    .await
    .unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|u| u.id == user.id));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_user_statuses() {
    let pool = common::get_pool().await;
    let statuses = generated::users::get_all_user_statuses(pool).await.unwrap();
    // Returns Vec<Option<UserStatus>> — at least empty vec is valid
    let _ = statuses;
}
