use std::sync::Arc;

use axum_test::TestServer;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::config::Config;
use crate::context::Context;
use crate::modules;

/// Build a test application with an in-memory SQLite database.
async fn test_app() -> (TestServer, SqlitePool) {
    let db = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    // Run migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let config = Config {
        database_url: "sqlite::memory:".into(),
        host: "127.0.0.1".into(),
        port: 0,
    };

    let context = Arc::new(Context {
        config,
        db: db.clone(),
    });

    let app = modules::all_routers(context.clone()).0.with_state(context);
    let server = TestServer::new(app);
    (server, db)
}

#[tokio::test]
async fn test_health_check() {
    let (server, _db) = test_app().await;

    let response = server.get("/api/proxy/health").await;
    assert_eq!(response.status_code(), 200);

    let body = response.json::<serde_json::Value>();
    assert_eq!(body["status"], "ok");
}

/// Tests that profile list returns empty array when no profiles exist.
#[tokio::test]
async fn test_list_profiles_empty() {
    let (server, _db) = test_app().await;

    let response = server.get("/api/proxy/profiles").await;
    assert_eq!(response.status_code(), 200);

    let body = response.json::<serde_json::Value>();
    assert!(body.as_array().unwrap().is_empty());
}
