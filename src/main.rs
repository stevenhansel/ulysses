#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::config::LeptosOptions;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, file_and_error_handler, LeptosRoutes};
    use tower_http::cors::CorsLayer;
    use tower_http::services::ServeDir;
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::EnvFilter;
    use utoipa_swagger_ui::SwaggerUi;

    use ulysses::api;
    use ulysses::config::Config;
    use ulysses::context::{AppState, Context};
    use ulysses::web::UlyssesShell;

    // Load .env
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse config
    let config = Config::from_env().expect("Failed to parse configuration");

    // Ensure the database directory exists
    ensure_db_directory(&config.database_url).expect("Failed to create database directory");

    // Connect to database and run migrations
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run database migrations");

    // Build the DI container
    let context = Arc::new(Context {
        config: config.clone(),
        db,
    });

    // Read Leptos configuration from env / defaults
    let leptos_options = leptos::config::get_configuration(None)
        .ok()
        .map_or_else(
            || {
                LeptosOptions::builder()
                    .output_name("ulysses")
                    .site_pkg_dir("pkg")
                    .site_root("site")
                    .build()
            },
            |conf| conf.leptos_options,
        );
    let site_addr = leptos_options.site_addr;

    // Build combined application state
    let state = AppState {
        context: context.clone(),
        leptos_options: leptos_options.clone(),
    };

    // Build REST API router and aggregated OpenAPI spec
    let (router, api_doc) = api::all_routers(context.clone());

    // Clone before capturing into closures
    let leptos_options_for_routes = leptos_options.clone();
    let leptos_options_for_shell = leptos_options.clone();

    // Generate Leptos route list (for SSR)
    let leptos_routes = generate_route_list(move || {
        view! { <UlyssesShell leptos_options=leptos_options_for_routes.clone() /> }
    });

    // Build the full application router:
    //   1. REST API routes (external)
    //   2. Swagger UI at /docs
    //   3. Leptos SSR routes
    //   4. file_and_error_handler fallback (serves static files / SSR fallback)
    let app = Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api_doc))
        .leptos_routes(&state, leptos_routes, {
            let leptos_options = leptos_options_for_shell.clone();
            move || view! { <UlyssesShell leptos_options=leptos_options.clone() /> }
        })
        .nest_service("/pkg", ServeDir::new("pkg"))
        .fallback(file_and_error_handler::<AppState, _>(
            move |options: leptos::config::LeptosOptions| {
                view! { <UlyssesShell leptos_options=options.clone() /> }
            },
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    tracing::info!("Ulysses API starting on http://{}", site_addr);
    let listener = tokio::net::TcpListener::bind(site_addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    eprintln!(
        "The ulysses server binary must be compiled with the 'ssr' feature. \
         Try: cargo run --bin ulysses --features ssr"
    );
}

/// Parse the `SQLite` database URL and ensure the parent directory exists.
#[cfg(feature = "ssr")]
fn ensure_db_directory(database_url: &str) -> std::io::Result<()> {
    let path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(database_url)
        .split('?')
        .next()
        .unwrap_or("");

    if let Some(parent) = std::path::Path::new(path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    Ok(())
}
