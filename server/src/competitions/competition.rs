use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Competition;

pub async fn competition(
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Competition>, ServerError> {
    let competition = sqlx::query_as!(
        Competition,
        r#"SELECT * FROM competitions WHERE id = ?"#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(competition))
}
