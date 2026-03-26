mod common;

use example_app::generated;
use example_app::models::UserTag;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_tags_set_and_get() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "tags_basic").await;

    // Set tags with mixed Some/None
    let tags = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "rust".to_string(),
        }),
        None,
        Some(UserTag {
            label: "role".to_string(),
            value: "dev".to_string(),
        }),
    ];
    let updated = generated::users_array_fields::update_user_tags(pool, tags, user.id)
        .await
        .unwrap();
    let result_tags = updated.tags.unwrap();
    assert_eq!(result_tags.len(), 3);
    assert!(result_tags[0].is_some());
    assert!(result_tags[1].is_none());
    assert!(result_tags[2].is_some());

    // Read back
    let read = generated::users_array_fields::get_user_tags(pool, user.id)
        .await
        .unwrap();
    let read_tags = read.tags.unwrap();
    assert_eq!(read_tags.len(), 3);
    assert_eq!(read_tags[0].as_ref().unwrap().label, "lang");
    assert!(read_tags[1].is_none());

    // Set to empty
    let updated2 = generated::users_array_fields::update_user_tags(pool, vec![], user.id)
        .await
        .unwrap();
    assert_eq!(updated2.tags.unwrap().len(), 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_tags_structured() {
    let pool = common::get_pool().await;

    let params = generated::users_array_fields::InsertUserTagsStructuredParams {
        name: "Tags Struct".to_string(),
        email: common::unique_email("tags_struct"),
        tags: vec![
            Some(UserTag {
                label: "team".to_string(),
                value: "backend".to_string(),
            }),
            None,
        ],
    };
    let result = generated::users_array_fields::insert_user_tags_structured(pool, &params)
        .await
        .unwrap();
    let tags = result.tags.unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags[1].is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_tags_diff() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "tags_diff").await;

    let initial_tags = vec![Some(UserTag {
        label: "lang".to_string(),
        value: "rust".to_string(),
    })];
    generated::users_array_fields::update_user_tags(pool, initial_tags.clone(), user.id)
        .await
        .unwrap();

    let old = generated::users_array_fields::UpdateUserTagsDiffParams {
        name: user.name.clone(),
        tags: initial_tags,
    };
    let new_tags = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "go".to_string(),
        }),
        None,
    ];
    let new = generated::users_array_fields::UpdateUserTagsDiffParams {
        name: user.name.clone(),
        tags: new_tags,
    };
    let updated = generated::users_array_fields::update_user_tags_diff(pool, &old, &new, user.id)
        .await
        .unwrap();
    let tags = updated.tags.unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags[1].is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_tags_conditional() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "tags_cond").await;

    generated::users_array_fields::update_user_tags(
        pool,
        vec![Some(UserTag {
            label: "init".to_string(),
            value: "true".to_string(),
        })],
        user.id,
    )
    .await
    .unwrap();

    // Update only tags
    let new_tags = vec![Some(UserTag {
        label: "updated".to_string(),
        value: "yes".to_string(),
    })];
    let updated = generated::users_array_fields::update_user_tags_conditional(
        pool,
        None,
        Some(new_tags),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, user.name);
    assert_eq!(updated.tags.unwrap()[0].as_ref().unwrap().label, "updated");

    // Update only name
    let updated2 = generated::users_array_fields::update_user_tags_conditional(
        pool,
        Some("Tags Cond Updated".to_string()),
        None,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated2.name, "Tags Cond Updated");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_tags_batch() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::users_array_fields::InsertUsersBatchTagsRecord {
            name: "Tag Batch 1".to_string(),
            email: common::unique_email("tag_batch1"),
            tags: vec![
                Some(UserTag {
                    label: "lang".to_string(),
                    value: "rust".to_string(),
                }),
                None,
            ],
        },
        generated::users_array_fields::InsertUsersBatchTagsRecord {
            name: "Tag Batch 2".to_string(),
            email: common::unique_email("tag_batch2"),
            tags: vec![],
        },
    ];
    let results = generated::users_array_fields::insert_users_batch_tags(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);

    let tags1 = results[0].tags.as_ref().unwrap();
    assert_eq!(tags1.len(), 2);
    assert!(tags1[1].is_none());

    let tags2 = results[1].tags.as_ref().unwrap();
    assert_eq!(tags2.len(), 0);
}
