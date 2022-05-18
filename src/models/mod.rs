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
