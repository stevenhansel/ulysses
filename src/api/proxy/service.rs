use sqlx::SqlitePool;

use crate::error::AppError;
use crate::types::ProfileResponse;
use crate::api::proxy::repository;

/// Retrieve all proxy profiles.
///
/// Delegates to the repository layer for data access.
///
/// # Errors
/// Returns `AppError::Database` if the underlying repository query fails.
pub async fn get_all_profiles(db: &SqlitePool) -> Result<Vec<ProfileResponse>, AppError> {
    let records = repository::find_all_profiles(db).await?;

    let profiles = records
        .into_iter()
        .map(|r| ProfileResponse {
            id: r.id,
            name: r.name,
            backend_url: r.backend_url,
            active: r.active,
        })
        .collect();

    Ok(profiles)
}
