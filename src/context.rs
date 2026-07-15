use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::SqlitePool;

use crate::config::Config;

/// Application-wide DI container holding all singleton dependencies.
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: SqlitePool,
}

/// Combined application state for Axum.
///
/// Wraps `Arc<Context>` and (with `ssr` feature) also provides `LeptosOptions`.
/// This newtype exists to work around Rust's orphan rules — we cannot implement
/// `FromRef<Arc<Context>>` for `LeptosOptions` since both are foreign types.
/// But we CAN implement `FromRef<AppState>` for both `Arc<Context>` (context is local)
/// and `LeptosOptions` (`AppState` is local).
#[derive(Clone)]
pub struct AppState {
    pub context: Arc<Context>,
    #[cfg(feature = "ssr")]
    pub leptos_options: leptos::config::LeptosOptions,
}

impl FromRef<AppState> for Arc<Context> {
    fn from_ref(state: &AppState) -> Self {
        state.context.clone()
    }
}

impl FromRef<AppState> for Context {
    fn from_ref(state: &AppState) -> Self {
        (*state.context).clone()
    }
}

#[cfg(feature = "ssr")]
impl FromRef<AppState> for leptos::config::LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
