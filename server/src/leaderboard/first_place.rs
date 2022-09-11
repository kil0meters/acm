use axum::{Extension, Json};
use sqlx::SqlitePool;

use super::LeaderboardItem;

pub async fn first_place(Extension(pool): Extension<SqlitePool>) -> Json<Vec<LeaderboardItem>> {
    Json(
        sqlx::query_as(
            r#"
        SELECT
            users.name AS name,
            users.username AS username,
            first_places.submission_count AS count
        FROM
        (
            SELECT
                user_id,
                COUNT(user_id) AS submission_count
            FROM
            (
                SELECT
                    user_id,
                    MIN(time)
                FROM
                    submissions
                WHERE
                    success = true
                GROUP BY
                    problem_id
            )
            GROUP BY
                user_id
        )
        AS first_places
        INNER JOIN
            users
        ON
            first_places.user_id = users.id
        ORDER BY
            count DESC
        "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default(),
    )
}
