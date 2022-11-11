use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{auth::Claims, competitions::verify_time_team, error::ServerError};

#[derive(Deserialize)]
pub struct JoinTeamForm {
    team_id: i64,
}

pub async fn join(
    Json(form): Json<JoinTeamForm>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<(), ServerError> {
    claims.validate_logged_in()?;

    if claims.validate_officer().is_err() && !verify_time_team(form.team_id, &pool).await? {
        return Err(ServerError::PermissionDenied);
    }

    // first we must verify that the user is currently not in any other teams in this competition
    let res = sqlx::query!(
        r#"
        SELECT user_id
        FROM team_members
        JOIN teams ON teams.id = team_members.team_id
        WHERE user_id = ? AND competition_id = (SELECT competition_id FROM teams WHERE team_id = ?)
    "#,
        claims.user_id,
        form.team_id
    )
    .fetch_one(&pool)
    .await
    .ok();

    if res.is_some() {
        return Err(ServerError::PermissionDenied);
    }

    sqlx::query!(
        r#"INSERT INTO team_members (user_id, team_id) VALUES (?, ?)"#,
        claims.user_id,
        form.team_id
    )
    .execute(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(())
}
