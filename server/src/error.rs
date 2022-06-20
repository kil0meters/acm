use acm::models::runner::RunnerError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::ValidationErrors;

pub enum ServerError {
    Auth(AuthError),
    Validation(FormValidationError),
    User(UserError),
    Runner(RunnerError),
    NotFound,
    InternalError,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Auth(err) => err.into_response(),
            ServerError::Validation(err) => err.into_response(),
            ServerError::User(err) => err.into_response(),
            ServerError::Runner(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": err.to_string()})),
            )
                .into_response(),
            ServerError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "The requested data was not found"})),
            )
                .into_response(),
            ServerError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal error"})),
            )
                .into_response(),
        }
    }
}

impl From<RunnerError> for ServerError {
    fn from(inner: RunnerError) -> Self {
        ServerError::Runner(inner)
    }
}

impl From<FormValidationError> for ServerError {
    fn from(inner: FormValidationError) -> Self {
        ServerError::Validation(inner)
    }
}

impl From<ValidationErrors> for ServerError {
    fn from(err: ValidationErrors) -> Self {
        ServerError::Validation(FormValidationError::InvalidField(
            err.errors().iter().nth(0).unwrap().0.to_string(),
        ))
    }
}

impl From<AuthError> for ServerError {
    fn from(inner: AuthError) -> Self {
        ServerError::Auth(inner)
    }
}

impl From<UserError> for ServerError {
    fn from(inner: UserError) -> Self {
        ServerError::User(inner)
    }
}

pub enum UserError {
    NotFound(String),
    InternalError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::NotFound(username) => (
                StatusCode::NOT_FOUND,
                format!("User \"{}\" not found", username),
            ),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch submissions".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub enum FormValidationError {
    InvalidField(String),
}

impl IntoResponse for FormValidationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            FormValidationError::InvalidField(field) => (
                StatusCode::BAD_REQUEST,
                format!("Field '{field}' is invalid"),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InvalidUsername,
    InvalidPassword,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::InvalidUsername => (StatusCode::BAD_REQUEST, "Invalid username"),
            AuthError::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
