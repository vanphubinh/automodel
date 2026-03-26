mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_create_user() {
    let pool = common::get_pool().await;
    let email = common::unique_email("model_create");
    let user = generated::user_model::create_user(
        pool,
        "Model Create".to_string(),
        email.clone(),
        Some(25),
    )
    .await
    .unwrap();
    assert_eq!(user.name, "Model Create");
    assert_eq!(user.email, email);
    assert_eq!(user.age, Some(25));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_full() {
    let pool = common::get_pool().await;
    let email = common::unique_email("model_full");
    let user = generated::user_model::create_user(pool, "Full Update".to_string(), email, Some(25))
        .await
        .unwrap();

    let new_email = common::unique_email("model_full_new");
    let params = generated::user_model::UserModel {
        id: user.id,
        name: "Full Updated".to_string(),
        email: new_email.clone(),
        age: Some(35),
    };
    let updated = generated::user_model::update_user_full(pool, &params)
        .await
        .unwrap();
    assert_eq!(updated.name, "Full Updated");
    assert_eq!(updated.email, new_email);
    assert_eq!(updated.age, Some(35));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_partial() {
    let pool = common::get_pool().await;
    let email = common::unique_email("model_partial");
    let user = generated::user_model::create_user(
        pool,
        "Partial Update".to_string(),
        email.clone(),
        Some(25),
    )
    .await
    .unwrap();

    let old = generated::user_model::UserModel {
        id: user.id,
        name: "Partial Update".to_string(),
        email: email.clone(),
        age: Some(25),
    };
    let new = generated::user_model::UserModel {
        id: user.id,
        name: "Partial Changed".to_string(),
        email: email.clone(),
        age: Some(25), // same
    };
    let updated = generated::user_model::update_user_partial(pool, &old, &new, user.id)
        .await
        .unwrap();
    assert_eq!(updated.name, "Partial Changed");
    assert_eq!(updated.email, email);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_find_user_by_email() {
    let pool = common::get_pool().await;
    let email = common::unique_email("model_find");
    generated::user_model::create_user(pool, "Model Find".to_string(), email.clone(), Some(30))
        .await
        .unwrap();

    let found = generated::user_model::find_user_by_email(pool, email.clone())
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Model Find");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_nullable() {
    let pool = common::get_pool().await;
    let email = common::unique_email("model_nullable");
    let user =
        generated::user_model::create_user(pool, "Nullable Test".to_string(), email, Some(25))
            .await
            .unwrap();

    // Case 1: None → skip (age unchanged)
    let updated = generated::user_model::update_user_nullable(
        pool,
        Some("Nullable v2".to_string()),
        None,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.age, Some(25));

    // Case 2: Some(None) → set NULL
    let updated = generated::user_model::update_user_nullable(pool, None, Some(None), user.id)
        .await
        .unwrap();
    assert_eq!(updated.age, None);

    // Case 3: Some(Some(42)) → set value
    let updated = generated::user_model::update_user_nullable(pool, None, Some(Some(42)), user.id)
        .await
        .unwrap();
    assert_eq!(updated.age, Some(42));
}
