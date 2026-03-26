mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_simple() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "simple").await;

    let found = generated::users::get_user_simple(pool, user.id)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_custom_derives() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "derives").await;

    let params = generated::users::TestCustomDerivesParams { user_id: user.id };
    let result = generated::users::test_custom_derives(pool, &params)
        .await
        .unwrap();
    assert_eq!(result.id, user.id);

    // Verify serde derives work
    let json = serde_json::to_string(&result).unwrap();
    let deserialized: generated::users::UserWithCustomDerives =
        serde_json::from_str(&json).unwrap();
    assert_eq!(result, deserialized);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_id_only() {
    let pool = common::get_pool().await;
    let email = common::unique_email("idonly");
    generated::users::insert_user(
        pool,
        "ID Only".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let user_id = generated::users::get_user_id_only(pool, email)
        .await
        .unwrap();
    assert!(user_id.id > 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_id_raw() {
    let pool = common::get_pool().await;
    let email = common::unique_email("idraw");
    generated::users::insert_user(
        pool,
        "ID Raw".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let id = generated::users::get_user_id_raw(pool, email)
        .await
        .unwrap();
    assert!(id > 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_nested_row() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "nested_row").await;

    let result = generated::users::test_nested_row(pool, user.id)
        .await
        .unwrap();
    assert_eq!(result.id, user.id);
    assert!(result.user_details.is_some());
    let details = result.user_details.unwrap();
    assert_eq!(details.id, user.id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_social_links() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "get_social").await;

    let result = generated::users::get_user_social_links(pool, user.id)
        .await
        .unwrap();
    assert_eq!(result.id, user.id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_social_links() {
    let pool = common::get_pool().await;
    let links = vec![example_app::models::UserSocialLink {
        name: "GitHub".to_string(),
        url: "https://github.com/test".to_string(),
    }];
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Social Update".to_string(),
        common::unique_email("soc_update"),
        links,
    )
    .await
    .unwrap();

    let new_links = vec![
        example_app::models::UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/test".to_string(),
        },
        example_app::models::UserSocialLink {
            name: "Website".to_string(),
            url: "https://example.com".to_string(),
        },
    ];
    let updated = generated::users::update_user_social_links(pool, new_links, user.id)
        .await
        .unwrap();
    let links = updated.social_links.unwrap();
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].name, "Twitter");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_explicit_native_multiunzip() {
    let pool = common::get_pool().await;
    let items = vec![generated::users::TestExplicitNativeMultiunzipRecord {
        names: "NativeMultiunzip".to_string(),
        age: Some(25),
    }];
    // Query inserts into users without email (NOT NULL) — expect constraint violation
    let err = generated::users::test_explicit_native_multiunzip(pool, items)
        .await
        .unwrap_err();
    assert!(
        matches!(
            err,
            example_app::generated::Error::ConstraintViolation(_, _)
        ),
        "expected NotNull constraint violation, got: {:?}",
        err
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_explicit_native_without_multiunzip() {
    let pool = common::get_pool().await;
    // Query inserts into users without email (NOT NULL) — expect constraint violation
    let err = generated::users::test_explicit_native_without_multiunzip(
        pool,
        vec!["NativeNoMultiunzip".to_string()],
        vec![Some(30)],
    )
    .await
    .unwrap_err();
    assert!(
        matches!(
            err,
            example_app::generated::Error::ConstraintViolation(_, _)
        ),
        "expected NotNull constraint violation, got: {:?}",
        err
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_optional_multiunzip() {
    let pool = common::get_pool().await;
    let items = vec![
        generated::users::TestOptionalMultiunzipRecord {
            name: "OptMulti1".to_string(),
            email: common::unique_email("optmulti1"),
            age: Some(20),
        },
        generated::users::TestOptionalMultiunzipRecord {
            name: "OptMulti2".to_string(),
            email: common::unique_email("optmulti2"),
            age: None,
        },
    ];
    let results = generated::users::test_optional_multiunzip(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].age, Some(20));
    assert_eq!(results[1].age, None);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_optional_without_multiunzip() {
    let pool = common::get_pool().await;
    let results = generated::users::test_optional_without_multiunzip(
        pool,
        vec!["OptNoMulti1".to_string(), "OptNoMulti2".to_string()],
        vec![
            common::unique_email("optnomulti1"),
            common::unique_email("optnomulti2"),
        ],
        vec![Some(40), None],
    )
    .await
    .unwrap();
    assert_eq!(results.len(), 2);
}
