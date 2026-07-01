use std::sync::Arc;

use axum_test::TestServer;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::config::Config;
use crate::context::Context;
use crate::modules;

/// Helper to build a test app for controller-level tests.
async fn test_server() -> (TestServer, SqlitePool) {
    let db = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let config = Config {
        database_url: "sqlite::memory:".into(),
        host: "127.0.0.1".into(),
        port: 0,
        web_dist: None,
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
async fn test_list_profiles_returns_200() {
    let (server, _db) = test_server().await;

    let response = server.get("/api/proxy/profiles").await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_health_returns_200() {
    let (server, _db) = test_server().await;

    let response = server.get("/api/proxy/health").await;
    assert_eq!(response.status_code(), 200);
}
