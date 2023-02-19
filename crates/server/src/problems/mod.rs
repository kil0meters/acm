use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

mod edit;
mod history;
mod index;
mod leaderboard;
mod new;
mod problem;
mod recent_submission;
mod recent_tests;
mod tests;

#[derive(Serialize, Deserialize, Clone, Type)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Clone, FromRow)]
pub struct Problem {
    pub id: i64,

    // A competition id, if the problem is in one.
    pub competition_id: Option<i64>,

    /// Problem title
    pub title: String,

    /// Problem description (markdown formatted)
    pub description: String,

    /// Code that parses standard input and outputs to standard out
    pub runner: String,

    /// Template that's shown when you start a problem
    pub template: String,

    pub visible: bool,

    pub difficulty: Difficulty,
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::problems))
        .route("/new", post(new::new))
        .route("/:problem_id", get(problem::problem))
        .route("/:problem_id/edit", post(edit::edit))
        .route("/:problem_id/tests", get(tests::tests))
        .route("/:problem_id/tests/:test_number", get(tests::problem_test))
        .route("/:problem_id/history", get(history::history))
        .route(
            "/:problem_id/leaderboard/users",
            get(leaderboard::leaderboard_users),
        )
        .route(
            "/:problem_id/leaderboard/submissions",
            get(leaderboard::leaderboard_submissions),
        )
        .route(
            "/:problem_id/recent-submission",
            get(recent_submission::recent_submission),
        )
        .route("/:problem_id/recent-tests", get(recent_tests::recent_tests))
        .route(
            "/:problem_id/recent-tests/:test_number",
            get(recent_tests::recent_tests_test),
        )
}
