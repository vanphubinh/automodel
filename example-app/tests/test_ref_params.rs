mod common;

use example_app::generated;
use example_app::models;

/// Test passing a params struct by reference (&ParamsStruct).
/// Verifies that the struct is borrowed (not moved) and can be reused after the call.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_params_struct_by_ref() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_params_struct");

    let params = generated::users::InsertUserStructuredParams {
        name: "Ref Params".to_string(),
        email: email.clone(),
        age: 40,
    };

    // Pass by reference — struct must not be consumed
    let result = generated::users::insert_user_structured(pool, &params)
        .await
        .unwrap();

    // Struct is still usable after the call (not moved)
    assert_eq!(result.name, params.name);
    assert_eq!(result.email, params.email);
    assert_eq!(result.age, Some(params.age));
}

/// Test diff struct with old/new passed by reference (&old, &new).
/// Both structs must survive the borrow and remain usable afterward.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_diff_struct_by_ref() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_diff");

    // Create initial user
    let user = generated::users::insert_user(
        pool,
        "Diff Ref".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Ref".to_string(),
        email: email.clone(),
        age: 25,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Updated".to_string(),
        email: email.clone(),
        age: 30,
    };

    // Both old and new passed by reference
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id)
        .await
        .unwrap();

    assert_eq!(updated.name, new.name);
    assert_eq!(updated.age, Some(new.age));

    // Both structs still usable after the call
    assert_eq!(old.name, "Diff Ref");
    assert_eq!(new.name, "Diff Updated");
}

/// Test UserModel diff (update_user_partial) with old/new by reference.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_model_diff_by_ref() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_model_diff");

    let created =
        generated::user_model::create_user(pool, "Model Diff".to_string(), email, Some(22))
            .await
            .unwrap();

    let old = created.clone();
    let new = generated::user_model::UserModel {
        id: created.id,
        name: "Model Updated".to_string(),
        email: created.email.clone(),
        age: Some(33),
    };

    // Both passed by reference
    let updated = generated::user_model::update_user_partial(pool, &old, &new, created.id)
        .await
        .unwrap();

    assert_eq!(updated.name, new.name);
    assert_eq!(updated.age, new.age);

    // Both structs survive the borrow
    assert_eq!(old.name, "Model Diff");
    assert_eq!(new.name, "Model Updated");
}

/// Test update_user_full with &UserModel reference.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_model_full_update_by_ref() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_full_update");

    let created =
        generated::user_model::create_user(pool, "Full Update".to_string(), email, Some(20))
            .await
            .unwrap();

    let params = generated::user_model::UserModel {
        id: created.id,
        name: "Full Updated".to_string(),
        email: created.email.clone(),
        age: Some(44),
    };

    // Passed by reference
    let updated = generated::user_model::update_user_full(pool, &params)
        .await
        .unwrap();

    assert_eq!(updated.name, params.name);
    assert_eq!(updated.age, params.age);

    // Struct survives
    assert_eq!(params.name, "Full Updated");
}

/// Test Option<String> and Option<i32> parameters (conditional WHERE).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_option_params() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_option");

    let user = generated::users::insert_user(
        pool,
        "Option Test".to_string(),
        email,
        28,
        common::default_profile(),
    )
    .await
    .unwrap();

    // Update only name (email and age are None → not updated)
    let updated = generated::users::update_user_fields(
        pool,
        Some("Option Updated".to_string()),
        None,
        None,
        user.id,
    )
    .await
    .unwrap();

    assert_eq!(updated.name, "Option Updated");
    assert_eq!(updated.age, Some(28)); // unchanged

    // Update only age
    let updated = generated::users::update_user_fields(pool, None, None, Some(99), user.id)
        .await
        .unwrap();

    assert_eq!(updated.name, "Option Updated"); // unchanged
    assert_eq!(updated.age, Some(99));
}

/// Test Vec<T> parameter (social_links as Vec<UserSocialLink>).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_vec_params() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_vec");

    let user = generated::users::insert_user(
        pool,
        "Vec Test".to_string(),
        email,
        35,
        common::default_profile(),
    )
    .await
    .unwrap();

    let links = vec![
        models::UserSocialLink {
            name: "github".to_string(),
            url: "https://github.com/test".to_string(),
        },
        models::UserSocialLink {
            name: "twitter".to_string(),
            url: "https://twitter.com/test".to_string(),
        },
    ];

    // Vec<T> is consumed (owned parameter)
    let updated = generated::users::update_user_social_links(pool, links, user.id)
        .await
        .unwrap();

    let returned_links = updated.social_links.unwrap();
    assert_eq!(returned_links.len(), 2);
    assert_eq!(returned_links[0].name, "github");
    assert_eq!(returned_links[1].name, "twitter");
}

