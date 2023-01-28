use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use shared::models::runner::RunnerError;
use validator::ValidationErrors;

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerError {
    Auth(AuthError),
    Validation(FormValidationError),
    User(UserError),
    Runner(RunnerError),
    InternalError,
    NotFound,
    PermissionDenied,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Auth(err) => err.into_response(),
            ServerError::Validation(err) => err.into_response(),
            ServerError::User(err) => err.into_response(),
            ServerError::Runner(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
            }
            ServerError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "The requested data was not found"})),
            )
                .into_response(),
            ServerError::PermissionDenied => (
                StatusCode::FORBIDDEN,
                Json(json!({"error": "You are not allowed to do that"})),
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

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub enum AuthError {
    WrongCredentials,
    TokenCreation,
    InvalidToken,
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "You must be logged in."),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}