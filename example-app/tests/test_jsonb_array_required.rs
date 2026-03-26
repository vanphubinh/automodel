mod common;

use example_app::generated;
use example_app::models::UserTag;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_labels_set_and_get() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "labels_basic").await;

    // Read default (should be empty array, NOT null)
    let read = generated::users_array_fields::get_user_labels(pool, user.id)
        .await
        .unwrap();
    assert_eq!(read.labels.len(), 0);

    // Set labels
    let labels = vec![
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
    let updated = generated::users_array_fields::update_user_labels(pool, labels, user.id)
        .await
        .unwrap();
    assert_eq!(updated.labels.len(), 3);
    assert!(updated.labels[0].is_some());
    assert!(updated.labels[1].is_none());

    // Read back
    let read = generated::users_array_fields::get_user_labels(pool, user.id)
        .await
        .unwrap();
    assert_eq!(read.labels[0].as_ref().unwrap().label, "lang");
    assert!(read.labels[1].is_none());

    // Set to empty
    let updated2 = generated::users_array_fields::update_user_labels(pool, vec![], user.id)
        .await
        .unwrap();
    assert_eq!(updated2.labels.len(), 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_labels_structured() {
    let pool = common::get_pool().await;

    let params = generated::users_array_fields::InsertUserLabelsStructuredParams {
        name: "Labels Struct".to_string(),
        email: common::unique_email("labels_struct"),
        labels: vec![
            Some(UserTag {
                label: "team".to_string(),
                value: "backend".to_string(),
            }),
            None,
        ],
    };
    let result = generated::users_array_fields::insert_user_labels_structured(pool, &params)
        .await
        .unwrap();
    assert_eq!(result.labels.len(), 2);
    assert!(result.labels[1].is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_labels_diff() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "labels_diff").await;

    let initial_labels = vec![Some(UserTag {
        label: "lang".to_string(),
        value: "rust".to_string(),
    })];
    generated::users_array_fields::update_user_labels(pool, initial_labels.clone(), user.id)
        .await
        .unwrap();

    let old = generated::users_array_fields::UpdateUserLabelsDiffParams {
        name: user.name.clone(),
        labels: initial_labels,
    };
    let new = generated::users_array_fields::UpdateUserLabelsDiffParams {
        name: user.name.clone(),
        labels: vec![
            Some(UserTag {
                label: "lang".to_string(),
                value: "go".to_string(),
            }),
            None,
        ],
    };
    let updated = generated::users_array_fields::update_user_labels_diff(pool, &old, &new, user.id)
        .await
        .unwrap();
    assert_eq!(updated.labels.len(), 2);
    assert!(updated.labels[1].is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_labels_conditional() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "labels_cond").await;

    generated::users_array_fields::update_user_labels(
        pool,
        vec![Some(UserTag {
            label: "init".to_string(),
            value: "true".to_string(),
        })],
        user.id,
    )
    .await
    .unwrap();

    // Update only labels
    let new_labels = vec![Some(UserTag {
        label: "updated".to_string(),
        value: "yes".to_string(),
    })];
    let updated = generated::users_array_fields::update_user_labels_conditional(
        pool,
        None,
        Some(new_labels),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, user.name);
    assert_eq!(updated.labels[0].as_ref().unwrap().label, "updated");

    // Update only name
    let updated2 = generated::users_array_fields::update_user_labels_conditional(
        pool,
        Some("Labels Cond Updated".to_string()),
        None,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated2.name, "Labels Cond Updated");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_labels_batch() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::users_array_fields::InsertUsersBatchLabelsRecord {
            name: "Label Batch 1".to_string(),
            email: common::unique_email("label_batch1"),
            labels: vec![
                Some(UserTag {
                    label: "lang".to_string(),
                    value: "rust".to_string(),
                }),
                None,
            ],
        },
        generated::users_array_fields::InsertUsersBatchLabelsRecord {
            name: "Label Batch 2".to_string(),
            email: common::unique_email("label_batch2"),
            labels: vec![],
        },
    ];
    let results = generated::users_array_fields::insert_users_batch_labels(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].labels.len(), 2);
    assert!(results[0].labels[1].is_none());
    assert_eq!(results[1].labels.len(), 0);
}
