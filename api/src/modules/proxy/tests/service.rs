use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

use crate::modules::proxy::service;

/// Helper to create an in-memory `SQLite` pool with migrations applied.
async fn test_db() -> SqlitePool {
    let db = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}

#[tokio::test]
async fn test_get_all_profiles_empty() {
    let db = test_db().await;
    let profiles = service::get_all_profiles(&db).await.unwrap();
    assert!(profiles.is_empty());
}
