mod common;

use example_app::generated;
use example_app::models;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_user_by_id_and_email() {
    let pool = common::get_pool().await;
    let email = common::unique_email("by_id_email");
    let user = generated::users::insert_user(
        pool,
        "ByIdEmail".to_string(),
        email.clone(),
        35,
        common::default_profile(),
    )
    .await
    .unwrap();

    let params = generated::users::GetUserByIdAndEmailParams {
        id: user.id,
        email: email.clone(),
    };
    let found = generated::users::get_user_by_id_and_email(pool, &params)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "ByIdEmail");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_delete_user_by_id_and_email() {
    let pool = common::get_pool().await;
    let email = common::unique_email("delete_test");
    let user = generated::users::insert_user(
        pool,
        "Delete Test".to_string(),
        email.clone(),
        25,
        common::default_profile(),
    )
    .await
    .unwrap();

    let params = generated::users::GetUserByIdAndEmailParams {
        id: user.id,
        email: email.clone(),
    };
    let deleted = generated::users::delete_user_by_id_and_email(pool, &params)
        .await
        .unwrap();
    assert_eq!(deleted.id, user.id);

    // verify deleted
    let found = generated::users::get_user_by_id_and_email(pool, &params)
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_contact_info() {
    let pool = common::get_pool().await;
    let email = common::unique_email("contact_info");
    let user = generated::users::insert_user(
        pool,
        "Contact Info".to_string(),
        email.clone(),
        30,
        common::default_profile(),
    )
    .await
    .unwrap();

    let update_params = generated::users::GetUserByIdAndEmailItem {
        id: user.id,
        name: "Contact Updated".to_string(),
        email: email.clone(),
    };
    let updated = generated::users::update_user_contact_info(pool, &update_params)
        .await
        .unwrap();
    assert_eq!(updated.name, "Contact Updated");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_find_user_by_criteria() {
    let pool = common::get_pool().await;
    let email = common::unique_email("criteria");
    let user = generated::users::insert_user(
        pool,
        "Criteria Test".to_string(),
        email.clone(),
        30,
        common::default_profile(),
    )
    .await
    .unwrap();

    let params = generated::users::GetUserByIdAndEmailParams {
        id: user.id,
        email: email.clone(),
    };
    let found = generated::users::find_user_by_criteria(pool, &params)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Criteria Test");
}
