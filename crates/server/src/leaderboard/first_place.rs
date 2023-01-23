use axum::{Extension, Json};
use sqlx::SqlitePool;

use super::LeaderboardItem;

pub async fn first_place(Extension(pool): Extension<SqlitePool>) -> Json<Vec<LeaderboardItem>> {
    Json(
        sqlx::query_as(
            r#"
        SELECT name, username, count
        FROM (
            SELECT user_id, COUNT(DISTINCT(problem_id)) as count
            FROM submissions
            WHERE success = true
            GROUP by user_id
        ) AS first_places
        JOIN users ON first_places.user_id = users.id
        ORDER BY count DESC
        "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default(),
    )
}
