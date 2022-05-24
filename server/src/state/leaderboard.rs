use acm::models::LeaderboardItem;

use super::State;

impl State {
    pub async fn first_place_submissions(&self) -> Vec<LeaderboardItem> {
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
        .fetch_all(&self.conn)
        .await
        .unwrap()
    }
}
