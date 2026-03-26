mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_widgets_bulk() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::types::public::Widgets {
            id: 0,
            name: "Widget-A".to_string(),
            weight: Some(3.14),
            metadata: Some(generated::types::public::WidgetMetadata {
                color: Some("red".to_string()),
                version: Some(1),
            }),
            created_at: None,
        },
        generated::types::public::Widgets {
            id: 0,
            name: "Widget-B".to_string(),
            weight: Some(2.72),
            metadata: None,
            created_at: None,
        },
    ];

    let results = generated::widgets::insert_widgets_bulk(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].name, "Widget-A");
    assert!(results[0].metadata.is_some());
    assert_eq!(
        results[0].metadata.as_ref().unwrap().color,
        Some("red".to_string())
    );
    assert!(results[1].metadata.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_widgets_custom_type() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::types::public::WidgetInput {
            name: "Custom-X".to_string(),
            weight: Some(9.81),
            metadata: Some(generated::types::public::WidgetMetadata {
                color: Some("blue".to_string()),
                version: Some(3),
            }),
        },
        generated::types::public::WidgetInput {
            name: "Custom-Y".to_string(),
            weight: None,
            metadata: None,
        },
    ];

    let results = generated::widgets::insert_widgets_custom_type(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].name, "Custom-X");
    assert!(results[0].metadata.is_some());
    assert!(results[1].metadata.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_widget_single() {
    let pool = common::get_pool().await;

    let item = generated::types::public::WidgetInput {
        name: "Single-Z".to_string(),
        weight: Some(42.0),
        metadata: Some(generated::types::public::WidgetMetadata {
            color: Some("green".to_string()),
            version: Some(7),
        }),
    };

    let result = generated::widgets::insert_widget_single(pool, item)
        .await
        .unwrap();
    assert_eq!(result.name, "Single-Z");
    assert_eq!(result.weight, Some(42.0));
    assert_eq!(
        result.metadata.as_ref().unwrap().color,
        Some("green".to_string())
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_all_widgets() {
    let pool = common::get_pool().await;

    // Insert one first to ensure non-empty
    let item = generated::types::public::WidgetInput {
        name: "GetAll Widget".to_string(),
        weight: Some(1.0),
        metadata: None,
    };
    generated::widgets::insert_widget_single(pool, item)
        .await
        .unwrap();

    let widgets = generated::widgets::get_all_widgets(pool).await.unwrap();
    assert!(!widgets.is_empty());
}
