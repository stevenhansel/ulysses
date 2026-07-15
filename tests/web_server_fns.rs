//! Server function tests for Leptos `#[server]` functions.
//!
//! These tests exercise the server-side implementation of `#[server]` functions
//! by setting up a reactive owner, providing an in-memory `AppState` via
//! `provide_context`, and calling the server function directly.
//!
//! This validates the full `#[server]` → service → repository → `SQLite` chain.

use std::sync::Arc;

use leptos::prelude::*;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

use ulysses::config::Config;
use ulysses::context::{AppState, Context};
use ulysses::web::server_fns;

/// Build an in-memory `SQLite` database with migrations applied.
async fn test_db() -> SqlitePool {
    let db = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("failed to create in-memory database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run migrations");

    db
}

/// Build an `AppState` with an in-memory database for testing.
async fn test_app_state() -> AppState {
    let db = test_db().await;

    let config = Config {
        database_url: "sqlite::memory:".into(),
        host: "127.0.0.1".into(),
        port: 0,
    };

    let context = Arc::new(Context {
        config,
        db: db.clone(),
    });

    AppState {
        context,
        #[cfg(feature = "ssr")]
        leptos_options: leptos::config::LeptosOptions::builder()
            .output_name("ulysses")
            .site_pkg_dir("pkg")
            .build(),
    }
}

/// Run an async test inside a fresh reactive owner scope with the given
/// `AppState` provided as Leptos context.
async fn with_app_state<T, F>(state: AppState, f: F) -> T
where
    F: std::future::Future<Output = T> + Send,
    T: Send,
{
    let owner = Owner::new();
    owner.set();
    provide_context(state);
    let result = f.await;
    owner.unset();
    result
}

// ---------------------------------------------------------------------------
// get_profiles
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_profiles_empty() {
    let state = test_app_state().await;
    let result = with_app_state(state, server_fns::get_profiles()).await;

    let profiles = result.expect("get_profiles should succeed");
    assert!(
        profiles.is_empty(),
        "expected no profiles in empty database"
    );
}

#[tokio::test]
async fn test_get_profiles_returns_profiles() {
    let state = test_app_state().await;

    // Seed a profile directly into the database
    sqlx::query(
        "INSERT INTO profiles (id, name, backend_url, active) VALUES (?, ?, ?, ?)",
    )
    .bind("test-id-1")
    .bind("Test Profile")
    .bind("http://localhost:8080")
    .bind(true)
    .execute(&state.context.db)
    .await
    .expect("failed to seed test profile");

    let result = with_app_state(state, server_fns::get_profiles()).await;

    let profiles = result.expect("get_profiles should succeed");
    assert_eq!(profiles.len(), 1, "expected one profile");

    assert_eq!(
        profiles[0].id,
        "test-id-1",
        "expected matching profile id"
    );
    assert_eq!(
        profiles[0].name,
        "Test Profile",
        "expected matching profile name"
    );
    assert_eq!(
        profiles[0].backend_url,
        "http://localhost:8080",
        "expected matching backend_url"
    );
    assert!(
        profiles[0].active,
        "expected profile to be active"
    );
}

#[tokio::test]
async fn test_get_profiles_returns_multiple_profiles() {
    let state = test_app_state().await;

    // Seed multiple profiles
    let profiles_in = vec![
        ("id-a", "Alpha", "http://alpha:8080", true),
        ("id-b", "Beta", "http://beta:8080", false),
        ("id-c", "Gamma", "http://gamma:8080", true),
    ];

    for (id, name, url, active) in &profiles_in {
        sqlx::query(
            "INSERT INTO profiles (id, name, backend_url, active) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(url)
        .bind(active)
        .execute(&state.context.db)
        .await
        .expect("failed to seed test profile");
    }

    let result = with_app_state(state, server_fns::get_profiles()).await;
    let profiles = result.expect("get_profiles should succeed");

    assert_eq!(profiles.len(), 3, "expected three profiles");

    // Check that the data matches (order may vary)
    let mut names: Vec<&str> = profiles.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(names, vec!["Alpha", "Beta", "Gamma"]);
}
