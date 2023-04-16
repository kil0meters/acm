use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use shared::models::runner::RunnerError;
use validator::ValidationErrors;

#[derive(Clone, Serialize, Debug, thiserror::Error)]
#[serde(tag = "type")]
pub enum ServerError {
    #[error("{0}")]
    Auth(AuthError),

    #[error("{0}")]
    Validation(FormValidationError),

    #[error("{0}")]
    User(UserError),

    #[error("{0}")]
    Runner(RunnerError),

    #[error("Internal server error.")]
    InternalError,

    #[error("The requested data was not found")]
    NotFound,

    #[error("You are not allowed to do that.")]
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
                Json(json!({"error": self.to_string()})),
            )
                .into_response(),
            ServerError::PermissionDenied => (
                StatusCode::FORBIDDEN,
                Json(json!({"error": self.to_string()})),
            )
                .into_response(),
            ServerError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": self.to_string()})),
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

#[derive(Clone, Serialize, Debug, thiserror::Error)]
pub enum UserError {
    #[error("User {0} not found")]
    NotFound(String),

    #[error("Failed to fetch submissions")]
    InternalError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));
        (status, body).into_response()
    }
}

#[derive(Clone, Serialize, Debug, thiserror::Error)]
pub enum FormValidationError {
    #[error("Invalid field: {0}")]
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

#[derive(Clone, Serialize, Debug, thiserror::Error)]
pub enum AuthError {
    #[error("You must be logged in to do that.")]
    WrongCredentials,

    #[error("Token creation error.")]
    TokenCreation,

    #[error("Invalid token.")]
    InvalidToken,

    #[error("Unauthorized.")]
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => StatusCode::BAD_REQUEST,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        let body = Json(json!({
            "error": self.to_string(),
        }));
        (status, body).into_response()
    }
}