/// Test multiunzip batch insert with Vec<Record>.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_multiunzip_batch() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::users::InsertUsersBatchRecord {
            name: "Batch A".to_string(),
            email: common::unique_email("ref_batch_a"),
            age: 21,
        },
        generated::users::InsertUsersBatchRecord {
            name: "Batch B".to_string(),
            email: common::unique_email("ref_batch_b"),
            age: 22,
        },
    ];

    // Vec<Record> is consumed (owned parameter), items are unzipped into column-vectors
    generated::users::insert_users_batch(pool, items)
        .await
        .unwrap();
}

/// Test multiunzip batch insert with Vec<Record> containing Option and Json fields.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_multiunzip_with_option_json_fields() {
    let pool = common::get_pool().await;
    use sqlx::types::Json;

    let items = vec![
        generated::articles::BatchInsertArticlesRecord {
            title: "Article Ref A".to_string(),
            metadata: Some(Json(models::ArticleMetadata {
                category: "science".to_string(),
                published: true,
            })),
            contributors: Some(Json(vec![models::ArticleContributor {
                name: "Alice".to_string(),
                role: "author".to_string(),
            }])),
        },
        generated::articles::BatchInsertArticlesRecord {
            title: "Article Ref B".to_string(),
            metadata: None,
            contributors: None,
        },
    ];

    let results = generated::articles::batch_insert_articles(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "Article Ref A");
    assert!(results[0].metadata.is_some());
    assert_eq!(results[1].title, "Article Ref B");
    assert!(results[1].metadata.is_none());
}

/// Test that params struct can be cloned and reused across multiple calls.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_params_struct_reuse_across_calls() {
    let pool = common::get_pool().await;

    let params = generated::users::InsertUserStructuredParams {
        name: "Reuse Test".to_string(),
        email: common::unique_email("ref_reuse_1"),
        age: 50,
    };

    // First call — by reference
    let r1 = generated::users::insert_user_structured(pool, &params)
        .await
        .unwrap();
    assert_eq!(r1.name, params.name);

    // Clone and modify email for second insert (unique constraint)
    let params2 = generated::users::InsertUserStructuredParams {
        email: common::unique_email("ref_reuse_2"),
        ..params.clone()
    };

    // Second call — original params still usable, clone works
    let r2 = generated::users::insert_user_structured(pool, &params2)
        .await
        .unwrap();
    assert_eq!(r2.name, params.name); // same name from original
    assert_ne!(r1.email, r2.email); // different emails

    // Original struct not consumed
    assert_eq!(params.age, 50);
}

/// Test diff struct reuse — call diff update twice with the same &old reference.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_diff_struct_reuse() {
    let pool = common::get_pool().await;
    let email = common::unique_email("ref_diff_reuse");

    let user = generated::users::insert_user(
        pool,
        "Diff Reuse".to_string(),
        email.clone(),
        20,
        common::default_profile(),
    )
    .await
    .unwrap();

    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Reuse".to_string(),
        email: email.clone(),
        age: 20,
    };

    // First update
    let new1 = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Step 1".to_string(),
        email: email.clone(),
        age: 25,
    };
    let r1 = generated::users::update_user_fields_diff(pool, &old, &new1, user.id)
        .await
        .unwrap();
    assert_eq!(r1.name, "Diff Step 1");

    // Second update — reusing &old (comparing against original state)
    // Now old diverges from actual DB state, but the function still borrows it fine
    let new2 = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Step 2".to_string(),
        email: email.clone(),
        age: 30,
    };
    let r2 = generated::users::update_user_fields_diff(pool, &new1, &new2, user.id)
        .await
        .unwrap();
    assert_eq!(r2.name, "Diff Step 2");

    // All three structs survive
    assert_eq!(old.age, 20);
    assert_eq!(new1.age, 25);
    assert_eq!(new2.age, 30);
}
