use sqlx::SqlitePool;

use crate::config::Config;

/// Application-wide DI container holding all singleton dependencies.
///
/// Injected into Axum as `State<Arc<Context>>`. Module-level singletons
/// (queues, active model handles, etc.) are added as fields here.
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: SqlitePool,
}
