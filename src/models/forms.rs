use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::User;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"[a-zA-Z0-9!@#$%^&*()\s]{8,256}").unwrap();
    static ref RE_USERNAME: Regex = Regex::new(r"[a-zA-Z0-9]{1,16}").unwrap();
}
#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct SignupForm {
    pub name: String,
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(regex = "RE_PASSWORD")]
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
