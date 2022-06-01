use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use lazy_static::lazy_static;
#[cfg(feature = "validate")]
use regex::Regex;
#[cfg(feature = "validate")]
use validator::Validate;

use crate::models::{test::Test, User};

use super::{Activity, Meeting};

#[cfg(feature = "validate")]
lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"[a-zA-Z0-9!@#$%^&*()\s]{8,256}").unwrap();
    static ref RE_USERNAME: Regex = Regex::new(r"[a-zA-Z0-9]{1,16}").unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct SignupForm {
    pub name: String,
    #[cfg_attr(feature = "validate", validate(regex = "RE_USERNAME"))]
    pub username: String,
    #[cfg_attr(feature = "validate", validate(regex = "RE_PASSWORD"))]
    pub password: String,
}

impl Into<User> for SignupForm {
    fn into(self) -> User {
        User {
            name: self.name,
            username: self.username,
            password: self.password,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct CreateProblemForm {
    pub title: String,
    pub description: String,
    pub runner: String,
    pub reference: String,
    pub template: String,
    pub tests: Vec<Test>,
    pub activity_id: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct RunnerForm {
    pub problem_id: i64,
    pub username: String,
    pub runner: String,
    pub implementation: String,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Serialize)]
pub struct RunTestsForm {
    pub problem_id: i64,
    pub test_code: String,
}

#[derive(Deserialize, Clone, PartialEq, Serialize)]
pub struct EditMeetingForm {
    /// If set to true, the server assumes you wish to update an existing meeting
    pub id: Option<i64>,

    pub title: String,
    pub description: String,
    pub meeting_time: NaiveDateTime,
    pub activities: Vec<Activity>,
}

impl Default for EditMeetingForm {
    fn default() -> Self {
        EditMeetingForm {
            id: None,
            title: String::new(),
            description: String::new(),
            meeting_time: Utc::now().naive_local(),
            activities: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GenerateTestsForm {
    pub runner: String,
    pub reference: String,
    pub username: String,
    pub inputs: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CustomProblemInputForm {
    pub problem_id: i64,
    pub implementation: String,
    pub input: String,
}

// TODO: Make naming less bad
#[derive(Deserialize, Serialize)]
pub struct RunnerCustomProblemInputForm {
    pub problem_id: i64,
    pub runner: String,
    pub username: String,
    pub implementation: String,
    pub reference: String,
    pub input: String,
}
