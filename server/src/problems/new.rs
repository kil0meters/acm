use acm::models::test::Test;
use axum::{Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::broadcast::Sender;

use super::Problem;
use crate::{auth::Claims, error::ServerError, ws::BroadcastMessage};

#[derive(Deserialize)]
pub struct NewForm {
    title: String,
    description: String,
    runner: String,
    reference: String,
    template: String,
    tests: Vec<Test>,
    activity_id: Option<i64>,
    publish_time: Option<NaiveDateTime>,
    runtime_multiplier: Option<f64>,
}

#[derive(Serialize)]
pub struct NewBody {
    id: i64,
}

pub async fn new(
    Json(form): Json<NewForm>,
    Extension(pool): Extension<SqlitePool>,
    Extension(broadcast): Extension<Sender<BroadcastMessage>>,
    claims: Claims,
) -> Result<Json<NewBody>, ServerError> {
    claims.validate_officer()?;

    let mut tx = pool.begin().await.map_err(|_| ServerError::InternalError)?;

    let visible = form.publish_time.is_none();

    let problem = sqlx::query_as!(
        Problem,
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
            runtime_multiplier
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id,
            title,
            description,
            runner,
            template
        "#,
        form.title,
        form.description,
        form.runner,
        form.reference,
        form.template,
        form.activity_id,
        visible,
        form.publish_time,
        form.runtime_multiplier
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    for test in &form.tests {
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
            test.input,
            test.expected_output,
            test.max_runtime
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
