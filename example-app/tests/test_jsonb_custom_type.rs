mod common;

use example_app::generated;
use example_app::models::UserSocialLink;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_social_links_crud() {
    let pool = common::get_pool().await;

    // Insert user with social links
    let links = vec![
        UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/test".to_string(),
        },
        UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/test".to_string(),
        },
    ];
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Social CRUD".to_string(),
        common::unique_email("social_crud"),
        links,
    )
    .await
    .unwrap();
    assert!(user.social_links.is_some());
    assert_eq!(user.social_links.as_ref().unwrap().len(), 2);

    // Retrieve
    let fetched = generated::users::get_user_social_links(pool, user.id)
        .await
        .unwrap();
    assert_eq!(fetched.social_links.as_ref().unwrap().len(), 2);

    // Update
    let new_links = vec![UserSocialLink {
        name: "Website".to_string(),
        url: "https://example.com".to_string(),
    }];
    let updated = generated::users::update_user_social_links(pool, new_links, user.id)
        .await
        .unwrap();
    assert_eq!(updated.social_links.as_ref().unwrap().len(), 1);

    // Clear
    let cleared = generated::users::update_user_social_links(pool, vec![], user.id)
        .await
        .unwrap();
    let cleared_links = cleared.social_links.unwrap();
    assert_eq!(cleared_links.len(), 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_nullable_social_links() {
    let pool = common::get_pool().await;

    let user = generated::users::insert_user_with_social_links(
        pool,
        "Nullable SL".to_string(),
        common::unique_email("nullable_sl"),
        vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/null".to_string(),
        }],
    )
    .await
    .unwrap();

    // Set to NULL
    let nulled =
        generated::users_array_fields::update_user_social_links_nullable(pool, None, user.id)
            .await
            .unwrap();
    assert!(nulled.social_links.is_none());

    // Restore
    let restored = generated::users_array_fields::update_user_social_links_nullable(
        pool,
        Some(vec![UserSocialLink {
            name: "Restored".to_string(),
            url: "https://restored.com".to_string(),
        }]),
        user.id,
    )
    .await
    .unwrap();
    assert!(restored.social_links.is_some());
    assert_eq!(restored.social_links.unwrap().len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_social_links_structured() {
    let pool = common::get_pool().await;

    let params = generated::users_array_fields::InsertUserSocialLinksStructuredParams {
        name: "SL Structured".to_string(),
        email: common::unique_email("sl_struct"),
        social_links: vec![
            UserSocialLink {
                name: "GitHub".to_string(),
                url: "https://github.com/structured".to_string(),
            },
            UserSocialLink {
                name: "Blog".to_string(),
                url: "https://blog.dev".to_string(),
            },
        ],
    };
    let result = generated::users_array_fields::insert_user_social_links_structured(pool, &params)
        .await
        .unwrap();
    let links = result.social_links.unwrap();
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].name, "GitHub");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_social_links_diff() {
    let pool = common::get_pool().await;

    let user = generated::users::insert_user_with_social_links(
        pool,
        "SL Diff".to_string(),
        common::unique_email("sl_diff"),
        vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/diff".to_string(),
        }],
    )
    .await
    .unwrap();

    let old = generated::users_array_fields::UpdateUserSocialLinksDiffParams {
        name: "SL Diff".to_string(),
        social_links: vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/diff".to_string(),
        }],
    };
    let new = generated::users_array_fields::UpdateUserSocialLinksDiffParams {
        name: "SL Diff".to_string(),
        social_links: vec![UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/diff".to_string(),
        }],
    };
    let updated =
        generated::users_array_fields::update_user_social_links_diff(pool, &old, &new, user.id)
            .await
            .unwrap();
    let links = updated.social_links.unwrap();
    assert_eq!(links[0].name, "Twitter");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_social_links_conditional() {
    let pool = common::get_pool().await;

    let user = generated::users::insert_user_with_social_links(
        pool,
        "SL Cond".to_string(),
        common::unique_email("sl_cond"),
        vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/cond".to_string(),
        }],
    )
    .await
    .unwrap();

    // Update only social_links
    let new_links = vec![UserSocialLink {
        name: "Twitter".to_string(),
        url: "https://twitter.com/cond".to_string(),
    }];
    let updated = generated::users_array_fields::update_user_social_links_conditional(
        pool,
        None,
        Some(new_links),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, "SL Cond");
    assert_eq!(updated.social_links.unwrap()[0].name, "Twitter");

    // Update only name
    let updated2 = generated::users_array_fields::update_user_social_links_conditional(
        pool,
        Some("SL Cond Updated".to_string()),
        None,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated2.name, "SL Cond Updated");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_social_links_batch() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::users_array_fields::InsertUsersBatchSocialLinksRecord {
            name: "SL Batch 1".to_string(),
            email: common::unique_email("sl_batch1"),
            social_links: Some(vec![UserSocialLink {
                name: "GitHub".to_string(),
                url: "https://github.com/batch1".to_string(),
            }]),
        },
        generated::users_array_fields::InsertUsersBatchSocialLinksRecord {
            name: "SL Batch 2".to_string(),
            email: common::unique_email("sl_batch2"),
            social_links: None,
        },
    ];
    let results = generated::users_array_fields::insert_users_batch_social_links(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert!(results[0].social_links.is_some());
    assert!(results[1].social_links.is_none());
}
