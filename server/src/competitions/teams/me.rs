use axum::{extract::Path, Extension, Json};
use sqlx::{Row, SqlitePool};

use crate::{
    auth::{Auth, Claims, User},
    error::ServerError,
};

use super::Team;

pub async fn me(
    claims: Claims,
    Path(competition_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Option<Team>>, ServerError> {
    if let None = claims.validate_logged_in().ok() {
        return Ok(Json(None));
    };

    // get the current id of the u
    let (id, name) = match sqlx::query(
        r#"
        SELECT teams.id, teams.name
        FROM teams JOIN team_members ON teams.id = team_members.team_id
        WHERE user_id = ? AND competition_id = ?
    "#,
    )
    .bind(claims.user_id)
    .bind(competition_id)
    .fetch_one(&pool)
    .await
    {
        Ok(row) => (row.get_unchecked("id"), row.get_unchecked("name")),
        Err(_) => return Ok(Json(None)),
    };

    let members = sqlx::query_as!(
        User,
        r#"
        SELECT
            users.id,
            users.name,
            users.username,
            users.discord_id,
            users.auth as "auth: Auth"
        FROM teams
        JOIN team_members ON teams.id = team_members.team_id
        JOIN users ON team_members.user_id = users.id
        WHERE teams.id = ?
    "#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    println!("{members:?}");

    Ok(Json(Some(Team { id, name, members })))
}
