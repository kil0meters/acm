use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::Type;

pub mod forms;
pub mod runner;
pub mod test;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Session {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub username: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,

    pub auth: Auth,
    pub star_count: i64,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct Problem {
    pub id: i64,

    pub title: String,
    pub description: String,
    pub runner: String,
    pub template: String,

    pub visible: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum Auth {
    ADMIN,
    OFFICER,
    MEMBER,
}

impl Default for Auth {
    fn default() -> Self {
        Auth::MEMBER
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    pub offset: Option<u32>,
    pub count: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Submission {
    pub problem_id: i64,
    pub success: bool,
    pub runtime: i64,
    pub error: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct LeaderboardItem {
    pub username: String,
    pub name: String,
    pub count: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Meeting {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub meeting_time: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ActivityType {
    SOLO,
    PAIR,
    LECT,
}

impl Default for ActivityType {
    fn default() -> Self {
        ActivityType::SOLO
    }
}

#[derive(Deserialize, PartialEq, Serialize, Clone, Default)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub activity_type: ActivityType,
}

#[derive(Deserialize, Serialize)]
pub struct MeetingActivities {
    pub meeting: Meeting,
    pub activities: Vec<Activity>,
}
