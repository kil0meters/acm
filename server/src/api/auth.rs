use actix_web::{http::StatusCode, post, web::Json, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use jsonwebtoken::{Header, EncodingKey};
use acm::models::{Auth, User, forms::{SignupForm, LoginForm}};

use crate::state::AppState;
use super::{api_error, api_success};

fn hash_password(username: &str, password: &str) -> String {
    let salted_pass = format!("{}{}", username, password);
    bcrypt::hash(salted_pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap()
}

fn verify_password(username: &str, password: &str, user: &User) -> bool {
    let salted_pass = format!("{}{}", username, password);
    bcrypt::verify(&salted_pass, &user.password).unwrap()
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    auth: Auth,
}

// Somehow this code started looking like Go. Curious.
#[post("/signup")]
async fn signup(form: Json<SignupForm>, state: AppState) -> impl Responder {
    let mut form = form.into_inner();

    // If the login is not valid, we return an error
    if let Err(e) = form.validate() {
        error!("signup {:?} error: {:?}", form, e);
        return api_error(StatusCode::BAD_REQUEST, e);
    }

    form.password = hash_password(&form.username, &form.password);
    let user = form.into();

    match state.get_ref().user_add(&user).await {
        Ok(_) => {
            info!("new user created: {:?}", user);
            api_success(user)
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

#[post("/login")]
async fn login(form: Json<LoginForm>, state: AppState) -> impl Responder {
    let form = form.into_inner();

    match state.get_ref().user_query(&form.username).await {
        Ok(user) => {
            if verify_password(&form.username, &form.password, &user) {
                let claims = Claims {
                    username: user.username.clone(),
                    auth: user.auth,
                };

                let key = state.jwt_private_key.as_bytes();
                let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key)).unwrap();

                api_success(json!({
                    "token": token,
                    "user": user
                }))
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
