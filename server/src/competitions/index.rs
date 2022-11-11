use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Competition;

pub async fn competitions(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Competition>>, ServerError> {
    let competition = sqlx::query_as!(Competition, r#"SELECT * FROM competitions"#)
        .fetch_all(&pool)
        .await
        .map_err(|_| ServerError::NotFound)?;

    Ok(Json(competition))
}
