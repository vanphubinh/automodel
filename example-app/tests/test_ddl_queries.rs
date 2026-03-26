mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_create_users_table() {
    let pool = common::get_pool().await;
    // DDL — CREATE TABLE IF NOT EXISTS, safe to call multiple times
    generated::setup::create_users_table(pool).await.unwrap();
}
