use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;

use crate::context::Context;

mod proxy;

/// Top-level `OpenAPI` specification for Ulysses API.
///
/// When adding a new module, register its handlers under `paths(...)`
/// and its types under `components(schemas(...))`.
///
/// Note: utoipa generates a `__path_<fn>` module alongside each annotated
/// handler, so paths must use direct function references (not re-exports).
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ulysses API",
        description = "Proxy and model manager for self-hosted LLMs",
        version = "0.1.0",
    ),
    paths(
        crate::modules::proxy::controller_http::list_profiles,
        crate::modules::proxy::health_check,
    ),
    components(schemas(
        crate::modules::proxy::controller_http::ProfileResponse,
        crate::modules::proxy::HealthResponse,
    )),
    tags(
        (name = "proxy", description = "Proxy & model management endpoints"),
    ),
)]
pub struct ApiDoc;

/// Aggregate all module sub-routers and return the `OpenAPI` doc.
#[allow(clippy::needless_pass_by_value)]
pub fn all_routers(context: Arc<Context>) -> (Router<Arc<Context>>, utoipa::openapi::OpenApi) {
    let proxy_router = proxy::router(context.clone());

    let router = Router::new().merge(proxy_router);

    let api = ApiDoc::openapi();

    (router, api)
}
