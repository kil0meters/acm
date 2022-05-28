use acm::models::{forms::EditMeetingForm, Activity, ActivityType, Meeting};

use super::State;

impl State {
    pub async fn edit_or_insert_meeting(&self, form: &EditMeetingForm) -> sqlx::Result<i64> {
        // TODO: Once again, we need to work on a way to structure transactions since this can
        // cause issues if something fails early.

        if let Some(id) = form.id {
            todo!();
        } else {
            let id = sqlx::query!(
                r#"
                INSERT INTO meetings (
                    title,
                    description,
                    meeting_time
                )
                VALUES (?, ?, ?)
                RETURNING id
                "#,
                form.title,
                form.description,
                form.meeting_time
            )
            .fetch_one(&self.conn)
            .await?
            .id;

            self.insert_activities(id, &form.activities).await?;

            Ok(id)
        }
    }

    pub async fn insert_activities(
        &self,
        meeting_id: i64,
        activities: &Vec<Activity>,
    ) -> sqlx::Result<()> {
        for activity in activities {
            sqlx::query!(
                r#"
                INSERT INTO activities (
                    meeting_id,
                    title,
                    description,
                    activity_type
                )
                VALUES (?, ?, ?, ?)
                "#,
                meeting_id,
                activity.title,
                activity.description,
                activity.activity_type
            )
            .execute(&self.conn)
            .await?;
        }

        Ok(())
    }

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
