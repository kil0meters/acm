use axum::{Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::models::test::Test;
use sqlx::SqlitePool;
use tokio::sync::broadcast::Sender;

use super::Problem;
use crate::{auth::Claims, error::ServerError, ws::BroadcastMessage};

#[derive(Deserialize)]
pub struct NewForm {
    title: String,
    description: String,
    reference: String,
    template: String,
    tests: Vec<Test>,
    activity_id: Option<i64>,
    publish_time: Option<NaiveDateTime>,
    competition_id: Option<i64>,
    runtime_multiplier: Option<f64>,
}

#[derive(Serialize)]
pub struct NewBody {
    id: i64,
}

pub async fn new(
    Extension(pool): Extension<SqlitePool>,
    Extension(broadcast): Extension<Sender<BroadcastMessage>>,
    claims: Claims,
    Json(form): Json<NewForm>,
) -> Result<Json<NewBody>, ServerError> {
    claims.validate_officer()?;

    let mut tx = pool.begin().await.map_err(|_| ServerError::InternalError)?;

    let visible = form.publish_time.is_none();

    let problem: Problem = sqlx::query_as(
        r#"
        INSERT INTO problems (
            title,
            description,
            runner,
            reference,
            template,
            activity_id,
            visible,
            publish_time,
            runtime_multiplier,
            competition_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id,
            title,
            description,
            runner,
            template,
            competition_id,
            runtime_multiplier,
            visible,
            difficulty
        "#,
    )
    .bind(form.title)
    .bind(form.description)
    .bind("")
    .bind(form.reference)
    .bind(form.template)
    .bind(form.activity_id)
    .bind(visible)
    .bind(form.publish_time)
    .bind(form.runtime_multiplier)
    .bind(form.competition_id)
    .fetch_one(&mut tx)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    for test in &form.tests {
        let input_string = serde_json::to_string(&test.input).unwrap();
        let expected_output_string = serde_json::to_string(&test.expected_output).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO tests (
                problem_id,
                test_number,
                input,
                expected_output,
                max_runtime
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
            problem.id,
            test.index,
            input_string,
            expected_output_string,
            test.max_fuel
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            log::error!("{e}");
            ServerError::InternalError
        })?;
    }

    // We only immediately broadcast that there's a new problem if its set to publish immediately
    if form.publish_time.is_none() {
        broadcast
            .send(BroadcastMessage::NewProblem(problem.clone()))
            .ok();
    }

    tx.commit().await.map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    Ok(Json(NewBody { id: problem.id }))
}
