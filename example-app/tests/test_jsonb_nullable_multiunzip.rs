mod common;

use example_app::generated;
use example_app::models::{ArticleContributor, ArticleMetadata};
use sqlx::types::Json;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_batch_insert_articles_all_fields() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::articles::BatchInsertArticlesRecord {
            title: "Article One".to_string(),
            metadata: Some(Json(ArticleMetadata {
                category: "tech".to_string(),
                published: true,
            })),
            contributors: Some(Json(vec![
                ArticleContributor {
                    name: "Alice".to_string(),
                    role: "author".to_string(),
                },
                ArticleContributor {
                    name: "Bob".to_string(),
                    role: "editor".to_string(),
                },
            ])),
        },
        generated::articles::BatchInsertArticlesRecord {
            title: "Article Two".to_string(),
            metadata: Some(Json(ArticleMetadata {
                category: "science".to_string(),
                published: false,
            })),
            contributors: Some(Json(vec![ArticleContributor {
                name: "Charlie".to_string(),
                role: "author".to_string(),
            }])),
        },
    ];

    let results = generated::articles::batch_insert_articles(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);

    // First article
    assert_eq!(results[0].title, "Article One");
    let meta0 = results[0].metadata.as_ref().unwrap();
    assert_eq!(meta0.category, "tech");
    assert!(meta0.published);
    let contribs0 = results[0].contributors.as_ref().unwrap();
    assert_eq!(contribs0.len(), 2);
    assert_eq!(contribs0[0].name, "Alice");
    assert_eq!(contribs0[1].role, "editor");

    // Second article
    assert_eq!(results[1].title, "Article Two");
    let meta1 = results[1].metadata.as_ref().unwrap();
    assert_eq!(meta1.category, "science");
    assert!(!meta1.published);
    let contribs1 = results[1].contributors.as_ref().unwrap();
    assert_eq!(contribs1.len(), 1);
    assert_eq!(contribs1[0].name, "Charlie");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_batch_insert_articles_null_fields() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::articles::BatchInsertArticlesRecord {
            title: "Null Meta Article".to_string(),
            metadata: None,
            contributors: Some(Json(vec![ArticleContributor {
                name: "Dave".to_string(),
                role: "author".to_string(),
            }])),
        },
        generated::articles::BatchInsertArticlesRecord {
            title: "Null Contribs Article".to_string(),
            metadata: Some(Json(ArticleMetadata {
                category: "art".to_string(),
                published: true,
            })),
            contributors: None,
        },
        generated::articles::BatchInsertArticlesRecord {
            title: "All Null Article".to_string(),
            metadata: None,
            contributors: None,
        },
    ];

    let results = generated::articles::batch_insert_articles(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 3);

    // First: metadata is None, contributors is Some
    assert!(results[0].metadata.is_none());
    let contribs0 = results[0].contributors.as_ref().unwrap();
    assert_eq!(contribs0.len(), 1);
    assert_eq!(contribs0[0].name, "Dave");

    // Second: metadata is Some, contributors is None
    let meta1 = results[1].metadata.as_ref().unwrap();
    assert_eq!(meta1.category, "art");
    assert!(results[1].contributors.is_none());

    // Third: both None
    assert!(results[2].metadata.is_none());
    assert!(results[2].contributors.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_batch_insert_articles_empty_contributors() {
    let pool = common::get_pool().await;

    let items = vec![generated::articles::BatchInsertArticlesRecord {
        title: "Empty Contribs Article".to_string(),
        metadata: Some(Json(ArticleMetadata {
            category: "misc".to_string(),
            published: false,
        })),
        contributors: Some(Json(vec![])),
    }];

    let results = generated::articles::batch_insert_articles(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Empty Contribs Article");
    let contribs = results[0].contributors.as_ref().unwrap();
    assert_eq!(contribs.len(), 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_batch_insert_articles_roundtrip() {
    let pool = common::get_pool().await;

    let meta = ArticleMetadata {
        category: "roundtrip".to_string(),
        published: true,
    };
    let contribs = vec![ArticleContributor {
        name: "Eve".to_string(),
        role: "reviewer".to_string(),
    }];

    let items = vec![generated::articles::BatchInsertArticlesRecord {
        title: "Roundtrip Article".to_string(),
        metadata: Some(Json(meta.clone())),
        contributors: Some(Json(contribs.clone())),
    }];

    let inserted = generated::articles::batch_insert_articles(pool, items)
        .await
        .unwrap();
    assert_eq!(inserted.len(), 1);

    // Read back via get_article_by_id
    let fetched = generated::articles::get_article_by_id(pool, inserted[0].id)
        .await
        .unwrap();

    assert_eq!(fetched.title, "Roundtrip Article");
    assert_eq!(fetched.metadata.as_ref().unwrap().0, meta);
    assert_eq!(fetched.contributors.as_ref().unwrap().0, contribs);
}

/// Regression test: scalar @native type mappings in multiunzip must not be truncated.
///
/// When a singular nullable column (e.g. JSONB) is used in a multiunzip query,
/// the PG-derived type_ref becomes Vec<serde_json::Value> (array for UNNEST),
/// but the user's mapped type is scalar (e.g. "Json<ArticleMetadata>@native").
/// A previous bug sliced the mapped type as if it were Vec<...>, truncating it
/// from both ends (e.g. "sqlx::types::Json<crate::models::ArticleMetadata>"
/// became "::types::Json<crate::models::ArticleMetadat").
///
/// This test exercises the scalar @native path end-to-end. If the types are
/// truncated, the generated code won't compile.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_batch_insert_scalar_native_types_not_truncated() {
    let pool = common::get_pool().await;

    // The Record struct uses scalar @native types (no Vec<> wrapper).
    // With the truncation bug, these fields would have garbled type names
    // and the code would not compile at all.
    let items = vec![
        generated::articles::BatchInsertArticlesScalarNativeRecord {
            title: "Scalar Native One".to_string(),
            metadata: Some(Json(ArticleMetadata {
                category: "regression".to_string(),
                published: true,
            })),
            contributors: Some(Json(vec![ArticleContributor {
                name: "Frank".to_string(),
                role: "author".to_string(),
            }])),
        },
        generated::articles::BatchInsertArticlesScalarNativeRecord {
            title: "Scalar Native Two".to_string(),
            metadata: None,
            contributors: None,
        },
    ];

    let results = generated::articles::batch_insert_articles_scalar_native(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);

    assert_eq!(results[0].title, "Scalar Native One");
    let meta = results[0].metadata.as_ref().unwrap();
    assert_eq!(meta.category, "regression");
    assert!(results[1].metadata.is_none());
    assert!(results[1].contributors.is_none());
}
