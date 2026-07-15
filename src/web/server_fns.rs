//! Leptos server functions — isomorphic RPC between WASM client and SSR server.
//!
//! These `#[server]` macros generate both a client stub (WASM → HTTP fetch) and a
//! server handler (Axum route). They call the existing service/repository layer
//! directly, replacing `TanStack` Query hooks from the React frontend.

use leptos::prelude::*;
use server_fn_macro_default::server;

use crate::types::ProfileResponse;

/// Get all proxy profiles.
///
/// Replaces `apiClient.get<Profile[]>('/profiles')` from the React app.
/// The existing service → repository chain is called directly — no REST
/// endpoint duplication.
///
/// On the server side (ssr), this calls `service::get_all_profiles()`.
/// On the client side (WASM), this generates an HTTP fetch stub.
#[server(GetProfiles, "/api")]
pub async fn get_profiles() -> Result<Vec<ProfileResponse>, ServerFnError> {
    use crate::api::proxy::service;
    use crate::context::AppState;

    let state = expect_context::<AppState>();
    service::get_all_profiles(&state.context.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
