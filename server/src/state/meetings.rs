use acm::models::{Activity, ActivityType, Meeting, MeetingActivities};

use super::State;

impl State {
    pub async fn get_future_meetings(&self) -> Vec<Meeting> {
        sqlx::query_as!(
            Meeting,
            r#"
            SELECT
                id,
                title,
                description,
                meeting_time
            FROM
                meetings
            WHERE
                DATETIME('now') < DATETIME(meeting_time)
            "#
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_default()
    }

    pub async fn get_meeting(&self, id: i64) -> Option<Meeting> {
        sqlx::query_as!(
            Meeting,
            r#"
            SELECT
                id,
                title,
                description,
                meeting_time
            FROM
                meetings
            WHERE
                id = ?
            "#,
            id
        )
        .fetch_one(&self.conn)
        .await
        .ok()
    }

    pub async fn get_next_meeting(&self) -> Option<Meeting> {
        sqlx::query_as!(
            Meeting,
            r#"
            SELECT
                id,
                title,
                description,
                meeting_time
            FROM
                meetings
            WHERE
                DATETIME('now') < DATETIME(meeting_time)
            ORDER BY
                DATETIME(meeting_time) ASC
            "#
        )
        .fetch_one(&self.conn)
        .await
        .ok()
    }

    pub async fn get_activities_for_meeting(&self, id: i64) -> Vec<Activity> {
        sqlx::query_as!(
            Activity,
            r#"
            SELECT
                id,
                title,
                description,
                activity_type as "activity_type: ActivityType"
            FROM
                activities
            WHERE
                meeting_id = ?
            "#,
            id
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_default()
    }
}