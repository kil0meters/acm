use axum::{extract::Path, Extension, Json};
use sqlx::{Row, SqlitePool};

use crate::{
    auth::{Auth, User},
    error::ServerError,
};

use super::Team;

pub async fn joinable(
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Team>>, ServerError> {
    let rows = sqlx::query(
        r#"
        SELECT teams.id, teams.name
        FROM teams LEFT JOIN team_members ON teams.id = team_members.team_id
        WHERE competition_id = ?
        GROUP BY teams.id
        HAVING COUNT(team_members.team_id) < 3
        ORDER BY teams.name"#,
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    let mut teams = vec![];

    for row in rows {
        let name: String = row.get_unchecked("name");
        let id: i64 = row.get_unchecked("id");

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

        teams.push(Team { id, name, members })
    }

    Ok(Json(teams))
}
