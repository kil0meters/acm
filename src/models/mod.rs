use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlx")]
use sqlx::Type;

pub mod forms;

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


#[cfg(feature = "sqlx")]
#[derive(Debug, Clone, Copy, Type, Deserialize, Serialize, PartialEq)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Auth {
    ADMIN,
    OFFICER,
    MEMBER,
}

#[cfg(not(feature = "sqlx"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
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
