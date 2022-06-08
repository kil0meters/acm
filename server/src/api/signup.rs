//! API endpoints relating to user authentication

use acm::models::{
    forms::{LoginForm, SignupForm},
    User,
};
use actix_web::{http::StatusCode, post, web::Json, Responder};
use log::{error, info};

use validator::Validate;

use super::{api_error, api_success};
use crate::state::AppState;

// Returns the hash of a password
fn hash_password(username: &str, password: &str) -> String {
    // TODO: This is probably not very secure, should use a random salt stored in the database
    // rather than the username for salt.
    let salted_pass = format!("{}{}", username, password);
    bcrypt::hash(salted_pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap()
}

/// Verifies that a password matches a given hash
fn verify_password(username: &str, password: &str, user: &User) -> bool {
    let salted_pass = format!("{}{}", username, password);
    bcrypt::verify(&salted_pass, &user.password).unwrap()
}

/// Signup form.
///
/// **AUTHORIZATION**: Any
#[post("/signup")]
async fn signup(form: Json<SignupForm>, state: AppState) -> impl Responder {
    let mut form = form.into_inner();

    // If the login is not valid, we return an error
    if form.validate().is_err() {
        return api_error(
            StatusCode::BAD_REQUEST,
            "Your username or password is invalid",
        );
    }

    form.password = hash_password(&form.username, &form.password);
    let user = form.into();

    match state.user_add(&user).await {
        Ok(_) => {
            info!("new user created: {:?}", user);
            api_success(state.get_session(user))
        }
        Err(e) => {
            error!("during signup {:?} error: {:?}", user, e);
            api_error(
                StatusCode::CONFLICT,
                "A user with that username already exists",
            )
        }
    }
}

/// Login form.
///
/// **AUTHORIZATION**: Any
#[post("/login")]
async fn login(form: Json<LoginForm>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    match state.get_ref().user_query(&form.username).await {
        Ok(user) => {
            if verify_password(&form.username, &form.password, &user) {
                api_success(state.get_session(user))
            } else {
                api_error(StatusCode::NOT_FOUND, "Incorrect password.")
            }
        }
        Err(e) => {
            error!(
                "during login for username {:?} error {:?}",
                form.username, e
            );
            api_error(StatusCode::NOT_FOUND, "No user exists with that username.")
        }
    }
}
