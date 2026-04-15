mod common;

use example_app::generated;

/// Test that {col!} syntax on count expression produces non-nullable i64
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_non_null_count_expression() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "nn_count").await;

    let result = generated::analytics::get_non_null_count_expression(pool)
        .await
        .unwrap();
    // total is i64, not Option<i64> — this would fail to compile if it were Option
    let total: i64 = result;
    assert!(total > 0);
}

/// Test that {col!} syntax on boolean literal in RETURNING produces non-nullable bool
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_returning_applied_non_null() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "nn_applied").await;

    let result =
        generated::users::update_user_returning_applied(pool, "Updated Name".to_string(), user.id)
            .await
            .unwrap();

    // possible_one returns Option<bool> where inner bool is non-nullable
    if let Some(applied) = result {
        let _: bool = applied;
        assert!(applied);
    }
}

/// Test that "col!" sqlx-compatible syntax on comparison expression produces non-nullable bool
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_is_recent_sqlx_compat_non_null() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "nn_recent").await;

    let result: bool = generated::users::get_user_is_recent(pool, user.id)
        .await
        .unwrap();
    let _ = result;
}

/// Test multiple non-null fields in a multi-row query using both syntaxes
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_non_null_multi_rows() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "nn_multi").await;

    let rows = generated::analytics::get_non_null_multi_rows(pool)
        .await
        .unwrap();
    assert!(!rows.is_empty());

    // All fields are non-nullable — these type annotations would fail to compile with Option<>
    let first = &rows[0];
    let _: i32 = first.user_id;
    let _: &str = &first.user_name;
    let _: bool = first.is_recent;
    let _: bool = first.is_active;
}
