use std::sync::Arc;

use axum::Router;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::SwaggerUi;

use ulysses_api::config::Config;
use ulysses_api::context::Context;
use ulysses_api::modules;

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse config
    let config = Config::from_env().expect("Failed to parse configuration");

    // Ensure the database directory exists
    ensure_db_directory(&config.database_url)
        .expect("Failed to create database directory");

    // Connect to database and run migrations
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run database migrations");

    // Build application context (DI container)
    let context = Arc::new(Context { config: config.clone(), db });

    // Build router and aggregated OpenAPI spec
    let (router, api) = modules::all_routers(context.clone());

    // Attach middleware and Swagger UI
    let app = Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(context);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Ulysses API starting on {}", addr);
    tracing::info!("API docs available at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

/// Parse the SQLite database URL and ensure the parent directory exists.
fn ensure_db_directory(database_url: &str) -> std::io::Result<()> {
    // Strip the `sqlite:` prefix and any query parameters
    let path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(database_url)
        .split('?')
        .next()
        .unwrap_or("");

    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
