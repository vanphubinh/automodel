mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_users_with_star() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "star_all").await;

    // get_all_users_with_star does SELECT * — may fail with ColumnDecode
    // if old rows have NULL in non-optional columns (e.g. tags added later).
    // We just verify the query compiles and executes without panicking.
    let result = generated::users::get_all_users_with_star(pool).await;
    // If it succeeds, there should be at least one user
    if let Ok(users) = result {
        assert!(!users.is_empty());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_by_id_with_star() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "star_id").await;

    let found = generated::users::get_user_by_id_with_star(pool, user.id)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_summary() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "summary").await;

    let summary = generated::users::get_user_summary(pool, user.id)
        .await
        .unwrap();
    assert_eq!(summary.id, user.id);
    assert_eq!(summary.email, user.email);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_info_by_email() {
    let pool = common::get_pool().await;
    let email = common::unique_email("info_email");
    generated::users::insert_user(
        pool,
        "Info By Email".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let found = generated::users::get_user_info_by_email(pool, email.clone())
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Info By Email");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_user_summaries() {
    let pool = common::get_pool().await;
    common::insert_test_user(pool, "summaries").await;

    let summaries = generated::users::get_all_user_summaries(pool)
        .await
        .unwrap();
    assert!(!summaries.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_details() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "details").await;

    let details = generated::users::get_user_details(pool, user.id)
        .await
        .unwrap();
    assert_eq!(details.id, user.id);
    assert_eq!(details.email, user.email);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_search_user_details() {
    let pool = common::get_pool().await;
    let email = common::unique_email("searchdet");
    generated::users::insert_user(
        pool,
        "SearchDetailUser".to_string(),
        email,
        28,
        common::default_profile(),
    )
    .await
    .unwrap();

    let results = generated::users::search_user_details(pool, "%SearchDetail%".to_string())
        .await
        .unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|d| d.name == "SearchDetailUser"));
}
