use sqlx::{FromRow, SqlitePool};

use crate::error::AppError;

/// Database row for a proxy profile.
#[derive(Debug, FromRow)]
pub struct ProfileRecord {
    pub id: String,
    pub name: String,
    pub backend_url: String,
    pub active: bool,
}

/// Fetch all profiles from the database.
pub async fn find_all_profiles(db: &SqlitePool) -> Result<Vec<ProfileRecord>, AppError> {
    let records = sqlx::query_as::<_, ProfileRecord>(
        "SELECT id, name, backend_url, active FROM profiles ORDER BY name",
    )
    .fetch_all(db)
    .await?;

    Ok(records)
}
