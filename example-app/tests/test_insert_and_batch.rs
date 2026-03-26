mod common;

use example_app::generated;
use example_app::models;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_user() {
    let pool = common::get_pool().await;
    let email = common::unique_email("insert");
    let user = generated::users::insert_user(
        pool,
        "Insert Test".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();
    assert_eq!(user.name, "Insert Test");
    assert_eq!(user.email, email);
    assert_eq!(user.age, Some(25));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_users_batch() {
    let pool = common::get_pool().await;
    let items = vec![
        generated::users::InsertUsersBatchRecord {
            name: "Batch 1".to_string(),
            email: common::unique_email("batch1"),
            age: 20,
        },
        generated::users::InsertUsersBatchRecord {
            name: "Batch 2".to_string(),
            email: common::unique_email("batch2"),
            age: 30,
        },
    ];
    // batch insert returns ()
    generated::users::insert_users_batch(pool, items)
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_users() {
    let pool = common::get_pool().await;
    // ensure at least one user exists
    common::insert_test_user(pool, "getall").await;
    let users = generated::users::get_all_users(pool).await.unwrap();
    assert!(!users.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_find_user_by_email() {
    let pool = common::get_pool().await;
    let email = common::unique_email("findbyemail");
    generated::users::insert_user(
        pool,
        "Find By Email".to_string(),
        email.clone(),
        28,
        common::default_profile(),
    )
    .await
    .unwrap();

    let found = generated::users::find_user_by_email(pool, email.clone())
        .await
        .unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.email, email);
    assert_eq!(found.name, "Find By Email");

    // non-existent email
    let not_found = generated::users::find_user_by_email(pool, "nonexistent@test.com".to_string())
        .await
        .unwrap();
    assert!(not_found.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_user_structured() {
    let pool = common::get_pool().await;
    let params = generated::users::InsertUserStructuredParams {
        name: "Structured Insert".to_string(),
        email: common::unique_email("structured"),
        age: 42,
    };
    let user = generated::users::insert_user_structured(pool, &params)
        .await
        .unwrap();
    assert_eq!(user.name, "Structured Insert");
    assert_eq!(user.age, Some(42));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_user_with_social_links() {
    let pool = common::get_pool().await;
    let links = vec![models::UserSocialLink {
        name: "GitHub".to_string(),
        url: "https://github.com/test".to_string(),
    }];
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Social User".to_string(),
        common::unique_email("social"),
        links.clone(),
    )
    .await
    .unwrap();
    assert!(user.social_links.is_some());
    assert_eq!(user.social_links.unwrap().len(), 1);
}
