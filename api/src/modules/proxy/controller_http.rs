use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use serde::Serialize;
use utoipa::ToSchema;

use crate::context::Context;
use crate::error::AppError;

/// Response shape for the profile list endpoint.
#[derive(Serialize, ToSchema)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub backend_url: String,
    pub active: bool,
}

/// List all configured proxy profiles.
///
/// Returns an array of all profiles registered in the system.
#[utoipa::path(
    get,
    path = "/api/proxy/profiles",
    tag = "proxy",
    responses(
        (status = 200, description = "Profiles retrieved successfully", body = Vec<ProfileResponse>),
        (status = 500, description = "Internal server error"),
    ),
)]
pub async fn list_profiles(
    State(context): State<Arc<Context>>,
) -> Result<Json<Vec<ProfileResponse>>, AppError> {
    let profiles = crate::modules::proxy::service::get_all_profiles(&context.db).await?;

    Ok(Json(profiles))
}
