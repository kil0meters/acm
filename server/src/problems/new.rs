use acm::models::test::Test;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

#[derive(Deserialize)]
pub struct NewForm {
    title: String,
    description: String,
    runner: String,
    reference: String,
    template: String,
    tests: Vec<Test>,
    activity_id: Option<i64>,
}

#[derive(Serialize)]
pub struct NewBody {
    id: i64,
}

pub async fn new(
    Json(form): Json<NewForm>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<NewBody>, ServerError> {
    claims.validate_officer()?;

    let mut tx = pool.begin().await.map_err(|_| ServerError::InternalError)?;

    let problem_id = sqlx::query!(
        r#"
        INSERT INTO problems (
            title,
            description,
            runner,
            reference,
            template,
            activity_id
        ) VALUES (?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        form.title,
        form.description,
        form.runner,
        form.reference,
        form.template,
        form.activity_id
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|_| ServerError::InternalError)?
    .id;

    for test in &form.tests {
        sqlx::query!(
            r#"
            INSERT INTO tests (
                problem_id,
                test_number,
                input,
                expected_output
            )
            VALUES (?, ?, ?, ?)
            "#,
            problem_id,
            test.index,
            test.input,
            test.expected_output
        )
        .execute(&mut tx)
        .await
        .map_err(|_| ServerError::InternalError)?;
    }

    tx.commit().await.map_err(|_| ServerError::InternalError)?;
    Ok(Json(NewBody { id: problem_id }))
}
