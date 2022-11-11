use axum::{extract::Path, Extension};
use sqlx::SqlitePool;

use crate::{auth::Claims, competitions::verify_time_competition, error::ServerError};

pub async fn leave(
    claims: Claims,
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<(), ServerError> {
    claims.validate_logged_in()?;

    if claims.validate_officer().is_err() && !verify_time_competition(id, &pool).await? {
        return Err(ServerError::PermissionDenied);
    }

    sqlx::query!(
        r#"
        DELETE FROM team_members
        WHERE id IN (
            SELECT team_members.id
            FROM team_members
            JOIN teams ON teams.id = team_members.team_id
            WHERE team_members.user_id = ? AND teams.competition_id = ?
        )"#,
        claims.user_id,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(())
}
