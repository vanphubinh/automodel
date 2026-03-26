mod common;

use example_app::generated;
use example_app::models;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_fields_name_only() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "upd_name").await;

    let updated = generated::users::update_user_fields(
        pool,
        Some("New Name".to_string()),
        None,
        None,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, "New Name");
    assert_eq!(updated.email, user.email);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_fields_age_only() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "upd_age").await;

    let updated = generated::users::update_user_fields(pool, None, None, Some(99), user.id)
        .await
        .unwrap();
    assert_eq!(updated.age, Some(99));
    assert_eq!(updated.name, user.name);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_fields_all() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "upd_all").await;
    let new_email = common::unique_email("upd_all_new");

    let updated = generated::users::update_user_fields(
        pool,
        Some("All Updated".to_string()),
        Some(new_email.clone()),
        Some(55),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, "All Updated");
    assert_eq!(updated.email, new_email);
    assert_eq!(updated.age, Some(55));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_fields_diff() {
    let pool = common::get_pool().await;
    let email = common::unique_email("diff_upd");
    let user = generated::users::insert_user(
        pool,
        "Diff Test".to_string(),
        email.clone(),
        28,
        common::default_profile(),
    )
    .await
    .unwrap();

    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Test".to_string(),
        email: email.clone(),
        age: 28,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Diff Updated".to_string(),
        email: email.clone(),
        age: 28,
    };
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id)
        .await
        .unwrap();
    assert_eq!(updated.name, "Diff Updated");
    assert_eq!(updated.email, email);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_status() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "upd_status").await;

    let updated = generated::users::update_user_status(
        pool,
        example_app::generated::types::public::UserStatus::Active,
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(
        updated.status,
        Some(example_app::generated::types::public::UserStatus::Active)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_profile() {
    let pool = common::get_pool().await;
    let user = common::insert_test_user(pool, "upd_profile").await;

    let new_profile = models::UserProfile {
        bio: Some("Updated bio".to_string()),
        avatar_url: Some("https://example.com/avatar.png".to_string()),
        preferences: models::UserPreferences {
            theme: "light".to_string(),
            language: "fr".to_string(),
            notifications_enabled: false,
        },
        social_links: vec![],
    };

    let updated = generated::users::update_user_profile(pool, new_profile, user.id)
        .await
        .unwrap();
    assert_eq!(updated.id, user.id);
    let profile = updated.profile.unwrap();
    assert_eq!(profile.bio, Some("Updated bio".to_string()));
    assert_eq!(profile.preferences.theme, "light");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_profile_diff() {
    let pool = common::get_pool().await;
    let email = common::unique_email("profile_diff");
    let user = generated::users::insert_user(
        pool,
        "Profile Diff".to_string(),
        email.clone(),
        30,
        common::default_profile(),
    )
    .await
    .unwrap();

    let old = generated::users::UpdateUserProfileDiffParams {
        name: "Profile Diff".to_string(),
        email: email.clone(),
    };
    let new = generated::users::UpdateUserProfileDiffParams {
        name: "Profile Diff Updated".to_string(),
        email: email.clone(),
    };
    let updated = generated::users::update_user_profile_diff(
        pool,
        &old,
        &new,
        common::default_profile(),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.name, "Profile Diff Updated");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_update_user_metadata_diff() {
    let pool = common::get_pool().await;
    let email = common::unique_email("meta_diff");
    let user = generated::users::insert_user(
        pool,
        "Meta Diff".to_string(),
        email.clone(),
        30,
        common::default_profile(),
    )
    .await
    .unwrap();

    let old = generated::users::UpdateUserProfileDiffParams {
        name: "Meta Diff".to_string(),
        email: email.clone(),
    };
    let new_email = common::unique_email("meta_diff_new");
    let new = generated::users::UpdateUserProfileDiffParams {
        name: "Meta Diff".to_string(),
        email: new_email.clone(),
    };
    let updated = generated::users::update_user_metadata_diff(
        pool,
        &old,
        &new,
        common::default_profile(),
        user.id,
    )
    .await
    .unwrap();
    assert_eq!(updated.email, new_email);
}
