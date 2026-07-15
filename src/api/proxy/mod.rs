use std::sync::Arc;

use crate::context::{AppState, Context};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

pub mod controller_http;
pub mod controller_ws;
pub mod repository;
pub mod service;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

/// Build the proxy module sub-router.
pub fn router(_context: Arc<Context>) -> Router<AppState> {
    Router::new()
        .route("/api/proxy/profiles", get(controller_http::list_profiles))
        .route("/api/proxy/health", get(health_check))
        .route("/api/proxy/ws", get(controller_ws::ws_handler))
}

/// Simple health check endpoint (module-level).
#[utoipa::path(
    get,
    path = "/api/proxy/health",
    tag = "proxy",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    ),
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
}
