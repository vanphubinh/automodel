mod common;

use example_app::generated;
use example_app::models::UserSocialLink;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_bulk_composite_unnest() {
    let pool = common::get_pool().await;

    let items = vec![
        generated::types::public::UserWithLinksInput {
            name: Some("Composite 1".to_string()),
            email: Some(common::unique_email("composite1")),
            social_links: Some(vec![
                UserSocialLink {
                    name: "GitHub".to_string(),
                    url: "https://github.com/composite1".to_string(),
                },
                UserSocialLink {
                    name: "LinkedIn".to_string(),
                    url: "https://linkedin.com/in/composite1".to_string(),
                },
            ]),
        },
        generated::types::public::UserWithLinksInput {
            name: Some("Composite 2".to_string()),
            email: Some(common::unique_email("composite2")),
            social_links: None,
        },
        generated::types::public::UserWithLinksInput {
            name: Some("Composite 3".to_string()),
            email: Some(common::unique_email("composite3")),
            social_links: Some(vec![UserSocialLink {
                name: "Website".to_string(),
                url: "https://composite3.dev".to_string(),
            }]),
        },
    ];

    let results = generated::users_array_fields::insert_users_bulk_composite(pool, items)
        .await
        .unwrap();
    assert_eq!(results.len(), 3);

    let links1 = results[0].social_links.as_ref().unwrap();
    assert_eq!(links1.len(), 2);

    assert!(results[1].social_links.is_none());

    let links3 = results[2].social_links.as_ref().unwrap();
    assert_eq!(links3.len(), 1);
}
