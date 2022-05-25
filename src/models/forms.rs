use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use lazy_static::lazy_static;
#[cfg(feature = "validate")]
use regex::Regex;
#[cfg(feature = "validate")]
use validator::Validate;

use crate::models::{test::Test, User};

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
    pub template: String,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Serialize)]
pub struct RunnerForm {
    pub problem_id: i64,
    pub runner_code: String,
    pub test_code: String,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Serialize)]
pub struct RunTestsForm {
    pub problem_id: i64,
    pub test_code: String,
}
