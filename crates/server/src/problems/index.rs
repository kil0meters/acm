use axum::{extract::Query, Extension, Json};
use serde::Deserialize;
use sqlx::{Execute, QueryBuilder, Row, SqlitePool};

use crate::{auth::Claims, error::ServerError, pagination::Pagination};

use super::Problem;

#[derive(Deserialize)]
enum ProblemOrdering {
    Newest,
    Oldest,
}

impl Default for ProblemOrdering {
    fn default() -> Self {
        Self::Newest
    }
}

#[derive(Deserialize)]
pub struct ProblemOptions {
    competition_id: Option<i64>,
    show_competition_problems: Option<bool>,

    // 0: Easy
    // 0: Medium
    // 0: Hard
    difficulty: Option<u8>,

    #[serde(default)]
    sort_by: ProblemOrdering,
}

pub async fn problems(
    Extension(pool): Extension<SqlitePool>,
    Query(options): Query<ProblemOptions>,
    Query(pagination): Query<Pagination<0, 10>>,
    claims: Claims,
) -> Result<Json<Vec<Problem>>, ServerError> {
    let is_officer = claims.validate_officer().is_ok();

    let mut query_builder = QueryBuilder::new(
        r#"SELECT id, title, description, runner, template, competition_id, visible, runtime_multiplier, difficulty as "difficulty: Difficulty" FROM problems WHERE "#,
    );

    let mut has_where = false;

    if !is_officer {
        query_builder.push("visible = true");
        has_where = true;
    }

    // This is only required from a bug in sqlx preventing me from upgrading to 0.6. Lovely.
    if let Some(competition_id) = options.competition_id {
        if has_where {
            query_builder.push(" AND ");
        }

        query_builder.push("competition_id = ");
        query_builder.push_bind(competition_id);
        has_where = true;
    }

    if let Some(difficulty) = options.difficulty {
        if difficulty != 0 {
            let mut difficulties = vec![];

            if difficulty & 1 != 0 {
                difficulties.push(r#"difficulty = "Easy""#);
            }

            if difficulty & 2 != 0 {
                difficulties.push(r#"difficulty = "Medium""#);
            }

            if difficulty & 4 != 0 {
                difficulties.push(r#"difficulty = "Hard""#);
            }

            if has_where {
                query_builder.push(" AND ");
            }
            query_builder.push(format!("({})", difficulties.join(" OR ")));
            has_where = true;
        }
    }

    if let Some(true) = options.show_competition_problems {
        if has_where {
            query_builder.push(" AND ");
        }
        query_builder.push("TRUE");
    } else if options.competition_id.is_none() {
        if has_where {
            query_builder.push(" AND ");
        }
        query_builder.push("competition_id IS NULL");
    }

    match options.sort_by {
        ProblemOrdering::Newest => {
            query_builder.push(" ORDER BY create_dt DESC ");
        }
        ProblemOrdering::Oldest => {
            query_builder.push(" ORDER BY create_dt ASC ");
        }
    }

    query_builder.push(" LIMIT ");
    query_builder.push_bind(pagination.count);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(pagination.offset);

    let query = query_builder.build();

    log::info!("{}", query.sql());

    let problems = query
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            log::error!("{e}");
            ServerError::InternalError
        })?
        .iter()
        .map(|row| Problem {
            id: row.get_unchecked("id"),
            competition_id: row.get_unchecked("competition_id"),
            title: row.get_unchecked("title"),
            description: row.get_unchecked("description"),
            runner: row.get_unchecked("runner"),
            runtime_multiplier: row.get_unchecked("runtime_multiplier"),
            template: row.get_unchecked("template"),
            visible: row.get_unchecked("visible"),
            difficulty: row.get_unchecked(r#"difficulty: Difficulty"#),
        })
        .collect();

    Ok(Json(problems))
}
