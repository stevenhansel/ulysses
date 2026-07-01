use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::modules::proxy::repository;

/// Helper to create an in-memory SQLite pool with migrations applied.
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
async fn test_find_all_profiles_empty() {
    let db = test_db().await;
    let records = repository::find_all_profiles(&db).await.unwrap();
    assert!(records.is_empty());
}
