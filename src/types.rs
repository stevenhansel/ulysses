//! Shared types available to both SSR and WASM builds.
//!
//! These types are used across the server (API) and client (server functions) boundary.
//! They must derive the appropriate serde traits so they work on both sides.
//! The `#[cfg_attr(feature = "ssr", derive(...))]` gates server-only derives
//! (like utoipa) so the types compile for WASM too.

use serde::{Deserialize, Serialize};

/// A proxy profile as returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(utoipa::ToSchema))]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub backend_url: String,
    pub active: bool,
}
