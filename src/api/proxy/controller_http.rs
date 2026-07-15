use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::context::Context;
use crate::error::AppError;
use crate::types::ProfileResponse;

/// List all configured proxy profiles.
///
/// Returns an array of all profiles registered in the system.
///
/// # Errors
/// Returns `AppError::Database` if the database query fails.
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
    let profiles = crate::api::proxy::service::get_all_profiles(&context.db).await?;

    Ok(Json(profiles))
}
