use acm::models::{forms::EditMeetingForm, Activity, ActivityType, Meeting};
use sqlx::{Sqlite, Transaction};

use super::State;

impl State {
    pub async fn edit_or_insert_meeting(&self, form: &EditMeetingForm) -> sqlx::Result<i64> {
        let mut tx = self.conn.begin().await?;

        if let Some(_id) = form.id {
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
            .fetch_one(&mut tx)
            .await?
            .id;

            Self::insert_activities(&mut tx, id, &form.activities).await?;

            tx.commit().await?;

            Ok(id)
        }
    }

    async fn insert_activities(
        tx: &mut Transaction<'_, Sqlite>,
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
            .execute(&mut *tx)
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
                DATETIME('now', 'localtime', 'start of day') < DATETIME(meeting_time)
            ORDER BY
                meeting_time ASC
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
                DATETIME('now', 'localtime', 'start of day') < DATETIME(meeting_time)
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
